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

//! Qoqo quantum operations for quantum computers
//!
//! Quantum programs are represented by linear sequences of quantum operations

use bincode::{deserialize, serialize};
use pyo3::exceptions::PyRuntimeError;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::types::PySet;
use qoqo::operations::PragmaChangeDeviceWrapper;
use roqoqo::prelude::*;
use roqoqo_qryd::{PragmaChangeQRydLayout, PragmaShiftQRydQubit};
use std::collections::HashMap;

#[pyclass(
    name = "PragmaChangeQRydLayout",
    module = "qoqo_qryd.pragma_operations"
)]
#[pyo3(text_signature = "(new_layout, /)")]
#[derive(Clone, Debug, PartialEq)]
/// This PRAGMA operation changes the layout of a QRyd device.
///
/// Before running a circuit a number of layouts can be registered
/// in the device with the `add_layout` method.
///
/// This PRAGMA operation switches between the predefined operations.
///
/// Args:
///     new_layout (int): The index of the new layout.
pub struct PragmaChangeQRydLayoutWrapper {
    /// PragmaStateVector to be wrapped and converted to Python.
    pub internal: PragmaChangeQRydLayout,
}

#[pymethods]
impl PragmaChangeQRydLayoutWrapper {
    /// Create a PragmaChangeQRydLayout.
    ///
    /// Args:
    ///     new_layout (int): The new layout the device is changed to.
    ///
    /// Returns:
    ///     self: The new PragmaChangeQRydLayout.
    #[new]
    fn new(new_layout: usize) -> Self {
        Self {
            internal: PragmaChangeQRydLayout::new(new_layout),
        }
    }

    /// Return the index of the new layout the Pragma changes the device to.
    ///
    /// Returns:
    ///     int: The index of the layout.
    fn new_layout(&self) -> usize {
        *self.internal.new_layout()
    }

    /// Wrap PragmaChangeQRydLayout in PragmaChangeDevice operation
    ///
    /// PragmaChangeQRydLayout is device specific and can not be directly added to a Circuit.
    /// Instead it is first wrapped in a PragmaChangeDevice operation that is in turn added
    /// to the circuit.
    ///
    /// Example
    /// -------
    ///
    /// >>> from qoqo import Circuit
    /// ... from qoqo_qryd.pragma_operations import PragmaChangeQRydLayout
    /// ... circuit = Circuit()
    /// ... circuit += PragmaChangeQRydLayout(new_layout=1).to_pragma_change_device()
    ///
    /// Returns:
    ///     PragmaChangeDevice
    pub fn to_pragma_change_device(&self) -> PyResult<PragmaChangeDeviceWrapper> {
        Ok(PragmaChangeDeviceWrapper {
            internal: self.internal.to_pragma_change_device().map_err(|err| {
                PyRuntimeError::new_err(format!(
                    "Error occured during serialisation of PragmaShiftQRydQubit {:?}",
                    err
                ))
            })?,
        })
    }

    /// List all involved qubits (here, all).
    ///
    /// Returns:
    ///     set[int]: The involved qubits of the PRAGMA operation.
    fn involved_qubits(&self) -> PyObject {
        let pyobject: PyObject =
            Python::with_gil(|py| -> PyObject { PySet::new(py, &["All"]).unwrap().to_object(py) });
        pyobject
    }

