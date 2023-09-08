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

use crate::phi_theta_relation;
use ndarray::Array2;
use roqoqo::devices::{Device, GenericDevice};
use roqoqo::RoqoqoBackendError;
use std::str::FromStr;

/// Collection of all QRyd devices for WebAPI.
///
/// At the moment only contains a square and a triangular device.
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub enum QRydAPIDevice {
    /// Square Device
    QrydEmuSquareDevice(QrydEmuSquareDevice),
    /// Triangular Device
    QrydEmuTriangularDevice(QrydEmuTriangularDevice),
}

/// Implements the trait to return field values of the QRydAPIDevice.
impl QRydAPIDevice {
    /// Returns the backend associated with the device.
    pub fn qrydbackend(&self) -> String {
        match self {
            Self::QrydEmuSquareDevice(x) => x.qrydbackend(),
            Self::QrydEmuTriangularDevice(x) => x.qrydbackend(),
        }
    }

    /// Returns the seed usized for the API.
    pub fn seed(&self) -> usize {
        match self {
            Self::QrydEmuSquareDevice(x) => x.seed(),
            Self::QrydEmuTriangularDevice(x) => x.seed(),
        }
    }

    /// Returns the PhaseShiftedControlledZ phase shift according to the device's relation.
    pub fn phase_shift_controlled_z(&self) -> Option<f64> {
        match self {
            Self::QrydEmuSquareDevice(x) => x.phase_shift_controlled_z(),
            Self::QrydEmuTriangularDevice(x) => x.phase_shift_controlled_z(),
        }
    }

    /// Returns the PhaseShiftedControlledPhase phase shift according to the device's relation.
    pub fn phase_shift_controlled_phase(&self, theta: f64) -> Option<f64> {
        match self {
            Self::QrydEmuSquareDevice(x) => x.phase_shift_controlled_phase(theta),
            Self::QrydEmuTriangularDevice(x) => x.phase_shift_controlled_phase(theta),
        }
    }

    /// Returns the gate time of a PhaseShiftedControlledZ operation with the given qubits and phi angle.
    pub fn gate_time_controlled_z(&self, control: &usize, target: &usize, phi: f64) -> Option<f64> {
        match self {
            Self::QrydEmuSquareDevice(x) => x.gate_time_controlled_z(control, target, phi),
            Self::QrydEmuTriangularDevice(x) => x.gate_time_controlled_z(control, target, phi),
        }
    }

    /// Returns the gate time of a PhaseShiftedControlledPhase operation with the given qubits and phi and theta angles.
    pub fn gate_time_controlled_phase(
        &self,
        control: &usize,
        target: &usize,
        phi: f64,
        theta: f64,
    ) -> Option<f64> {
        match self {
            Self::QrydEmuSquareDevice(x) => {
                x.gate_time_controlled_phase(control, target, phi, theta)
            }
            Self::QrydEmuTriangularDevice(x) => {
                x.gate_time_controlled_phase(control, target, phi, theta)
            }
        }
    }
}

