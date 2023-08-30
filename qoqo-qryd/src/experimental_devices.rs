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

use bincode::{deserialize, serialize};
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::{IntoPyDict, PyByteArray},
};

use qoqo::devices::GenericDeviceWrapper;
use roqoqo::devices::Device;
use roqoqo_qryd::ExperimentalDevice;

/// Experimental Device
///
/// This interface does not allow setting any piece of information about the device
/// tweezers. This class is meant to be used by the end user.
#[pyclass(name = "ExperimentalDevice", module = "qoqo_qryd")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExperimentalDeviceWrapper {
    /// Internal storage of [roqoqo_qryd::ExperimentalDevice]
    pub internal: ExperimentalDevice,
}

#[pymethods]
impl ExperimentalDeviceWrapper {
    /// Creates a new ExperimentalDevice instance.
    ///
    /// Returns:
    ///     ExperimentalDevice: The new ExperimentalDevice instance.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: ExperimentalDevice::new(),
        }
    }

    /// Creates a new ExperimentalDevice instance containing populated tweezer data.
    ///
    /// This requires a valid QRYD_API_TOKEN. Visit `https://thequantumlaend.de/get-access/` to get one.
    ///
    /// Args
    ///     device_name (Optional[str]): The name of the device to instantiate. Defaults to "Default".
    ///     access_token (Optional[str]): An access_token is required to access QRYD hardware and emulators.
    ///                         The access_token can either be given as an argument here
    ///                             or set via the environmental variable `$QRYD_API_TOKEN`.
    ///     mock_port (Optional[str]): Server port to be used for testing purposes.
    ///
    /// Returns
    ///     ExperimentalDevice: The new ExperimentalDevice instance with populated tweezer data.
    ///
    /// Raises:
    ///     RoqoqoBackendError
    #[staticmethod]
    #[pyo3(text_signature = "(device_name, access_token, /)")]
    pub fn from_api(
        device_name: Option<String>,
        access_token: Option<String>,
        mock_port: Option<String>,
    ) -> PyResult<Self> {
        let internal = ExperimentalDevice::from_api(device_name, access_token, mock_port)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))?;
        Ok(ExperimentalDeviceWrapper { internal })
    }

    /// Get the name of the current layout.
    ///
    /// Returns:
    ///     str: The name of the current layout.
    pub fn current_layout(&self) -> &str {
        self.internal.current_layout.as_str()
    }

    /// Switch to a different pre-defined layout.
    ///
    /// Args:
    ///     layout_number (str): The number index of the new layout
    ///
    /// Raises:
    ///     PyValueError
    #[pyo3(text_signature = "(name, /)")]
    pub fn switch_layout(&mut self, name: &str) -> PyResult<()> {
        self.internal
            .switch_layout(name)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Returns a list of all available Layout names.
    ///
    /// Returns:
    ///     List[str]: The list of all available Layout names.
    pub fn available_layouts(&self) -> Vec<&str> {
        self.internal.available_layouts()
    }

    /// Modifies the qubit -> tweezer mapping of the device.
    ///
    /// If a qubit -> tweezer mapping is already present, it is overwritten.
    ///
    /// Args:
    ///     qubit (int): The index of the qubit.
    ///     tweezer (int): The index of the tweezer.
    ///
    /// Raises:
    ///     PyValueError: The qubit is not present in the device.
    #[pyo3(text_signature = "(qubit, tweezer, /)")]
    pub fn add_qubit_tweezer_mapping(&mut self, qubit: usize, tweezer: usize) -> PyResult<()> {
        self.internal
            .add_qubit_tweezer_mapping(qubit, tweezer)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Deactivate the given qubit in the device.
    ///
    /// Args:
    ///     qubit (int): The input qubit identifier.
    ///
    /// Returns:
    ///     dict[int, int]: The updated qubit -> tweezer mapping.
    ///
    /// Raises:
    ///     PyValueError: If the given qubit identifier is not present in the mapping.
    #[pyo3(text_signature = "(qubit, /)")]
    pub fn deactivate_qubit(&mut self, qubit: usize) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            match self.internal.deactivate_qubit(qubit) {
                Ok(tweezers) => Ok(tweezers.into_py_dict(py).into()),
                Err(err) => Err(PyValueError::new_err(format!("{:}", err))),
            }
        })
    }

    /// Returns the gate time of a single qubit operation on this device.
    ///
    /// Returns:
    ///     f64: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available in the device.
    #[pyo3(text_signature = "(hqslang, qubit, /)")]
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
    #[pyo3(text_signature = "(hqslang, control, target, /)")]
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
    #[pyo3(text_signature = "(hqslang, control_0, control_1, target, /)")]
    pub fn three_qubit_gate_time(
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
    #[pyo3(text_signature = "(hqslang, qubits, /)")]
    pub fn multi_qubit_gate_time(&self, hqslang: &str, qubits: Vec<usize>) -> PyResult<f64> {
        self.internal
            .multi_qubit_gate_time(hqslang, &qubits)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
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

    /// Return a copy of the ExperimentalDevice (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     ExperimentalDevice: A deep copy of self.
    pub fn __copy__(&self) -> ExperimentalDeviceWrapper {
        self.clone()
    }

    /// Return a deep copy of the ExperimentalDevice.
    ///
    /// Returns:
    ///     ExperimentalDevice: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> ExperimentalDeviceWrapper {
        self.clone()
    }

    /// Return the bincode representation of the ExperimentalDevice using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized ExperimentalDevice (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize ExperimentalDevice to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize ExperimentalDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the ExperimentalDevice to a ExperimentalDevice using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized ExperimentalDevice (in bincode form).
    ///
    /// Returns:
    ///     ExperimentalDevice: The deserialized ExperimentalDevice.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to ExperimentalDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    pub fn from_bincode(input: &PyAny) -> PyResult<ExperimentalDeviceWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(ExperimentalDeviceWrapper {
            internal: deserialize(&bytes[..]).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to ExperimentalDevice")
            })?,
        })
    }

    /// Return the json representation of the ExperimentalDevice.
    ///
    /// Returns:
    ///     str: The serialized form of ExperimentalDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize ExperimentalDevice to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize ExperimentalDevice to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a ExperimentalDevice to a ExperimentalDevice.
    ///
    /// Args:
    ///     input (str): The serialized ExperimentalDevice in json form.
    ///
    /// Returns:
    ///     ExperimentalDevice: The deserialized ExperimentalDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to ExperimentalDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    fn from_json(input: &str) -> PyResult<ExperimentalDeviceWrapper> {
        Ok(ExperimentalDeviceWrapper {
            internal: serde_json::from_str(input).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to ExperimentalDevice")
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
    /// Returns:
    ///     Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        self.internal.two_qubit_edges()
    }
}

