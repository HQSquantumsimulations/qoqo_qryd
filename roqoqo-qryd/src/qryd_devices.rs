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

//! QRyd Devices
//!
//! Provides the devices that are used to execute quantum programs with the QRyd backend.
//! QRyd devices can be physical hardware or simulators.

use bincode::deserialize;
use itertools::Itertools;
use ndarray::Array2;
use std::collections::HashMap;
use std::str::FromStr;

use roqoqo::devices::{Device, GenericDevice};
use roqoqo::RoqoqoBackendError;

use crate::{phi_theta_relation, PragmaChangeQRydLayout, PragmaShiftQRydQubit};

/// Collection of all QRyd devices
///
/// At the moment only contains a prototype `FirstDevice` that showcases the fundamental desing
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum QRydDevice {
    /// Temporary name to be replaced by codeword or simulator device
    FirstDevice(FirstDevice),
}

impl QRydDevice {
    /// Returns the position of each qubit in the row-column grid of tweezer positions.
    pub fn qubit_positions(&self) -> &HashMap<usize, (usize, usize)> {
        match self {
            QRydDevice::FirstDevice(x) => x.qubit_positions(),
        }
    }
    /// Returns the number of rows of optical tweezers in the two-dimensional grid of potential qubit positions.
    pub fn number_rows(&self) -> usize {
        match self {
            QRydDevice::FirstDevice(x) => x.number_rows(),
        }
    }
    /// Returns the number of columns of optical tweezers in the two-dimensional grid of potential qubit positions.
    pub fn number_columns(&self) -> usize {
        match self {
            QRydDevice::FirstDevice(x) => x.number_columns(),
        }
    }

    /// Change the positions of the qubits in their rows.
    ///
    /// The occupation of the available tweezer positions can be changed.
    /// This allows us to chang the positions of the qubits in each row.
    ///
    /// # Arguments
    ///
    /// `new_positions` - The new column positions of the qubits, given as a map between qubits and new positions.
    pub fn change_qubit_positions(
        &mut self,
        new_positions: &HashMap<usize, (usize, usize)>,
    ) -> Result<(), RoqoqoBackendError> {
        match self {
            QRydDevice::FirstDevice(x) => x.change_qubit_positions(new_positions),
        }
    }

    /// Switch to a different pre-defined layout.
    ///
    /// # Arguments
    ///
    /// `layout_number` - The index of the new layout
    pub fn switch_layout(&mut self, layout_number: &usize) -> Result<(), RoqoqoBackendError> {
        match self {
            QRydDevice::FirstDevice(x) => x.switch_layout(layout_number),
        }
    }

    /// Returns the PhaseShiftedControlledZ phase shift according to the device's relation.
    pub fn phase_shift_controlled_z(&self) -> Option<f64> {
        match self {
            Self::FirstDevice(x) => x.phase_shift_controlled_z(),
        }
    }

    /// Returns the PhaseShiftedControlledPhase phase shift according to the device's relation.
    pub fn phase_shift_controlled_phase(&self, theta: f64) -> Option<f64> {
        match self {
            Self::FirstDevice(x) => x.phase_shift_controlled_phase(theta),
        }
    }

    /// Returns the gate time of a PhaseShiftedControlledZ operation with the given qubits and phi angle.
    pub fn gate_time_controlled_z(&self, control: &usize, target: &usize, phi: f64) -> Option<f64> {
        match self {
            Self::FirstDevice(x) => x.gate_time_controlled_z(control, target, phi),
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
            Self::FirstDevice(x) => x.gate_time_controlled_phase(control, target, phi, theta),
        }
    }

    /// Add a new layout to the device.
    ///
    /// A layout is a two-dimensional representation of the y-positions of the tweezers in each row.
    /// The x-position is fixed by the row-distance.
    ///
    /// # Arguments
    ///
    /// `layout_number` - The number index that is assigned to the new layout
    /// `layout` - The new layout that is added
    ///
    /// # Returns
    ///
    /// `Ok(Self)` - A clone of the device with the new layout added
    /// `Err(RoqoqoBackendError)` - The layout_number index is already in use
    ///                             or the layout does not fit the fixed row and column number
    pub fn add_layout(
        &self,
        layout_number: usize,
        layout: Array2<f64>,
    ) -> Result<Self, RoqoqoBackendError> {
        match self {
            QRydDevice::FirstDevice(x) => x
                .add_layout(layout_number, layout)
                .map(QRydDevice::FirstDevice),
        }
    }
}

