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
use itertools::{iproduct, Itertools};
use ndarray::Array2;
use std::{
    collections::{HashMap, HashSet},
    env,
    str::FromStr,
};

use crate::{
    phi_theta_relation, PragmaDeactivateQRydQubit, PragmaShiftQubitsTweezers,
    PragmaSwitchDeviceLayout,
};

use image::DynamicImage;
use roqollage::render_typst_str;
use roqoqo::{
    devices::{Device, GenericDevice},
    RoqoqoBackendError, RoqoqoError,
};

/// Native single-qubit gates allowed by the QRyd backend.
pub static ALLOWED_NATIVE_SINGLE_QUBIT_GATES: [&str; 5] = [
    "RotateZ",
    "RotateX",
    "RotateXY",
    "PhaseShiftState0",
    "PhaseShiftState1",
];

/// Native two-qubit gates allowed by the QRyd backend.
pub static ALLOWED_NATIVE_TWO_QUBIT_GATES: [&str; 4] = [
    "ControlledPhaseShift",
    "ControlledPauliZ",
    "PhaseShiftedControlledZ",
    "PhaseShiftedControlledPhase",
];

/// Native three-qubit gates allowed by the QRyd backend.
pub static ALLOWED_NATIVE_THREE_QUBIT_GATES: [&str; 2] = [
    "ControlledControlledPauliZ",
    "ControlledControlledPhaseShift",
];

/// Native multi-qubit gates allowed by the QRyd backend.
pub static ALLOWED_NATIVE_MULTI_QUBIT_GATES: [&str; 0] = [];

/// Tweezer Device
///
#[derive(Debug, PartialEq, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TweezerDevice {
    /// Mapping from qubit to tweezer.
    pub qubit_to_tweezer: Option<HashMap<usize, usize>>,
    /// Register of Layouts.
    pub layout_register: Option<HashMap<String, TweezerLayoutInfo>>,
    /// Current Layout.
    pub current_layout: Option<String>,
    /// The specific PhaseShiftedControlledZ relation to use.
    pub controlled_z_phase_relation: String,
    /// The specific PhaseShiftedControlledPhase relation to use.
    pub controlled_phase_phase_relation: String,
    /// The default layout to use at first intantiation.
    pub default_layout: Option<String>,
    /// Optional seed, for simulation purposes.
    pub(crate) seed: Option<usize>,
    /// Whether to allow PragmaActiveReset operations on the device.
    pub allow_reset: bool,
    /// Device name.
    pub device_name: String,
    /// Available gates (EmulatorDevice).
    #[serde(default)]
    pub available_gates: Option<Vec<String>>,
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
    /// Allowed shifts from one tweezer to others.
    /// The keys give the tweezer a qubit can be shifted out of.
    /// The values are lists over the directions the qubit in the tweezer can be shifted into.
    /// The items in the list give the allowed tweezers the qubit can be shifted into in order.
    /// For a list 1,2,3 the qubit can be shifted into tweezer 1, into tweezer 2 if tweezer 1 is not occupied,
    /// and into tweezer 3 if tweezer 1 and 2 are not occupied.
    pub allowed_tweezer_shifts: HashMap<usize, Vec<Vec<usize>>>,
    /// Specifies how many tweezers per row are present. Dynamic layout switching is only allowed between layouts
    /// having the same number of tweezers per row.
    pub tweezers_per_row: Option<Vec<usize>>,
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
    /// Allowed shifts from one tweezer to others
    allowed_tweezer_shifts: Vec<(usize, Vec<Vec<usize>>)>,
    /// Specifies how many tweezers per row are present.
    tweezers_per_row: Option<Vec<usize>>,
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
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        let tweezer_two_qubit_gate_times: HashMap<String, HashMap<(usize, usize), f64>> = info
            .tweezer_two_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        let tweezer_three_qubit_gate_times: HashMap<String, HashMap<(usize, usize, usize), f64>> =
            info.tweezer_three_qubit_gate_times
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().collect()))
                .collect();
        let tweezer_multi_qubit_gate_times: HashMap<String, HashMap<Vec<usize>, f64>> = info
            .tweezer_multi_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        let allowed_tweezer_shifts: HashMap<usize, Vec<Vec<usize>>> =
            info.allowed_tweezer_shifts.into_iter().collect();
        let tweezers_per_row = info.tweezers_per_row;

        Self {
            tweezer_single_qubit_gate_times,
            tweezer_two_qubit_gate_times,
            tweezer_three_qubit_gate_times,
            tweezer_multi_qubit_gate_times,
            allowed_tweezer_shifts,
            tweezers_per_row,
        }
    }
}

