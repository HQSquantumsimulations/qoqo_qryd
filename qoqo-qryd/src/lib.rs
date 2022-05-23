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

//! # qoqo-qryd
//!
//! Components for the qoqo/roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de) that support QRyd quantum computers.
//!
//! The qoqo-qryd/roqoqo-qryd packages provide three components
//!
//! * devices: python/rust representation of QRyd devices
//! * operations: roqoqo Pragma operations specific to QRyd devices that can change the topology of QRyd devices
//! * simulator (optional): A QuEST based simulator for QRyd devices that checks the availability of the quantum operations on a chosen device during simulation

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

/// qoqo utilities for QRyd quantum computers.
///
/// Provides devices for the QRyd quantum hardware for the qoqo quantum toolkit.
/// Also provides qoqo PRAGMA operations specific to those devices.
/// Includes an optional QRydSimulator backend.
/// Furthermore, provides a collection of all QRyd devices for the WebAPI.
///
/// .. autosummary::
///     :toctree: generated/
///
///     api_devices
///     APIBackend
///     Backend
///     pragma_operations
///     devices
///
pub mod qryd_devices;
pub use qryd_devices::*;

pub mod pragma_operations;
pub use pragma_operations::*;

#[cfg(feature = "simulator")]
pub mod simulator_backend;
#[cfg(feature = "simulator")]
pub use simulator_backend::SimulatorBackendWrapper;

/// QRyd WebAPI Backend.
///
/// The WebAPI Backend implements methods available in the QRyd Web API.
///
pub mod api_backend;
pub use api_backend::APIBackendWrapper;

/// Collection of all QRyd devices for WebAPI.
///
/// At the moment only contains a square and a triangular device.
///
pub mod api_devices;
pub use api_devices::*;

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
///
///
#[pymodule]
fn qoqo_qryd(_py: Python, module: &PyModule) -> PyResult<()> {
    #[cfg(feature = "simulator")]
    module.add_class::<SimulatorBackendWrapper>()?;
    module.add_class::<APIBackendWrapper>()?;
    let wrapper = wrap_pymodule!(qryd_devices);
    module.add_wrapped(wrapper)?;
    let wrapper2 = wrap_pymodule!(api_devices);
    module.add_wrapped(wrapper2)?;
    // Adding nice imports corresponding to maturin example
    let wrapper = wrap_pymodule!(pragma_operations);
    module.add_wrapped(wrapper)?;
    // Adding nice imports corresponding to maturin example
    let system = PyModule::import(_py, "sys")?;
    let system_modules: &PyDict = system.getattr("modules")?.downcast()?;
    system_modules.set_item(
        "qoqo_qryd.pragma_operations",
        module.getattr("pragma_operations")?,
    )?;
    system_modules.set_item("qoqo_qryd.qryd_devices", module.getattr("qryd_devices")?)?;
    system_modules.set_item("qoqo_qryd.api_devices", module.getattr("api_devices")?)?;
    Ok(())
}