impl Device for QRydDevice {
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        match self {
            Self::FirstDevice(d) => d.single_qubit_gate_time(hqslang, qubit),
        }
    }
    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        match self {
            Self::FirstDevice(d) => d.two_qubit_gate_time(hqslang, control, target),
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
            Self::FirstDevice(d) => d.three_qubit_gate_time(hqslang, control_0, control_1, target),
        }
    }
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        match self {
            Self::FirstDevice(d) => d.multi_qubit_gate_time(hqslang, qubits),
        }
    }
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        match self {
            Self::FirstDevice(d) => d.qubit_decoherence_rates(qubit),
        }
    }
    fn number_qubits(&self) -> usize {
        match self {
            Self::FirstDevice(d) => d.number_qubits(),
        }
    }

    fn change_device(&mut self, hqslang: &str, operation: &[u8]) -> Result<(), RoqoqoBackendError> {
        match self {
            Self::FirstDevice(d) => d.change_device(hqslang, operation),
        }
    }

    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        match self {
            Self::FirstDevice(d) => d.two_qubit_edges(),
        }
    }

    fn to_generic_device(&self) -> roqoqo::devices::GenericDevice {
        match self {
            Self::FirstDevice(d) => d.to_generic_device(),
        }
    }
}

impl From<&FirstDevice> for QRydDevice {
    fn from(input: &FirstDevice) -> Self {
        Self::FirstDevice(input.clone())
    }
}

impl From<FirstDevice> for QRydDevice {
    fn from(input: FirstDevice) -> Self {
        Self::FirstDevice(input)
    }
}

/// First Qryd Device
///
/// At the moment only a prototype that showcases the fundamental design.
#[doc(hidden)]
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct FirstDevice {
    /// Fixed number of rows in the optical lattice
    number_rows: usize,
    /// Fixed number of columns in the optical lattice
    number_columns: usize,
    /// Each numbered qubit is assigned to a position in the row-column grid.
    /// The first tuple value gives the integer index of the row, the second of the column.
    /// The data structure can handle arbitrary changes in occupation, but we enforce a fixed
    /// number of occupied tweezer positions per row.
    qubit_positions: HashMap<usize, (usize, usize)>,
    /// Distance between rows
    row_distance: f64,
    /// Positions of tweezers in each row
    layout_register: HashMap<usize, Array2<f64>>,
    /// The current chosen layout;
    current_layout: usize,
    /// The distance cut-off above which two-qubit gates are not possible
    cutoff: f64,
    /// The specific PhaseShiftedControlledZ relation to use.
    controlled_z_phase_relation: String,
    /// The specific PhaseShiftedControlledPhase relation to use.
    controlled_phase_phase_relation: String,
    // Controls if multi_qubit_operations are present
    // multi_qubit_operations: bool,
    /// Whether the device allows ControlledControlledPauliZ operations.
    allow_ccz_gate: bool,
    /// Whether the device allows ControlledControlledPhaseShift operations.
    allow_ccp_gate: bool,
}

