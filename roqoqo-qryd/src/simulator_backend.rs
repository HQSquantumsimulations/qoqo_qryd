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

use roqoqo::backends::EvaluatingBackend;
use roqoqo::backends::RegisterResult;
use roqoqo::devices::Device;
use roqoqo::operations::*;

use crate::TweezerDevice;

/// QRyd simulator backend
///
/// A QRyd simulator simulates the action of each operation in a circuit on a quantum register.
/// The underlying simulator uses the QuEST library.
/// Although the underlying simulator supports arbitrary unitary gates, the QRyd simulator only
/// allows operations that are available on a device model of a QRyd device (stored in a [crate::TweezerDevice]).
/// This limitation is introduced by design to check the compatability of circuits with a model of the QRyd hardware.
/// For unrestricted simulations use the backend simulator of the roqoqo-quest crate.
///
///
/// The simulator backend implements the [roqoqo::backends::EvaluatingBackend] trait
/// and is compatible with running single circuits, running and evaluating measurements
/// and running QuantumPrograms on simulated QRyd devices.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SimulatorBackend {
    /// Device representing the model of a QRyd device.
    pub device: TweezerDevice,
    /// The number of qubits allocated by the simulator.
    pub number_qubits: usize,
}

impl SimulatorBackend {
    /// Creates a new QRyd SimulatorBackend.
    ///
    /// # Arguments
    ///
    /// `device` - The TweezerDevice used for the simulation.
    /// `number_qubits` - The number of qubits the simulator should use. Defaults to `device.number_qubits()`.
    pub fn new(device: TweezerDevice, number_qubits: Option<usize>) -> Self {
        Self {
            device: device.clone(),
            number_qubits: number_qubits.unwrap_or(device.number_qubits()),
        }
    }
}

impl EvaluatingBackend for SimulatorBackend {
    fn run_circuit_iterator<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation>,
    ) -> RegisterResult {
        let mut tmp_device: Option<Box<dyn Device>> = Some(Box::new(self.device.clone()));

        let quest_backend = roqoqo_quest::Backend::new(self.number_qubits, None);

        quest_backend.run_circuit_iterator_with_device(circuit, &mut tmp_device)
    }
}
