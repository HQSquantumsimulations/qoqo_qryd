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

use std::collections::HashMap;

use roqoqo::RoqoqoBackendError;

use crate::tweezer_devices::TweezerDevice;

/// Emulator Device
///
#[derive(Debug, PartialEq, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmulatorDevice {
    /// Mapping from qubit to tweezer.
    pub internal: TweezerDevice,
}

impl EmulatorDevice {
    /// Creates a new EmulatorDevice instance.
    ///
    /// # Arguments
    ///
    /// * `seed` - Optional seed, for simulation purposes.
    /// * `controlled_z_phase_relation` - The relation to use for the PhaseShiftedControlledZ gate.
    ///                                   It can be hardcoded to a specific value if a float is passed in as String.
    /// * `controlled_phase_phase_relation` - The relation to use for the PhaseShiftedControlledPhase gate.
    ///
    /// # Returns
    ///
    /// * `EmulatorDevice` - The new EmulatorDevice instance.
    pub fn new(
        seed: Option<usize>,
        controlled_z_phase_relation: Option<String>,
        controlled_phase_phase_relation: Option<String>,
    ) -> Self {
        let controlled_z_phase_relation =
            controlled_z_phase_relation.unwrap_or_else(|| "DefaultRelation".to_string());
        let controlled_phase_phase_relation =
            controlled_phase_phase_relation.unwrap_or_else(|| "DefaultRelation".to_string());

        EmulatorDevice {
            internal: TweezerDevice {
                qubit_to_tweezer: None,
                layout_register: None,
                current_layout: None,
                controlled_z_phase_relation,
                controlled_phase_phase_relation,
                default_layout: None,
                seed,
                allow_reset: false,
                device_name: String::from("qryd_tweezer_device"),
            },
        }
    }

    /// Creates a new EmulatorDevice instance containing populated tweezer data.
    ///
    /// This requires a valid QRYD_API_TOKEN. Visit `https://thequantumlaend.de/get-access/` to get one.
    ///
    /// # Arguments
    ///
    /// * `device_name` - The name of the device to instantiate. Defaults to "qryd_emulator".
    /// * `access_token` - An access_token is required to access QRYD hardware and emulators.
    ///                    The access_token can either be given as an argument here
    ///                         or set via the environmental variable `$QRYD_API_TOKEN`.
    /// * `mock_port` - The address of the Mock server, used for testing purposes.
    /// * `seed` - Optionally overwrite seed value from downloaded device instance.
    /// * `dev` - The boolean to set the dev header to.
    /// * `api_version` - The version of the QRYD API to use. Defaults to "v1_1".
    ///
    /// # Returns
    ///
    /// * `EmulatorDevice` - The new EmulatorDevice instance with populated tweezer data.
    ///
    /// # Errors
    ///
    /// * `RoqoqoBackendError`
    // #[cfg(feature = "web-api")]
    // pub fn from_api(
    //     device_name: Option<String>,
    //     access_token: Option<String>,
    //     mock_port: Option<String>,
    //     seed: Option<usize>,
    //     dev: Option<bool>,
    //     api_version: Option<String>,
    // ) -> Result<Self, RoqoqoBackendError> {
    //     // Preparing variables
    //     let device_name_internal = device_name.unwrap_or_else(|| String::from("qryd_emulator"));
    //     let api_version = api_version.unwrap_or_else(|| String::from("v1_1"));
    //     let dev = dev.unwrap_or(false);
    //     let hqs_env_var = env::var("QRYD_API_HQS").is_ok();
    //     let access_token_internal: String = if mock_port.is_some() {
    //         "".to_string()
    //     } else {
    //         match access_token {
    //             Some(s) => s,
    //             None => env::var("QRYD_API_TOKEN").map_err(|_| {
    //                 RoqoqoBackendError::MissingAuthentication {
    //                     msg: "QRYD access token is missing.".to_string(),
    //                 }
    //             })?,
    //         }
    //     };

