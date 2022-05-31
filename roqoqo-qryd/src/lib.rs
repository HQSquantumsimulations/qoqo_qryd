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

/// Devices representing QRyd quantum computer(s)
pub mod api_devices;
pub use api_devices::*;

/// QRyd specific PragmaOperations that support changing the QRyd device during a circuit evaluation
pub mod pragma_operations;
pub use pragma_operations::*;

/// Simulator backend for the QRyd quantum computer
#[cfg(feature = "simulator")]
mod simulator_backend;
#[cfg(feature = "simulator")]
pub use simulator_backend::*;

/// WebAPI backend for the QRyd quantum computer(s)
pub mod api_backend;
pub use api_backend::*;
