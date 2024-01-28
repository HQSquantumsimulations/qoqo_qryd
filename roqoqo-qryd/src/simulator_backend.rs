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
use crate::FirstDevice;
use crate::TweezerDevice;
use roqoqo::backends::EvaluatingBackend;
use roqoqo::backends::RegisterResult;
use roqoqo::devices::Device;
use roqoqo::operations::*;

/// Collection of all QRyd devices
///
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum SimulatorDevice {
    /// Old Qryd device
    QRydDevice(QRydDevice),
    /// New TweezerDevice
    TweezerDevice(TweezerDevice),
}

impl Device for SimulatorDevice {
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        match self {
            SimulatorDevice::QRydDevice(x) => x.single_qubit_gate_time(hqslang, qubit),
            SimulatorDevice::TweezerDevice(x) => x.single_qubit_gate_time(hqslang, qubit),
        }
    }

    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        match self {
            SimulatorDevice::QRydDevice(x) => x.two_qubit_gate_time(hqslang, control, target),
            SimulatorDevice::TweezerDevice(x) => x.two_qubit_gate_time(hqslang, control, target),
        }
    }

    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        control_0: &usize,
        control_1: &usize,
        target: &usize,
    ) -> Option<f64> {
        match self {
            SimulatorDevice::QRydDevice(x) => {
                x.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
            SimulatorDevice::TweezerDevice(x) => {
                x.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
        }
    }

    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        match self {
            SimulatorDevice::QRydDevice(x) => x.multi_qubit_gate_time(hqslang, qubits),
            SimulatorDevice::TweezerDevice(x) => x.multi_qubit_gate_time(hqslang, qubits),
        }
    }

    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<ndarray::prelude::Array2<f64>> {
        match self {
            SimulatorDevice::QRydDevice(x) => x.qubit_decoherence_rates(qubit),
            SimulatorDevice::TweezerDevice(x) => x.qubit_decoherence_rates(qubit),
        }
    }

    fn number_qubits(&self) -> usize {
        match self {
            SimulatorDevice::QRydDevice(x) => x.number_qubits(),
            SimulatorDevice::TweezerDevice(x) => x.number_qubits(),
        }
    }

    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        match self {
            SimulatorDevice::QRydDevice(x) => x.two_qubit_edges(),
            SimulatorDevice::TweezerDevice(x) => x.two_qubit_edges(),
        }
    }

    fn to_generic_device(&self) -> roqoqo::devices::GenericDevice {
        match self {
            SimulatorDevice::QRydDevice(x) => x.to_generic_device(),
            SimulatorDevice::TweezerDevice(x) => x.to_generic_device(),
        }
    }
}

impl From<FirstDevice> for SimulatorDevice {
    fn from(input: FirstDevice) -> Self {
        Self::QRydDevice(QRydDevice::FirstDevice(input))
    }
}

impl From<TweezerDevice> for SimulatorDevice {
    fn from(input: TweezerDevice) -> Self {
        Self::TweezerDevice(input)
    }
}

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
/// and is compatible with running single circuits, running and evaluating measurements
/// and running QuantumPrograms on simulated QRyd devices.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SimulatorBackend {
    /// Device representing the model of a QRyd device.
    pub device: SimulatorDevice,
    /// The number of qubits allocated by the simulator.
    pub number_qubits: usize,
}

impl SimulatorBackend {
    /// Creates a new QRyd simulator backend.
    ///
    /// # Arguments
    ///
    /// `device` - The QRyd device used for the simulation.
    /// `number_qubits` - The number of qubits the simulator should use. Defaults to `device.number_qubits()`.
    pub fn new(device: SimulatorDevice, number_qubits: Option<usize>) -> Self {
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

        let quest_backend = roqoqo_quest::Backend::new(self.number_qubits);

        quest_backend.run_circuit_iterator_with_device(circuit, &mut tmp_device)
    }
}
