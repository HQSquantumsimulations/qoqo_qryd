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
// limitations under the License.use bincode::{deserialize, serialize};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo::devices::GenericDeviceWrapper;
use qoqo::QoqoBackendError;
use qoqo_calculator_pyo3::convert_into_calculator_float;
use roqoqo::devices::Device;
use roqoqo_qryd::api_devices::{QRydAPIDevice, QrydEmuSquareDevice, QrydEmuTriangularDevice};

/// QRyd quantum device having a squared configuration.
///
/// Provides an emulated quantum computing device with up to 30 qubits
/// that can be accessed via the QRyd WebAPI.
///
/// Args:
///     seed (int): Seed, if not provided will be set to 0 per default (not recommended!)
///     controlled_z_phase_relation (Optinal[Union[str, float]]): The String used to choose what kind of phi-theta relation
///                                                 to use for the PhaseShiftedControlledZ gate
///     controlled_phase_phase_relation (Optinal[Union[str, float]]): The String used to choose what kind of phi-theta relation
///                                                     to use for the PhaseShiftedControlledPhase gate
#[pyclass(name = "QrydEmuSquareDevice", module = "qoqo_qryd")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QrydEmuSquareDeviceWrapper {
    /// Internal storage of [roqoqo_qryd::QrydEmuSquareDevice]
    pub internal: QrydEmuSquareDevice,
}