impl FirstDevice {
    /// Create new `First` QRyd device.
    ///
    /// # Arguments
    ///
    /// * `number_rows` - The fixed number of rows in device, needs to be the same for all layouts.
    /// * `number_columns` - Fixed number of tweezers in each row, needs to be the same for all layouts.
    /// * `qubits_per_row` - Fixed number of occupied tweezer position in each row.
    ///                    At the moment assumes that number of qubits in the traps is fixed. No loading/unloading once device is created
    /// * `row_distance` - Fixed distance between rows.
    /// * `initial_layout` - The device needs at least one layout. After creation the device will be in this layout with layout number 0.
    /// * `controlled_z_phase_relation` - The relation to use for the PhaseShiftedControlledZ gate.
    ///                                   It can be hardcoded to a specific value if a float is passed in as String.
    /// * `controlled_phase_phase_relation` - The relation to use for the PhaseShiftedControlledPhase gate.
    /// * `allow_ccz_gate` - Whether to allow ControlledControlledPauliZ operations in the device.
    /// * `allow_ccp_gate` - Whether to allow ControlledControlledPhaseShift operations in the device.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        number_rows: usize,
        number_columns: usize,
        qubits_per_row: &[usize],
        row_distance: f64,
        initial_layout: Array2<f64>,
        controlled_z_phase_relation: Option<String>,
        controlled_phase_phase_relation: Option<String>,
        allow_ccz_gate: Option<bool>,
        allow_ccp_gate: Option<bool>,
    ) -> Result<Self, RoqoqoBackendError> {
        if qubits_per_row.len() != number_rows {
            return Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "Device has {} rows but for {} rows qubit numbers have been specified",
                    number_rows,
                    qubits_per_row.len()
                ),
            });
        }
        for (row, number_qubits_row) in qubits_per_row.iter().enumerate() {
            if number_qubits_row > &number_columns {
                return Err(RoqoqoBackendError::GenericError {
                    msg: format!(
                        "Device has {} columns but for column {}, {} qubit numbers have been specified",
                        number_columns, row, number_qubits_row
                    ),
                });
            }
        }
        let mut qubit_positions: HashMap<usize, (usize, usize)> = HashMap::new();
        let mut number_qubits: usize = 0;
        for (row, number_qubits_row) in qubits_per_row.iter().enumerate() {
            // add all qubits in a row
            for i in 0..*number_qubits_row {
                qubit_positions.insert(number_qubits + i, (row, i));
            }
            // count up the total qubit number
            number_qubits += number_qubits_row;
        }
        let layout_register: HashMap<usize, Array2<f64>> = HashMap::new();
        let current_layout = 0;
        let controlled_z_phase_relation =
            controlled_z_phase_relation.unwrap_or_else(|| "DefaultRelation".to_string());
        let controlled_phase_phase_relation =
            controlled_phase_phase_relation.unwrap_or_else(|| "DefaultRelation".to_string());
        let allow_ccz_gate = allow_ccz_gate.unwrap_or(true);
        let allow_ccp_gate = allow_ccp_gate.unwrap_or(false);
        let return_self = Self {
            number_rows,
            number_columns,
            qubit_positions,
            row_distance,
            layout_register,
            current_layout,
            cutoff: 1.0,
            controlled_z_phase_relation,
            controlled_phase_phase_relation,
            // multi_qubit_operations: true,
            allow_ccz_gate,
            allow_ccp_gate,
        }
        .add_layout(0, initial_layout)?;
        Ok(return_self)
    }

    /// Returns the number of qubits in the device.
    pub fn set_cutoff(&mut self, cutoff: f64) {
        self.cutoff = cutoff;
    }

    /// Returns the number of rows of optical tweezers in the two-dimensional grid of potential qubit positions.
    pub fn number_rows(&self) -> usize {
        self.number_rows
    }

    /// Returns the number of columns of optical tweezers in the two-dimensional grid of potential qubit positions.
    pub fn number_columns(&self) -> usize {
        self.number_columns
    }

    /// Returns the position of each qubit in the row-column grid of tweezer positions.
    pub fn qubit_positions(&self) -> &HashMap<usize, (usize, usize)> {
        &self.qubit_positions
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

    /// Add a new layout to the device.
    ///
    /// A layout is a two-dimensional representation of the y-positions of the tweezers in each row.
    /// The x-position is fixed by the row-distance.
    ///
    /// # Arguments
    ///
    /// `layout_number` - The number index that is assigned to the new layout
    /// `layout` - The new layout that is added
    ///
    /// # Returns
    ///
    /// `Ok(Self)` - A clone of the device with the new layout added
    /// `Err(RoqoqoBackendError)` - The layout_number index is already in use
    ///                             or the layout does not fit the fixed row and column number
    pub fn add_layout(
        &self,
        layout_number: usize,
        layout: Array2<f64>,
    ) -> Result<Self, RoqoqoBackendError> {
        if self.layout_register.contains_key(&layout_number) {
            return Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "Error adding layout to QRyd device layout key {} is already used for layout {:?}",
                    layout_number,
                    self.layout_register.get(&layout_number)
                ),
            });
        }
        if layout.ncols() != self.number_columns() || layout.nrows() != self.number_rows() {
            return Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "Error adding layout to QRyd device new layout {} rows and {} columns required",
                    self.number_rows(),
                    self.number_columns()
                ),
            });
        }
        let mut self_clone = self.clone();
        self_clone.layout_register.insert(layout_number, layout);
        Ok(self_clone)
    }

    /// Switch to a different pre-defined layout.
    ///
    /// # Arguments
    ///
    /// `layout_number` - The number index of the new layout
    pub fn switch_layout(&mut self, layout_number: &usize) -> Result<(), RoqoqoBackendError> {
        if self.layout_register.contains_key(layout_number) {
            self.current_layout = *layout_number;
            Ok(())
        } else {
            Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "Error switching layout of QRyd device. Layout {} is not set",
                    layout_number
                ),
            })
        }
    }

    /// Change the positions of the qubits in their rows.
    ///
    /// The occupation of the available tweezer positions can be changed.
    /// This allows us to chang the positions of the qubits in each row
    ///
    /// # Arguments
    ///
    /// `new_positions` - The new column positions of the qubits, given as a map between qubits and new positions.
    ///                   While the new positions are
    pub fn change_qubit_positions(
        &mut self,
        new_positions: &HashMap<usize, (usize, usize)>,
    ) -> Result<(), RoqoqoBackendError> {
        for (qubit, (old_row, _)) in self.qubit_positions.iter() {
            let (new_row, _) =
                new_positions
                    .get(qubit)
                    .ok_or(RoqoqoBackendError::GenericError {
                        msg: format!("Qubit {} is missing from new qubit positions", qubit),
                    })?;
            if old_row != new_row {
                return Err(RoqoqoBackendError::GenericError {
                    msg: format!("New qubit positions has a mismatch in rows for qubit {} old row {} new row {}", qubit, old_row, new_row)});
            }
        }

        if new_positions
            .keys()
            .any(|k| !self.qubit_positions.contains_key(k))
        {
            return Err(RoqoqoBackendError::GenericError {
                msg: "There are additional keys in the new_positions input which do not exist in the old qubit positions".to_string()
            });
        }

        // Change the qubit positions if no error has been found
        self.qubit_positions = new_positions.clone();
        Ok(())
    }
}