    //     // Client setup
    //     let client = if mock_port.is_some() {
    //         reqwest::blocking::Client::builder().build().map_err(|x| {
    //             RoqoqoBackendError::NetworkError {
    //                 msg: format!("Could not create test client {:?}.", x),
    //             }
    //         })?
    //     } else {
    //         reqwest::blocking::Client::builder()
    //             .https_only(true)
    //             .build()
    //             .map_err(|x| RoqoqoBackendError::NetworkError {
    //                 msg: format!("Could not create https client {:?}.", x),
    //             })?
    //     };

    //     // Response gathering
    //     let resp = if let Some(port) = mock_port {
    //         client
    //             .get(format!("http://127.0.0.1:{}", port))
    //             .body(device_name_internal.clone())
    //             .send()
    //             .map_err(|e| RoqoqoBackendError::NetworkError {
    //                 msg: format!("{:?}", e),
    //             })?
    //     } else {
    //         match (dev, hqs_env_var) {
    //             (true, true) => client
    //                 .get(format!(
    //                     "https://api.qryddemo.itp3.uni-stuttgart.de/{}/devices/{}",
    //                     api_version, device_name_internal
    //                 ))
    //                 .header("X-API-KEY", access_token_internal)
    //                 .header("X-DEV", "?1")
    //                 .header("X-HQS", "?1")
    //                 .send()
    //                 .map_err(|e| RoqoqoBackendError::NetworkError {
    //                     msg: format!("{:?}", e),
    //                 })?,
    //             (true, false) => client
    //                 .get(format!(
    //                     "https://api.qryddemo.itp3.uni-stuttgart.de/{}/devices/{}",
    //                     api_version, device_name_internal
    //                 ))
    //                 .header("X-API-KEY", access_token_internal)
    //                 .header("X-DEV", "?1")
    //                 .send()
    //                 .map_err(|e| RoqoqoBackendError::NetworkError {
    //                     msg: format!("{:?}", e),
    //                 })?,
    //             (false, true) => client
    //                 .get(format!(
    //                     "https://api.qryddemo.itp3.uni-stuttgart.de/{}/devices/{}",
    //                     api_version, device_name_internal
    //                 ))
    //                 .header("X-API-KEY", access_token_internal)
    //                 .header("X-HQS", "?1")
    //                 .send()
    //                 .map_err(|e| RoqoqoBackendError::NetworkError {
    //                     msg: format!("{:?}", e),
    //                 })?,
    //             (false, false) => client
    //                 .get(format!(
    //                     "https://api.qryddemo.itp3.uni-stuttgart.de/{}/devices/{}",
    //                     api_version, device_name_internal
    //                 ))
    //                 .header("X-API-KEY", access_token_internal)
    //                 .send()
    //                 .map_err(|e| RoqoqoBackendError::NetworkError {
    //                     msg: format!("{:?}", e),
    //                 })?,
    //         }
    //     };

    //     // Response handling
    //     let status_code = resp.status();
    //     if status_code == reqwest::StatusCode::OK {
    //         let mut device = resp.json::<EmulatorDevice>().unwrap();
    //         if let Some(default) = device.default_layout.clone() {
    //             device.switch_layout(&default, None).unwrap();
    //         }
    //         if let Some(new_seed) = seed {
    //             device.seed = Some(new_seed);
    //         }
    //         device.device_name = device_name_internal;
    //         Ok(device)
    //     } else {
    //         Err(RoqoqoBackendError::NetworkError {
    //             msg: format!(
    //                 "Request to server failed with HTTP status code {:?}.",
    //                 status_code
    //             ),
    //         })
    //     }
    // }

    /// Adds a new empty Layout to the device's register.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the new Layout to be added to the register.
    pub fn add_layout(&mut self, name: &str) -> Result<(), RoqoqoBackendError> {
        self.internal.add_layout(name)
    }

