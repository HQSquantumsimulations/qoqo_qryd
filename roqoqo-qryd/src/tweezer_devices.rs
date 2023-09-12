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

//! Tweezer QRyd Devices
//!
//! Provides the devices that are used to execute quantum programs with the QRyd backend.
//! QRyd devices can be physical hardware or simulators.

use bincode::deserialize;
use itertools::Itertools;
use ndarray::Array2;
use std::{collections::HashMap, env, str::FromStr};

use roqoqo::{
    devices::{Device, GenericDevice},
    RoqoqoBackendError,
};

use crate::{phi_theta_relation, PragmaChangeQRydLayout, PragmaDeactivateQRydQubit};

/// Tweezer Device
///
#[derive(Debug, PartialEq, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TweezerDevice {
    /// Mapping from qubit to tweezer.
    pub qubit_to_tweezer: Option<HashMap<usize, usize>>,
    /// Register of Layouts.
    pub layout_register: HashMap<String, TweezerLayoutInfo>,
    /// Current Layout.
    pub current_layout: String,
    /// The specific PhaseShiftedControlledZ relation to use.
    pub controlled_z_phase_relation: String,
    /// The specific PhaseShiftedControlledPhase relation to use.
    pub controlled_phase_phase_relation: String,
}

/// Tweezers information relative to a Layout
///
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(from = "TweezerLayoutInfoSerialize")]
#[serde(into = "TweezerLayoutInfoSerialize")]
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

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
struct TweezerLayoutInfoSerialize {
    /// Maps a single-qubit gate name to a tweezer -> time mapping
    tweezer_single_qubit_gate_times: Vec<(String, SingleTweezerTimes)>,
    /// Maps a two-qubit gate name to a (tweezer, tweezer) -> time mapping
    tweezer_two_qubit_gate_times: Vec<(String, TwoTweezersTimes)>,
    /// Maps a three-qubit gate name to a (tweezer, tweezer, tweezer) -> time mapping
    tweezer_three_qubit_gate_times: Vec<(String, ThreeTweezersTimes)>,
    /// Maps a multi-qubit gate name to a Vec<tweezer> -> time mapping
    tweezer_multi_qubit_gate_times: Vec<(String, MultiTweezersTimes)>,
}
type SingleTweezerTimes = Vec<(usize, f64)>;
type TwoTweezersTimes = Vec<((usize, usize), f64)>;
type ThreeTweezersTimes = Vec<((usize, usize, usize), f64)>;
type MultiTweezersTimes = Vec<(Vec<usize>, f64)>;

impl From<TweezerLayoutInfoSerialize> for TweezerLayoutInfo {
    fn from(info: TweezerLayoutInfoSerialize) -> Self {
        let tweezer_single_qubit_gate_times: HashMap<String, HashMap<usize, f64>> = info
            .tweezer_single_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|(i_k, i_v)| (i_k, i_v)).collect()))
            .collect();
        let tweezer_two_qubit_gate_times: HashMap<String, HashMap<(usize, usize), f64>> = info
            .tweezer_two_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|(i_k, i_v)| (i_k, i_v)).collect()))
            .collect();
        let tweezer_three_qubit_gate_times: HashMap<String, HashMap<(usize, usize, usize), f64>> =
            info.tweezer_three_qubit_gate_times
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(|(i_k, i_v)| (i_k, i_v)).collect()))
                .collect();
        let tweezer_multi_qubit_gate_times: HashMap<String, HashMap<Vec<usize>, f64>> = info
            .tweezer_multi_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|(i_k, i_v)| (i_k, i_v)).collect()))
            .collect();

        Self {
            tweezer_single_qubit_gate_times,
            tweezer_two_qubit_gate_times,
            tweezer_three_qubit_gate_times,
            tweezer_multi_qubit_gate_times,
        }
    }
}

impl From<TweezerLayoutInfo> for TweezerLayoutInfoSerialize {
    fn from(info: TweezerLayoutInfo) -> Self {
        let tweezer_single_qubit_gate_times: Vec<(String, SingleTweezerTimes)> = info
            .tweezer_single_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|(i_k, i_v)| (i_k, i_v)).collect()))
            .collect();
        let tweezer_two_qubit_gate_times: Vec<(String, TwoTweezersTimes)> = info
            .tweezer_two_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|(i_k, i_v)| (i_k, i_v)).collect()))
            .collect();
        let tweezer_three_qubit_gate_times: Vec<(String, ThreeTweezersTimes)> = info
            .tweezer_three_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|(i_k, i_v)| (i_k, i_v)).collect()))
            .collect();
        let tweezer_multi_qubit_gate_times: Vec<(String, MultiTweezersTimes)> = info
            .tweezer_multi_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|(i_k, i_v)| (i_k, i_v)).collect()))
            .collect();
        Self {
            tweezer_single_qubit_gate_times,
            tweezer_two_qubit_gate_times,
            tweezer_three_qubit_gate_times,
            tweezer_multi_qubit_gate_times,
        }
    }
}