/// Experimental Mutable Device
///
/// This interface allows setting any piece of information about the device
/// tweezer.
#[pyclass(name = "ExperimentalMutableDevice", module = "qoqo_qryd")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExperimentalMutableDeviceWrapper {
    /// Internal storage of [roqoqo_qryd::ExperimentalDevice]
    internal: ExperimentalDevice,
}

#[pymethods]
impl ExperimentalMutableDeviceWrapper {
    /// Creates a new ExperimentalMutableDevice instance.
    ///
    /// Returns:
    ///     ExperimentalMutableDevice: The new ExperimentalMutableDevice instance.
    #[new]
    pub fn new() -> Self {
        Self {
            internal: ExperimentalDevice::new(),
        }
    }

    /// Get the name of the current layout.
    ///
    /// Returns:
    ///     str: The name of the current layout.
    pub fn current_layout(&self) -> &str {
        self.internal.current_layout.as_str()
    }

    /// Add a new layout to the device.
    ///
    /// Args:
    ///     name (str): The name that is assigned to the new Layout.
    #[pyo3(text_signature = "(name, /)")]
    pub fn add_layout(&mut self, name: &str) -> PyResult<()> {
        self.internal
            .add_layout(name)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Switch to a different pre-defined layout.
    ///
    /// Args:
    ///     layout_number (str): The number index of the new layout
    ///
    /// Raises:
    ///     PyValueError
    #[pyo3(text_signature = "(name, /)")]
    pub fn switch_layout(&mut self, name: &str) -> PyResult<()> {
        self.internal
            .switch_layout(name)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Returns a list of all available Layout names.
    ///
    /// Returns:
    ///     List[str]: The list of all available Layout names.
    pub fn available_layouts(&self) -> Vec<&str> {
        self.internal.available_layouts()
    }

    /// Modifies the qubit -> tweezer mapping of the device.
    ///
    /// If a qubit -> tweezer mapping is already present, it is overwritten.
    ///
    /// Args:
    ///     qubit (int): The index of the qubit.
    ///     tweezer (int): The index of the tweezer.
    ///
    /// Raises:
    ///     PyValueError: The qubit is not present in the device.
    #[pyo3(text_signature = "(qubit, tweezer, /)")]
    pub fn add_qubit_tweezer_mapping(&mut self, qubit: usize, tweezer: usize) -> PyResult<()> {
        self.internal
            .add_qubit_tweezer_mapping(qubit, tweezer)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Deactivate the given qubit in the device.
    ///
    /// Args:
    ///     qubit (int): The input qubit identifier.
    ///
    /// Returns:
    ///     dict[int, int]: The updated qubit -> tweezer mapping.
    ///
    /// Raises:
    ///     PyValueError: If the given qubit identifier is not present in the mapping.
    #[pyo3(text_signature = "(qubit, /)")]
    pub fn deactivate_qubit(&mut self, qubit: usize) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            match self.internal.deactivate_qubit(qubit) {
                Ok(tweezers) => Ok(tweezers.into_py_dict(py).into()),
                Err(err) => Err(PyValueError::new_err(format!("{:}", err))),
            }
        })
    }