/// Implements the Device trait for QRydAPIDevice.
///
/// Defines standard functions available for roqoqo-qryd devices.
impl Device for QRydAPIDevice {
    /// Returns the gate time of a single qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the single qubit gate as defined in roqoqo.
    /// * `qubit` - The qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        match self {
            Self::QrydEmuSquareDevice(d) => d.single_qubit_gate_time(hqslang, qubit),
            Self::QrydEmuTriangularDevice(d) => d.single_qubit_gate_time(hqslang, qubit),
        }
    }

    /// Returns the gate time of a two qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the two qubit gate as defined in roqoqo.
    /// * `control` - The control qubit the gate acts on
    /// * `target` - The target qubit the gate acts on
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        match self {
            Self::QrydEmuSquareDevice(d) => d.two_qubit_gate_time(hqslang, control, target),
            Self::QrydEmuTriangularDevice(d) => d.two_qubit_gate_time(hqslang, control, target),
        }
    }

    /// Returns the gate time of a three qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a three-qubit gate as defined in roqoqo.
    /// * `control_0` - The first control qubit the gate acts on.
    /// * `control_1` - The second control qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        control_0: &usize,
        control_1: &usize,
        target: &usize,
    ) -> Option<f64> {
        match self {
            Self::QrydEmuSquareDevice(d) => {
                d.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
            Self::QrydEmuTriangularDevice(d) => {
                d.three_qubit_gate_time(hqslang, control_0, control_1, target)
            }
        }
    }

    /// Returns the gate time of a multi qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a multi qubit gate as defined in roqoqo.
    /// * `qubits` - The qubits the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        match self {
            Self::QrydEmuSquareDevice(d) => d.multi_qubit_gate_time(hqslang, qubits),
            Self::QrydEmuTriangularDevice(d) => d.multi_qubit_gate_time(hqslang, qubits),
        }
    }

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// $$
    /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
    ///     L_0 = \sigma^{+} \\\\
    ///     L_1 = \sigma^{-} \\\\
    ///     L_3 = \sigma^{z}
    /// $$
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the rate matrix M is returned.
    ///
    /// # Returns
    ///
    /// * `Some<Array2<f64>>` - The decoherence rates.
    /// * `None` - The qubit is not part of the device.
    ///
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        match self {
            Self::QrydEmuSquareDevice(d) => d.qubit_decoherence_rates(qubit),
            Self::QrydEmuTriangularDevice(d) => d.qubit_decoherence_rates(qubit),
        }
    }

    /// Returns the number of qubits the device supports.
    ///
    /// # Returns
    ///
    /// The number of qubits in the device.
    ///
    fn number_qubits(&self) -> usize {
        match self {
            Self::QrydEmuSquareDevice(d) => d.number_qubits(),
            Self::QrydEmuTriangularDevice(d) => d.number_qubits(),
        }
    }

    /// Changes the device topology based on a Pragma operation.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the wrapped operation as defined in roqoqo.
    /// * `operation` - The Pragma operation encoded in binary form using the [bincode] crate.
    ///
    /// # Returns
    ///
    /// Result of changing the device.
    /// If the device can not be changed a generic RoqoqoBackendError is returned.
    ///
    fn change_device(&mut self, hqslang: &str, operation: &[u8]) -> Result<(), RoqoqoBackendError> {
        match self {
            Self::QrydEmuSquareDevice(d) => d.change_device(hqslang, operation),
            Self::QrydEmuTriangularDevice(d) => d.change_device(hqslang, operation),
        }
    }

    /// Returns the list of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    /// A pair of qubits is considered linked by a native two-qubit-gate if the device
    /// can implement a two-qubit-gate between the two qubits without decomposing it
    /// into a sequence of gates that involves a third qubit of the device.
    /// The two-qubit-gate also has to form a universal set together with the available
    /// single qubit gates.
    ///
    /// The returned vectors is a simple, graph-library independent, representation of
    /// the undirected connectivity graph of the device.
    /// It can be used to construct the connectivity graph in a graph library of the users
    /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
    ///
    /// # Returns
    ///
    /// A list (Vec) of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        match self {
            Self::QrydEmuSquareDevice(d) => d.two_qubit_edges(),
            Self::QrydEmuTriangularDevice(d) => d.two_qubit_edges(),
        }
    }

    /// Turns Device into GenericDevice
    ///
    /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
    /// (for example when the interface needs to be serialized)
    ///
    /// # Notes
    ///
    /// GenericDevice uses nested HashMaps to represent the most general device connectivity.
    /// The memory usage will be inefficient for devices with large qubit numbers.
    ///
    /// # Returns
    ///
    /// * `GenericDevice` - The device in generic representation
    ///
    fn to_generic_device(&self) -> roqoqo::devices::GenericDevice {
        match self {
            Self::QrydEmuSquareDevice(d) => d.to_generic_device(),
            Self::QrydEmuTriangularDevice(d) => d.to_generic_device(),
        }
    }
}

impl From<&QrydEmuSquareDevice> for QRydAPIDevice {
    fn from(input: &QrydEmuSquareDevice) -> Self {
        Self::QrydEmuSquareDevice(input.clone())
    }
}

impl From<QrydEmuSquareDevice> for QRydAPIDevice {
    fn from(input: QrydEmuSquareDevice) -> Self {
        Self::QrydEmuSquareDevice(input)
    }
}

impl From<&QrydEmuTriangularDevice> for QRydAPIDevice {
    fn from(input: &QrydEmuTriangularDevice) -> Self {
        Self::QrydEmuTriangularDevice(input.clone())
    }
}

