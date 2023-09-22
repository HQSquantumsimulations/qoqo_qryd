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

use qoqo::{devices::GenericDeviceWrapper, QoqoBackendError};
use qoqo_calculator_pyo3::convert_into_calculator_float;
use roqoqo::devices::Device;

use roqoqo_qryd::TweezerDevice;

/// Tweezer Device
///
/// This interface does not allow setting any piece of information about the device
/// tweezers. This class is meant to be used by the end user.
///
/// Args:
///     controlled_z_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledZ gate.
///                                   It can be hardcoded to a specific value if a float is passed in as String.
///     controlled_phase_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledPhase gate.
#[pyclass(name = "TweezerDevice", module = "qoqo_qryd")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TweezerDeviceWrapper {
    /// Internal storage of [roqoqo_qryd::TweezerDevice]
    pub internal: TweezerDevice,
}

#[pymethods]
impl TweezerDeviceWrapper {
    /// Creates a new TweezerDevice instance.
    ///
    /// Args:
    ///     controlled_z_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledZ gate.
    ///     controlled_phase_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledPhase gate.
    ///
    /// Returns:
    ///     TweezerDevice: The new TweezerDevice instance.
    #[new]
    #[pyo3(text_signature = "(controlled_z_phase_relation, controlled_phase_phase_relation, /)")]
    pub fn new(
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
            internal: TweezerDevice::new(czpr, cppr),
        }
    }

    /// Creates a new TweezerDevice instance containing populated tweezer data.
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
    ///     TweezerDevice: The new TweezerDevice instance with populated tweezer data.
    ///
    /// Raises:
    ///     RoqoqoBackendError
    #[staticmethod]
    #[cfg(feature = "web-api")]
    #[pyo3(text_signature = "(device_name, access_token, /)")]
    pub fn from_api(
        device_name: Option<String>,
        access_token: Option<String>,
        mock_port: Option<String>,
    ) -> PyResult<Self> {
        let internal = TweezerDevice::from_api(device_name, access_token, mock_port)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))?;
        Ok(TweezerDeviceWrapper { internal })
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
    /// Returns:
    ///     dict[int, int]: The updated qubit -> tweezer mapping.
    ///     
    /// Raises:
    ///     ValueError: The tweezer is not present in the device.
    #[pyo3(text_signature = "(qubit, tweezer, /)")]
    pub fn add_qubit_tweezer_mapping(
        &mut self,
        qubit: usize,
        tweezer: usize,
    ) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            match self.internal.add_qubit_tweezer_mapping(qubit, tweezer) {
                Ok(mapping) => Ok(mapping.into_py_dict(py).into()),
                Err(err) => Err(PyValueError::new_err(format!("{:}", err))),
            }
        })
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

    /// Returns the PhaseShiftedControlledZ phase shift according to the device's relation.
    ///
    /// Returns:
    ///     float: The PhaseShiftedControlledZ phase shift.
    ///
    /// Raises:
    ///     ValueError: Error in relation selection.
    pub fn phase_shift_controlled_z(&self) -> PyResult<f64> {
        self.internal
            .phase_shift_controlled_z()
            .ok_or_else(|| PyValueError::new_err("Error in relation selection."))
    }

    /// Returns the PhaseShiftedControlledPhase phase shift according to the device's relation.
    ///
    /// Returns:
    ///     float: The PhaseShiftedControlledPhase phase shift.
    ///
    /// Raises:
    ///     ValueError: Error in relation selection.
    #[pyo3(text_signature = "(theta, /)")]
    pub fn phase_shift_controlled_phase(&self, theta: f64) -> PyResult<f64> {
        self.internal
            .phase_shift_controlled_phase(theta)
            .ok_or_else(|| PyValueError::new_err("Error in relation selection."))
    }

    /// Returns the gate time of a PhaseShiftedControlledZ operation with the given qubits and phi angle.
    ///
    /// Args:
    ///     control (int): The control qubit the gate acts on
    ///     target (int): The target qubit the gate acts on
    ///     phi (float): The phi angle to be checked.
    ///
    /// Returns:
    ///     float: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available on the device.
    #[pyo3(text_signature = "(control, target, phi, /)")]
    pub fn gate_time_controlled_z(&self, control: usize, target: usize, phi: f64) -> PyResult<f64> {
        self.internal
            .gate_time_controlled_z(&control, &target, phi)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the gate time of a PhaseShiftedControlledPhase operation with the given qubits and phi and theta angles.
    ///
    /// Args:
    ///     control (int): The control qubit the gate acts on
    ///     target (int): The target qubit the gate acts on
    ///     phi (float): The phi angle to be checked.
    ///     theta (float): The theta angle to be checked.
    ///
    /// Returns:
    ///     float: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available on the device.
    #[pyo3(text_signature = "(control, target, phi, theta, /)")]
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

    /// Return a copy of the TweezerDevice (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     TweezerDevice: A deep copy of self.
    pub fn __copy__(&self) -> TweezerDeviceWrapper {
        self.clone()
    }

    /// Return a deep copy of the TweezerDevice.
    ///
    /// Returns:
    ///     TweezerDevice: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> TweezerDeviceWrapper {
        self.clone()
    }

    /// Return the bincode representation of the TweezerDevice using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized TweezerDevice (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize TweezerDevice to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize TweezerDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the TweezerDevice to a TweezerDevice using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized TweezerDevice (in bincode form).
    ///
    /// Returns:
    ///     TweezerDevice: The deserialized TweezerDevice.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to TweezerDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    pub fn from_bincode(input: &PyAny) -> PyResult<TweezerDeviceWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(TweezerDeviceWrapper {
            internal: deserialize(&bytes[..]).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to TweezerDevice")
            })?,
        })
    }

    /// Return the json representation of the TweezerDevice.
    ///
    /// Returns:
    ///     str: The serialized form of TweezerDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize TweezerDevice to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize TweezerDevice to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a TweezerDevice to a TweezerDevice.
    ///
    /// Args:
    ///     input (str): The serialized TweezerDevice in json form.
    ///
    /// Returns:
    ///     TweezerDevice: The deserialized TweezerDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to TweezerDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    fn from_json(input: &str) -> PyResult<TweezerDeviceWrapper> {
        Ok(TweezerDeviceWrapper {
            internal: serde_json::from_str(input).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to TweezerDevice")
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

    /// Returns the two tweezer edges of the device.
    ///
    /// And edge between two tweezer is valid only if the
    /// PhaseShiftedControlledPhase gate can be performed.
    ///
    /// Returns:
    ///     Sequence[(int, int)]: List of two tweezer edges
    fn two_tweezer_edges(&self) -> Vec<(usize, usize)> {
        self.internal.two_tweezer_edges()
    }
}

/// Tweezer Mutable Device
///
/// This interface allows setting any piece of information about the device
/// tweezer.
///
/// Args:
///     controlled_z_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledZ gate.
///                                   It can be hardcoded to a specific value if a float is passed in as String.
///     controlled_phase_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledPhase gate.
#[pyclass(name = "TweezerMutableDevice", module = "qoqo_qryd")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TweezerMutableDeviceWrapper {
    /// Internal storage of [roqoqo_qryd::TweezerDevice]
    internal: TweezerDevice,
}