impl TweezerDevice {
    /// Creates a new TweezerDevice instance.
    ///
    /// # Arguments
    ///
    /// * `controlled_z_phase_relation` - The relation to use for the PhaseShiftedControlledZ gate.
    ///                                   It can be hardcoded to a specific value if a float is passed in as String.
    /// * `controlled_phase_phase_relation` - The relation to use for the PhaseShiftedControlledPhase gate.
    ///
    /// # Returns
    ///
    /// * `TweezerDevice` - The new TweezerDevice instance.
    pub fn new(
        controlled_z_phase_relation: Option<String>,
        controlled_phase_phase_relation: Option<String>,
    ) -> Self {
        let mut layout_register: HashMap<String, TweezerLayoutInfo> = HashMap::new();
        layout_register.insert(String::from("Default"), TweezerLayoutInfo::default());
        let controlled_z_phase_relation =
            controlled_z_phase_relation.unwrap_or_else(|| "DefaultRelation".to_string());
        let controlled_phase_phase_relation =
            controlled_phase_phase_relation.unwrap_or_else(|| "DefaultRelation".to_string());

        TweezerDevice {
            qubit_to_tweezer: None,
            layout_register,
            current_layout: String::from("Default"),
            controlled_z_phase_relation,
            controlled_phase_phase_relation,
        }
    }