impl From<QrydEmuTriangularDevice> for QRydAPIDevice {
    fn from(input: QrydEmuTriangularDevice) -> Self {
        Self::QrydEmuTriangularDevice(input)
    }
}

/// Square Device for the emulator API.
///
/// Provides an emulated quantum computing device with up to 30 qubits
/// that can be accessed via the QRyd WebAPI.
/// For more detailed information about the device an qubit layout see the
/// documentation of the QRyd WebAPI: https://thequantumlaend.de/get-access/
#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct QrydEmuSquareDevice {
    /// Use local or cloud computation for transpilation
    local: bool,
    /// Seed, if not provided will be set to 0 per default (not recommended!)
    seed: usize,
    /// The specific PhaseShiftedControlledZ relation to use.
    controlled_z_phase_relation: String,
    /// The specific PhaseShiftedControlledPhase relation to use.
    controlled_phase_phase_relation: String,
}

/// Implements the trait to create a new QrydEmuSquareDevice and to return its field values.
impl QrydEmuSquareDevice {
    /// Create new QrydEmuSquareDevice device
    ///
    /// # Arguments
    ///
    /// * `seed` - Seed, if not provided will be set to 0 per default (not recommended!)
    /// * `controlled_z_phase_relation` - The relation to use for the PhaseShiftedControlledZ gate.
    ///                                   It can be hardcoded to a specific value if a float is passed in as String.
    /// * `controlled_phase_phase_relation` - The relation to use for the PhaseShiftedControlledPhase gate.
    pub fn new(
        seed: Option<usize>,
        controlled_z_phase_relation: Option<String>,
        controlled_phase_phase_relation: Option<String>,
    ) -> Self {
        Self {
            local: false,
            seed: seed.unwrap_or_default(),
            controlled_z_phase_relation: controlled_z_phase_relation
                .unwrap_or_else(|| "DefaultRelation".to_string()),
            controlled_phase_phase_relation: controlled_phase_phase_relation
                .unwrap_or_else(|| "DefaultRelation".to_string()),
        }
    }

    /// Returns the backend associated with the device.
    pub fn qrydbackend(&self) -> String {
        if self.local {
            "qryd_emu_localcomp_square".to_string()
        } else {
            "qryd_emu_cloudcomp_square".to_string()
        }
    }

    /// Returns the seed usized for the API.
    pub fn seed(&self) -> usize {
        self.seed
    }

    /// Returns the PhaseShiftedControlledZ phase shift according to the device's relation.
    ///
    /// # Returns
    ///
    /// * `f64` - The PhaseShiftedControlledZ phase shift.
    ///
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
    ///
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
    ///
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
    ///
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
                    Some(1e-6)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Implements the Device trait for QrydEmuSquareDevice.
///
/// Defines standard functions available for roqoqo-qryd devices.
impl Device for QrydEmuSquareDevice {
    /// Returns the gate time of a single qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the single qubit gate as defined in roqoqo.
    /// * `qubit` - The qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        // The availability of gates is checked by returning Some
        // When a gate is not available simply return None
        // Check if the qubit is even in the device
        if qubit >= &30 {
            return None;
        }
        // The gate time can optionally be used for noise considerations
        // For the first device it is hardcoded, eventually for later device models
        // it could be extracted from callibration data
        match hqslang {
            // "PhaseShiftState0" => Some(1e-6), // Updated gate definition as of April 2022
            "PhaseShiftState1" => Some(1e-6),
            "RotateX" => Some(1e-6),
            "RotateY" => Some(1e-6), // Updated gate definition as of April 2022
            "RotateZ" => Some(1e-6), // Updated gate definition as of February 2023
            "RotateXY" => Some(1e-6), // Updated gate definition as of April 2022
            "PauliX" => Some(1e-6),  // Updated gate definition as of February 2023
            "PauliY" => Some(1e-6),  // Updated gate definition as of February 2023
            "PauliZ" => Some(1e-6),  // Updated gate definition as of February 2023
            "SqrtPauliX" => Some(1e-6), // Updated gate definition as of February 2023
            "InvSqrtPauliX" => Some(1e-6), // Updated gate definition as of February 2023
            // still needs to be implemented in qoqo
            // All other single qubit gates are not available on the hardware
            _ => None,
        }
    }