#[pymethods]
impl TweezerMutableDeviceWrapper {
    /// Creates a new TweezerMutableDevice instance.
    ///
    /// Args:
    ///     controlled_z_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledZ gate.
    ///     controlled_phase_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledPhase gate.
    ///
    /// Returns:
    ///     TweezerMutableDevice: The new TweezerMutableDevice instance.
    #[new]
    #[pyo3(text_signature = "(controlled_z_phase_relation, controlled_phase_phase_relation, /)")]
    pub fn new(
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
            internal: TweezerDevice::new(czpr, cppr),
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
    /// Returns:
    ///     dict[int, int]: The updated qubit -> tweezer mapping.
    ///     
    /// Raises:
    ///     ValueError: The tweezer is not present in the device.
    #[pyo3(text_signature = "(qubit, tweezer, /)")]
    pub fn add_qubit_tweezer_mapping(
        &mut self,
        qubit: usize,
        tweezer: usize,
    ) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            match self.internal.add_qubit_tweezer_mapping(qubit, tweezer) {
                Ok(mapping) => Ok(mapping.into_py_dict(py).into()),
                Err(err) => Err(PyValueError::new_err(format!("{:}", err))),
            }
        })
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

    /// Returns the PhaseShiftedControlledZ phase shift according to the device's relation.
    ///
    /// Returns:
    ///     float: The PhaseShiftedControlledZ phase shift.
    ///
    /// Raises:
    ///     ValueError: Error in relation selection.
    pub fn phase_shift_controlled_z(&self) -> PyResult<f64> {
        self.internal
            .phase_shift_controlled_z()
            .ok_or_else(|| PyValueError::new_err("Error in relation selection."))
    }

    /// Returns the PhaseShiftedControlledPhase phase shift according to the device's relation.
    ///
    /// Returns:
    ///     float: The PhaseShiftedControlledPhase phase shift.
    ///
    /// Raises:
    ///     ValueError: Error in relation selection.
    #[pyo3(text_signature = "(theta, /)")]
    pub fn phase_shift_controlled_phase(&self, theta: f64) -> PyResult<f64> {
        self.internal
            .phase_shift_controlled_phase(theta)
            .ok_or_else(|| PyValueError::new_err("Error in relation selection."))
    }

    /// Returns the gate time of a PhaseShiftedControlledZ operation with the given qubits and phi angle.
    ///
    /// Args:
    ///     control (int): The control qubit the gate acts on
    ///     target (int): The target qubit the gate acts on
    ///     phi (float): The phi angle to be checked.
    ///
    /// Returns:
    ///     float: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available on the device.
    #[pyo3(text_signature = "(control, target, phi, /)")]
    pub fn gate_time_controlled_z(&self, control: usize, target: usize, phi: f64) -> PyResult<f64> {
        self.internal
            .gate_time_controlled_z(&control, &target, phi)
            .ok_or_else(|| PyValueError::new_err("The gate is not available on the device."))
    }

    /// Returns the gate time of a PhaseShiftedControlledPhase operation with the given qubits and phi and theta angles.
    ///
    /// Args:
    ///     control (int): The control qubit the gate acts on
    ///     target (int): The target qubit the gate acts on
    ///     phi (float): The phi angle to be checked.
    ///     theta (float): The theta angle to be checked.
    ///
    /// Returns:
    ///     float: The gate time.
    ///
    /// Raises:
    ///     ValueError: The gate is not available on the device.
    #[pyo3(text_signature = "(control, target, phi, theta, /)")]
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

    /// Return a copy of the TweezerMutableDevice (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     TweezerMutableDevice: A deep copy of self.
    pub fn __copy__(&self) -> TweezerMutableDeviceWrapper {
        self.clone()
    }

    /// Return a deep copy of the TweezerMutableDevice.
    ///
    /// Returns:
    ///     TweezerMutableDevice: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> TweezerMutableDeviceWrapper {
        self.clone()
    }

    /// Return the bincode representation of the TweezerMutableDevice using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized TweezerMutableDevice (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize TweezerMutableDevice to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize TweezerMutableDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the TweezerMutableDevice to an
    /// TweezerMutableDevice using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized TweezerMutableDevice (in bincode form).
    ///
    /// Returns:
    ///     TweezerMutableDevice: The deserialized TweezerMutableDevice.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to TweezerMutableDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    pub fn from_bincode(input: &PyAny) -> PyResult<TweezerMutableDeviceWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(TweezerMutableDeviceWrapper {
            internal: deserialize(&bytes[..]).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to TweezerMutableDevice")
            })?,
        })
    }

    /// Return the json representation of the TweezerMutableDevice.
    ///
    /// Returns:
    ///     str: The serialized form of TweezerMutableDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize TweezerMutableDevice to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal).map_err(|_| {
            PyValueError::new_err("Cannot serialize ExperimentalMutableDevice to json")
        })?;
        Ok(serialized)
    }

    /// Convert the json representation of a TweezerMutableDevice to an TweezerMutableDevice.
    ///
    /// Args:
    ///     input (str): The serialized TweezerMutableDevice in json form.
    ///
    /// Returns:
    ///     TweezerMutableDevice: The deserialized TweezerMutableDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to TweezerMutableDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    fn from_json(input: &str) -> PyResult<TweezerMutableDeviceWrapper> {
        Ok(TweezerMutableDeviceWrapper {
            internal: serde_json::from_str(input).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to TweezerMutableDevice")
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

    /// Returns the two tweezer edges of the device.
    ///
    /// And edge between two tweezer is valid only if the
    /// PhaseShiftedControlledPhase gate can be performed.
    ///
    /// Returns:
    ///     Sequence[(int, int)]: List of two tweezer edges
    fn two_tweezer_edges(&self) -> Vec<(usize, usize)> {
        self.internal.two_tweezer_edges()
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

    /// Set the allowed Tweezer shifts of a specified Tweezer.
    ///
    /// The tweezer give the tweezer a qubit can be shifted out of. The values are lists
    /// over the directions the qubit in the tweezer can be shifted into.
    /// The items in the list give the allowed tweezers the qubit can be shifted into in order.
    /// For a list 1,2,3 the qubit can be shifted into tweezer 1, into tweezer 2 if tweezer 1 is not occupied,
    /// and into tweezer 3 if tweezer 1 and 2 are not occupied.
    ///
    /// Args:
    ///     tweezer (int): The index of the tweezer.
    ///     allowed_shifts (list(list(int))): The allowed tweezer shifts.
    ///     layout_name (Optional[str]): The name of the Layout to apply the allowed shifts in.
    ///         Defaults to the current Layout.
    ///
    /// Raises:
    ///     ValueError: The tweezer or shifts are not present in the device or
    ///         the given tweezer is contained in the shift list.
    #[pyo3(text_signature = "(tweezer, allowed_shifts, layout_name, /)")]
    pub fn set_allowed_tweezer_shifts(
        &mut self,
        tweezer: usize,
        allowed_shifts: Vec<Vec<usize>>,
        layout_name: Option<String>,
    ) -> PyResult<()> {
        self.internal
            .set_allowed_tweezer_shifts(
                &tweezer,
                allowed_shifts
                    .iter()
                    .map(|vec| vec.as_slice())
                    .collect::<Vec<&[usize]>>()
                    .as_slice(),
                layout_name,
            )
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Set the allowed Tweezer shifts from a list of tweezers.
    ///
    /// The items in the rows give the allowed tweezers that qubit can be shifted into.
    /// For a row defined as 1,2,3, a qubit in tweezer 1 can be shifted into tweezer 2,
    /// and into tweezer 3 if tweezer 2 is not occupied by a qubit.
    ///
    /// Args:
    ///     row_shifts (list(list(int))): A list of lists, each representing a row of tweezers.
    ///     layout_name (Optional[str]): The name of the Layout to apply the allowed shifts in.
    ///         Defaults to the current Layout.
    ///
    /// Raises:
    ///     ValueError: The involved tweezers are not present in the device.
    #[pyo3(text_signature = "(row_shifts, layout_name, /)")]
    pub fn set_allowed_tweezer_shifts_from_rows(
        &mut self,
        row_shifts: Vec<Vec<usize>>,
        layout_name: Option<String>,
    ) -> PyResult<()> {
        self.internal
            .set_allowed_tweezer_shifts_from_rows(
                row_shifts
                    .iter()
                    .map(|vec| vec.as_slice())
                    .collect::<Vec<&[usize]>>()
                    .as_slice(),
                layout_name,
            )
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Set the name of the default layout to use.
    ///
    /// Args:
    ///     layout (str): The name of the layout to use.
    ///
    /// Raises:
    ///     ValueError: The given layout name is not present in the layout register.
    pub fn set_default_layout(&mut self, layout: &str) -> PyResult<()> {
        self.internal
            .set_default_layout(layout)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }
}

/// Convert generic python object to [roqoqo_qryd::TweezerDevice].
///
/// Fallible conversion of generic python object to [roqoqo_qryd::TweezerDevice].
pub fn convert_into_device(input: &PyAny) -> Result<TweezerDevice, QoqoBackendError> {
    let get_bytes = input
        .call_method0("to_bincode")
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    let bytes = get_bytes
        .extract::<Vec<u8>>()
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    bincode::deserialize(&bytes[..]).map_err(|_| QoqoBackendError::CannotExtractObject)
}

/// Tweezer devices for the QRyd platform.
///
/// .. autosummary::
///    :toctree: generated/
///
///    TweezerDevice
///    TweezerMutableDevice
///
#[pymodule]
pub fn tweezer_devices(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TweezerDeviceWrapper>()?;
    m.add_class::<TweezerMutableDeviceWrapper>()?;
    Ok(())
}