impl From<TweezerLayoutInfo> for TweezerLayoutInfoSerialize {
    fn from(info: TweezerLayoutInfo) -> Self {
        let tweezer_single_qubit_gate_times: Vec<(String, SingleTweezerTimes)> = info
            .tweezer_single_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        let tweezer_two_qubit_gate_times: Vec<(String, TwoTweezersTimes)> = info
            .tweezer_two_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        let tweezer_three_qubit_gate_times: Vec<(String, ThreeTweezersTimes)> = info
            .tweezer_three_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        let tweezer_multi_qubit_gate_times: Vec<(String, MultiTweezersTimes)> = info
            .tweezer_multi_qubit_gate_times
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        let allowed_tweezer_shifts: Vec<(usize, Vec<Vec<usize>>)> =
            info.allowed_tweezer_shifts.into_iter().collect();
        let tweezers_per_row = info.tweezers_per_row;

        Self {
            tweezer_single_qubit_gate_times,
            tweezer_two_qubit_gate_times,
            tweezer_three_qubit_gate_times,
            tweezer_multi_qubit_gate_times,
            allowed_tweezer_shifts,
            tweezers_per_row,
        }
    }
}

impl TweezerDevice {
    /// Creates a new TweezerDevice instance.
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
    /// * `TweezerDevice` - The new TweezerDevice instance.
    pub fn new(
        seed: Option<usize>,
        controlled_z_phase_relation: Option<String>,
        controlled_phase_phase_relation: Option<String>,
    ) -> Self {
        let layout_register: Option<HashMap<String, TweezerLayoutInfo>> = Some(HashMap::new());
        let controlled_z_phase_relation =
            controlled_z_phase_relation.unwrap_or_else(|| "DefaultRelation".to_string());
        let controlled_phase_phase_relation =
            controlled_phase_phase_relation.unwrap_or_else(|| "DefaultRelation".to_string());

        TweezerDevice {
            qubit_to_tweezer: None,
            layout_register,
            current_layout: None,
            controlled_z_phase_relation,
            controlled_phase_phase_relation,
            default_layout: None,
            seed,
            allow_reset: false,
            device_name: String::from("qryd_tweezer_device"),
            available_gates: None,
        }
    }

