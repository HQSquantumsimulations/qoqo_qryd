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

#![deny(missing_docs)]
#![deny(missing_crate_level_docs)]
#![deny(missing_debug_implementations)]
#![allow(ambiguous_glob_reexports)]

//! # qoqo-qryd
//!
//! Components for the qoqo/roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de) that support QRyd quantum computers.
//!
//! The qoqo-qryd/roqoqo-qryd packages provide three components
//!
//! * devices: python/rust representation of QRyd devices
//! * operations: roqoqo Pragma operations specific to QRyd devices that can change the topology of QRyd devices
//! * simulator (optional): A QuEST based simulator for QRyd devices that checks the availability of the quantum operations on a chosen device during simulation

#[cfg(feature = "web-api")]
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

/// qoqo utilities for QRyd quantum computers.
///
/// Provides devices for the QRyd quantum hardware for the qoqo quantum toolkit.
/// Also provides qoqo PRAGMA operations specific to those devices.
/// Includes an optional QRydSimulator backend.
/// Furthermore, provides a collection of all QRyd as well as Tweezer devices for the WebAPI.
///
/// .. autosummary::
///     :toctree: generated/
///
///     qryd_devices
///     api_devices
///     pragma_operations
///     SimulatorBackend
///     APIBackend
///     tweezer_devices
///     emulator_devices
///
pub mod qryd_devices;
pub use qryd_devices::*;

pub mod pragma_operations;
pub use pragma_operations::*;

/// QRyd Tweezer Devices.
///
pub mod tweezer_devices;
pub use tweezer_devices::*;

/// QRyd Emulator Devices.
///
pub mod emulator_devices;
pub use emulator_devices::*;

#[cfg(feature = "simulator")]
pub mod simulator_backend;
#[cfg(feature = "simulator")]
pub use simulator_backend::SimulatorBackendWrapper;

/// QRyd WebAPI Backend.
///
/// The WebAPI Backend implements methods available in the QRyd Web API.
///
#[cfg(feature = "web-api")]
pub mod api_backend;
#[cfg(feature = "web-api")]
pub use api_backend::APIBackendWrapper;

/// Collection of all QRyd devices for WebAPI.
///
/// At the moment only contains a square and a triangular device.
///
pub mod api_devices;
pub use api_devices::*;

/// Creates a new TweezerDevice instance containing populated tweezer data or EmulatorDevice instance.
///
/// This requires a valid QRYD_API_TOKEN. Visit `https://thequantumlaend.de/get-access/` to get one.
///
/// Args
///     device_name (Optional[str]): The name of the device to instantiate. Defaults to "qryd_emulator".
///     access_token (Optional[str]): An access_token is required to access QRYD hardware and emulators.
///                         The access_token can either be given as an argument here
///                             or set via the environmental variable `$QRYD_API_TOKEN`.
///     seed (Optional[int]): Optionally overwrite seed value from downloaded device instance.
///     dev (Optional[bool]): The boolean to set the dev header to.
///     api_version (Optional[str]): The version of the QRYD API to use. Defaults to "v1_1".
///
/// Returns
///     Union[TweezerDevice, EmulatorDevice]: Either the TweezerDevice or EmulatorDevice instance
///         depending on the pulled information.
///
/// Raises:
///     RoqoqoBackendError
#[cfg(feature = "web-api")]
#[pyfunction]
pub fn device_from_api(
    device_name: Option<String>,
    access_token: Option<String>,
    seed: Option<usize>,
    dev: Option<bool>,
    api_version: Option<String>,
) -> PyResult<PyObject> {
    Python::with_gil(|py| -> PyResult<PyObject> {
        match roqoqo_qryd::device_from_api(device_name, access_token, seed, dev, api_version) {
            Ok(device) => match device {
                roqoqo_qryd::CombinedDevice::Tweezer(tweezer_device) => Ok(TweezerDeviceWrapper {
                    internal: tweezer_device,
                }
                .into_py(py)),
                roqoqo_qryd::CombinedDevice::Emulator(emulator_device) => {
                    Ok(EmulatorDeviceWrapper {
                        internal: emulator_device,
                    }
                    .into_py(py))
                }
            },
            Err(err) => Err(PyValueError::new_err(format!("{:}", err))),
        }
    })
}

/// QRyd utilities for qoqo quantum computation toolkit.
///
/// qoqo is the HQS python package to represent quantum circuits.
///
/// .. autosummary::
///     :toctree: generated/
///
///     api_devices
///     Backend
///     pragma_operations
///     qryd_devices
///     tweezer_devices
///     emulator_devices
///     device_from_api
///
///
#[pymodule]
fn qoqo_qryd(_py: Python, module: &Bound<PyModule>) -> PyResult<()> {
    #[cfg(feature = "simulator")]
    module.add_class::<SimulatorBackendWrapper>()?;
    #[cfg(feature = "web-api")]
    module.add_class::<APIBackendWrapper>()?;
    #[cfg(feature = "web-api")]
    module.add_function(wrap_pyfunction!(device_from_api, module)?)?;
    let wrapper = wrap_pymodule!(qryd_devices::qryd_devices);
    module.add_wrapped(wrapper)?;
    let wrapper = wrap_pymodule!(api_devices::api_devices);
    module.add_wrapped(wrapper)?;
    let wrapper = wrap_pymodule!(tweezer_devices::tweezer_devices);
    module.add_wrapped(wrapper)?;
    // Adding nice imports corresponding to maturin example
    let wrapper = wrap_pymodule!(pragma_operations::pragma_operations);
    module.add_wrapped(wrapper)?;
    let wrapper = wrap_pymodule!(emulator_devices::emulator_devices);
    module.add_wrapped(wrapper)?;
    // Adding nice imports corresponding to maturin example
    let system = PyModule::import_bound(_py, "sys")?;
    let binding = system.getattr("modules")?;
    let system_modules: &Bound<PyDict> = binding.downcast()?;
    system_modules.set_item(
        "qoqo_qryd.pragma_operations",
        module.getattr("pragma_operations")?,
    )?;
    system_modules.set_item("qoqo_qryd.qryd_devices", module.getattr("qryd_devices")?)?;
    system_modules.set_item("qoqo_qryd.api_devices", module.getattr("api_devices")?)?;
    system_modules.set_item(
        "qoqo_qryd.tweezer_devices",
        module.getattr("tweezer_devices")?,
    )?;
    system_modules.set_item(
        "qoqo_qryd.emulator_devices",
        module.getattr("emulator_devices")?,
    )?;
    Ok(())
}