    /// Return the bincode representation of the PragmaChangeQRydLayout using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized Circuit (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize PragmaChangeQRydLayout to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize Circuit to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the PragmaChangeQRydLayout to a PragmaChangeQRydLayout using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized PragmaChangeQRydLayout (in bincode form).
    ///
    /// Returns:
    ///     PragmaChangeQRydLayout: The deserialized PragmaChangeQRydLayout.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to PragmaChangeQRydLayout.
    pub fn from_bincode(&self, input: &PyAny) -> PyResult<PragmaChangeQRydLayoutWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(PragmaChangeQRydLayoutWrapper {
            internal: deserialize(&bytes[..])
                .map_err(|_| PyValueError::new_err("Input cannot be deserialized to Circuit"))?,
        })
    }

    /// Return tags classifying the type of the operation.
    ///
    /// Used for the type based dispatch in ffi interfaces.
    ///
    /// Returns:
    ///     list[str]: The tags of the operation.
    fn tags(&self) -> Vec<String> {
        self.internal.tags().iter().map(|s| s.to_string()).collect()
    }

    /// Return hqslang name of the operation.
    ///
    /// Returns:
    ///     str: The hqslang name of the operation.
    fn hqslang(&self) -> &'static str {
        self.internal.hqslang()
    }

    /// Return true when the operation has symbolic parameters.
    ///
    /// Returns:
    ///     bool: True if the operation contains symbolic parameters, False if it does not.
    fn is_parametrized(&self) -> bool {
        self.internal.is_parametrized()
    }

    /// Substitute the symbolic parameters in a clone of the PRAGMA operation according to the substitution_parameters input.
    ///
    /// Args:
    ///     substitution_parameters (dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.
    ///
    /// Returns:
    ///     self: The PRAGMA operation operation with the parameters substituted.
    ///
    /// Raises:
    ///     RuntimeError: The parameter substitution failed.
    #[pyo3(text_signature = "(substitution_parameters, /)")]
    fn substitute_parameters(
        &self,
        substitution_parameters: std::collections::HashMap<&str, f64>,
    ) -> PyResult<Self> {
        let mut calculator = qoqo_calculator::Calculator::new();
        for (key, val) in substitution_parameters.iter() {
            calculator.set_variable(key, *val);
        }
        Ok(Self {
            internal: self
                .internal
                .substitute_parameters(&calculator)
                .map_err(|x| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "Parameter Substitution failed: {:?}",
                        x
                    ))
                })?,
        })
    }

    /// Remap qubits in a clone of the PRAGMA operation.
    ///
    /// Args:
    ///     mapping (dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.
    ///
    /// Returns:
    ///     self: The PRAGMA operation with the qubits remapped.
    ///
    /// Raises:
    ///     RuntimeError: The qubit remapping failed.
    #[pyo3(text_signature = "(mapping, /)")]
    fn remap_qubits(&self, mapping: std::collections::HashMap<usize, usize>) -> PyResult<Self> {
        let new_internal = self
            .internal
            .remap_qubits(&mapping)
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Qubit remapping failed: "))?;
        Ok(Self {
            internal: new_internal,
        })
    }

    /// Return a copy of the PRAGMA operation (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     PragmaChangeQRydLayout: A deep copy of self.
    fn __copy__(&self) -> PragmaChangeQRydLayoutWrapper {
        self.clone()
    }

    /// Return a deep copy of the PRAGMA operation.
    ///
    /// Returns:
    ///     PragmaChangeQRydLayout: A deep copy of self.
    fn __deepcopy__(&self, _memodict: Py<PyAny>) -> PragmaChangeQRydLayoutWrapper {
        self.clone()
    }

    /// Return a string containing a formatted (string) representation of the PRAGMA operation.
    ///
    /// Returns:
    ///     str: The string representation of the operation.
    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        Ok(format!("{:?}", self.internal))
    }

    /// Return a string containing a printable representation of the PRAGMA operation.
    ///
    /// Returns:
    ///     str: The printable string representation of the operation.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.internal))
    }

    /// Return the __richcmp__ magic method to perform rich comparison operations on PragmaChangeQRydLayout.
    ///
    /// Args:
    ///     self: The PragmaChangeQRydLayout object.
    ///     other: The object to compare self to.
    ///     op: Whether they should be equal or not.
    ///
    /// Returns:
    ///     bool: Whether the two operations compared evaluated to True or False.
    fn __richcmp__(&self, other: Py<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
        let other: PragmaChangeQRydLayoutWrapper =
            Python::with_gil(|py| -> PyResult<PragmaChangeQRydLayoutWrapper> {
                let other_extracted: PyResult<PragmaChangeQRydLayoutWrapper> = other.extract(py);
                other_extracted
            })?;
        match op {
            pyo3::class::basic::CompareOp::Eq => Ok(self.internal == other.internal),
            pyo3::class::basic::CompareOp::Ne => Ok(self.internal != other.internal),
            _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "Other comparison not implemented.",
            )),
        }
    }
}