    /// Returns the gate time of a two qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the two qubit gate as defined in roqoqo.
    /// * `control` - The control qubit the gate acts on
    /// * `target` - The target qubit the gate acts on
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        // Check for availability of control and target on device
        if control >= &30 {
            return None;
        }
        if target >= &30 || target == control {
            return None;
        }

        let smaller = target.min(control);
        let larger = target.max(control);

        if (larger - smaller == 1 && smaller % 5 != 4) || (larger - smaller == 5) {
            match hqslang {
                "PhaseShiftedControlledZ" => Some(1e-6),
                "PhaseShiftedControlledPhase" => Some(1e-6),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Returns the gate time of a three qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a three-qubit gate as defined in roqoqo.
    /// * `control_0` - The first control qubit the gate acts on.
    /// * `control_1` - The second control qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    #[allow(unused_variables)]
    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        control_0: &usize,
        control_1: &usize,
        target: &usize,
    ) -> Option<f64> {
        None
    }

    /// Returns the gate time of a multi qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a multi qubit gate as defined in roqoqo.
    /// * `qubits` - The qubits the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    #[allow(unused_variables)]
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        // If any qubit is not in device operation is not available
        None
    }

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// $$
    /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
    ///     L_0 = \sigma^{+} \\\\
    ///     L_1 = \sigma^{-} \\\\
    ///     L_3 = \sigma^{z}
    /// $$
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the rate matrix M is returned.
    ///
    /// # Returns
    ///
    /// * `Some<Array2<f64>>` - The decoherence rates.
    /// * `None` - The qubit is not part of the device.
    ///
    #[allow(unused_variables)]
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        // At the moment we hard-code a noise free model
        Some(Array2::zeros((3, 3).to_owned()))
    }

    /// Returns the number of qubits the device supports.
    ///
    /// # Returns
    ///
    /// The number of qubits in the device.
    ///
    fn number_qubits(&self) -> usize {
        30
    }

    /// Returns the list of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    /// A pair of qubits is considered linked by a native two-qubit-gate if the device
    /// can implement a two-qubit-gate between the two qubits without decomposing it
    /// into a sequence of gates that involves a third qubit of the device.
    /// The two-qubit-gate also has to form a universal set together with the available
    /// single qubit gates.
    ///
    /// The returned vectors is a simple, graph-library independent, representation of
    /// the undirected connectivity graph of the device.
    /// It can be used to construct the connectivity graph in a graph library of the users
    /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
    ///
    /// # Returns
    ///
    /// A list (Vec) of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for row in 0..self.number_qubits() {
            for column in row + 1..self.number_qubits() {
                if self
                    .two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                    .is_some()
                {
                    edges.push((row, column));
                }
            }
        }
        edges
    }

    /// Changes the device topology based on a Pragma operation.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the wrapped operation as defined in roqoqo.
    /// * `operation` - The Pragma operation encoded in binary form using the [bincode] crate.
    ///
    /// # Returns
    ///
    /// Result of changing the device.
    /// This device is not allowed to be changed and thus a generic RoqoqoBackendError is returned.
    ///
    fn change_device(
        &mut self,
        _hqslang: &str,
        _operation: &[u8],
    ) -> Result<(), RoqoqoBackendError> {
        Err(RoqoqoBackendError::GenericError {
            msg: "Wrapped operation not supported in QRydAPIDevice".to_string(),
        })
    }

    /// Turns Device into GenericDevice
    ///
    /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
    /// (for example when the interface needs to be serialized)
    ///
    /// # Notes
    ///
    /// GenericDevice uses nested HashMaps to represent the most general device connectivity.
    /// The memory usage will be inefficient for devices with large qubit numbers.
    ///
    /// # Returns
    ///
    /// * `GenericDevice` - The device in generic representation
    ///
    fn to_generic_device(&self) -> roqoqo::devices::GenericDevice {
        let mut new_generic_device = GenericDevice::new(self.number_qubits());

        for gate_name in ["PhaseShiftState1", "RotateX", "RotateY", "RotateXY"] {
            for qubit in 0..self.number_qubits() {
                new_generic_device
                    .set_single_qubit_gate_time(
                        gate_name,
                        qubit,
                        self.single_qubit_gate_time(gate_name, &qubit).unwrap(),
                    )
                    .unwrap();
            }
        }
        for qubit in 0..self.number_qubits() {
            new_generic_device
                .set_qubit_decoherence_rates(qubit, self.qubit_decoherence_rates(&qubit).unwrap())
                .unwrap();
        }
        for row in 0..self.number_qubits() {
            for column in row + 1..self.number_qubits() {
                if self
                    .two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                    .is_some()
                {
                    new_generic_device
                        .set_two_qubit_gate_time(
                            "PhaseShiftedControlledZ",
                            row,
                            column,
                            self.two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                                .unwrap(),
                        )
                        .unwrap();
                    new_generic_device
                        .set_two_qubit_gate_time(
                            "PhaseShiftedControlledZ",
                            column,
                            row,
                            self.two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                                .unwrap(),
                        )
                        .unwrap();
                }
            }
        }
        new_generic_device
    }
}

