// Copyright © 2023 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.
//

//! Experimental QRyd Devices
//!
//! Provides the devices that are used to execute quantum programs with the QRyd backend.
//! QRyd devices can be physical hardware or simulators.

use itertools::Itertools;
use ndarray::Array2;
use std::{collections::HashMap, env};

use roqoqo::{
    devices::{Device, GenericDevice},
    RoqoqoBackendError,
};

/// Experimental Device
///
#[derive(Debug, PartialEq, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExperimentalDevice {
    /// Mapping from qubit to tweezer
    pub qubit_to_tweezer: HashMap<usize, usize>, // TODO: starts as optional. when switch layout if none, set it as trivial full based on new layout, otherwise keep it
    /// Register of Layouts
    pub layout_register: HashMap<String, TweezerLayoutInfo>,
    /// Current Layout
    pub current_layout: String,
}

/// Tweezers information relative to a Layout
///
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TweezerLayoutInfo {
    /// Maps a single-qubit gate name to a tweezer -> time mapping
    pub tweezer_single_qubit_gate_times: HashMap<String, HashMap<usize, f64>>,
    /// Maps a two-qubit gate name to a (tweezer, tweezer) -> time mapping
    pub tweezer_two_qubit_gate_times: HashMap<String, HashMap<(usize, usize), f64>>,
    /// Maps a three-qubit gate name to a (tweezer, tweezer, tweezer) -> time mapping
    pub tweezer_three_qubit_gate_times: HashMap<String, HashMap<(usize, usize, usize), f64>>,
    /// Maps a multi-qubit gate name to a Vec<tweezer> -> time mapping
    pub tweezer_multi_qubit_gate_times: HashMap<String, HashMap<Vec<usize>, f64>>,
}

impl ExperimentalDevice {
    /// Creates a new ExperimentalDevice instance.
    ///
    /// # Returns
    ///
    /// * `ExperimentalDevice` - The new ExperimentalDevice instance.
    ///
    pub fn new() -> Self {
        let mut layout_register: HashMap<String, TweezerLayoutInfo> = HashMap::new();
        layout_register.insert(String::from("Default"), TweezerLayoutInfo::default());

        ExperimentalDevice {
            qubit_to_tweezer: HashMap::new(),
            layout_register,
            current_layout: String::from("Default"),
        }
    }

    /// Creates a new ExperimentalDevice instance containing populated tweezer data.
    ///
    /// This requires a valid QRYD_API_TOKEN. Visit `https://thequantumlaend.de/get-access/` to get one.
    ///
    /// # Arguments
    ///
    /// * `device_name` - The name of the device to instantiate. Defaults to "Default".
    /// * `access_token` - An access_token is required to access QRYD hardware and emulators.
    ///                    The access_token can either be given as an argument here
    ///                         or set via the environmental variable `$QRYD_API_TOKEN`.
    ///
    /// # Returns
    ///
    /// * `ExperimentalDevice` - The new ExperimentalDevice instance with populated tweezer data.
    ///
    /// # Errors
    ///
    /// * `RoqoqoBackendError`
    ///
    pub fn from_api(
        device_name: Option<String>,
        access_token: Option<String>,
    ) -> Result<Self, RoqoqoBackendError> {
        let device_name_internal = device_name.unwrap_or_else(|| String::from("Default"));
        let access_token_internal: String = match access_token {
            Some(s) => s,
            None => env::var("QRYD_API_TOKEN").map_err(|_| {
                RoqoqoBackendError::MissingAuthentification {
                    msg: "QRYD access token is missing.".to_string(),
                }
            })?,
        };
        let client = reqwest::blocking::Client::builder()
            .https_only(true)
            .build()
            .map_err(|x| RoqoqoBackendError::NetworkError {
                msg: format!("Could not create https client {:?}.", x),
            })?;
        let resp = client
            .post("https://api.qryddemo.itp3.uni-stuttgart.de/v2_0/get_device")
            .header("X-API-KEY", access_token_internal)
            .body(device_name_internal)
            .send()
            .map_err(|e| RoqoqoBackendError::NetworkError {
                msg: format!("{:?}", e),
            })?;

        let status_code = resp.status();
        if status_code == reqwest::StatusCode::OK {
            Ok(resp.json::<ExperimentalDevice>().unwrap())
        } else {
            Err(RoqoqoBackendError::NetworkError {
                msg: format!(
                    "Request to server failed with HTTP status code {:?}.",
                    status_code
                ),
            })
        }
    }