    /// Switch to a different pre-defined Layout.
    ///
    /// It is updated only if the given Layout name is present in the device's
    /// Layout register. If the qubit -> tweezer mapping is empty, it is
    /// trivially populated by default.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the new Layout.
    /// * `with_trivial_map` - Whether the qubit -> tweezer mapping should be trivially populated. Defaults to true.
    pub fn switch_layout(
        &mut self,
        name: &str,
        with_trivial_map: Option<bool>,
    ) -> Result<(), RoqoqoBackendError> {
        self.internal.switch_layout(name, with_trivial_map)
    }

    /// Returns a vector of all available Layout names.
    ///
    /// # Returns:
    ///
    /// * `Vec<&str>` - The vector of all available Layout names.
    pub fn available_layouts(&self) -> Vec<&str> {
        self.internal.available_layouts()
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
    /// * `Ok(HashMap<usize,usize>)` - The updated qubit -> tweezer mapping.
    /// * `Err(RoqoqoBackendError)` - The tweezer does not exist.
    pub fn add_qubit_tweezer_mapping(
        &mut self,
        qubit: usize,
        tweezer: usize,
    ) -> Result<HashMap<usize, usize>, RoqoqoBackendError> {
        self.internal.add_qubit_tweezer_mapping(qubit, tweezer)
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
    ) -> Result<(), RoqoqoBackendError> {
        self.internal
            .set_tweezer_single_qubit_gate_time(hqslang, tweezer, gate_time, layout_name)
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
    ) -> Result<(), RoqoqoBackendError> {
        self.internal.set_tweezer_two_qubit_gate_time(
            hqslang,
            tweezer0,
            tweezer1,
            gate_time,
            layout_name,
        )
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
    ) -> Result<(), RoqoqoBackendError> {
        self.internal.set_tweezer_three_qubit_gate_time(
            hqslang,
            tweezer0,
            tweezer1,
            tweezer2,
            gate_time,
            layout_name,
        )
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
    ) -> Result<(), RoqoqoBackendError> {
        self.internal
            .set_tweezer_multi_qubit_gate_time(hqslang, tweezers, gate_time, layout_name)
    }

    /// Set the allowed Tweezer shifts of a specified Tweezer.
    ///
    /// The tweezer give the tweezer a qubit can be shifted out of. The values are lists
    /// over the directions the qubit in the tweezer can be shifted into.
    /// The items in the list give the allowed tweezers the qubit can be shifted into in order.
    /// For a list 1,2,3 the qubit can be shifted into tweezer 1, into tweezer 2 if tweezer 1 is not occupied,
    /// and into tweezer 3 if tweezer 1 and 2 are not occupied.
    ///
    /// # Arguments
    ///
    /// * `tweezer` - The index of the tweezer.
    /// * `allowed_shifts` - The allowed Tweezer shifts.
    /// * `layout_name` - The name of the Layout to apply the gate time in. Defaults to the current Layout.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The allowed shifts have been set.
    /// * `Err(RoqoqoBackendError)` - The given shifts are not valid.
    pub fn set_allowed_tweezer_shifts(
        &mut self,
        tweezer: &usize,
        allowed_shifts: &[&[usize]],
        layout_name: Option<String>,
    ) -> Result<(), RoqoqoBackendError> {
        self.internal
            .set_allowed_tweezer_shifts(tweezer, allowed_shifts, layout_name)
    }

    /// Set the allowed Tweezer shifts from a list of tweezers.
    ///
    /// # Arguments
    ///
    /// * `row_shifts` - A list of lists, each representing a row of tweezers.
    /// * `layout_name` - The name of the Layout to apply the gate time in. Defaults to the current Layout.
    pub fn set_allowed_tweezer_shifts_from_rows(
        &mut self,
        row_shifts: &[&[usize]],
        layout_name: Option<String>,
    ) -> Result<(), RoqoqoBackendError> {
        self.internal
            .set_allowed_tweezer_shifts_from_rows(row_shifts, layout_name)
    }

    /// Set the tweezer per row value for a given Layout.
    ///
    /// This is needed for dynamically switching layouts during circuit execution.
    /// Only switching between layouts having the same tweezer per row value is supported.
    ///
    /// # Arguments
    ///
    /// * `tweezers_per_row` - Vector containing the number of tweezers per row to set.
    /// * `layout_name` - The name of the Layout to set the tweezer per row for. Defaults to the current Layout.
    pub fn set_tweezers_per_row(
        &mut self,
        tweezers_per_row: Vec<usize>,
        layout_name: Option<String>,
    ) -> Result<(), RoqoqoBackendError> {
        self.internal
            .set_tweezers_per_row(tweezers_per_row, layout_name)
    }

    /// Set whether the device allows PragmaActiveReset operations or not.
    ///
    /// # Arguments
    ///
    /// * `allow_reset` - Whether the device should allow PragmaActiveReset operations or not.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The device now supports PragmaActiveReset.
    /// * `Err(RoqoqoBackendError)` - The device isn't compatible with PragmaActiveReset.
    pub fn set_allow_reset(&mut self, allow_reset: bool) -> Result<(), RoqoqoBackendError> {
        self.internal.set_allow_reset(allow_reset)
    }

    /// Set the name of the default layout to use and switch to it.
    ///
    /// # Arguments
    ///
    /// * `layout` - The name of the layout to use.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The default layout has been set and switched to.
    /// * `Err(RoqoqoBackendError)` - The given layout name is not present in the layout register.
    pub fn set_default_layout(&mut self, layout: &str) -> Result<(), RoqoqoBackendError> {
        self.internal.set_default_layout(layout)
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
        self.internal.get_tweezer_from_qubit(qubit)
    }

    /// Get the names of the available gates in the given layout.
    ///
    /// # Arguments
    ///
    /// * `layout` - The name of the layout. Defaults to the current Layout.
    ///
    /// # Returns
    ///
    /// * `Vec<&str>` - Vector of the names of the available gates in the given layout.
    /// * `Err(RoqoqoBackendError)` - The given layout name is not present in the layout register.
    pub fn get_available_gates_names(
        &self,
        layout_name: Option<String>,
    ) -> Result<Vec<&str>, RoqoqoBackendError> {
        self.internal.get_available_gates_names(layout_name)
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
        self.internal.deactivate_qubit(qubit)
    }

    /// Returns the PhaseShiftedControlledZ phase shift according to the device's relation.
    ///
    /// # Returns
    ///
    /// * `f64` - The PhaseShiftedControlledZ phase shift.
    pub fn phase_shift_controlled_z(&self) -> Option<f64> {
        self.internal.phase_shift_controlled_z()
    }

    /// Returns the PhaseShiftedControlledPhase phase shift according to the device's relation.
    ///
    /// # Returns
    ///
    /// * `f64` - The PhaseShiftedControlledPhase phase shift.
    pub fn phase_shift_controlled_phase(&self, theta: f64) -> Option<f64> {
        self.internal.phase_shift_controlled_phase(theta)
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
        self.internal.gate_time_controlled_z(control, target, phi)
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
        self.internal
            .gate_time_controlled_phase(control, target, phi, theta)
    }

    /// Returns the two tweezer edges of the device.
    ///
    /// And edge between two tweezer is valid only if the
    /// PhaseShiftedControlledPhase gate can be performed.
    ///
    /// # Returns:
    ///
    /// * `Vec<(usize, usize)>` - The vector containing the edges.
    pub fn two_tweezer_edges(&self) -> Vec<(usize, usize)> {
        self.internal.two_tweezer_edges()
    }

    /// Returns the number of total tweezer positions in the device.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of tweezer positions in the device.
    /// * `layout_name` - The name of the layout to reference. Defaults to the current layout.
    pub fn number_tweezer_positions(
        &self,
        layout_name: Option<String>,
    ) -> Result<usize, RoqoqoBackendError> {
        self.internal.number_tweezer_positions(layout_name)
    }

    /// Returns the seed usized for the API.
    pub fn seed(&self) -> Option<usize> {
        self.internal.seed
    }

    /// Returns the backend associated with the device.
    pub fn qrydbackend(&self) -> String {
        self.internal.device_name.clone()
    }
}