/// Triangular Device for the emulator API.
///
/// Provides an emulated quantum computing device with up to 30 qubits
/// that can be accessed via the QRyd WebAPI.
/// For more detailed information about the device an qubit layout see the
/// documentation of the QRyd WebAPI: https://thequantumlaend.de/get-access/
#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct QrydEmuTriangularDevice {
    /// Use local or cloud computation for transpilation
    local: bool,
    /// Seed, if not provided will be set to 0 per default (not recommended!)
    seed: usize,
    /// The specific PhaseShiftedControlledZ relation to use.
    controlled_z_phase_relation: String,
    /// The specific PhaseShiftedControlledPhase relation to use.
    controlled_phase_phase_relation: String,
    /// Whether the device allows ControlledControlledPauliZ operations.
    allow_ccz_gate: bool,
    /// Whether the device allows ControlledControlledPhaseShift operations.
    allow_ccp_gate: bool,
}

/// Implements the trait to create a new QrydEmuTriangularDevice and to return its field values.
impl QrydEmuTriangularDevice {
    /// Create new QrydEmuTriangularDevice device
    ///
    /// # Arguments
    ///
    /// * `seed` - Seed, if not provided will be set to 0 per default (not recommended!)
    /// * `controlled_z_phase_relation` - The relation to use for the PhaseShiftedControlledZ gate.
    ///                                   It can be hardcoded to a specific value if a float is passed in as String.
    /// * `controlled_phase_phase_relation` - The relation to use for the PhaseShiftedControlledPhase gate.
    /// * `allow_ccz_gate` - Whether to allow ControlledControlledPauliZ operations in the device.
    /// * `allow_ccp_gate` - Whether to allow ControlledControlledPhaseShift operations in the device.
    pub fn new(
        seed: Option<usize>,
        controlled_z_phase_relation: Option<String>,
        controlled_phase_phase_relation: Option<String>,
        allow_ccz_gate: Option<bool>,
        allow_ccp_gate: Option<bool>,
    ) -> Self {
        Self {
            local: false,
            seed: seed.unwrap_or_default(),
            controlled_z_phase_relation: controlled_z_phase_relation
                .unwrap_or_else(|| "DefaultRelation".to_string()),
            controlled_phase_phase_relation: controlled_phase_phase_relation
                .unwrap_or_else(|| "DefaultRelation".to_string()),
            allow_ccz_gate: allow_ccz_gate.unwrap_or(true),
            allow_ccp_gate: allow_ccp_gate.unwrap_or(false),
        }
    }

    /// Returns the backend associated with the device.
    pub fn qrydbackend(&self) -> String {
        if self.local {
            "qryd_emu_localcomp_triangle".to_string()
        } else {
            "qryd_emu_cloudcomp_triangle".to_string()
        }
    }

    /// Returns the seed usized for the API.
    pub fn seed(&self) -> usize {
        self.seed
    }

    /// Returns the PhaseShiftedControlledZ phase shift according to the device's relation.
    ///
    /// # Returns
    ///
    /// * `f64` - The PhaseShiftedControlledZ phase shift.
    ///
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
    ///
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
    ///
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
    ///
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
}

/// Implements the Device trait for QrydEmuTriangularDevice.
///
/// Defines standard functions available for roqoqo-qryd devices.
impl Device for QrydEmuTriangularDevice {
    /// Returns the gate time of a single qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the single qubit gate as defined in roqoqo.
    /// * `qubit` - The qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        // The availability of gates is checked by returning Some
        // When a gate is not available simply return None
        // Check if the qubit is even in the device
        if qubit >= &30 {
            return None;
        }

