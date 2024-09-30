// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

#![deny(missing_docs)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::private_doc_tests)]
#![deny(missing_debug_implementations)]

//! # roqoqo-qryd
//!
//! The `roqoqo-qryd` rust crate implements [qoqo](https://github.com/HQSquantumsimulations/qoqo) support for quantum computers and quantum computer emulators of the [QRydDemo](https://thequantumlaend.de/qryddemo/) project.
//!
//! The QRydDemo project builds on Quantum computers using Rydberg atoms.
//! qoqo is quantum computing toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).
//!
//! The roqoqo-qryd package contains the following functionality:
//!
//! ### Interface to the current QRydDemo WebAPI
//!
//! At the moment QRydDemo WebAPI allows access to Quantum Hardware Emulators of different device topology. roqoqo-qryd supports interfacing with the corresponding [REST-API](https://api.qryddemo.itp3.uni-stuttgart.de/docs) with low level calls as well as a high-level backend to qoqo quantum programs. For this it provides the backend `APIBackend` to evaluate roqoqo quantum programs and the `api_devices` module to represent devices available on the emulators.
//!
//! ### QRydDemo specific hardware operations (prototype)
//!
//! Rydberg atom based quantum devices support, in principle, operations not commonly found in other quantum hardware. Changes in device topology are one of these operations. roqoqo-qryd adds support for changes in device topology to roqoqo via the operations in its `pragma_operations` module.
//! Note that this is a preview prototype and does not represent a finalized set of operations on the QRydDemo hardware.
//!
//! ### Local simulator supporting specific hardware operations
//!
//! roqoqo-qryd includes a local [QuEST](https://github.com/QuEST-Kit/QuEST) based simulator for quantum devices supporting the Rydberg specific quantum operations. The simulator is intended to let users test the capabilities of quantum hardware with the additional operations.
//! roqoqo-qryd provides the simulator via the `SimulatorBackend` backend the implements the roqoqo `Backend` trait.The backend uses the device prototypes in roqoqo-qryd's `qryd_devices` module.
//! Note that the devices for the simulator do not represent a finalised design for QRydDemo.

/// Devices representing QRyd quantum computer(s)
pub mod qryd_devices;
pub use qryd_devices::*;

/// Tweezer devices representing QRyd quantum computer(s)
pub mod tweezer_devices;
pub use tweezer_devices::*;

/// Devices representing QRyd quantum computer(s)
pub mod api_devices;
pub use api_devices::*;

/// QRyd specific PragmaOperations that support changing the QRyd device during a circuit evaluation
pub mod pragma_operations;
pub use pragma_operations::*;

/// Emulator device, TweezerDevice instance with all-to-all connectivity
pub mod emulator_devices;
pub use emulator_devices::*;

/// Simulator backend for the QRyd quantum computer
#[cfg(feature = "simulator")]
mod simulator_backend;
#[cfg(feature = "simulator")]
pub use simulator_backend::*;

/// WebAPI backend for the QRyd quantum computer(s)
#[cfg(feature = "web-api")]
pub mod api_backend;
#[cfg(feature = "web-api")]
pub use api_backend::*;

#[cfg(feature = "web-api")]
use roqoqo::RoqoqoBackendError;
#[cfg(feature = "web-api")]
use std::env;

/// Compute the angle according to the appropriate relation and phi/theta values.
///
/// # Arguments
///
/// `relation_name` - The name of the relation to refer to.
/// `theta` - The theta angle to check.
///
/// # Returns
///
/// `Some<f64>` - The phi-theta relation.
/// 'None' - The relation does not exist.
///
pub fn phi_theta_relation(relation_name: &str, mut theta: f64) -> Option<f64> {
    while theta < 0.0 {
        theta += 2.0 * std::f64::consts::PI;
    }
    while theta > 2.0 * std::f64::consts::PI {
        theta -= 2.0 * std::f64::consts::PI
    }
    match relation_name {
        "DefaultRelation" => Some(
            5.11382
                - 0.32933
                    * f64::ln(1.63085 * theta * theta * f64::exp(2.0 * theta) + theta + 0.02889),
        ),
        _ => None,
    }
}

/// Enum for a Device that can be a TweezerDevice or an EmulatorDevice.
#[derive(Debug)]
pub enum CombinedDevice {
    /// Variant for Tweezer devices
    Tweezer(TweezerDevice),
    /// Variant for Emulator devices
    Emulator(EmulatorDevice),
}

/// Creates a new TweezerDevice instance containing populated tweezer data or EmulatorDevice instance.
///
/// This requires a valid QRYD_API_TOKEN. Visit `https://thequantumlaend.de/get-access/` to get one.
///
/// # Arguments
///
/// * `device_name` - The name of the device to instantiate. Defaults to "qryd_emulator".
/// * `access_token` - An access_token is required to access QRYD hardware and emulators.
///                    The access_token can either be given as an argument here
///                         or set via the environmental variable `$QRYD_API_TOKEN`.
/// * `seed` - Optionally overwrite seed value from downloaded device instance.
/// * `dev` - The boolean to set the dev header to.
/// * `api_version` - The version of the QRYD API to use. Defaults to "v1_1".
///
/// # Returns
///
/// * `CombinedDevice` - The new CombinedDevice instance, with variant TweezerDevice or
///     EmulatorDevice depending on the pulled information.
///
/// # Errors
///
/// * `RoqoqoBackendError`
#[cfg(feature = "web-api")]
pub fn device_from_api(
    device_name: Option<String>,
    access_token: Option<String>,
    seed: Option<usize>,
    dev: Option<bool>,
    api_version: Option<String>,
) -> Result<CombinedDevice, RoqoqoBackendError> {
    // Preparing variables
    let device_name_internal = device_name.unwrap_or_else(|| String::from("qryd_emulator"));
    let api_version = api_version.unwrap_or_else(|| String::from("v1_1"));
    let dev = dev.unwrap_or(false);
    let hqs_env_var = env::var("QRYD_API_HQS").is_ok();
    let access_token_internal: String = match access_token {
        Some(s) => s,
        None => {
            env::var("QRYD_API_TOKEN").map_err(|_| RoqoqoBackendError::MissingAuthentication {
                msg: "QRYD access token is missing.".to_string(),
            })?
        }
    };

    // Client setup
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .build()
        .map_err(|x| RoqoqoBackendError::NetworkError {
            msg: format!("Could not create https client {:?}.", x),
        })?;

    // Response gathering
    let resp = match (dev, hqs_env_var) {
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
    };

    // Response handling
    let status_code = resp.status();
    if status_code == reqwest::StatusCode::OK {
        if let Ok(mut device) = resp.json::<TweezerDevice>() {
            if device.available_gates.is_some() {
                if let Some(new_seed) = seed {
                    device.seed = Some(new_seed);
                }
                device.device_name = device_name_internal;
                Ok(CombinedDevice::Emulator(EmulatorDevice {
                    internal: device,
                }))
            } else {
                if let Some(default) = device.default_layout.clone() {
                    device.switch_layout(&default, None).unwrap();
                }
                if let Some(new_seed) = seed {
                    device.seed = Some(new_seed);
                }
                device.device_name = device_name_internal;
                Ok(CombinedDevice::Tweezer(device))
            }
        } else {
            Err(RoqoqoBackendError::GenericError {
                msg: "Failed deserialization from device_from_api().".to_string(),
            })
        }
    } else {
        Err(RoqoqoBackendError::NetworkError {
            msg: format!(
                "Request to server failed with HTTP status code {:?}.",
                status_code
            ),
        })
    }
}
