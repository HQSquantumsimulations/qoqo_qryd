// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyByteArray};

use qoqo::{devices::GenericDeviceWrapper, QoqoBackendError};
use qoqo_calculator_pyo3::convert_into_calculator_float;
use roqoqo::devices::Device;
use roqoqo_qryd::{EmulatorDevice, TweezerDevice};

/// Emulator Device
///
/// Args:
///     seed (Optional[int]): Optional seed, for simulation purposes.
///     controlled_z_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledZ gate.
///                                   It can be hardcoded to a specific value if a float is passed in as String.
///     controlled_phase_phase_relation ((Optional[Union[str, float]])): The relation to use for the PhaseShiftedControlledPhase gate.
#[pyclass(name = "EmulatorDevice", module = "qoqo_qryd")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EmulatorDeviceWrapper {
    /// Internal storage of [roqoqo_qryd::EmulatorDevice]
    pub internal: EmulatorDevice,
}

#[pymethods]
impl EmulatorDeviceWrapper {
    /// Creates a new EmulatorDevice instance.
    ///
    /// Args:
    ///     seed (Optional[int]): Optional seed, for simulation purposes.
    ///     controlled_z_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledZ gate.
    ///     controlled_phase_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledPhase gate.
    ///
    /// Returns:
    ///     EmulatorDevice: The new EmulatorDevice instance.
    #[new]
    #[pyo3(
        text_signature = "(seed, controlled_z_phase_relation, controlled_phase_phase_relation, /)"
    )]
    pub fn new(
        seed: Option<usize>,
        controlled_z_phase_relation: Option<&Bound<PyAny>>,
        controlled_phase_phase_relation: Option<&Bound<PyAny>>,
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
            internal: EmulatorDevice::new(seed, czpr, cppr),
        }
    }

    /// Creates a new Emulator instance containing populated tweezer data.
    ///
    /// This requires a valid QRYD_API_TOKEN. Visit `https://thequantumlaend.de/get-access/` to get one.
    ///
    /// Args
    ///     device_name (Optional[str]): The name of the device to instantiate. Defaults to "qryd_emulator".
    ///     access_token (Optional[str]): An access_token is required to access QRYD hardware and emulators.
    ///                         The access_token can either be given as an argument here
    ///                             or set via the environmental variable `$QRYD_API_TOKEN`.
    ///     mock_port (Optional[str]): Server port to be used for testing purposes.
    ///     seed (Optional[int]): Optionally overwrite seed value from downloaded device instance.
    ///     dev (Optional[bool]): The boolean to set the dev header to.
    ///     api_version (Optional[str]): The version of the QRYD API to use. Defaults to "v1_1".
    ///
    /// Returns
    ///     TweezerDevice: The new TweezerDevice instance with populated tweezer data.
    ///
    /// Raises:
    ///     RoqoqoBackendError
    #[staticmethod]
    #[cfg(feature = "web-api")]
    #[pyo3(text_signature = "(device_name, access_token, mock_port, seed, api_version, /)")]
    pub fn from_api(
        device_name: Option<String>,
        access_token: Option<String>,
        mock_port: Option<String>,
        seed: Option<usize>,
        dev: Option<bool>,
        api_version: Option<String>,
    ) -> PyResult<Self> {
        let internal =
            EmulatorDevice::from_api(device_name, access_token, mock_port, seed, dev, api_version)
                .map_err(|err| PyValueError::new_err(format!("{:}", err)))?;
        Ok(EmulatorDeviceWrapper { internal })
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
                Ok(mapping) => Ok(mapping.into_py_dict_bound(py).into()),
                Err(err) => Err(PyValueError::new_err(format!("{:}", err))),
            }
        })
    }

    /// Modifies the qubit -> tweezer mapping of the device.
    ///
    /// Args:
    ///     hqslang (str): The hqslang name of the gate.
    ///     
    /// Raises:
    ///     ValueError: The gate does not exist.
    pub fn add_available_gate(&mut self, hqslang: &str) -> PyResult<()> {
        self.internal
            .add_available_gate(hqslang)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Get the qubit -> tweezer mapping of the device.
    ///
    /// Returns:
    ///     dict[int, int]: The qubit -> tweezer mapping.
    ///     None: The mapping is empty.
    pub fn get_qubit_to_tweezer_mapping(&self) -> Option<PyObject> {
        Python::with_gil(|py| -> Option<PyObject> {
            self.internal
                .internal
                .qubit_to_tweezer
                .as_ref()
                .map(|mapping| mapping.into_py_dict_bound(py).into())
        })
    }

    /// Get the names of the available gates in the given layout.
    ///
    /// Returns:
    ///     list[str]: List of the names of the available gates in the given layout.
    ///
    /// Raises:
    ///     ValueError: No layout name provided and no current layout set.
    pub fn get_available_gates_names(&self) -> PyResult<Vec<&str>> {
        self.internal
            .get_available_gates_names()
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Set whether the device allows PragmaActiveReset operations or not.
    ///
    /// Args:
    ///     allow_reset(bool): Whether the device should allow PragmaActiveReset operations or not.
    ///
    /// Raises:
    ///     ValueError: The device isn't compatible with PragmaActiveReset.
    #[pyo3(text_signature = "(allow_reset, /)")]
    pub fn set_allow_reset(&mut self, allow_reset: bool) -> PyResult<()> {
        self.internal
            .set_allow_reset(allow_reset)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Get whether the device allows PragmaActiveReset operations or not.
    ///
    /// Returns:
    ///     bool: Whether the device allows PragmaActiveReset operations or not.
    pub fn get_allow_reset(&self) -> bool {
        self.internal.internal.allow_reset
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
                Ok(tweezers) => Ok(tweezers.into_py_dict_bound(py).into()),
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
    pub fn phase_shift_controlled_phase(&self, theta: &Bound<PyAny>) -> PyResult<f64> {
        let float = if let Ok(conv) = convert_into_calculator_float(theta) {
            *conv
                .float()
                .map_err(|err| PyValueError::new_err(format!("{:}", err)))?
        } else {
            theta.extract::<f64>()?
        };

        self.internal
            .phase_shift_controlled_phase(float)
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

    /// Return a copy of the EmulatorDevice (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     EmulatorDevice: A deep copy of self.
    pub fn __copy__(&self) -> EmulatorDeviceWrapper {
        self.clone()
    }

    /// Return a deep copy of the EmulatorDevice.
    ///
    /// Returns:
    ///     EmulatorDevice: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> EmulatorDeviceWrapper {
        self.clone()
    }

    /// Return the bincode representation of the EmulatorDevice using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized EmulatorDevice (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize EmulatorDevice to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize EmulatorDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new_bound(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the EmulatorDevice to a EmulatorDevice using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized EmulatorDevice (in bincode form).
    ///
    /// Returns:
    ///     EmulatorDevice: The deserialized EmulatorDevice.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to EmulatorDevice.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    pub fn from_bincode(input: &Bound<PyAny>) -> PyResult<EmulatorDeviceWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(EmulatorDeviceWrapper {
            internal: EmulatorDevice {
                internal: deserialize(&bytes[..]).map_err(|_| {
                    PyValueError::new_err("Input cannot be deserialized to EmulatorDevice")
                })?,
            },
        })
    }

    /// Return the json representation of the EmulatorDevice.
    ///
    /// Additionally, a gate set check is performed.
    ///
    /// Returns:
    ///     str: The serialized form of EmulatorDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize EmulatorDevice to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize EmulatorDevice to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a EmulatorDevice to a EmulatorDevice.
    ///
    /// If a default_layout is found in the input, a layout switch is executed.
    /// Additionally, a gate set check is performed.
    ///
    /// Args:
    ///     input (str): The serialized EmulatorDevice in json form.
    ///
    /// Returns:
    ///     EmulatorDevice: The deserialized EmulatorDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to EmulatorDevice or
    ///         the device does not have valid QRyd gates available.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    fn from_json(input: &str) -> PyResult<EmulatorDeviceWrapper> {
        let tw: TweezerDevice = serde_json::from_str(input)
            .map_err(|_| PyValueError::new_err("Input cannot be deserialized to EmulatorDevice"))?;
        let internal = EmulatorDevice { internal: tw };
        Ok(EmulatorDeviceWrapper { internal })
    }

    /// Return number of qubits in device.
    ///
    /// Returns:
    ///     int: The number of qubits.
    pub fn number_qubits(&self) -> usize {
        self.internal.number_qubits()
    }

    /// Returns the number of total tweezer positions in the device.
    ///
    /// Returns:
    ///     int: The number of tweezer positions in the device.
    pub fn number_tweezer_positions(&self) -> usize {
        self.internal.number_tweezer_positions()
    }

    /// Returns the backend associated with the device.
    pub fn qrydbackend(&self) -> String {
        self.internal.qrydbackend()
    }

    /// Returns the seed usized for the API.
    pub fn seed(&self) -> Option<usize> {
        self.internal.seed()
    }
}

/// Convert generic python object to [roqoqo_qryd::EmulatorDevice].
///
/// Fallible conversion of generic python object to [roqoqo_qryd::EmulatorDevice].
pub fn convert_into_device(input: &Bound<PyAny>) -> Result<EmulatorDevice, QoqoBackendError> {
    let get_bytes = input
        .call_method0("to_bincode")
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    let bytes = get_bytes
        .extract::<Vec<u8>>()
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    bincode::deserialize(&bytes[..]).map_err(|_| QoqoBackendError::CannotExtractObject)
}

/// Emulator devices for the QRyd platform.
///
/// .. autosummary::
///    :toctree: generated/
///
///    EmulatorDevice
///
#[pymodule]
pub fn emulator_devices(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<EmulatorDeviceWrapper>()?;
    Ok(())
}
