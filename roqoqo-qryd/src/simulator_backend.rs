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

use crate::qryd_devices::QRydDevice;
use roqoqo::backends::EvaluatingBackend;
use roqoqo::backends::RegisterResult;
use roqoqo::devices::Device;
use roqoqo::operations::*;

/// QRyd simulator backend
///
/// A QRyd simulator simulates the action of each operation in a circuit on a quantum register.
/// The underlying simulator uses the QuEST library.
/// Although the underlying simulator supports arbitrary unitary gates, the QRyd simulator only
/// allows operations that are available on a device model of a QRyd device (stored in a [crate::QRydDevice]).
/// This limitation is introduced by design to check the compatability of circuits with a model of the QRyd hardware.
/// For unrestricted simulations use the backend simulator of the roqoqo-quest crate.
///
///
/// The simulator backend implements the [roqoqo::backends::EvaluatingBackend] trait
/// and is compatible with  running single circuits, running and evaluating measurements
/// and running QuantumPrograms on simulated QRyd devices.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SimulatorBackend {
    /// Device representing the model of a QRyd device.
    pub device: QRydDevice,
}

impl SimulatorBackend {
    /// Creates a new QRyd simulator backend.
    ///
    /// # Arguments
    ///
    /// `device` - The QRyd device used for the simulation.
    pub fn new(device: QRydDevice) -> Self {
        Self { device }
    }
}

impl EvaluatingBackend for SimulatorBackend {
    fn run_circuit_iterator<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation>,
    ) -> RegisterResult {
        let mut tmp_device: Option<Box<dyn Device>> = Some(Box::new(self.device.clone()));

        let quest_backend = roqoqo_quest::Backend::new(self.device.number_qubits());

        quest_backend.run_circuit_iterator_with_device(circuit, &mut tmp_device)
    }
}