#[pyclass(name = "PragmaShiftQRydQubit", module = "qoqo_qryd.pragma_operations")]
#[pyo3(text_signature = "(new_positions, /)")]
#[derive(Clone, Debug, PartialEq)]
/// This PRAGMA operation shifts qubits between tweezer positions.
///
/// The tweezer positions in a FirstQryd device do not all have to be occupied.
/// In a partially occupied device the qubits can be shifted between positions inside a row.
/// The shift is defined by giving a mapping of qubit number and new row-column positions.
///
/// Args:
///     new_positions (Dict[int, (int, int)]): The new positions of the qubits.
pub struct PragmaShiftQRydQubitWrapper {
    /// PragmaShiftQRydQubit to be wrapped and converted to Python.
    pub internal: PragmaShiftQRydQubit,
}

#[pymethods]
impl PragmaShiftQRydQubitWrapper {
    /// Create a PragmaChangeQRydLayout.
    ///
    /// Args:
    ///     new_positions (Dict[int, (int, int)]): The new positions of the qubits.
    ///
    /// Returns:
    ///     self: The new PragmaChangeQRydLayout.
    #[new]
    fn new(new_positions: HashMap<usize, (usize, usize)>) -> Self {
        Self {
            internal: PragmaShiftQRydQubit::new(new_positions),
        }
    }

    /// Return the map of qubit numbers to new positions in the QRyd device.
    ///
    /// The new positions are the
    ///
    /// Returns:
    ///     Dict[int, (int, int)]: Map of qubits to new positions in the 2d grid.
    fn new_positions(&self) -> HashMap<usize, (usize, usize)> {
        self.internal.new_positions().clone()
    }

    /// Wrap PragmaShiftQRydQubit in PragmaChangeDevice operation
    ///
    /// PragmaShiftQRydQubit is device specific and can not be directly added to a Circuit.
    /// Instead it is first wrapped in a PragmaChangeDevice operation that is in turn added
    /// to the circuit.
    ///
    /// Example
    /// -------
    ///
    /// >>> from qoqo import Circuit
    /// ... from qoqo_qryd.pragma_operations import PragmaShiftQRydQubit
    /// ... circuit = Circuit()
    /// ... circuit += PragmaShiftQRydQubit(new_layout=1).to_pragma_change_device()
    ///
    /// Returns:
    ///     PragmaChangeDevice
    pub fn to_pragma_change_device(&self) -> PyResult<PragmaChangeDeviceWrapper> {
        Ok(PragmaChangeDeviceWrapper {
            internal: self.internal.to_pragma_change_device().map_err(|err| {
                PyRuntimeError::new_err(format!(
                    "Error occured during serialisation of PragmaShiftQRydQubit {:?}",
                    err
                ))
            })?,
        })
    }

    /// List all involved qubits (here, all).
    ///
    /// Returns:
    ///     set[int]: The involved qubits of the PRAGMA operation.
    fn involved_qubits(&self) -> PyObject {
        let pyobject: PyObject =
            Python::with_gil(|py| -> PyObject { PySet::new(py, &["All"]).unwrap().to_object(py) });
        pyobject
    }

    /// Return tags classifying the type of the operation.
    ///
    /// Used for the type based dispatch in ffi interfaces.
    ///
    /// Returns:
    ///     list[str]: The tags of the operation.
    fn tags(&self) -> Vec<String> {
        self.internal.tags().iter().map(|s| s.to_string()).collect()
    }

