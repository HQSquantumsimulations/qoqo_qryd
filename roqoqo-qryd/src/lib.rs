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
//! Components for the qoqo/roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de) that support QRyd quantum computers.
//!
//! The qoqo-qryd/roqoqo-qryd packages provide three components
//!
//! * devices: python/rust representation of QRyd devices
//! * operations: roqoqo Pragma operations specific to QRyd devices that can change the topology of QRyd devices
//! * simulator (optional): A QuEST based simulator for QRyd devices that checks the availability of the quantum operations on a chosen device during simulation

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
