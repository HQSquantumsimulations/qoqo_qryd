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
use ndarray::Array2;
use std::collections::HashMap;
use std::env;

use roqoqo::devices::{Device, GenericDevice};
use roqoqo::operations::*;
use roqoqo::RoqoqoBackendError;

use crate::{tweezer_devices::TweezerDevice, PragmaDeactivateQRydQubit, PragmaShiftQubitsTweezers};

/// Emulator Device
///
#[derive(Debug, PartialEq, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmulatorDevice {
    /// Internal TweezerDevice instance.
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
                available_gates: Some(vec![]),
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
    #[cfg(feature = "web-api")]
    pub fn from_api(
        device_name: Option<String>,
        access_token: Option<String>,
        mock_port: Option<String>,
        seed: Option<usize>,
        dev: Option<bool>,
        api_version: Option<String>,
    ) -> Result<Self, RoqoqoBackendError> {
        // Preparing variables
        let device_name_internal = device_name.unwrap_or_else(|| String::from("qryd_emulator"));
        let api_version = api_version.unwrap_or_else(|| String::from("v1_1"));
        let dev = dev.unwrap_or(false);
        let hqs_env_var = env::var("QRYD_API_HQS").is_ok();
        let access_token_internal: String = if mock_port.is_some() {
            "".to_string()
        } else {
            match access_token {
                Some(s) => s,
                None => env::var("QRYD_API_TOKEN").map_err(|_| {
                    RoqoqoBackendError::MissingAuthentication {
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
                .get(format!("http://127.0.0.1:{}", port))
                .body(device_name_internal.clone())
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?
        } else {
            match (dev, hqs_env_var) {
                (true, true) => client
                    .get(format!(
                        "https://api.qryddemo.itp3.uni-stuttgart.de/{}/devices/{}",
                        api_version, device_name_internal
                    ))
                    .header("X-API-KEY", access_token_internal)
                    .header("X-DEV", "?1")
                    .header("X-HQS", "?1")
                    .send()
                    .map_err(|e| RoqoqoBackendError::NetworkError {
                        msg: format!("{:?}", e),
                    })?,
                (true, false) => client
                    .get(format!(
                        "https://api.qryddemo.itp3.uni-stuttgart.de/{}/devices/{}",
                        api_version, device_name_internal
                    ))
                    .header("X-API-KEY", access_token_internal)
                    .header("X-DEV", "?1")
                    .send()
                    .map_err(|e| RoqoqoBackendError::NetworkError {
                        msg: format!("{:?}", e),
                    })?,
                (false, true) => client
                    .get(format!(
                        "https://api.qryddemo.itp3.uni-stuttgart.de/{}/devices/{}",
                        api_version, device_name_internal
                    ))
                    .header("X-API-KEY", access_token_internal)
                    .header("X-HQS", "?1")
                    .send()
                    .map_err(|e| RoqoqoBackendError::NetworkError {
                        msg: format!("{:?}", e),
                    })?,
                (false, false) => client
                    .get(format!(
                        "https://api.qryddemo.itp3.uni-stuttgart.de/{}/devices/{}",
                        api_version, device_name_internal
                    ))
                    .header("X-API-KEY", access_token_internal)
                    .send()
                    .map_err(|e| RoqoqoBackendError::NetworkError {
                        msg: format!("{:?}", e),
                    })?,
            }
        };

        // Response handling
        let status_code = resp.status();
        if status_code == reqwest::StatusCode::OK {
            let mut device = resp.json::<TweezerDevice>().unwrap();
            if device.layout_register.is_some() {
                return Err(RoqoqoBackendError::NetworkError {
                    msg: "`.from_api()` pulled a TweezerDevice instance incompatible with EmulatorDevice.".to_string(),
                });
            }
            if let Some(new_seed) = seed {
                device.seed = Some(new_seed);
            }
            device.device_name = device_name_internal;
            Ok(EmulatorDevice { internal: device })
        } else {
            Err(RoqoqoBackendError::NetworkError {
                msg: format!(
                    "Request to server failed with HTTP status code {:?}.",
                    status_code
                ),
            })
        }
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

    /// Adds a gate to the available list.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the gate.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The gate has been successfully added to the available ones.
    /// * `Err(RoqoqoBackendError)` - The gate does not exist.
    pub fn add_available_gate(&mut self, hqslang: &str) -> Result<(), RoqoqoBackendError> {
        if !AVAILABLE_GATES_HQSLANG.contains(&hqslang) {
            return Err(RoqoqoBackendError::GenericError {
                msg: format!("Gate '{}' does not exist.", hqslang),
            });
        }
        if let Some(available) = self.internal.available_gates.as_mut() {
            available.push(hqslang.to_string());
        }
        Ok(())
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

    /// Get the names of the available gates in the device.
    ///
    /// # Returns
    ///
    /// * `Vec<&str>` - Vector of the names of the available gates in the device.
    /// * `Err(RoqoqoBackendError)` - The given layout name is not present in the device.
    pub fn get_available_gates_names(&self) -> Result<Vec<&str>, RoqoqoBackendError> {
        if let Some(available) = self.internal.available_gates.as_ref() {
            Ok(available.iter().map(|g| g.as_str()).collect())
        } else {
            Ok(vec![])
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

    /// Returns the number of total tweezer positions in the device.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of tweezer positions in the device.
    pub fn number_tweezer_positions(&self) -> usize {
        if let Some(map) = &self.internal.qubit_to_tweezer {
            return map.len();
        }
        0
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

impl Device for EmulatorDevice {
    fn single_qubit_gate_time(&self, hqslang: &str, _qubit: &usize) -> Option<f64> {
        if let Some(available) = &self.internal.available_gates {
            if available.contains(&hqslang.to_string()) {
                return Some(1.0);
            }
        }
        None
    }

    fn two_qubit_gate_time(&self, hqslang: &str, _control: &usize, _target: &usize) -> Option<f64> {
        if let Some(available) = &self.internal.available_gates {
            if available.contains(&hqslang.to_string()) {
                return Some(1.0);
            }
        }
        None
    }

    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        _control_0: &usize,
        _control_1: &usize,
        _target: &usize,
    ) -> Option<f64> {
        if let Some(available) = &self.internal.available_gates {
            if available.contains(&hqslang.to_string()) {
                return Some(1.0);
            }
        }
        None
    }

    fn multi_qubit_gate_time(&self, hqslang: &str, _qubits: &[usize]) -> Option<f64> {
        if let Some(available) = &self.internal.available_gates {
            if available.contains(&hqslang.to_string()) {
                return Some(1.0);
            }
        }
        None
    }

    #[allow(unused_variables)]
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        self.internal.qubit_decoherence_rates(qubit)
    }

    fn number_qubits(&self) -> usize {
        self.internal.number_qubits()
    }

    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        vec![]
    }

    fn change_device(&mut self, hqslang: &str, operation: &[u8]) -> Result<(), RoqoqoBackendError> {
        match hqslang {
            "PragmaChangeQRydLayout" => Err(RoqoqoBackendError::GenericError {
                msg: "Operation PragmaChangeQRydLayout not supported in EmulatorDevice.".to_string(),
            }),
            "PragmaSwitchDeviceLayout" => Err(RoqoqoBackendError::GenericError {
                msg: "Operation PragmaSwitchDeviceLayout not supported in EmulatorDevice.".to_string(),
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
                        msg: "Wrapped operation not supported in EmulatorDevice".to_string(),
                    }),
                }
            },
            "PragmaShiftQRydQubit" => Err(RoqoqoBackendError::GenericError {
                msg: "Operation PragmaShiftQRydQubit not supported in EmulatorDevice. Please use PragmaShiftQubitsTweezers.".to_string(),
            }),
            "PragmaShiftQubitsTweezers" => {
                let de_shift_qubits_tweezers: Result<
                    PragmaShiftQubitsTweezers,
                    Box<bincode::ErrorKind>,
                > = deserialize(operation);
                match de_shift_qubits_tweezers {
                    Ok(pragma) => {
                        // Check if the there are qubits to move
                        if self.internal.qubit_to_tweezer.is_none() {
                            return Err(RoqoqoBackendError::GenericError {
                                msg: "The device qubit -> tweezer mapping is empty: no qubits to shift.".to_string(),
                            });
                        }
                        // Start applying the shifts
                        if let Some(map) = &mut self.internal.qubit_to_tweezer {
                            for (shift_start, shift_end) in &pragma.shifts {
                                if let Some(qubit_to_move) =
                                    map.iter()
                                        .find_map(|(&qbt, &twz)| if twz == *shift_start { Some(qbt) } else { None })
                                {
                                    // Move the qubit into the new tweezer
                                    map.remove(&qubit_to_move);
                                    map.insert(qubit_to_move, *shift_end);
                                }
                            }
                        }
                        Ok(())
                    }
                    Err(_) => Err(RoqoqoBackendError::GenericError {
                        msg: "Wrapped operation not supported in TweezerDevice".to_string(),
                    }),
                }
            },
            _ => Err(RoqoqoBackendError::GenericError {
                msg: "Wrapped operation not supported in TweezerDevice".to_string(),
            }),
        }
    }

    fn to_generic_device(&self) -> GenericDevice {
        let mut new_generic_device = GenericDevice::new(self.number_qubits());
        let single_qubit_gates_names = vec![
            "SingleQubitGate",
            "RotateZ",
            "RotateX",
            "RotateY",
            "PauliX",
            "PauliY",
            "PauliZ",
            "SqrtPauliX",
            "InvSqrtPauliX",
            "Hadamard",
            "SGate",
            "TGate",
            "PhaseShiftState1",
            "PhaseShiftState0",
            "RotateAroundSphericalAxis",
            "RotateXY",
            "GPi",
            "GPi2",
            "Identity",
            "SqrtPauliY",
            "InvSqrtPauliY",
        ];
        let two_qubit_gates_names = vec![
            "CNOT",
            "SWAP",
            "ISwap",
            "FSwap",
            "SqrtISwap",
            "InvSqrtISwap",
            "XY",
            "ControlledPhaseShift",
            "ControlledPauliY",
            "ControlledPauliZ",
            "MolmerSorensenXX",
            "VariableMSXX",
            "GivensRotation",
            "GivensRotationLittleEndian",
            "Qsim",
            "Fsim",
            "SpinInteraction",
            "Bogoliubov",
            "PMInteraction",
            "ComplexPMInteraction",
            "PhaseShiftedControlledZ",
            "PhaseShiftedControlledPhase",
            "ControlledRotateX",
            "ControlledRotateXY",
            "EchoCrossResonance",
        ];

        for single_qubit_gate_name in single_qubit_gates_names {
            for i in 0..self.number_qubits() {
                new_generic_device
                    .set_single_qubit_gate_time(single_qubit_gate_name, i, 1.0)
                    .unwrap();
            }
        }

        for two_qubit_gate_name in two_qubit_gates_names {
            for i in 0..self.number_qubits() {
                for j in 0..self.number_qubits() {
                    if i != j {
                        new_generic_device
                            .set_two_qubit_gate_time(two_qubit_gate_name, i, j, 1.0)
                            .unwrap();
                    }
                }
            }
        }

        for qubit in 0..self.number_qubits() {
            new_generic_device
                .set_qubit_decoherence_rates(qubit, self.qubit_decoherence_rates(&qubit).unwrap())
                .unwrap();
        }

        new_generic_device
    }
}