#[pymethods]
impl QrydEmuSquareDeviceWrapper {
    /// Create new QrydEmuSquareDevice device
    ///
    /// Args:
    ///     seed (int): Seed, if not provided will be set to 0 per default (not recommended!)
    ///     controlled_z_phase_relation (Optinal[Union[str, float]]): The String used to choose what kind of phi-theta relation
    ///                                                 to use for the PhaseShiftedControlledZ gate
    ///     controlled_phase_phase_relation (Optinal[Union[str, float]]): The String used to choose what kind of phi-theta relation
    ///                                                     to use for the PhaseShiftedControlledPhase gate
    ///
    /// Returns:
    ///     QrydEmuSquareDevice: New device
    #[new]
    #[pyo3(
        text_signature = "(seed, controlled_z_phase_relation, controlled_phase_phase_relation, /)"
    )]
    pub fn new(
        seed: Option<usize>,
        controlled_z_phase_relation: Option<&PyAny>,
        controlled_phase_phase_relation: Option<&PyAny>,
    ) -> Self {
        let czpr = if let Some(value) = controlled_z_phase_relation {
            if convert_into_calculator_float(value).is_ok() {
                Some(convert_into_calculator_float(value).unwrap().to_string())
            } else {
                Some(
                    controlled_z_phase_relation
                        .unwrap()
                        .extract::<String>()
                        .unwrap(),
                )
            }
        } else {
            None
        };
        let cppr = if let Some(value) = controlled_phase_phase_relation {
            if convert_into_calculator_float(value).is_ok() {
                Some(convert_into_calculator_float(value).unwrap().to_string())
            } else {
                Some(
                    controlled_phase_phase_relation
                        .unwrap()
                        .extract::<String>()
                        .unwrap(),
                )
            }
        } else {
            None
        };
        Self {
            internal: QrydEmuSquareDevice::new(seed, czpr, cppr),
        }
    }

    /// Turns Device into GenericDevice
    ///
    /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
    /// (for example when the interface needs to be serialized)
    ///
    /// Returns:
    ///     GenericDevice: The device in generic representation
    ///
    /// Note:
    ///     GenericDevice uses nested HashMaps to represent the most general device connectivity.
    ///     The memory usage will be inefficient for devices with large qubit numbers.
    fn generic_device(&self) -> GenericDeviceWrapper {
        GenericDeviceWrapper {
            internal: self.internal.to_generic_device(),
        }
    }

    /// Return a copy of the QRydAPIDevice (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     QRydAPIDevice: A deep copy of self.
    pub fn __copy__(&self) -> QrydEmuSquareDeviceWrapper {
        self.clone()
    }

    /// Return a deep copy of the QRydAPIDevice.
    ///
    /// Returns:
    ///     QRydAPIDevice: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> QrydEmuSquareDeviceWrapper {
        self.clone()
    }

    /// Return the bincode representation of the QrydEmuSquareDevice using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized QrydEmuSquareDevice (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize QrydEmuSquareDevice to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = bincode::serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize QrydEmuSquareDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the QrydEmuSquareDevice to a QrydEmuSquareDevice using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized QrydEmuSquareDevice (in bincode form).
    ///
    /// Returns:
    ///     QrydEmuSquareDevice: The deserialized QrydEmuSquareDevice.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to QrydEmuSquareDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    pub fn from_bincode(input: &PyAny) -> PyResult<QrydEmuSquareDeviceWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(QrydEmuSquareDeviceWrapper {
            internal: bincode::deserialize(&bytes[..]).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to QrydEmuSquareDevice")
            })?,
        })
    }

    /// Return the json representation of the QrydEmuSquareDevice.
    ///
    /// Returns:
    ///     str: The serialized form of QrydEmuSquareDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize QrydEmuSquareDevice to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize QrydEmuSquareDevice to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a QrydEmuSquareDevice to a QrydEmuSquareDevice.
    ///
    /// Args:
    ///     input (str): The serialized QrydEmuSquareDevice in json form.
    ///
    /// Returns:
    ///     QrydEmuSquareDevice: The deserialized QrydEmuSquareDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to QrydEmuSquareDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    fn from_json(input: &str) -> PyResult<QrydEmuSquareDeviceWrapper> {
        Ok(QrydEmuSquareDeviceWrapper {
            internal: serde_json::from_str(input).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to QrydEmuSquareDevice")
            })?,
        })
    }

    /// Return number of qubits in device.
    ///
    /// Returns:
    ///     int: The number of qubits.
    pub fn number_qubits(&self) -> usize {
        self.internal.number_qubits()
    }

    /// Return the bincode representation of the Enum variant of the Device.
    ///
    /// Only used for internal interfacing.
    ///
    /// Returns:
    ///     ByteArray: The serialized QrydDevice (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize Device to bytes.
    pub fn _enum_to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let qryd_enum: QRydAPIDevice = (&self.internal).into();
        let serialized = bincode::serialize(&qryd_enum)
            .map_err(|_| PyValueError::new_err("Cannot serialize QrydEmuSquareDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Return the list of pairs of qubits linked by a native two-qubit-gate in the device.
    ///
    /// A pair of qubits is considered linked by a native two-qubit-gate if the device
    /// can implement a two-qubit-gate between the two qubits without decomposing it
    /// into a sequence of gates that involves a third qubit of the device.
    /// The two-qubit-gate also has to form a universal set together with the available
    /// single qubit gates.
    ///
    /// The returned vectors is a simple, graph-library independent, representation of
    /// the undirected connectivity graph of the device.
    /// It can be used to construct the connectivity graph in a graph library of the user's
    /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
    ///
    /// Example
    /// -------
    ///
    /// To construct a networkx graph from this output one can use
    ///
    /// >>> import networkx as nx
    /// ... from qoqo_qryd import QrydEmuSquareDevice
    /// ...
    /// ... device = QrydEmuSquareDevice(number_rows=2,number_columns= 2 qubits_per_row=[2,2], row_distance=1.0)
    /// ... edges = device.two_qubit_edges()
    /// ... graph = nx.Graph()
    /// ... graph.add_edges_from(edges)
    ///
    ///
    /// Returns:
    ///     Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        self.internal.two_qubit_edges()
    }

    /// Returns the backend associated with the device.
    pub fn qrydbackend(&self) -> String {
        self.internal.qrydbackend()
    }

    /// Returns the seed usized for the API.
    pub fn seed(&self) -> usize {
        self.internal.seed()
    }

    /// Returns the gate time of a single qubit operation on this device.
    ///
    /// Returns:
    ///     f64: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available in the device.
    pub fn single_qubit_gate_time(&self, hqslang: &str, qubit: usize) -> PyResult<f64> {
        self.internal
            .single_qubit_gate_time(hqslang, &qubit)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the gate time of a two qubit operation on this device.
    ///
    /// Returns:
    ///     f64: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available in the device.
    pub fn two_qubit_gate_time(
        &self,
        hqslang: &str,
        control: usize,
        target: usize,
    ) -> PyResult<f64> {
        self.internal
            .two_qubit_gate_time(hqslang, &control, &target)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the gate time of a three qubit operation on this device.
    ///
    /// Returns:
    ///     f64: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available in the device.
    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        control_0: usize,
        control_1: usize,
        target: usize,
    ) -> PyResult<f64> {
        self.internal
            .three_qubit_gate_time(hqslang, &control_0, &control_1, &target)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the gate time of a multi qubit operation on this device.
    ///
    /// Returns:
    ///     f64: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available in the device.
    pub fn multi_qubit_gate_time(&self, hqslang: &str, qubits: Vec<usize>) -> PyResult<f64> {
        self.internal
            .multi_qubit_gate_time(hqslang, &qubits)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the PhaseShiftedControlledZ phase shift according to the device's relation.
    pub fn phase_shift_controlled_z(&self) -> PyResult<f64> {
        self.internal
            .phase_shift_controlled_z()
            .ok_or_else(|| PyValueError::new_err("Error in relation selection."))
    }

    /// Returns the PhaseShiftedControlledPhase phase shift according to the device's relation.
    pub fn phase_shift_controlled_phase(&self, theta: f64) -> PyResult<f64> {
        self.internal
            .phase_shift_controlled_phase(theta)
            .ok_or_else(|| PyValueError::new_err("Error in relation selection."))
    }

    /// Returns the gate time of a PhaseShiftedControlledZ operation with the given qubits and phi angle.
    pub fn gate_time_controlled_z(&self, control: usize, target: usize, phi: f64) -> PyResult<f64> {
        self.internal
            .gate_time_controlled_z(&control, &target, phi)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the gate time of a PhaseShiftedControlledPhase operation with the given qubits and phi and theta angles.
    pub fn gate_time_controlled_phase(
        &self,
        control: usize,
        target: usize,
        phi: f64,
        theta: f64,
    ) -> PyResult<f64> {
        self.internal
            .gate_time_controlled_phase(&control, &target, phi, theta)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }
}

/// QRyd quantum device having a triangular configuration.
///
/// Provides an emulated quantum computing device with up to 30 qubits
/// that can be accessed via the QRyd WebAPI.
///
/// Args:
///     seed (int): Seed, if not provided will be set to 0 per default (not recommended!)
///     controlled_z_phase_relation (Optinal[Union[str, float]]): The String used to choose what kind of phi-theta relation
///                                                 to use for the PhaseShiftedControlledZ gate.
///     controlled_phase_phase_relation (Optinal[Union[str, float]]): The String used to choose what kind of phi-theta relation
///                                                     to use for the PhaseShiftedControlledPhase gate.
///     allow_ccz_gate (Optional[bool]): Whether to allow ControlledControlledPauliZ operations in the device.
///     allow_ccp_gate (Optional[bool]): Whether to allow ControlledControlledPhaseShift operations in the device.
#[pyclass(name = "QrydEmuTriangularDevice", module = "qoqo_qryd")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QrydEmuTriangularDeviceWrapper {
    /// Internal storage of [roqoqo_qryd::QrydEmuTriangularDevice]
    pub internal: QrydEmuTriangularDevice,
}

#[pymethods]
impl QrydEmuTriangularDeviceWrapper {
    /// Create new QrydEmuTriangularDevice device
    ///
    /// Args:
    ///     seed (int): Seed, if not provided will be set to 0 per default (not recommended!)
    ///     controlled_z_phase_relation (Optinal[Union[str, float]]): The String used to choose what kind of phi-theta relation
    ///                                                 to use for the PhaseShiftedControlledZ gate.
    ///     controlled_phase_phase_relation (Optinal[Union[str, float]]): The String used to choose what kind of phi-theta relation
    ///                                                     to use for the PhaseShiftedControlledPhase gate.
    ///     allow_ccz_gate (Optional[bool]): Whether to allow ControlledControlledPauliZ operations in the device.
    ///     allow_ccp_gate (Optional[bool]): Whether to allow ControlledControlledPhaseShift operations in the device.
    ///
    /// Returns:
    ///     QrydEmuTriangularDevice: New device
    #[new]
    #[pyo3(
        text_signature = "(seed, controlled_z_phase_relation, controlled_phase_phase_relation, allow_ccz_gate, allow_ccp_gate, /)"
    )]
    pub fn new(
        seed: Option<usize>,
        controlled_z_phase_relation: Option<&PyAny>,
        controlled_phase_phase_relation: Option<&PyAny>,
        allow_ccz_gate: Option<bool>,
        allow_ccp_gate: Option<bool>,
    ) -> Self {
        let czpr = if let Some(value) = controlled_z_phase_relation {
            if convert_into_calculator_float(value).is_ok() {
                Some(convert_into_calculator_float(value).unwrap().to_string())
            } else {
                Some(
                    controlled_z_phase_relation
                        .unwrap()
                        .extract::<String>()
                        .unwrap(),
                )
            }
        } else {
            None
        };
        let cppr = if let Some(value) = controlled_phase_phase_relation {
            if convert_into_calculator_float(value).is_ok() {
                Some(convert_into_calculator_float(value).unwrap().to_string())
            } else {
                Some(
                    controlled_phase_phase_relation
                        .unwrap()
                        .extract::<String>()
                        .unwrap(),
                )
            }
        } else {
            None
        };
        Self {
            internal: QrydEmuTriangularDevice::new(
                seed,
                czpr,
                cppr,
                allow_ccz_gate,
                allow_ccp_gate,
            ),
        }
    }

    /// Return a copy of the QRydAPIDevice (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     QRydAPIDevice: A deep copy of self.
    pub fn __copy__(&self) -> QrydEmuTriangularDeviceWrapper {
        self.clone()
    }

    /// Return a deep copy of the QRydAPIDevice.
    ///
    /// Returns:
    ///     QRydAPIDevice: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> QrydEmuTriangularDeviceWrapper {
        self.clone()
    }

    /// Return the bincode representation of the QrydEmuTriangularDevice using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized QrydEmuTriangularDevice (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize QrydEmuTriangularDevice to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = bincode::serialize(&self.internal).map_err(|_| {
            PyValueError::new_err("Cannot serialize QrydEmuTriangularDevice to bytes")
        })?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Turns Device into GenericDevice
    ///
    /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
    /// (for example when the interface needs to be serialized)
    ///
    /// Returns:
    ///     GenericDevice: The device in generic representation
    ///
    /// Note:
    ///     GenericDevice uses nested HashMaps to represent the most general device connectivity.
    ///     The memory usage will be inefficient for devices with large qubit numbers.
    fn generic_device(&self) -> GenericDeviceWrapper {
        GenericDeviceWrapper {
            internal: self.internal.to_generic_device(),
        }
    }

    /// Convert the bincode representation of the QrydEmuTriangularDevice to a QrydEmuTriangularDevice the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized QrydEmuTriangularDevice (in bincode form).
    ///
    /// Returns:
    ///     QrydEmuTriangularDevice: The deserialized QrydEmuTriangularDevice.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to QrydEmuTriangularDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    pub fn from_bincode(input: &PyAny) -> PyResult<QrydEmuTriangularDeviceWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(QrydEmuTriangularDeviceWrapper {
            internal: bincode::deserialize(&bytes[..]).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to QrydEmuTriangularDevice")
            })?,
        })
    }

    /// Return the json representation of the QrydEmuTriangularDevice.
    ///
    /// Returns:
    ///     str: The serialized form of QrydEmuTriangularDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize QrydEmuTriangularDevice to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal).map_err(|_| {
            PyValueError::new_err("Cannot serialize QrydEmuTriangularDevice to json")
        })?;
        Ok(serialized)
    }

    /// Convert the json representation of a QrydEmuTriangularDevice to a QrydEmuTriangularDevice.
    ///
    /// Args:
    ///     input (str): The serialized QrydEmuTriangularDevice in json form.
    ///
    /// Returns:
    ///     QrydEmuTriangularDevice: The deserialized QrydEmuTriangularDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to QrydEmuTriangularDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    fn from_json(input: &str) -> PyResult<QrydEmuTriangularDeviceWrapper> {
        Ok(QrydEmuTriangularDeviceWrapper {
            internal: serde_json::from_str(input).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to QrydEmuTriangularDevice")
            })?,
        })
    }

    /// Return number of qubits in device.
    ///
    /// Returns:
    ///     int: The number of qubits.
    ///
    pub fn number_qubits(&self) -> usize {
        self.internal.number_qubits()
    }

    /// Return the bincode representation of the Enum variant of the Device.
    ///
    /// Only used for internal interfacing.
    ///
    /// Returns:
    ///     ByteArray: The serialized device (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize Device to bytes.
    pub fn _enum_to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let qryd_enum: QRydAPIDevice = (&self.internal).into();
        let serialized = bincode::serialize(&qryd_enum).map_err(|_| {
            PyValueError::new_err("Cannot serialize QrydEmuTriangularDevice to bytes")
        })?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Return the list of pairs of qubits linked by a native two-qubit-gate in the device.
    ///
    /// A pair of qubits is considered linked by a native two-qubit-gate if the device
    /// can implement a two-qubit-gate between the two qubits without decomposing it
    /// into a sequence of gates that involves a third qubit of the device.
    /// The two-qubit-gate also has to form a universal set together with the available
    /// single qubit gates.
    ///
    /// The returned vectors is a simple, graph-library independent, representation of
    /// the undirected connectivity graph of the device.
    /// It can be used to construct the connectivity graph in a graph library of the user's
    /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
    ///
    /// Example
    /// -------
    ///
    /// To construct a networkx graph from this output one can use
    ///
    /// >>> import networkx as nx
    /// ... from qoqo_qryd import QrydEmuTriangularDevice
    /// ...
    /// ... device = QrydEmuTriangularDevice(number_rows=2,number_columns= 2 qubits_per_row=[2,2], row_distance=1.0)
    /// ... edges = device.two_qubit_edges()
    /// ... graph = nx.Graph()
    /// ... graph.add_edges_from(edges)
    ///
    ///
    /// Returns:
    ///     Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        self.internal.two_qubit_edges()
    }

    /// Returns the backend associated with the device.
    pub fn qrydbackend(&self) -> String {
        self.internal.qrydbackend()
    }

    /// Returns the seed usized for the API.
    pub fn seed(&self) -> usize {
        self.internal.seed()
    }

    /// Returns the gate time of a single qubit operation on this device.
    ///
    /// Returns:
    ///     f64: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available in the device.
    pub fn single_qubit_gate_time(&self, hqslang: &str, qubit: usize) -> PyResult<f64> {
        self.internal
            .single_qubit_gate_time(hqslang, &qubit)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the gate time of a two qubit operation on this device.
    ///
    /// Returns:
    ///     f64: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available in the device.
    pub fn two_qubit_gate_time(
        &self,
        hqslang: &str,
        control: usize,
        target: usize,
    ) -> PyResult<f64> {
        self.internal
            .two_qubit_gate_time(hqslang, &control, &target)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the gate time of a three qubit operation on this device.
    ///
    /// Returns:
    ///     f64: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available in the device.
    fn three_qubit_gate_time(
        &self,
        hqslang: &str,
        control_0: usize,
        control_1: usize,
        target: usize,
    ) -> PyResult<f64> {
        self.internal
            .three_qubit_gate_time(hqslang, &control_0, &control_1, &target)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the gate time of a multi qubit operation on this device.
    ///
    /// Returns:
    ///     f64: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available in the device.
    pub fn multi_qubit_gate_time(&self, hqslang: &str, qubits: Vec<usize>) -> PyResult<f64> {
        self.internal
            .multi_qubit_gate_time(hqslang, &qubits)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the PhaseShiftedControlledZ phase shift according to the device's relation.
    pub fn phase_shift_controlled_z(&self) -> PyResult<f64> {
        self.internal
            .phase_shift_controlled_z()
            .ok_or_else(|| PyValueError::new_err("Error in relation selection."))
    }

    /// Returns the PhaseShiftedControlledPhase phase shift according to the device's relation.
    pub fn phase_shift_controlled_phase(&self, theta: f64) -> PyResult<f64> {
        self.internal
            .phase_shift_controlled_phase(theta)
            .ok_or_else(|| PyValueError::new_err("Error in relation selection."))
    }

    /// Returns the gate time of a PhaseShiftedControlledZ operation with the given qubits and phi angle.
    pub fn gate_time_controlled_z(&self, control: usize, target: usize, phi: f64) -> PyResult<f64> {
        self.internal
            .gate_time_controlled_z(&control, &target, phi)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the gate time of a PhaseShiftedControlledPhase operation with the given qubits and phi and theta angles.
    pub fn gate_time_controlled_phase(
        &self,
        control: usize,
        target: usize,
        phi: f64,
        theta: f64,
    ) -> PyResult<f64> {
        self.internal
            .gate_time_controlled_phase(&control, &target, phi, theta)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }
}

/// Convert generic python object to [roqoqo_qryd::QRydAPIDevice].
///
/// Fallible conversion of generic python object to [roqoqo_qryd::QRydAPIDevice].
pub fn convert_into_device(input: &PyAny) -> Result<QRydAPIDevice, QoqoBackendError> {
    // Everything that follows tries to extract the circuit when two separately
    // compiled python packages are involved
    let get_bytes = input
        .call_method0("_enum_to_bincode")
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    let bytes = get_bytes
        .extract::<Vec<u8>>()
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    bincode::deserialize(&bytes[..]).map_err(|_| QoqoBackendError::CannotExtractObject)
}

/// Devices available on the QRydDemo WebAPI.
///
/// .. autosummary::
///    :toctree: generated/
///
///    QrydEmuSquareDevice
///    QrydEmuTriangularDevice
///
#[pymodule]
pub fn api_devices(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<QrydEmuSquareDeviceWrapper>()?;
    m.add_class::<QrydEmuTriangularDeviceWrapper>()?;
    Ok(())
}