    /// Creates a new TweezerDevice instance containing populated tweezer data.
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
    /// * `TweezerDevice` - The new TweezerDevice instance with populated tweezer data.
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
            if let Some(default) = device.default_layout.clone() {
                device.switch_layout(&default, None).unwrap();
            }
            if let Some(new_seed) = seed {
                device.seed = Some(new_seed);
            }
            device.device_name = device_name_internal;
            Ok(device)
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
        if let Some(int_register) = &self.layout_register {
            if int_register.contains_key(name) {
                return Err(RoqoqoBackendError::GenericError {
                    msg: format!(
                        "Error adding layout to TweezerDevice. Layout name {} is already in use in the Layout register.",
                        name,
                    ),
                });
            }
            self.layout_register
                .as_mut()
                .unwrap()
                .insert(name.to_string(), TweezerLayoutInfo::default());
        }
        Ok(())
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
        if let Some(int_register) = &self.layout_register {
            if !int_register.keys().contains(&name.to_string()) {
                return Err(RoqoqoBackendError::GenericError {
                    msg: format!(
                        "Error switching layout of TweezerDevice. Layout {} is not set.",
                        name
                    ),
                });
            }
            self.current_layout = Some(name.to_string());
            if self.qubit_to_tweezer.is_none() && with_trivial_map.unwrap_or(true) {
                self.qubit_to_tweezer = Some(self.new_trivial_mapping());
            }
        }
        Ok(())
    }

    /// Returns a vector of all available Layout names.
    ///
    /// # Returns:
    ///
    /// * `Vec<&str>` - The vector of all available Layout names.
    pub fn available_layouts(&self) -> Vec<&str> {
        if let Some(int_register) = &self.layout_register {
            return int_register
                .keys()
                .collect_vec()
                .iter()
                .map(|x| x.as_str())
                .collect();
        }
        vec![]
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
        if !self.is_tweezer_present(tweezer, None) {
            return Err(RoqoqoBackendError::GenericError {
                msg: "The given tweezer is not present in the device Tweezer data.".to_string(),
            });
        }
        if let Some(map) = &mut self.qubit_to_tweezer {
            // Remove the previous qubit present in the tweezer
            if let Some(qubit_to_remove) =
                map.iter()
                    .find_map(|(&qbt, &twz)| if twz == tweezer { Some(qbt) } else { None })
            {
                map.remove(&qubit_to_remove);
            }
            map.insert(qubit, tweezer);
        } else {
            self.qubit_to_tweezer = Some(HashMap::from([(qubit, tweezer)]));
        }
        Ok(self
            .qubit_to_tweezer
            .as_ref()
            .expect("Internal error: qubit_to_tweezer mapping supposed to be Some().")
            .clone())
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
        if !ALLOWED_NATIVE_SINGLE_QUBIT_GATES.contains(&hqslang) {
            return Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "Error setting the gate time of a single-qubit gate. Gate {} is not supported.",
                    hqslang
                ),
            });
        }
        let layout_name = layout_name
            .or_else(|| self.current_layout.as_ref().map(|s| s.to_string()))
            .ok_or_else(|| RoqoqoBackendError::GenericError {
                msg: "No layout name provided and no current layout set.".to_string(),
            })?;
        self.qubit_to_tweezer = None;

        if let Some(info) = self.layout_register.as_mut().unwrap().get_mut(&layout_name) {
            let sqt = &mut info.tweezer_single_qubit_gate_times;
            if let Some(present_hm) = sqt.get_mut(hqslang) {
                present_hm.insert(tweezer, gate_time);
            } else {
                let mut hm = HashMap::new();
                hm.insert(tweezer, gate_time);
                sqt.insert(hqslang.to_string(), hm);
            }
        }
        Ok(())
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
        if !ALLOWED_NATIVE_TWO_QUBIT_GATES.contains(&hqslang) {
            return Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "Error setting the gate time of a two-qubit gate. Gate {} is not supported.",
                    hqslang
                ),
            });
        }
        let layout_name = layout_name
            .or_else(|| self.current_layout.as_ref().map(|s| s.to_string()))
            .ok_or_else(|| RoqoqoBackendError::GenericError {
                msg: "No layout name provided and no current layout set.".to_string(),
            })?;
        self.qubit_to_tweezer = None;

        if let Some(info) = self.layout_register.as_mut().unwrap().get_mut(&layout_name) {
            let sqt = &mut info.tweezer_two_qubit_gate_times;
            if let Some(present_hm) = sqt.get_mut(hqslang) {
                present_hm.insert((tweezer0, tweezer1), gate_time);
            } else {
                let mut hm = HashMap::new();
                hm.insert((tweezer0, tweezer1), gate_time);
                sqt.insert(hqslang.to_string(), hm);
            }
        }
        Ok(())
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
        if !ALLOWED_NATIVE_THREE_QUBIT_GATES.contains(&hqslang) {
            return Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "Error setting the gate time of a three-qubit gate. Gate {} is not supported.",
                    hqslang
                ),
            });
        }
        let layout_name = layout_name
            .or_else(|| self.current_layout.as_ref().map(|s| s.to_string()))
            .ok_or_else(|| RoqoqoBackendError::GenericError {
                msg: "No layout name provided and no current layout set.".to_string(),
            })?;
        self.qubit_to_tweezer = None;

        if let Some(info) = self.layout_register.as_mut().unwrap().get_mut(&layout_name) {
            let sqt = &mut info.tweezer_three_qubit_gate_times;
            if let Some(present_hm) = sqt.get_mut(hqslang) {
                present_hm.insert((tweezer0, tweezer1, tweezer2), gate_time);
            } else {
                let mut hm = HashMap::new();
                hm.insert((tweezer0, tweezer1, tweezer2), gate_time);
                sqt.insert(hqslang.to_string(), hm);
            }
        }
        Ok(())
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
        if !ALLOWED_NATIVE_MULTI_QUBIT_GATES.contains(&hqslang) {
            return Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "Error setting the gate time of a multi-qubit gate. Gate {} is not supported.",
                    hqslang
                ),
            });
        }
        let layout_name = layout_name
            .or_else(|| self.current_layout.as_ref().map(|s| s.to_string()))
            .ok_or_else(|| RoqoqoBackendError::GenericError {
                msg: "No layout name provided and no current layout set.".to_string(),
            })?;
        self.qubit_to_tweezer = None;

        if let Some(info) = self.layout_register.as_mut().unwrap().get_mut(&layout_name) {
            let sqt = &mut info.tweezer_multi_qubit_gate_times;
            if let Some(present_hm) = sqt.get_mut(hqslang) {
                present_hm.insert(tweezers.to_vec(), gate_time);
            } else {
                let mut hm = HashMap::new();
                hm.insert(tweezers.to_vec(), gate_time);
                sqt.insert(hqslang.to_string(), hm);
            }
        }
        Ok(())
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
        let layout_name = layout_name
            .or_else(|| self.current_layout.as_ref().map(|s| s.to_string()))
            .ok_or_else(|| RoqoqoBackendError::GenericError {
                msg: "No layout name provided and no current layout set.".to_string(),
            })?;

        // Check that all the involved tweezers exist
        if !self.is_tweezer_present(*tweezer, Some(layout_name.clone()))
            || allowed_shifts.iter().any(|s| {
                s.iter()
                    .any(|t| !self.is_tweezer_present(*t, Some(layout_name.clone())))
            })
        {
            return Err(RoqoqoBackendError::GenericError {
                msg: "The given tweezer, or shifts tweezers, are not present in the device Tweezer data."
                    .to_string(),
            });
        }
        // Check the input tweezer is not present in the input allowed shifts
        if allowed_shifts
            .iter()
            .any(|shift_list| shift_list.contains(tweezer))
        {
            return Err(RoqoqoBackendError::GenericError {
                msg: "The allowed shifts contain the given tweezer.".to_string(),
            });
        }
        if let Some(info) = self.layout_register.as_mut().unwrap().get_mut(&layout_name) {
            info.allowed_tweezer_shifts
                .entry(*tweezer)
                .or_insert_with(Vec::new)
                .extend(allowed_shifts.iter().map(|&slice| slice.to_vec()));
        }
        Ok(())
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
        let layout_name = layout_name
            .or_else(|| self.current_layout.as_ref().map(|s| s.to_string()))
            .ok_or_else(|| RoqoqoBackendError::GenericError {
                msg: "No layout name provided and no current layout set.".to_string(),
            })?;

        // Check that all the involved tweezers exist
        if row_shifts.iter().any(|row| {
            row.iter()
                .any(|t| !self.is_tweezer_present(*t, Some(layout_name.clone())))
        }) {
            return Err(RoqoqoBackendError::GenericError {
                msg: "A given Tweezer is not present in the device Tweezer data.".to_string(),
            });
        }
        // Check that there are no repetitions in the input shifts
        for row in row_shifts.iter() {
            if row.iter().duplicates().count() > 0 {
                return Err(RoqoqoBackendError::GenericError {
                    msg: "The given Tweezers contain repetitions.".to_string(),
                });
            }
        }

        let allowed_shifts = &mut self
            .layout_register
            .as_mut()
            .unwrap()
            .get_mut(&layout_name)
            .unwrap()
            .allowed_tweezer_shifts;

        // For each row in the input..
        row_shifts.iter().for_each(|row| {
            // ... divide in left, mid (the tweezer) and right parts
            for i in 0..row.len() {
                let (left_slice, mid) = row.split_at(i);
                let mid = mid.first().unwrap_or(&0);
                let mut vec_left = left_slice.to_vec();
                vec_left.reverse();
                let vec_right = &row[i + 1..].to_vec();

                // Insert the left and right side
                let val = allowed_shifts.entry(*mid).or_default();
                if !vec_left.is_empty() && !val.contains(&vec_left) {
                    val.push(vec_left.to_vec());
                }
                if !vec_right.is_empty() && !val.contains(vec_right) {
                    val.push(vec_right.to_vec());
                }
            }
        });

        Ok(())
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
        let layout_name = layout_name
            .or_else(|| self.current_layout.as_ref().map(|s| s.to_string()))
            .ok_or_else(|| RoqoqoBackendError::GenericError {
                msg: "No layout name provided and no current layout set.".to_string(),
            })?;

        if let Some(info) = self.layout_register.as_mut().unwrap().get_mut(&layout_name) {
            info.tweezers_per_row = Some(tweezers_per_row);
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
        self.allow_reset = allow_reset;
        Ok(())
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
        if !self._extract_layout_register()?.contains_key(layout) {
            return Err(RoqoqoBackendError::GenericError {
                msg: "The given layout name is not present in the layout register.".to_string(),
            });
        }
        self.default_layout = Some(layout.to_string());
        self.switch_layout(layout, None)?;
        Ok(())
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
        let layout_name = layout_name
            .or_else(|| self.current_layout.as_ref().map(|s| s.to_string()))
            .ok_or_else(|| RoqoqoBackendError::GenericError {
                msg: "No layout name provided and no current layout set.".to_string(),
            })?;

        let mut names: HashSet<&str> = HashSet::new();
        if let Some(info) = self._extract_layout_register()?.get(&layout_name) {
            let sqg = &info.tweezer_single_qubit_gate_times;
            for name in sqg.keys().by_ref() {
                names.insert(name);
            }

            let dqg = &info.tweezer_two_qubit_gate_times;
            for name in dqg.keys().by_ref() {
                names.insert(name);
            }

            let tqg = &info.tweezer_three_qubit_gate_times;
            for name in tqg.keys().by_ref() {
                names.insert(name);
            }

            let mqg = &info.tweezer_multi_qubit_gate_times;
            for name in mqg.keys().by_ref() {
                names.insert(name);
            }
        }
        Ok(names.into_iter().collect())
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

    /// Returns the two tweezer edges of the device.
    ///
    /// And edge between two tweezer is valid only if the
    /// PhaseShiftedControlledPhase gate can be performed.
    ///
    /// # Returns:
    ///
    /// * `Vec<(usize, usize)>` - The vector containing the edges.
    pub fn two_tweezer_edges(&self) -> Vec<(usize, usize)> {
        let mut edges: Vec<(usize, usize)> = vec![];
        if let Some(hm) = self
            .get_current_layout_info()
            .unwrap()
            .tweezer_two_qubit_gate_times
            .get("PhaseShiftedControlledPhase")
        {
            for ((start_tw, end_tw), _) in hm.iter() {
                edges.push((*start_tw, *end_tw));
            }
        }
        edges
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
        let mut set_tweezer_indices: HashSet<usize> = HashSet::new();
        let tweezer_info = if let Some(layout_name) = layout_name {
            if let Some(tw) = self._extract_layout_register()?.get(&layout_name) {
                tw
            } else {
                return Err(RoqoqoBackendError::GenericError {
                    msg: "The given layout name is not present in the layout register.".to_string(),
                });
            }
        } else {
            self.get_current_layout_info()?
        };
        for single_qubit_gate_struct in &tweezer_info.tweezer_single_qubit_gate_times {
            for tw_id in single_qubit_gate_struct.1.keys() {
                set_tweezer_indices.insert(*tw_id);
            }
        }
        for two_qubit_gate_struct in &tweezer_info.tweezer_two_qubit_gate_times {
            for tw_id in two_qubit_gate_struct.1.keys() {
                set_tweezer_indices.insert(tw_id.0);
                set_tweezer_indices.insert(tw_id.1);
            }
        }
        for three_qubit_gate_struct in &tweezer_info.tweezer_three_qubit_gate_times {
            for tw_id in three_qubit_gate_struct.1.keys() {
                set_tweezer_indices.insert(tw_id.0);
                set_tweezer_indices.insert(tw_id.1);
                set_tweezer_indices.insert(tw_id.2);
            }
        }
        for multi_qubit_gate_struct in &tweezer_info.tweezer_multi_qubit_gate_times {
            for tw_ids in multi_qubit_gate_struct.1.keys() {
                for id in tw_ids.iter() {
                    set_tweezer_indices.insert(*id);
                }
            }
        }

        Ok(set_tweezer_indices.len())
    }

    #[inline]
    fn get_current_layout_info(&self) -> Result<&TweezerLayoutInfo, RoqoqoBackendError> {
        if let Some(current) = &self.current_layout {
            Ok(self
                .layout_register
                .as_ref()
                .unwrap()
                .get(current)
                .expect("Unexpectedly did not find current layout. Bug in roqoqo-qryd."))
        } else {
            Err(RoqoqoBackendError::GenericError {
                msg: "Tried to access current layout info but no current layout is set."
                    .to_string(),
            })
        }
    }

    fn is_tweezer_present(&self, tweezer: usize, layout_name: Option<String>) -> bool {
        // For the EmulatorDevice, the tweezer check must not be performed
        if self.layout_register.is_none() {
            return true;
        }
        let tweezer_info = if let Some(x) = layout_name {
            self.layout_register
                .as_ref()
                .unwrap()
                .get(&x)
                .expect("The specified layout does not exist.")
        } else {
            self.get_current_layout_info().unwrap()
        };
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

    fn max_tweezer(&self) -> Result<Option<usize>, RoqoqoBackendError> {
        let tweezer_info = self.get_current_layout_info()?;
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
        Ok(max_tweezer_id)
    }

    fn new_trivial_mapping(&self) -> HashMap<usize, usize> {
        if let Some(max_tweezer_id) = self.max_tweezer().unwrap() {
            (0..=max_tweezer_id)
                .map(|i| (i, i))
                .collect::<HashMap<usize, usize>>()
        } else {
            HashMap::new()
        }
    }

    fn _extract_layout_register(
        &self,
    ) -> Result<&HashMap<String, TweezerLayoutInfo>, RoqoqoBackendError> {
        match &self.layout_register {
            Some(layout_register) => Ok(layout_register),
            None => Err(RoqoqoBackendError::GenericError {
                msg: "Internal error: layout_register supposed to be Some().".to_string(),
            }),
        }
    }

    fn _are_all_shifts_valid(&mut self, pragma: &PragmaShiftQubitsTweezers) -> bool {
        #[inline]
        fn _is_tweezer_in_shift_lists(tweezer_id: &usize, shift_lists: &[Vec<usize>]) -> bool {
            shift_lists.iter().any(|list| list.contains(tweezer_id))
        }
        #[inline]
        fn _is_tweezer_occupied(qbt_to_twz: &HashMap<usize, usize>, tweezer_id: &usize) -> bool {
            qbt_to_twz.iter().any(|(_, twz)| twz == tweezer_id)
        }
        #[inline]
        fn _is_path_free(
            qbt_to_twz: &HashMap<usize, usize>,
            end_tweezer: &usize,
            shift_lists: &[Vec<usize>],
        ) -> bool {
            let correct_shift_list = shift_lists
                .iter()
                .find(|list| list.contains(end_tweezer))
                .unwrap();
            // Check the path up to the target tweezer
            for el in correct_shift_list
                .iter()
                .take_while(|tw| *tw != end_tweezer)
            {
                if _is_tweezer_occupied(qbt_to_twz, el) {
                    return false;
                }
            }
            // Check the target tweezer itself
            if _is_tweezer_occupied(qbt_to_twz, end_tweezer) {
                return false;
            }
            true
        }
        // Temporary clone: pretending the shift of the qubits in order to understand
        //  if the whole row can indeed be shifted or not
        let mut tmp_qubit_to_tweezer = self.qubit_to_tweezer.clone();
        // Checks for all shifts from pragma:
        // - if the starting tweezer has any valid shifts associated with it in the device
        // - if the ending tweezer is contained in the associated valid shifts
        // - if the device in the starting tweezer position is already occupied
        // - if any tweezer in between the starting and ending tweezers is free (ending included)
        for (shift_start, shift_end) in &pragma.shifts {
            match self
                .get_current_layout_info()
                .unwrap()
                .allowed_tweezer_shifts
                .get(shift_start)
            {
                Some(allowed_shifts) => {
                    if !_is_tweezer_in_shift_lists(shift_end, allowed_shifts)
                        || !_is_tweezer_occupied(
                            tmp_qubit_to_tweezer.as_ref().expect(
                                "Internal error: qubit_to_tweezer mapping supposed to be Some().",
                            ),
                            shift_start,
                        )
                        || !_is_path_free(
                            tmp_qubit_to_tweezer.as_ref().expect(
                                "Internal error: qubit_to_tweezer mapping supposed to be Some().",
                            ),
                            shift_end,
                            allowed_shifts,
                        )
                    {
                        return false;
                    }
                }
                // If no shifts are allowed by the device for this tweezer, then it's not valid
                None => return false,
            }
            // "Faking" the movement of the qubit
            if let Some((key, _)) = tmp_qubit_to_tweezer
                .as_ref()
                .unwrap()
                .iter()
                .find(|&(_, &value)| value == *shift_start)
                .map(|(&key, &value)| (key, value))
            {
                tmp_qubit_to_tweezer.as_mut().unwrap().remove(&key);
                tmp_qubit_to_tweezer
                    .as_mut()
                    .unwrap()
                    .insert(key, *shift_end);
            }
        }

        true
    }

    /// Returns the seed usized for the API.
    pub fn seed(&self) -> Option<usize> {
        self.seed
    }

    /// Returns the backend associated with the device.
    pub fn qrydbackend(&self) -> String {
        self.device_name.clone()
    }

    /// Creates a graph representing a TweezerDevice.
    ///
    /// ## Arguments
    ///
    /// * `device` -  The device to represent.
    ///
    /// ## Returns
    ///
    /// * Ok(DynamicImage) - The representation of the device.
    /// * Err(RoqoqoBackendError) - if there is no layout or an error occurred during the compilation.
    ///
    pub fn draw(
        &self,
        pixels_per_point: Option<f32>,
        draw_shifts: bool,
        file_save_path: &Option<String>,
    ) -> Result<DynamicImage, RoqoqoBackendError> {
        let layout = match &self.layout_register {
            Some(x) => x.get(
                &self
                    .current_layout
                    .clone()
                    .or_else(|| self.default_layout.clone())
                    .unwrap_or_default(),
            ),
            None => {
                return Err(RoqoqoBackendError::GenericError {
                    msg: "Draw method not available for EmulatorDevice.".to_owned(),
                })
            }
        };
        if layout.is_none() {
            return Err(RoqoqoBackendError::GenericError {
                msg: "No layout found for the device.".to_owned(),
            });
        }
        let current_layout = layout.unwrap();
        let nb_tweezers = current_layout
            .tweezer_single_qubit_gate_times
            .values()
            .map(|single_gate_map| single_gate_map.keys().max().unwrap_or(&0_usize))
            .chain(
                current_layout
                    .tweezer_two_qubit_gate_times
                    .values()
                    .map(|vals| {
                        vals.keys()
                            .map(|(key1, key2)| key1.max(key2))
                            .max()
                            .unwrap_or(&0_usize)
                    }),
            )
            .max()
            .unwrap_or(&0_usize)
            .to_owned()
            + 1;
        let mut tweezers_positions = Vec::new();
        let mut edges_map = HashMap::new();
        let nodes = create_nodes(
            nb_tweezers,
            current_layout.tweezers_per_row.clone(),
            &mut tweezers_positions,
            &self.qubit_to_tweezer,
        )?;
        map_edges(
            current_layout.tweezer_two_qubit_gate_times.clone(),
            &mut edges_map,
        )?;
        if draw_shifts {
            map_shifts(
                current_layout.allowed_tweezer_shifts.clone(),
                current_layout.tweezer_two_qubit_gate_times.clone(),
                &mut edges_map,
            )?
        }
        let edges = create_edges(&edges_map, &tweezers_positions)?;
        let mut typst_str = r#"#import "@preview/fletcher:0.5.0" as fletcher: diagram, node, edge
#set page(width: auto, height: auto, margin: 5mm, fill: white)
#show math.equation: set text(font: "Fira Math")

#diagram(
 edge-stroke: 1pt,
 node-stroke: black,
	crossing-thickness: 3,
	node-outset: 3pt,
"#
        .to_owned();

        typst_str.push_str(nodes.as_str());
        typst_str.push_str("\n	{\n");
        typst_str.push_str(edges.as_str());
        typst_str.push_str("\n	}\n)");
        let image = render_typst_str(typst_str, pixels_per_point)?;
        if let Some(file_path) = file_save_path {
            image
                .save(file_path)
                .map_err(|x| RoqoqoBackendError::GenericError {
                    msg: format!("Error during image saving: {x:?}"),
                })?;
        }
        Ok(image)
    }
}

impl Device for TweezerDevice {
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        let tweezer_layout_info = self.get_current_layout_info();
        let mapped_qubit = self.get_tweezer_from_qubit(qubit).ok()?;

        if let Some(hqslang_map) = tweezer_layout_info
            .unwrap()
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
            .unwrap()
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
            .unwrap()
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
            .unwrap()
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
            if map.is_empty() {
                return 0;
            }
            return *map.keys().max().unwrap_or(&0) + 1;
        }
        0
    }

    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        let tw_two_qubit_gate_times = &self
            .get_current_layout_info()
            .expect("Tried to access current layout info but no current layout is set.")
            .tweezer_two_qubit_gate_times;
        if let Some(map) = &self.qubit_to_tweezer {
            let mut edges: Vec<(usize, usize)> = Vec::new();
            for (qbt0, qbt1) in iproduct!(map.keys(), map.keys()) {
                if let (Some(mapped_qbt0), Some(mapped_qbt1)) = (
                    self.get_tweezer_from_qubit(qbt0).ok(),
                    self.get_tweezer_from_qubit(qbt1).ok(),
                ) {
                    if tw_two_qubit_gate_times
                        .values()
                        .any(|times| times.get(&(mapped_qbt0, mapped_qbt1)).is_some())
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
            "PragmaChangeQRydLayout" => Err(RoqoqoBackendError::GenericError {
                msg: "Operation not supported in TweezerDevice. Please use PragmaSwitchDeviceLayout.".to_string(),
            }),
            "PragmaSwitchDeviceLayout" => {
                let de_change_layout: Result<PragmaSwitchDeviceLayout, Box<bincode::ErrorKind>> =
                    deserialize(operation);
                match de_change_layout {
                    Ok(pragma) => {
                        // Check layout existance
                        match self._extract_layout_register()?.get(pragma.new_layout()) {
                            Some(new_layout_tweezer_info) => {
                                // Check layout tweezers per row
                                match (&self.get_current_layout_info()?.tweezers_per_row, &new_layout_tweezer_info.tweezers_per_row) {
                                    (Some(current_tweezers_per_row), Some(new_tweezers_per_row)) => {
                                        // Switch if the number of tweezers per row is the same
                                        if current_tweezers_per_row == new_tweezers_per_row {
                                            self.current_layout = Some(pragma.new_layout().to_string());
                                            Ok(())
                                        } else {
                                            Err(RoqoqoBackendError::GenericError {
                                                msg: format!(
                                                    "Error with dynamic layout switching of TweezerDevice. Current tweezers per row is {:?} but switching to a layout with {:?} tweezers per row.",
                                                    current_tweezers_per_row,
                                                    new_tweezers_per_row,
                                                ),
                                            })
                                        }
                                    },
                                    _ => Err(RoqoqoBackendError::GenericError {
                                        msg: "Error with dynamic layout switching of TweezerDevice. Tweezers per row info missing from current or new layout.".to_string()
                                    })
                                }
                            },
                            None => {
                                Err(RoqoqoBackendError::GenericError {
                                    msg: format!(
                                        "Error with dynamic layout switching of TweezerDevice. Layout {} is not set.",
                                        pragma.new_layout()
                                    ),
                                })
                            },
                        }
                    },
                    Err(_) => Err(RoqoqoBackendError::GenericError {
                        msg: "Wrapped operation not supported in TweezerDevice".to_string(),
                    }),
                }
            },
            "PragmaDeactivateQRydQubit" => {
                let de_change_layout: Result<PragmaDeactivateQRydQubit, Box<bincode::ErrorKind>> =
                    deserialize(operation);
                match de_change_layout {
                    Ok(pragma) => {
                        self.deactivate_qubit(pragma.qubit)?;
                        Ok(())
                    }
                    Err(_) => Err(RoqoqoBackendError::GenericError {
                        msg: "Wrapped operation not supported in TweezerDevice".to_string(),
                    }),
                }
            },
            "PragmaShiftQRydQubit" => Err(RoqoqoBackendError::GenericError {
                msg: "Operation not supported in TweezerDevice. Please use PragmaShiftQubitsTweezers.".to_string(),
            }),
            "PragmaShiftQubitsTweezers" => {
                let de_shift_qubits_tweezers: Result<
                    PragmaShiftQubitsTweezers,
                    Box<bincode::ErrorKind>,
                > = deserialize(operation);
                match de_shift_qubits_tweezers {
                    Ok(pragma) => {
                        // Check if the there are qubits to move
                        if self.qubit_to_tweezer.is_none() {
                            return Err(RoqoqoBackendError::GenericError {
                                msg: "The device qubit -> tweezer mapping is empty: no qubits to shift.".to_string(),
                            });
                        }
                        // Check if the shifts in the operation are valid on the device
                        if !self._are_all_shifts_valid(&pragma) {
                            return Err(RoqoqoBackendError::GenericError {
                                msg: "The PragmaShiftQubitsTweezers operation is not valid on this device."
                                    .to_string(),
                            });
                        }
                        // Start applying the shifts
                        if let Some(map) = &mut self.qubit_to_tweezer {
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
        let tweezer_info = self.get_current_layout_info().unwrap();

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

enum ShiftType {
    None,
    LeftToRight,
    RightToLeft,
    Both,
}

fn create_nodes(
    nb_tweezers: usize,
    tweezers_per_row: Option<Vec<usize>>,
    tweezers_positions: &mut Vec<(usize, usize)>,
    qubit_to_tweezer: &Option<HashMap<usize, usize>>,
) -> Result<String, RoqoqoBackendError> {
    let mut nodes = "".to_owned();
    if tweezers_per_row.is_some()
        && tweezers_per_row
            .clone()
            .unwrap()
            .iter()
            .sum::<usize>()
            .ge(&nb_tweezers)
    {
        let nb_tweezers_per_row = tweezers_per_row.unwrap();
        let mut x = 0;
        let mut y = 0;
        for tweezer in 0..nb_tweezers {
            tweezers_positions.insert(tweezer, (x, y));
            nodes.push_str(&format!(
                "node(({x},{y}), ${tweezer}_t{}, shape: circle),\n",
                qubit_to_tweezer
                    .clone()
                    .map(|qubit_map| {
                        for (qubit, tweez) in qubit_map {
                            if tweez == tweezer {
                                return format!("|{qubit}_q$, radius: 2.3em");
                            }
                        }
                        "$, radius: 1.3em".to_owned()
                    })
                    .unwrap_or("$, radius: 1.3em".to_owned())
            ));
            x += 1;
            if x == nb_tweezers_per_row[y] {
                x = 0;
                y += 1;
            }
        }
    } else {
        return Err(RoqoqoBackendError::RoqoqoError(
            RoqoqoError::MismatchedRegisterDimension {
                dim: tweezers_per_row
                    .map(|tweezers_per_row: Vec<usize>| tweezers_per_row.iter().sum::<usize>())
                    .unwrap_or_default(),
                number_qubits: nb_tweezers,
            },
        ));
    }
    Ok(nodes)
}

fn map_edges(
    tweezer_two_qubit_gate_times: HashMap<String, HashMap<(usize, usize), f64>>,
    edges_map: &mut HashMap<(usize, usize), ShiftType>,
) -> Result<(), RoqoqoBackendError> {
    let mut links: Vec<(usize, usize)> = tweezer_two_qubit_gate_times
        .values()
        .flat_map(|value| value.keys().cloned().collect::<Vec<(usize, usize)>>())
        .collect();
    links.sort();
    links.dedup();
    for &(qb1, qb2) in links.iter() {
        if !edges_map.contains_key(&(qb1, qb2)) && !edges_map.contains_key(&(qb2, qb1)) {
            edges_map.insert((qb1, qb2), ShiftType::None);
        }
    }
    Ok(())
}

fn create_edges(
    edges_map: &HashMap<(usize, usize), ShiftType>,
    tweezers_positions: &[(usize, usize)],
) -> Result<String, RoqoqoBackendError> {
    let mut edges = "".to_owned();
    for (&(qb1, qb2), shift_type) in edges_map.iter() {
        edges.push_str(&format!(
            "   edge(({},{}), ({},{}){})\n",
            tweezers_positions[qb1].0,
            tweezers_positions[qb1].1,
            tweezers_positions[qb2].0,
            tweezers_positions[qb2].1,
            match shift_type {
                ShiftType::None => "",
                ShiftType::Both => ", \"<|-|>\"",
                ShiftType::LeftToRight => ", \"-|>\"",
                ShiftType::RightToLeft => ", \"<|-\"",
            }
        ))
    }
    Ok(edges)
}

fn map_shifts(
    allowed_tweezer_shifts: HashMap<usize, Vec<Vec<usize>>>,
    tweezer_two_qubit_gate_times: HashMap<String, HashMap<(usize, usize), f64>>,
    edges_map: &mut HashMap<(usize, usize), ShiftType>,
) -> Result<(), RoqoqoBackendError> {
    let mut links: Vec<(usize, usize)> = tweezer_two_qubit_gate_times
        .values()
        .flat_map(|value| value.keys().cloned().collect::<Vec<(usize, usize)>>())
        .collect();
    links.sort();
    links.dedup();
    for (&tweezer, allowed_shifts) in allowed_tweezer_shifts.iter() {
        for allowed_shift in allowed_shifts {
            for &directly_linked_shift in allowed_shift.iter().filter(|&&tweezer_shift| {
                links.contains(&(tweezer, tweezer_shift))
                    || links.contains(&(tweezer_shift, tweezer))
            }) {
                if let Some(shift_type) = edges_map.get(&(tweezer, directly_linked_shift)) {
                    let key = (tweezer, directly_linked_shift);
                    match shift_type {
                        ShiftType::None => edges_map.insert(key, ShiftType::LeftToRight),
                        ShiftType::Both => edges_map.insert(key, ShiftType::Both),
                        ShiftType::LeftToRight => edges_map.insert(key, ShiftType::LeftToRight),
                        ShiftType::RightToLeft => edges_map.insert(key, ShiftType::Both),
                    };
                } else if let Some(shift_type) = edges_map.get(&(directly_linked_shift, tweezer)) {
                    let key = (directly_linked_shift, tweezer);
                    match shift_type {
                        ShiftType::None => edges_map.insert(key, ShiftType::RightToLeft),
                        ShiftType::Both => edges_map.insert(key, ShiftType::Both),
                        ShiftType::LeftToRight => edges_map.insert(key, ShiftType::Both),
                        ShiftType::RightToLeft => edges_map.insert(key, ShiftType::RightToLeft),
                    };
                } else {
                    return Err(RoqoqoBackendError::GenericError {
                        msg: format!(
                            "Unexpected tweezer shift: {tweezer}->{directly_linked_shift}"
                        ),
                    });
                }
            }
        }
    }
    Ok(())
}
