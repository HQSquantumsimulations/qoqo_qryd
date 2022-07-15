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

//! Collection of roqoqo Pragma operations for QRyd devices.
//!
//! These Pragma operations are used to change QRyd devices mid circuit.

use bincode::serialize;
use roqoqo::operations::{
    InvolveQubits, InvolvedQubits, Operate, OperatePragma, PragmaChangeDevice, Substitute,
};
use roqoqo::{RoqoqoBackendError, RoqoqoError};
use std::collections::HashMap;

/// This PRAGMA Operation changes a QRyd device to a new predefined layout.
///
/// QRyd devices have a set of predefined tweezer position layouts set at the start of the circuit.
/// During circuit execution the device can be switched between the predefined layouts with this PRAGMA.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::OperatePragma,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct PragmaChangeQRydLayout {
    /// The index of the new layout the device is changed to.
    new_layout: usize,
}

impl Substitute for PragmaChangeQRydLayout {
    fn substitute_parameters(
        &self,
        _calculator: &qoqo_calculator::Calculator,
    ) -> Result<Self, RoqoqoError> {
        Ok(self.clone())
    }

    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        if let Some((index, _)) = mapping.iter().next() {
            Err(RoqoqoError::QubitMappingError { qubit: *index })
        } else {
            Ok(self.clone())
        }
    }
}

impl PragmaChangeQRydLayout {
    /// Wrap PragmaChangeQRydLayout in PragmaChangeDevice operation
    ///
    /// PragmaChangeQRydLayout is device specific and can not be directly added to a Circuit.
    /// Instead it is first wrapped in a PragmaChangeDevice operation that is in turn added
    /// to the circuit.
    pub fn to_pragma_change_device(&self) -> Result<PragmaChangeDevice, RoqoqoBackendError> {
        Ok(PragmaChangeDevice {
            wrapped_tags: self.tags().iter().map(|s| s.to_string()).collect(),
            wrapped_hqslang: self.hqslang().to_string(),
            wrapped_operation: serialize(&self).map_err(|err| {
                RoqoqoBackendError::GenericError {
                    msg: format!(
                        "Error occured during serialisation of PragmaChangeQRydLayout {:?}",
                        err
                    ),
                }
            })?,
        })
    }
}

// Implementing the InvolveQubits trait for PragmaChangeQRydLayout.
impl InvolveQubits for PragmaChangeQRydLayout {
    /// Lists all involved qubits (here, All).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaChangeQRydLayout: &[&str; 3] =
    &["Operation", "PragmaOperation", "PragmaChangeQRydLayout"];

/// This PRAGMA Operation changes the occupied potential qubit positions in a QRyd device.
///
/// In QRyd devices not all perdefined potential positions of qubits need to be occupied.
/// If not all potential positions are occupied qubits can be shifted between potential positions during the circuit execution.
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    roqoqo_derive::Operate,
    roqoqo_derive::OperatePragma,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct PragmaShiftQRydQubit {
    /// The new qubit positions in the row-column grid of the QRyd device.
    new_positions: HashMap<usize, (usize, usize)>,
}

impl Substitute for PragmaShiftQRydQubit {
    fn substitute_parameters(
        &self,
        _calculator: &qoqo_calculator::Calculator,
    ) -> Result<Self, RoqoqoError> {
        Ok(self.clone())
    }

    fn remap_qubits(&self, mapping: &HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
        let mut new_new_positions: HashMap<usize, (usize, usize)> =
            HashMap::with_capacity(self.new_positions.len());
        for (index, (row, col)) in self.new_positions.iter() {
            let new_index = mapping.get(index).unwrap_or(index);
            new_new_positions.insert(*new_index, (*row, *col));
        }
        Ok(Self {
            new_positions: new_new_positions,
        })
    }
}

impl PragmaShiftQRydQubit {
    /// Wrap PragmaShiftQRydQubit in PragmaChangeDevice operation
    ///
    /// PragmaShiftQRydQubit is device specific and can not be directly added to a Circuit.
    /// Instead it is first wrapped in a PragmaChangeDevice operation that is in turn added
    /// to the circuit.
    pub fn to_pragma_change_device(&self) -> Result<PragmaChangeDevice, RoqoqoBackendError> {
        Ok(PragmaChangeDevice {
            wrapped_tags: self.tags().iter().map(|s| s.to_string()).collect(),
            wrapped_hqslang: self.hqslang().to_string(),
            wrapped_operation: serialize(&self).map_err(|err| {
                RoqoqoBackendError::GenericError {
                    msg: format!(
                        "Error occured during serialisation of PragmaShiftQRydQubit {:?}",
                        err
                    ),
                }
            })?,
        })
    }
}

// Implementing the InvolveQubits trait for PragmaShiftQRydQubit.
impl InvolveQubits for PragmaShiftQRydQubit {
    /// Lists all involved qubits (here, All).
    fn involved_qubits(&self) -> InvolvedQubits {
        InvolvedQubits::All
    }
}

#[allow(non_upper_case_globals)]
const TAGS_PragmaShiftQRydQubit: &[&str; 3] =
    &["Operation", "PragmaOperation", "PragmaShiftQRydQubit"];