    /// Creates a new TweezerDevice instance containing populated tweezer data.
    ///
    /// This requires a valid QRYD_API_TOKEN. Visit `https://thequantumlaend.de/get-access/` to get one.
    ///
    /// # Arguments
    ///
    /// * `device_name` - The name of the device to instantiate. Defaults to "Default".
    /// * `access_token` - An access_token is required to access QRYD hardware and emulators.
    ///                    The access_token can either be given as an argument here
    ///                         or set via the environmental variable `$QRYD_API_TOKEN`.
    /// * `mock_port` - The address of the Mock server, used for testing purposes.
    ///
    /// # Returns
    ///
    /// * `TweezerDevice` - The new TweezerDevice instance with populated tweezer data.
    ///
    /// # Errors
    ///
    /// * `RoqoqoBackendError`
    pub fn from_api(
        device_name: Option<String>,
        access_token: Option<String>,
        mock_port: Option<String>,
    ) -> Result<Self, RoqoqoBackendError> {
        // Preparing variables
        let device_name_internal = device_name.unwrap_or_else(|| String::from("Default"));
        let access_token_internal: String = if mock_port.is_some() {
            "".to_string()
        } else {
            match access_token {
                Some(s) => s,
                None => env::var("QRYD_API_TOKEN").map_err(|_| {
                    RoqoqoBackendError::MissingAuthentification {
                        msg: "QRYD access token is missing.".to_string(),
                    }
                })?,
            }
        };

        // Client setup
        let client = if mock_port.is_some() {
            reqwest::blocking::Client::builder().build().map_err(|x| {
                RoqoqoBackendError::NetworkError {
                    msg: format!("Could not create test client {:?}.", x),
                }
            })?
        } else {
            reqwest::blocking::Client::builder()
                .https_only(true)
                .build()
                .map_err(|x| RoqoqoBackendError::NetworkError {
                    msg: format!("Could not create https client {:?}.", x),
                })?
        };

        // Response gathering
        let resp = if let Some(port) = mock_port {
            client
                .post(format!("http://127.0.0.1:{}", port))
                .body(device_name_internal)
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?
        } else {
            client
                .post("https://api.qryddemo.itp3.uni-stuttgart.de/v2_0/get_device")
                .header("X-API-KEY", access_token_internal)
                .body(device_name_internal)
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?
        };

        // Response handling
        let status_code = resp.status();
        if status_code == reqwest::StatusCode::OK {
            Ok(resp.json::<TweezerDevice>().unwrap())
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
        if self.qubit_to_tweezer.is_none() {
            self.qubit_to_tweezer = Some(self.new_trivial_mapping());
        }
        Ok(())
    }

    /// Returns a vector of all available Layout names.
    ///
    /// # Returns:
    ///
    /// * `Vec<&String>` - The vector of all available Layout names.
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
    /// # Returns
    ///
    /// * `Ok(())` - The qubit -> tweezer mapping has been added/modified.
    /// * `Err(RoqoqoBackendError)` - The tweezer does not exist.
    pub fn add_qubit_tweezer_mapping(
        &mut self,
        qubit: usize,
        tweezer: usize,
    ) -> Result<(), RoqoqoBackendError> {
        if self.is_tweezer_present(tweezer) {
            if let Some(map) = &mut self.qubit_to_tweezer {
                map.insert(qubit, tweezer);
            } else {
                self.qubit_to_tweezer = Some(self.new_trivial_mapping());
                self.qubit_to_tweezer
                    .as_mut()
                    .unwrap()
                    .insert(qubit, tweezer);
            }
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
    pub fn set_tweezer_single_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        self.qubit_to_tweezer = None;
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
    pub fn set_tweezer_two_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer0: usize,
        tweezer1: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        self.qubit_to_tweezer = None;
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
    pub fn set_tweezer_three_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer0: usize,
        tweezer1: usize,
        tweezer2: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        self.qubit_to_tweezer = None;
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
    pub fn set_tweezer_multi_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezers: &[usize],
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        self.qubit_to_tweezer = None;
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
    pub fn get_tweezer_from_qubit(&self, qubit: &usize) -> Result<usize, RoqoqoBackendError> {
        if let Some(map) = &self.qubit_to_tweezer {
            map.get(qubit)
                .ok_or(RoqoqoBackendError::GenericError {
                    msg: "The given qubit is not present in the Layout.".to_string(),
                })
                .copied()
        } else {
            Err(RoqoqoBackendError::GenericError {
                msg: "The device qubit -> tweezer mapping is empty.".to_string(),
            })
        }
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
    pub fn deactivate_qubit(
        &mut self,
        qubit: usize,
    ) -> Result<HashMap<usize, usize>, RoqoqoBackendError> {
        if let Some(map) = &mut self.qubit_to_tweezer {
            if map.remove(&qubit).is_none() {
                Err(RoqoqoBackendError::GenericError {
                    msg: "The given qubit is not present in the Layout.".to_string(),
                })
            } else {
                Ok(map.clone())
            }
        } else {
            Err(RoqoqoBackendError::GenericError {
                msg: "The device qubit -> tweezer mapping is empty.".to_string(),
            })
        }
    }

    /// Returns the PhaseShiftedControlledZ phase shift according to the device's relation.
    ///
    /// # Returns
    ///
    /// * `f64` - The PhaseShiftedControlledZ phase shift.
    pub fn phase_shift_controlled_z(&self) -> Option<f64> {
        if let Ok(phase_shift_value) = f64::from_str(&self.controlled_z_phase_relation) {
            Some(phase_shift_value)
        } else {
            phi_theta_relation(&self.controlled_z_phase_relation, std::f64::consts::PI)
        }
    }

    /// Returns the PhaseShiftedControlledPhase phase shift according to the device's relation.
    ///
    /// # Returns
    ///
    /// * `f64` - The PhaseShiftedControlledPhase phase shift.
    pub fn phase_shift_controlled_phase(&self, theta: f64) -> Option<f64> {
        if let Ok(phase_shift_value) = f64::from_str(&self.controlled_phase_phase_relation) {
            Some(phase_shift_value)
        } else {
            phi_theta_relation(&self.controlled_phase_phase_relation, theta)
        }
    }

    /// Returns the gate time of a PhaseShiftedControlledZ operation with the given qubits and phi angle.
    ///
    /// # Arguments
    ///
    /// * `control` - The control qubit the gate acts on
    /// * `target` - The target qubit the gate acts on
    /// * `phi` - The phi angle to be checked.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    pub fn gate_time_controlled_z(&self, control: &usize, target: &usize, phi: f64) -> Option<f64> {
        if self
            .two_qubit_gate_time("PhaseShiftedControlledZ", control, target)
            .is_some()
        {
            if let Some(relation_phi) = self.phase_shift_controlled_z() {
                if (relation_phi.abs() - phi.abs()).abs() < 0.0001 {
                    return Some(1e-6);
                }
            }
        }
        None
    }

    /// Returns the gate time of a PhaseShiftedControlledPhase operation with the given qubits and phi and theta angles.
    ///
    /// # Arguments
    ///
    /// * `control` - The control qubit the gate acts on
    /// * `target` - The target qubit the gate acts on
    /// * `phi` - The phi angle to be checked.
    /// * `theta` - The theta angle to be checked.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    pub fn gate_time_controlled_phase(
        &self,
        control: &usize,
        target: &usize,
        phi: f64,
        theta: f64,
    ) -> Option<f64> {
        if self
            .two_qubit_gate_time("PhaseShiftedControlledPhase", control, target)
            .is_some()
        {
            if let Some(relation_phi) = self.phase_shift_controlled_phase(theta) {
                if (relation_phi.abs() - phi.abs()).abs() < 0.0001 {
                    return Some(1e-6);
                }
            }
        }
        None
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

    fn max_tweezer(&self) -> Option<usize> {
        let tweezer_info = self.get_current_layout_info();
        let mut max_tweezer_id: Option<usize> = None;

        for single_qubit_struct in &tweezer_info.tweezer_single_qubit_gate_times {
            if let Some(max) = single_qubit_struct.1.keys().max() {
                if let Some(current_max) = max_tweezer_id {
                    max_tweezer_id = Some(*max.max(&current_max));
                } else {
                    max_tweezer_id = Some(*max);
                }
            }
        }
        for two_qubit_struct in &tweezer_info.tweezer_two_qubit_gate_times {
            if let Some(max) = two_qubit_struct
                .1
                .keys()
                .flat_map(|&(a, b)| vec![a, b])
                .max()
            {
                if let Some(current_max) = max_tweezer_id {
                    max_tweezer_id = Some(max.max(current_max));
                } else {
                    max_tweezer_id = Some(max);
                }
            }
        }
        for three_qubit_struct in &tweezer_info.tweezer_three_qubit_gate_times {
            if let Some(max) = three_qubit_struct
                .1
                .keys()
                .flat_map(|&(a, b, c)| vec![a, b, c])
                .max()
            {
                if let Some(current_max) = max_tweezer_id {
                    max_tweezer_id = Some(max.max(current_max));
                } else {
                    max_tweezer_id = Some(max);
                }
            }
        }
        for multi_qubit_struct in &tweezer_info.tweezer_multi_qubit_gate_times {
            if let Some(max) = multi_qubit_struct.1.keys().flatten().max() {
                if let Some(current_max) = max_tweezer_id {
                    max_tweezer_id = Some(*max.max(&current_max));
                } else {
                    max_tweezer_id = Some(*max);
                };
            }
        }
        max_tweezer_id
    }

    fn new_trivial_mapping(&self) -> HashMap<usize, usize> {
        if let Some(max_tweezer_id) = self.max_tweezer() {
            (0..=max_tweezer_id)
                .map(|i| (i, i))
                .collect::<HashMap<usize, usize>>()
        } else {
            HashMap::new()
        }
    }
}

impl Device for TweezerDevice {
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
        if let Some(map) = &self.qubit_to_tweezer {
            return map.keys().len();
        }
        if let Some(max) = self.max_tweezer() {
            return max + 1;
        }
        0
    }

    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        if let Some(map) = &self.qubit_to_tweezer {
            let mut edges: Vec<(usize, usize)> = Vec::new();
            for qbt0 in map.keys() {
                for qbt1 in map.keys() {
                    if self
                        .two_qubit_gate_time("PhaseShiftedControlledPhase", qbt0, qbt1)
                        .is_some()
                    {
                        edges.push((*qbt0, *qbt1));
                    }
                }
            }
            return edges;
        }
        vec![]
    }

    fn change_device(&mut self, hqslang: &str, operation: &[u8]) -> Result<(), RoqoqoBackendError> {
        match hqslang {
            "PragmaChangeQRydLayout" => {
                let de_change_layout: Result<PragmaChangeQRydLayout, Box<bincode::ErrorKind>> =
                    deserialize(operation);
                match de_change_layout {
                    Ok(pragma) => self.switch_layout(&pragma.new_layout().to_string()), // PROBLEM: PragmaChangeQRydLayout indexes by usize, not str
                    Err(_) => Err(RoqoqoBackendError::GenericError {
                        msg: "Wrapped operation not supported in ExperimentalDevice".to_string(),
                    }),
                }
            }
            "PragmaShiftQRydQubit" => Err(RoqoqoBackendError::GenericError {
                msg: "Operation not supported in ExperimentalDevice".to_string(),
            }),
            "PragmaDeactivateQRydQubit" => {
                let de_change_layout: Result<PragmaDeactivateQRydQubit, Box<bincode::ErrorKind>> =
                    deserialize(operation);
                match de_change_layout {
                    Ok(pragma) => {
                        self.deactivate_qubit(pragma.qubit)?;
                        Ok(())
                    }
                    Err(_) => Err(RoqoqoBackendError::GenericError {
                        msg: "Wrapped operation not supported in ExperimentalDevice".to_string(),
                    }),
                }
            }
            _ => Err(RoqoqoBackendError::GenericError {
                msg: "Wrapped operation not supported in ExperimentalDevice".to_string(),
            }),
        }
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