        // The gate time can optionally be used for noise considerations
        // For the first device it is hardcoded, eventually for later device models
        // it could be extracted from callibration data
        match hqslang {
            // "PhaseShiftState0" => Some(1e-6), // Updated gate definition as of April 2022
            "PhaseShiftState1" => Some(1e-6),
            "RotateX" => Some(1e-6),
            "RotateY" => Some(1e-6), // Updated gate definition as of April 2022
            "RotateZ" => Some(1e-6), // Updated gate definition as of February 2023
            "RotateXY" => Some(1e-6), // Updated gate definition as of April 2022
            "PauliX" => Some(1e-6),  // Updated gate definition as of February 2023
            "PauliY" => Some(1e-6),  // Updated gate definition as of February 2023
            "PauliZ" => Some(1e-6),  // Updated gate definition as of February 2023
            "SqrtPauliX" => Some(1e-6), // Updated gate definition as of February 2023
            "InvSqrtPauliX" => Some(1e-6), // Updated gate definition as of February 2023
            // still needs to be implemented in qoqo
            // All other single qubit gates are not available on the hardware
            _ => None,
        }
    }

    /// Returns the gate time of a two qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the two qubit gate as defined in roqoqo.
    /// * `control` - The control qubit the gate acts on
    /// * `target` - The target qubit the gate acts on
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        // Check for availability of control and target on device
        if control >= &30 {
            return None;
        }
        if target >= &30 || target == control {
            return None;
        }

        let smaller = target.min(control);
        let larger = target.max(control);

        if smaller % 10 < 5 {
            if (larger - smaller == 5)
                || (larger - smaller == 6 && smaller % 5 != 4)
                || (larger - smaller == 1 && larger % 5 != 0)
            {
                match hqslang {
                    "PhaseShiftedControlledZ" => Some(1e-6),
                    "PhaseShiftedControlledPhase" => Some(1e-6),
                    _ => None,
                }
            } else {
                None
            }
        } else if (larger - smaller == 5)
            || (larger - smaller == 4 && smaller % 5 != 0)
            || (larger - smaller == 1 && larger % 5 != 0)
        {
            match hqslang {
                "PhaseShiftedControlledZ" => Some(1e-6),
                "PhaseShiftedControlledPhase" => Some(1e-6),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Returns the gate time of a three qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a three-qubit gate as defined in roqoqo.
    /// * `control_0` - The first control qubit the gate acts on.
    /// * `control_1` - The second control qubit the gate acts on.
    /// * `target` - The target qubit the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device.
    ///
    #[allow(unused_variables)]
    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        control_0: &usize,
        control_1: &usize,
        target: &usize,
    ) -> Option<f64> {
        if control_0 >= &30 {
            return None;
        }
        if control_1 >= &30 {
            return None;
        }
        if target >= &30 {
            return None;
        }

        match hqslang {
            "ControlledControlledPauliZ" => {
                if self.allow_ccz_gate
                    && self
                        .two_qubit_gate_time("PhaseShiftedControlledZ", control_0, target)
                        .is_some()
                    && self
                        .two_qubit_gate_time("PhaseShiftedControlledZ", control_0, control_1)
                        .is_some()
                    && self
                        .two_qubit_gate_time("PhaseShiftedControlledZ", control_1, target)
                        .is_some()
                {
                    Some(1e-6)
                } else {
                    None
                }
            }
            "ControlledControlledPhaseShift" => {
                if self.allow_ccp_gate
                    && self
                        .two_qubit_gate_time("PhaseShiftedControlledPhase", control_0, target)
                        .is_some()
                    && self
                        .two_qubit_gate_time("PhaseShiftedControlledPhase", control_0, control_1)
                        .is_some()
                    && self
                        .two_qubit_gate_time("PhaseShiftedControlledPhase", control_1, target)
                        .is_some()
                {
                    Some(1e-6)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns the gate time of a multi qubit operation on this device.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of a multi qubit gate as defined in roqoqo.
    /// * `qubits` - The qubits the gate acts on.
    ///
    /// # Returns
    ///
    /// * `Some<f64>` - The gate time.
    /// * `None` - The gate is not available on the device. Current default option.
    ///
    #[allow(unused_variables)]
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        // If any qubit is not in device operation is not available
        None
    }

    /// Returns the matrix of the decoherence rates of the Lindblad equation.
    ///
    /// $$
    /// \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \} \\\\
    ///     L_0 = \sigma^{+} \\\\
    ///     L_1 = \sigma^{-} \\\\
    ///     L_3 = \sigma^{z}
    /// $$
    ///
    /// # Arguments
    ///
    /// * `qubit` - The qubit for which the rate matrix M is returned.
    ///
    /// # Returns
    ///
    /// * `Some<Array2<f64>>` - The decoherence rates.
    /// * `None` - The qubit is not part of the device.
    ///
    #[allow(unused_variables)]
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        // At the moment we hard-code a noise free model
        Some(Array2::zeros((3, 3).to_owned()))
    }

    /// Returns the number of qubits the device supports.
    ///
    /// # Returns
    ///
    /// The number of qubits in the device.
    ///
    fn number_qubits(&self) -> usize {
        30
    }

    /// Returns the list of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    /// A pair of qubits is considered linked by a native two-qubit-gate if the device
    /// can implement a two-qubit-gate between the two qubits without decomposing it
    /// into a sequence of gates that involves a third qubit of the device.
    /// The two-qubit-gate also has to form a universal set together with the available
    /// single qubit gates.
    ///
    /// The returned vectors is a simple, graph-library independent, representation of
    /// the undirected connectivity graph of the device.
    /// It can be used to construct the connectivity graph in a graph library of the users
    /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
    ///
    /// # Returns
    ///
    /// A list (Vec) of pairs of qubits linked with a native two-qubit-gate in the device.
    ///
    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for row in 0..self.number_qubits() {
            for column in row + 1..self.number_qubits() {
                if self
                    .two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                    .is_some()
                {
                    edges.push((row, column));
                }
            }
        }
        edges
    }

    /// Changes the device topology based on a Pragma operation.
    ///
    /// # Arguments
    ///
    /// * `hqslang` - The hqslang name of the wrapped operation as defined in roqoqo.
    /// * `operation` - The Pragma operation encoded in binary form using the [bincode] crate.
    ///
    /// # Returns
    ///
    /// Result of changing the device.
    /// This device is not allowed to be changed and thus a generic RoqoqoBackendError is returned.
    ///
    fn change_device(
        &mut self,
        _hqslang: &str,
        _operation: &[u8],
    ) -> Result<(), RoqoqoBackendError> {
        Err(RoqoqoBackendError::GenericError {
            msg: "Wrapped operation not supported in QRydAPIDevice".to_string(),
        })
    }

    /// Turns Device into GenericDevice
    ///
    /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
    /// (for example when the interface needs to be serialized)
    ///
    /// # Notes
    ///
    /// GenericDevice uses nested HashMaps to represent the most general device connectivity.
    /// The memory usage will be inefficient for devices with large qubit numbers.
    ///
    /// # Returns
    ///
    /// * `GenericDevice` - The device in generic representation
    ///
    fn to_generic_device(&self) -> GenericDevice {
        let mut new_generic_device = GenericDevice::new(self.number_qubits());

        for gate_name in ["PhaseShiftState1", "RotateX", "RotateY", "RotateXY"] {
            for qubit in 0..self.number_qubits() {
                new_generic_device
                    .set_single_qubit_gate_time(
                        gate_name,
                        qubit,
                        self.single_qubit_gate_time(gate_name, &qubit).unwrap(),
                    )
                    .unwrap();
            }
        }
        for qubit in 0..self.number_qubits() {
            new_generic_device
                .set_qubit_decoherence_rates(qubit, self.qubit_decoherence_rates(&qubit).unwrap())
                .unwrap();
        }
        for row in 0..self.number_qubits() {
            for column in row + 1..self.number_qubits() {
                if self
                    .two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                    .is_some()
                {
                    new_generic_device
                        .set_two_qubit_gate_time(
                            "PhaseShiftedControlledZ",
                            row,
                            column,
                            self.two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                                .unwrap(),
                        )
                        .unwrap();
                    new_generic_device
                        .set_two_qubit_gate_time(
                            "PhaseShiftedControlledZ",
                            column,
                            row,
                            self.two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                                .unwrap(),
                        )
                        .unwrap();
                }
            }
        }
        new_generic_device
    }
}