    /// Return hqslang name of the operation.
    ///
    /// Returns:
    ///     str: The hqslang name of the operation.
    fn hqslang(&self) -> &'static str {
        self.internal.hqslang()
    }

    /// Return true when the operation has symbolic parameters.
    ///
    /// Returns:
    ///     bool: True if the operation contains symbolic parameters, False if it does not.
    fn is_parametrized(&self) -> bool {
        self.internal.is_parametrized()
    }

    /// Return the bincode representation of the PragmaShiftQRydQubit using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized PragmaShiftQRydQubit (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize PragmaShiftQRydQubit to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize PragmaShiftQRydQubit to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the PragmaShiftQRydQubit to a PragmaShiftQRydQubit using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized PragmaShiftQRydQubit (in bincode form).
    ///
    /// Returns:
    ///     PragmaShiftQRydQubit: The deserialized PragmaShiftQRydQubit.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to PragmaShiftQRydQubit.
    #[pyo3(text_signature = "(input, /)")]
    pub fn from_bincode(&self, input: &PyAny) -> PyResult<PragmaShiftQRydQubitWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(PragmaShiftQRydQubitWrapper {
            internal: deserialize(&bytes[..])
                .map_err(|_| PyValueError::new_err("Input cannot be deserialized to Circuit"))?,
        })
    }

    /// Substitute the symbolic parameters in a clone of the PRAGMA operation according to the substitution_parameters input.
    ///
    /// Args:
    ///     substitution_parameters (dict[str, float]): The dictionary containing the substitutions to use in the PRAGMA operation.
    ///
    /// Returns:
    ///     self: The PRAGMA operation operation with the parameters substituted.
    ///
    /// Raises:
    ///     RuntimeError: The parameter substitution failed.
    #[pyo3(text_signature = "(substitution_parameters, /)")]
    fn substitute_parameters(
        &self,
        substitution_parameters: std::collections::HashMap<&str, f64>,
    ) -> PyResult<Self> {
        let mut calculator = qoqo_calculator::Calculator::new();
        for (key, val) in substitution_parameters.iter() {
            calculator.set_variable(key, *val);
        }
        Ok(Self {
            internal: self
                .internal
                .substitute_parameters(&calculator)
                .map_err(|x| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "Parameter Substitution failed: {:?}",
                        x
                    ))
                })?,
        })
    }

    /// Remap qubits in a clone of the PRAGMA operation.
    ///
    /// Args:
    ///     mapping (dict[int, int]): The dictionary containing the {qubit: qubit} mapping to use in the PRAGMA operation.
    ///
    /// Returns:
    ///     self: The PRAGMA operation with the qubits remapped.
    ///
    /// Raises:
    ///     RuntimeError: The qubit remapping failed.
    #[pyo3(text_signature = "(mapping, /)")]
    fn remap_qubits(&self, mapping: std::collections::HashMap<usize, usize>) -> PyResult<Self> {
        let new_internal = self
            .internal
            .remap_qubits(&mapping)
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Qubit remapping failed: "))?;
        Ok(Self {
            internal: new_internal,
        })
    }

    /// Return a copy of the PRAGMA operation (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     PragmaShiftQRydQubit: A deep copy of self.
    fn __copy__(&self) -> PragmaShiftQRydQubitWrapper {
        self.clone()
    }

    /// Return a deep copy of the PRAGMA operation.
    ///
    /// Returns:
    ///     PragmaShiftQRydQubit: A deep copy of self.
    fn __deepcopy__(&self, _memodict: Py<PyAny>) -> PragmaShiftQRydQubitWrapper {
        self.clone()
    }

    /// Return a string containing a formatted (string) representation of the PRAGMA operation.
    ///
    /// Returns:
    ///     str: The string representation of the operation.
    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        Ok(format!("{:?}", self.internal))
    }

    /// Return a string containing a printable representation of the PRAGMA operation.
    ///
    /// Returns:
    ///     str: The printable string representation of the operation.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.internal))
    }

    /// Return the __richcmp__ magic method to perform rich comparison operations on PragmaShiftQRydQubit.
    ///
    /// Args:
    ///     self: The PragmaShiftQRydQubit object.
    ///     other: The object to compare self to.
    ///     op: Whether they should be equal or not.
    ///
    /// Returns:
    ///     bool: Whether the two operations compared evaluated to True or False.
    fn __richcmp__(&self, other: Py<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
        let other: PragmaShiftQRydQubitWrapper =
            Python::with_gil(|py| -> PyResult<PragmaShiftQRydQubitWrapper> {
                let other_extracted: PyResult<PragmaShiftQRydQubitWrapper> = other.extract(py);
                other_extracted
            })?;
        match op {
            pyo3::class::basic::CompareOp::Eq => Ok(self.internal == other.internal),
            pyo3::class::basic::CompareOp::Ne => Ok(self.internal != other.internal),
            _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "Other comparison not implemented.",
            )),
        }
    }
}

/// QRyd specific PragmaOperations that support changing the QRyd device during a circuit evaluation
///
/// .. autosummary::
///    :toctree: generated/
///
///    PragmaChangeQRydLayout
///    PragmaShiftQRydQubit
#[pymodule]
pub fn pragma_operations(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PragmaChangeQRydLayoutWrapper>()?;
    m.add_class::<PragmaShiftQRydQubitWrapper>()?;
    Ok(())
}