    /// Adds a new empty Layout to the device's register.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the new Layout to be added to the register.
    ///
    pub fn add_layout(&mut self, name: &str) -> Result<(), RoqoqoBackendError> {
        if self.layout_register.contains_key(name) {
            return Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "Error adding layout to ExperimentalDevice. Layout name {} is already in use in the Layout register.",
                    name,
                ),
            });
        }
        self.layout_register
            .insert(name.to_string(), TweezerLayoutInfo::default());
        Ok(())
    }

    /// Change the current Layout.
    ///
    /// It is updated only if the new Layout is present in the device's
    /// Layout register.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the new Layout.
    ///
    pub fn switch_layout(&mut self, name: &str) -> Result<(), RoqoqoBackendError> {
        if !self.layout_register.keys().contains(&name.to_string()) {
            return Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "Error switching layout of ExperimentalDevice. Layout {} is not set.",
                    name
                ),
            });
        }
        self.current_layout = name.to_string();
        Ok(())
    }

    /// Returns a vector of all available Layout names.
    ///
    /// # Returns:
    ///
    /// * `Vec<&String>` - The vector of all available Layout names.
    ///
    pub fn available_layouts(&self) -> Vec<&str> {
        self.layout_register
            .keys()
            .collect_vec()
            .iter()
            .map(|x| x.as_str())
            .collect()
    }

    /// Modifies the qubit -> tweezer mapping of the device.
    ///
    /// If a qubit -> tweezer mapping is already present, it is overwritten.
    /// Returns an error in the the tweezer does not exist.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The index of the qubit.
    /// * `tweezer` - The index of the tweezer.
    ///
    /// # Returns:
    ///
    /// * `Ok(())` - The qubit -> tweezer mapping has been added/modified.
    /// * `Err(RoqoqoBackendError)` - The tweezer does not exist.
    ///
    pub fn add_qubit_tweezer_mapping(
        &mut self,
        qubit: usize,
        tweezer: usize,
    ) -> Result<(), RoqoqoBackendError> {
        // TODO: call everytime a timing has been set
        if self.is_tweezer_present(tweezer) {
            self.qubit_to_tweezer.insert(qubit, tweezer);
            Ok(())
        } else {
            Err(RoqoqoBackendError::GenericError {
                msg: "The given tweezer is not present in the Layout.".to_string(),
            })
        }
    }

    /// Set the time of a single-qubit gate for a tweezer in a given Layout.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a single-qubit gate.
    /// * `tweezer` - The index of the tweezer.
    /// * `gate_time` - The the gate time for the given gate.
    /// * `layout_name` - The name of the Layout to apply the gate time in. Defaults to the current Layout.
    ///
    pub fn set_tweezer_single_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        // TODO: set mapping to None when any setter is called
        let layout_name = layout_name.unwrap_or_else(|| self.current_layout.clone());

        if let Some(info) = self.layout_register.get_mut(&layout_name) {
            let sqt = &mut info.tweezer_single_qubit_gate_times;
            if let Some(present_hm) = sqt.get_mut(hqslang) {
                present_hm.insert(tweezer, gate_time);
            } else {
                let mut hm = HashMap::new();
                hm.insert(tweezer, gate_time);
                sqt.insert(hqslang.to_string(), hm);
            }
        }
    }

    /// Set the time of a two-qubit gate for a tweezer couple in a given Layout.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a two-qubit gate.
    /// * `tweezer0` - The index of the first tweezer.
    /// * `tweezer1` - The index of the second tweezer.
    /// * `gate_time` - The the gate time for the given gate.
    /// * `layout_name` - The name of the Layout to apply the gate time in. Defaults to the current Layout.
    ///
    pub fn set_tweezer_two_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer0: usize,
        tweezer1: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        let layout_name = layout_name.unwrap_or_else(|| self.current_layout.clone());

        if let Some(info) = self.layout_register.get_mut(&layout_name) {
            let sqt = &mut info.tweezer_two_qubit_gate_times;
            if let Some(present_hm) = sqt.get_mut(hqslang) {
                present_hm.insert((tweezer0, tweezer1), gate_time);
            } else {
                let mut hm = HashMap::new();
                hm.insert((tweezer0, tweezer1), gate_time);
                sqt.insert(hqslang.to_string(), hm);
            }
        }
    }

    /// Set the time of a three-qubit gate for a tweezer trio in a given Layout.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a three-qubit gate.
    /// * `tweezer0` - The index of the first tweezer.
    /// * `tweezer1` - The index of the second tweezer.
    /// * `tweezer2` - The index of the third tweezer.
    /// * `gate_time` - The the gate time for the given gate.
    /// * `layout_name` - The name of the Layout to apply the gate time in. Defaults to the current Layout.
    ///
    pub fn set_tweezer_three_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer0: usize,
        tweezer1: usize,
        tweezer2: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        let layout_name = layout_name.unwrap_or_else(|| self.current_layout.clone());

        if let Some(info) = self.layout_register.get_mut(&layout_name) {
            let sqt = &mut info.tweezer_three_qubit_gate_times;
            if let Some(present_hm) = sqt.get_mut(hqslang) {
                present_hm.insert((tweezer0, tweezer1, tweezer2), gate_time);
            } else {
                let mut hm = HashMap::new();
                hm.insert((tweezer0, tweezer1, tweezer2), gate_time);
                sqt.insert(hqslang.to_string(), hm);
            }
        }
    }

    /// Set the time of a multi-qubit gate for a list of tweezers in a given Layout.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a multi-qubit gate.
    /// * `tweezers` - The list of tweezer indexes.
    /// * `gate_time` - The the gate time for the given gate.
    /// * `layout_name` - The name of the Layout to apply the gate time in. Defaults to the current Layout.
    ///
    pub fn set_tweezer_multi_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezers: &[usize],
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        let layout_name = layout_name.unwrap_or_else(|| self.current_layout.clone());

        if let Some(info) = self.layout_register.get_mut(&layout_name) {
            let sqt = &mut info.tweezer_multi_qubit_gate_times;
            if let Some(present_hm) = sqt.get_mut(hqslang) {
                present_hm.insert(tweezers.to_vec(), gate_time);
            } else {
                let mut hm = HashMap::new();
                hm.insert(tweezers.to_vec(), gate_time);
                sqt.insert(hqslang.to_string(), hm);
            }
        }
    }

    /// Get the tweezer identifier of the given qubit.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The input qubit identifier.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The tweezer identifier relative to the given qubit.
    /// * `Err(RoqoqoBackendError)` - If the qubit identifier is not related to any tweezer.
    ///
    pub fn get_tweezer_from_qubit(&self, qubit: &usize) -> Result<usize, RoqoqoBackendError> {
        self.qubit_to_tweezer
            .get(qubit)
            .ok_or(RoqoqoBackendError::GenericError {
                msg: "The given qubit is not present in the Layout.".to_string(),
            })
            .copied()
    }

    /// Deactivate the given qubit in the device.
    ///
    /// # Arguments
    ///
    /// * `qubit` - The input qubit identifier.
    ///
    /// # Returns
    ///
    /// * `Ok(HashMap<usize,usize>)` - The updated qubit -> tweezer mapping.
    /// * `Err(RoqoqoBackendError)` - If the given qubit identifier is not present in the mapping.
    ///
    pub fn deactivate_qubit(
        &mut self,
        qubit: usize,
    ) -> Result<HashMap<usize, usize>, RoqoqoBackendError> {
        if self.qubit_to_tweezer.remove(&qubit).is_none() {
            Err(RoqoqoBackendError::GenericError {
                msg: "The given qubit is not present in the Layout.".to_string(),
            })
        } else {
            Ok(self.qubit_to_tweezer.clone())
        }
    }

    #[inline]
    fn get_current_layout_info(&self) -> &TweezerLayoutInfo {
        self.layout_register
            .get(&self.current_layout)
            .expect("Unexpectedly did not find current layout. Bug in roqoqo-qryd.")
    }

    fn is_tweezer_present(&self, tweezer: usize) -> bool {
        let tweezer_info = self.get_current_layout_info();
        let mut present: bool = false;
        for single_qubit_gate_struct in &tweezer_info.tweezer_single_qubit_gate_times {
            if single_qubit_gate_struct.1.contains_key(&tweezer) {
                present = true;
            }
        }
        for two_qubit_gate_struct in &tweezer_info.tweezer_two_qubit_gate_times {
            if two_qubit_gate_struct
                .1
                .keys()
                .any(|k| k.0 == tweezer || k.1 == tweezer)
            {
                present = true;
            }
        }
        for three_qubit_gate_struct in &tweezer_info.tweezer_three_qubit_gate_times {
            if three_qubit_gate_struct
                .1
                .keys()
                .any(|k| k.0 == tweezer || k.1 == tweezer || k.2 == tweezer)
            {
                present = true;
            }
        }
        for multi_qubit_gate_struct in &tweezer_info.tweezer_multi_qubit_gate_times {
            if multi_qubit_gate_struct
                .1
                .keys()
                .any(|k| k.contains(&tweezer))
            {
                present = true;
            }
        }
        present
    }
}