impl Device for FirstDevice {
    fn single_qubit_gate_time(&self, hqslang: &str, qubit: &usize) -> Option<f64> {
        // The availability of gates is checked by returning Some
        // When a gate is not available simply return None
        // Check if the qubit is even in the device
        if !self.qubit_positions().contains_key(qubit) {
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

    fn two_qubit_gate_time(&self, hqslang: &str, control: &usize, target: &usize) -> Option<f64> {
        // Check for availability of control and target on device
        if !self.qubit_positions().contains_key(control) {
            return None;
        }
        if !self.qubit_positions().contains_key(target) {
            return None;
        }

        // Check if a layout has been selected and already prepare layout
        let layout = self
            .layout_register
            .get(&self.current_layout)
            .expect("Unexpectedly did not find current layout. Bug in roqoqo-qryd");
        // Check for type of gate (as well as checking phi-theta relation)
        match hqslang {
            "PhaseShiftedControlledZ" => (),
            "PhaseShiftedControlledPhase" => (),
            _ => return None,
        }
        let control_position = self
            .qubit_positions
            .get(control)
            .expect("Internal error entry in hashmap that was already checked not found");
        let target_position = self
            .qubit_positions
            .get(target)
            .expect("Internal error entry in hashmap that was already checked not found");
        // The following is just an example of how the availability of gates and the gate time could be calculated based on a simple theoretical model (using physical distance)
        // For the actual device  more complex models or a lookup of callibration data can be performed instead
        // Calculate the physical distance
        let x_distance = layout[*control_position] - layout[*target_position];
        let y_distance =
            self.row_distance * ((control_position.0 as isize - target_position.0 as isize) as f64);
        let total_distance = (x_distance.powi(2) + y_distance.powi(2)).sqrt();
        if total_distance > self.cutoff {
            None
        } else {
            // Example of gate time dependence on distance. Here gate time increases with the square of the distance.
            Some(2e-6 * total_distance.powi(2))
        }
    }

    #[allow(unused_variables)]
    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        control_0: &usize,
        control_1: &usize,
        target: &usize,
    ) -> Option<f64> {
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

    #[allow(unused_variables)]
    fn multi_qubit_gate_time(&self, hqslang: &str, qubits: &[usize]) -> Option<f64> {
        // if !self.multi_qubit_operations {
        //     return None;
        // }
        // If any qubit is not in device operation is not available
        if qubits
            .iter()
            .any(|q| !self.qubit_positions().contains_key(q))
        {
            return None;
        };
        // We assume the native multi-qubit-gate is a rotation under a product of Pauli Z operators
        match hqslang {
            "MultiQubitZZ" => (),
            _ => return None,
        };
        // Return a time if all qubits are in the same row
        let row = self
            .qubit_positions
            .get(&qubits[0])
            .expect("Internal error, qubit unexpectedly not found in qubit positions map")
            .0;
        if qubits.iter().all(|q| {
            row == self
                .qubit_positions
                .get(q)
                .expect("Internal error, qubit unexpectedly not found in qubit positions map")
                .0
        }) {
            // Hardcoding a value for example
            Some(2e-5)
        } else {
            None
        }
    }

    #[allow(unused_variables)]
    fn qubit_decoherence_rates(&self, qubit: &usize) -> Option<Array2<f64>> {
        // At the moment we hard-code a noise free model
        Some(Array2::zeros((3, 3).to_owned()))
    }

    fn number_qubits(&self) -> usize {
        self.qubit_positions.len()
    }

    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for row in 0..self.number_qubits() {
            for column in row + 1..self.number_qubits() {
                if self
                    .two_qubit_gate_time("PhaseShiftedControlledPhase", &row, &column)
                    .is_some()
                {
                    edges.push((row, column));
                }
            }
        }
        edges
    }

    fn change_device(&mut self, hqslang: &str, operation: &[u8]) -> Result<(), RoqoqoBackendError> {
        match hqslang {
            "PragmaChangeQRydLayout" => {
                let de_change_layout: Result<PragmaChangeQRydLayout, Box<bincode::ErrorKind>> =
                    deserialize(operation);
                match de_change_layout {
                    Ok(pragma) => self.switch_layout(pragma.new_layout()),
                    Err(_) => Err(RoqoqoBackendError::GenericError {
                        msg: "Wrapped operation not supported in QRydDevice".to_string(),
                    }),
                }
            }
            "PragmaShiftQRydQubit" => {
                let de_shift: Result<PragmaShiftQRydQubit, Box<bincode::ErrorKind>> =
                    deserialize(operation);
                match de_shift {
                    Ok(pragma) => self.change_qubit_positions(pragma.new_positions()),
                    Err(_) => Err(RoqoqoBackendError::GenericError {
                        msg: "Wrapped operation not supported in QRydDevice".to_string(),
                    }),
                }
            }
            _ => Err(RoqoqoBackendError::GenericError {
                msg: "Wrapped operation not supported in QRydDevice".to_string(),
            }),
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
                    .two_qubit_gate_time("PhaseShiftedControlledPhase", &row, &column)
                    .is_some()
                {
                    new_generic_device
                        .set_two_qubit_gate_time(
                            "PhaseShiftedControlledPhase",
                            row,
                            column,
                            self.two_qubit_gate_time("PhaseShiftedControlledPhase", &row, &column)
                                .unwrap(),
                        )
                        .unwrap();
                    new_generic_device
                        .set_two_qubit_gate_time(
                            "PhaseShiftedControlledPhase",
                            column,
                            row,
                            self.two_qubit_gate_time("PhaseShiftedControlledPhase", &row, &column)
                                .unwrap(),
                        )
                        .unwrap();
                }
            }
        }
        // if self.multi_qubit_operations {
        for row in 0..self.number_rows() {
            let potential_qubits: Vec<usize> = self
                .qubit_positions()
                .iter()
                .filter_map(
                    |(qubit, (qrow, _column))| if qrow == &row { Some(*qubit) } else { None },
                )
                .collect();
            let max_length = potential_qubits.len();
            for combination_length in 3..=max_length {
                for qubits in potential_qubits
                    .iter()
                    .copied()
                    .combinations(combination_length)
                {
                    new_generic_device
                        .set_multi_qubit_gate_time("MultiQubitZZ", qubits, 2e-5)
                        .unwrap();
                }
            }
            // }
        }
        new_generic_device
    }
}