    /// Returns the gate time of a single qubit operation on this device.
    ///
    /// Returns:
    ///     f64: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available in the device.
    #[pyo3(text_signature = "(hqslang, qubit, /)")]
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
    #[pyo3(text_signature = "(hqslang, control, target, /)")]
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
    #[pyo3(text_signature = "(hqslang, control_0, control_1, target, /)")]
    pub fn three_qubit_gate_time(
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
    #[pyo3(text_signature = "(hqslang, qubits, /)")]
    pub fn multi_qubit_gate_time(&self, hqslang: &str, qubits: Vec<usize>) -> PyResult<f64> {
        self.internal
            .multi_qubit_gate_time(hqslang, &qubits)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
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

    /// Return a copy of the ExperimentalDevice (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     ExperimentalDevice: A deep copy of self.
    pub fn __copy__(&self) -> ExperimentalMutableDeviceWrapper {
        self.clone()
    }

    /// Return a deep copy of the ExperimentalDevice.
    ///
    /// Returns:
    ///     ExperimentalDevice: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> ExperimentalMutableDeviceWrapper {
        self.clone()
    }

    /// Return the bincode representation of the ExperimentalMutableDevice using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized ExperimentalMutableDevice (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize ExperimentalMutableDevice to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal).map_err(|_| {
            PyValueError::new_err("Cannot serialize ExperimentalMutableDevice to bytes")
        })?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the ExperimentalMutableDevice to an
    /// ExperimentalMutableDevice using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized ExperimentalMutableDevice (in bincode form).
    ///
    /// Returns:
    ///     ExperimentalDevice: The deserialized ExperimentalMutableDevice.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to ExperimentalMutableDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    pub fn from_bincode(input: &PyAny) -> PyResult<ExperimentalMutableDeviceWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(ExperimentalMutableDeviceWrapper {
            internal: deserialize(&bytes[..]).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to ExperimentalMutableDevice")
            })?,
        })
    }

    /// Return the json representation of the ExperimentalMutableDevice.
    ///
    /// Returns:
    ///     str: The serialized form of ExperimentalMutableDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize ExperimentalMutableDevice to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal).map_err(|_| {
            PyValueError::new_err("Cannot serialize ExperimentalMutableDevice to json")
        })?;
        Ok(serialized)
    }

    /// Convert the json representation of a ExperimentalMutableDevice to an ExperimentalMutableDevice.
    ///
    /// Args:
    ///     input (str): The serialized ExperimentalMutableDevice in json form.
    ///
    /// Returns:
    ///     ExperimentalDevice: The deserialized ExperimentalMutableDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to ExperimentalMutableDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    fn from_json(input: &str) -> PyResult<ExperimentalMutableDeviceWrapper> {
        Ok(ExperimentalMutableDeviceWrapper {
            internal: serde_json::from_str(input).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to ExperimentalMutableDevice")
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
    /// Returns:
    ///     Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
    fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
        self.internal.two_qubit_edges()
    }

    /// Set the time of a single-qubit gate for a tweezer in a given Layout.
    ///
    /// Args:
    ///     hqslang (str): The hqslang name of a single-qubit gate.
    ///     tweezer (usize): The index of the tweezer.
    ///     gate_time (float): The the gate time for the given gate.
    ///     layout_name (Optional[str]): The name of the Layout to apply the gate time in.
    ///         Defaults to the current Layout.
    #[pyo3(text_signature = "(hqslang, tweezer, gate_time, layout_name, /)")]
    pub fn set_tweezer_single_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        self.internal
            .set_tweezer_single_qubit_gate_time(hqslang, tweezer, gate_time, layout_name)
    }

    /// Set the time of a two-qubit gate for a tweezer couple in a given Layout.
    ///
    /// Args:
    ///     hqslang (str): The hqslang name of a single-qubit gate.
    ///     tweezer0 (usize): The index of the first tweezer.
    ///     tweezer1 (usize): The index of the second tweezer.
    ///     gate_time (float): The the gate time for the given gate.
    ///     layout_name (Optional[str]): The name of the Layout to apply the gate time in.
    ///         Defaults to the current Layout.
    #[pyo3(text_signature = "(hqslang, tweezer0, tweezer1, gate_time, layout_name, /)")]
    pub fn set_tweezer_two_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer0: usize,
        tweezer1: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        self.internal.set_tweezer_two_qubit_gate_time(
            hqslang,
            tweezer0,
            tweezer1,
            gate_time,
            layout_name,
        )
    }

    /// Set the time of a three-qubit gate for a tweezer trio in a given Layout.
    ///
    /// Args:
    ///     hqslang (str): The hqslang name of a three-qubit gate.
    ///     tweezer0 (usize): The index of the first tweezer.
    ///     tweezer1 (usize): The index of the second tweezer.
    ///     tweezer2 (usize): The index of the third tweezer.
    ///     gate_time (float): The the gate time for the given gate.
    ///     layout_name (Optional[str]): The name of the Layout to apply the gate time in.
    ///         Defaults to the current Layout.
    #[pyo3(text_signature = "(hqslang, tweezer0, tweezer1, tweezer2, gate_time, layout_name, /)")]
    pub fn set_tweezer_three_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer0: usize,
        tweezer1: usize,
        tweezer2: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        self.internal.set_tweezer_three_qubit_gate_time(
            hqslang,
            tweezer0,
            tweezer1,
            tweezer2,
            gate_time,
            layout_name,
        )
    }

    /// Set the time of a multi-qubit gate for a list of tweezers in a given Layout.
    ///
    /// Args:
    ///     hqslang (name): The hqslang name of a multi-qubit gate.
    ///     tweezers (List[usize]): The list of tweezer indexes.
    ///     gate_time (float): The the gate time for the given gate.
    ///     layout_name (Optional[str]): The name of the Layout to apply the gate time in.
    ///         Defaults to the current Layout.
    #[pyo3(text_signature = "(hqslang, tweezers, gate_time, layout_name, /)")]
    pub fn set_tweezer_multi_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezers: Vec<usize>,
        gate_time: f64,
        layout_name: Option<String>,
    ) {
        self.internal
            .set_tweezer_multi_qubit_gate_time(hqslang, &tweezers, gate_time, layout_name)
    }
}

/// Experimental devices for the QRyd platform.
///
/// .. autosummary::
///    :toctree: generated/
///
///    ExperimentalDevice
///    ExperimentalMutableDevice
///
#[pymodule]
pub fn experimental_devices(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ExperimentalDeviceWrapper>()?;
    m.add_class::<ExperimentalMutableDeviceWrapper>()?;
    Ok(())
}