impl Device for ExperimentalDevice {
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        let tweezer_layout_info = self.get_current_layout_info();
        let mapped_qubit = self.get_tweezer_from_qubit(qubit).ok()?;

        if let Some(hqslang_map) = tweezer_layout_info
            .tweezer_single_qubit_gate_times
            .get(hqslang)
        {
            return hqslang_map.get(&mapped_qubit).copied();
        }
        None
    }

    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        let tweezer_layout_info = self.get_current_layout_info();
        let mapped_control_qubit = self.get_tweezer_from_qubit(control).ok()?;
        let mapped_target_qubit = self.get_tweezer_from_qubit(target).ok()?;

        if let Some(hqslang_map) = tweezer_layout_info
            .tweezer_two_qubit_gate_times
            .get(hqslang)
        {
            return hqslang_map
                .get(&(mapped_control_qubit, mapped_target_qubit))
                .copied();
        }
        None
    }

    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        control_0: &usize,
        control_1: &usize,
        target: &usize,
    ) -> Option<f64> {
        let tweezer_layout_info = self.get_current_layout_info();
        let mapped_control0_qubit = self.get_tweezer_from_qubit(control_0).ok()?;
        let mapped_control1_qubit = self.get_tweezer_from_qubit(control_1).ok()?;
        let mapped_target_qubit = self.get_tweezer_from_qubit(target).ok()?;

        if let Some(hqslang_map) = tweezer_layout_info
            .tweezer_three_qubit_gate_times
            .get(hqslang)
        {
            return hqslang_map
                .get(&(
                    mapped_control0_qubit,
                    mapped_control1_qubit,
                    mapped_target_qubit,
                ))
                .copied();
        }
        None
    }

    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        let tweezer_layout_info = self.get_current_layout_info();
        let mut mapped_qubits: Vec<usize> = Vec::new();
        for qubit in qubits {
            let mapped_qubit = self.get_tweezer_from_qubit(qubit).ok()?;
            mapped_qubits.push(mapped_qubit);
        }

        if let Some(hqslang_map) = tweezer_layout_info
            .tweezer_multi_qubit_gate_times
            .get(hqslang)
        {
            return hqslang_map.get(&mapped_qubits).copied();
        }
        None
    }

    #[allow(unused_variables)]
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        // At the moment we hard-code a noise free model
        Some(Array2::zeros((3, 3).to_owned()))
    }

    fn number_qubits(&self) -> usize {
        // TODO: to check if this still makes sense with deactivate_qubit()
        // UPDATE: I don't see the problem
        self.qubit_to_tweezer.keys().len()
    }

    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        // TODO: do we keep PSCP as edge-defining gate?
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for qbt0 in self.qubit_to_tweezer.keys() {
            for qbt1 in self.qubit_to_tweezer.keys() {
                if self
                    .two_qubit_gate_time("PhaseShiftedControlledPhase", qbt0, qbt1)
                    .is_some()
                {
                    edges.push((*qbt0, *qbt1));
                }
            }
        }
        edges
    }

    /// Turns Device into GenericDevice.
    ///
    /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
    /// (for example when the interface needs to be serialized).
    ///
    /// # Notes
    ///
    /// GenericDevice uses nested HashMaps to represent the most general device connectivity.
    /// The memory usage will be inefficient for devices with large qubit numbers.
    ///
    /// # Returns
    ///
    /// * `GenericDevice` - The device in generic representation.
    ///
    fn to_generic_device(&self) -> GenericDevice {
        let mut new_generic_device = GenericDevice::new(self.number_qubits());
        let tweezer_info = self.get_current_layout_info();

        for single_qubit_gate_struct in &tweezer_info.tweezer_single_qubit_gate_times {
            let gate_name = single_qubit_gate_struct.0.clone();
            for single_qubit_gate_info in single_qubit_gate_struct.1 {
                new_generic_device
                    .set_single_qubit_gate_time(
                        gate_name.as_str(),
                        *single_qubit_gate_info.0,
                        *single_qubit_gate_info.1,
                    )
                    .unwrap();
            }
        }
        for qubit in 0..self.number_qubits() {
            new_generic_device
                .set_qubit_decoherence_rates(qubit, self.qubit_decoherence_rates(&qubit).unwrap())
                .unwrap();
        }
        for two_qubit_gate_struct in &tweezer_info.tweezer_two_qubit_gate_times {
            let gate_name = two_qubit_gate_struct.0.clone();
            for two_qubit_gate_info in two_qubit_gate_struct.1 {
                new_generic_device
                    .set_two_qubit_gate_time(
                        gate_name.as_str(),
                        two_qubit_gate_info.0 .0,
                        two_qubit_gate_info.0 .1,
                        *two_qubit_gate_info.1,
                    )
                    .unwrap();
            }
        }
        new_generic_device
    }
}
