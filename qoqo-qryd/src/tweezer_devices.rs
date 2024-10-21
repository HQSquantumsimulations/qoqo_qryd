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

use std::{collections::HashSet, io::Cursor};

use bincode::{deserialize, serialize};
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::{IntoPyDict, PyByteArray},
};

use qoqo::{devices::GenericDeviceWrapper, QoqoBackendError};
use qoqo_calculator_pyo3::convert_into_calculator_float;
use roqoqo::devices::Device;

use roqoqo_qryd::tweezer_devices::{
    ALLOWED_NATIVE_SINGLE_QUBIT_GATES, ALLOWED_NATIVE_THREE_QUBIT_GATES,
    ALLOWED_NATIVE_TWO_QUBIT_GATES,
};
use roqoqo_qryd::{QRydAPIDevice, TweezerDevice};

/// Tweezer Device
///
/// This interface does not allow setting any piece of information about the device
/// tweezers. This class is meant to be used by the end user.
///
/// Args:
///     seed ((Optional[int])): Optional seed, for simulation purposes.
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
    ///     seed (Optional[int]): Optional seed, for simulation purposes.
    ///     controlled_z_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledZ gate.
    ///     controlled_phase_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledPhase gate.
    ///
    /// Returns:
    ///     TweezerDevice: The new TweezerDevice instance.
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
            internal: TweezerDevice::new(seed, czpr, cppr),
        }
    }

    /// Creates a new TweezerDevice instance from a TweezerMutableDevice instance.
    ///
    /// Args:
    ///     device (TweezerMutableDevice): The TweezerMutableDevice instance.
    ///
    /// Returns:
    ///     TweezerDevice: The new TweezerDevice instance.
    #[staticmethod]
    #[pyo3(text_signature = "(device, /)")]
    fn from_mutable(device: Py<PyAny>) -> PyResult<Self> {
        let rust_dev = TweezerMutableDeviceWrapper::from_pyany(device)?;
        Ok(Self { internal: rust_dev })
    }

    /// Creates a new TweezerDevice instance containing populated tweezer data.
    ///
    /// This requires a valid QRYD_API_TOKEN. Visit `https://thequantumlaend.de/get-access/` to get one.
    ///
    /// Args:
    ///     device_name (Optional[str]): The name of the device to instantiate. Defaults to "qryd_emulator".
    ///     access_token (Optional[str]): An access_token is required to access QRYD hardware and emulators.
    ///                         The access_token can either be given as an argument here
    ///                             or set via the environmental variable `$QRYD_API_TOKEN`.
    ///     mock_port (Optional[str]): Server port to be used for testing purposes.
    ///     seed (Optional[int]): Optionally overwrite seed value from downloaded device instance.
    ///     dev (Optional[bool]): The boolean to set the dev header to.
    ///     api_version (Optional[str]): The version of the QRYD API to use. Defaults to "v1_1".
    ///
    /// Returns:
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
            TweezerDevice::from_api(device_name, access_token, mock_port, seed, dev, api_version)
                .map_err(|err| PyValueError::new_err(format!("{:}", err)))?;
        Ok(TweezerDeviceWrapper { internal })
    }

    /// Get the name of the current layout.
    ///
    /// Returns:
    ///     str: The name of the current layout.
    pub fn current_layout(&self) -> &str {
        self.internal
            .current_layout
            .as_ref()
            .expect("None")
            .as_str()
    }

    /// Switch to a different pre-defined Layout.
    ///
    /// It is updated only if the given Layout name is present in the device's
    /// Layout register. If the qubit -> tweezer mapping is empty, it is
    /// trivially populated by default.
    ///
    /// Args:
    ///     layout_number (str): The number index of the new Layout.
    ///     with_trivial_map (bool): Whether the qubit -> tweezer mapping should be trivially populated. Defaults to true.
    ///
    /// Raises:
    ///     PyValueError
    #[pyo3(text_signature = "(name, with_trivial_map, /)")]
    pub fn switch_layout(&mut self, name: &str, with_trivial_map: Option<bool>) -> PyResult<()> {
        self.internal
            .switch_layout(name, with_trivial_map)
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
                Ok(mapping) => Ok(mapping.into_py_dict_bound(py).into()),
                Err(err) => Err(PyValueError::new_err(format!("{:}", err))),
            }
        })
    }

    /// Get the qubit -> tweezer mapping of the device.
    ///
    /// Returns:
    ///     dict[int, int]: The qubit -> tweezer mapping.
    ///     None: The mapping is empty.
    pub fn get_qubit_to_tweezer_mapping(&self) -> Option<PyObject> {
        Python::with_gil(|py| -> Option<PyObject> {
            self.internal
                .qubit_to_tweezer
                .as_ref()
                .map(|mapping| mapping.into_py_dict_bound(py).into())
        })
    }

    /// Get the names of the available gates in the given layout.
    ///
    /// Args:
    ///     layout_name (Optional[str]): The name of the layout. Defaults to the current Layout.
    ///
    /// Returns:
    ///     list[str]: List of the names of the available gates in the given layout.
    ///
    /// Raises:
    ///     ValueError: No layout name provided and no current layout set.
    #[pyo3(text_signature = "(layout_name, /)")]
    pub fn get_available_gates_names(&self, layout_name: Option<String>) -> PyResult<Vec<&str>> {
        self.internal
            .get_available_gates_names(layout_name)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Get whether the device allows PragmaActiveReset operations or not.
    ///
    /// Returns:
    ///     bool: Whether the device allows PragmaActiveReset operations or not.
    pub fn get_allow_reset(&self) -> bool {
        self.internal.allow_reset
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
    ///     float: The gate time.
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
    ///     float: The gate time.
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
    ///     float: The gate time.
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
    ///     float: The gate time.
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
            PyByteArray::new_bound(py, &serialized[..]).into()
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
    pub fn from_bincode(input: &Bound<PyAny>) -> PyResult<TweezerDeviceWrapper> {
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
    /// Additionally, a gate set check is performed.
    ///
    /// Returns:
    ///     str: The serialized form of TweezerDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize TweezerDevice to json or
    ///         the device does not have valid QRyd gates available.
    fn to_json(&self) -> PyResult<String> {
        let mut all_gates_names: HashSet<&str> = HashSet::new();
        for layout in self.internal.available_layouts() {
            all_gates_names.extend(
                &self
                    .internal
                    .get_available_gates_names(Some(layout.to_string()))
                    .unwrap(),
            );
        }
        if all_gates_names.iter().any(|name| {
            !ALLOWED_NATIVE_SINGLE_QUBIT_GATES.contains(name)
                && !ALLOWED_NATIVE_TWO_QUBIT_GATES.contains(name)
                && !ALLOWED_NATIVE_THREE_QUBIT_GATES.contains(name)
        }) || all_gates_names.is_empty()
        {
            return Err(PyValueError::new_err(
                "The device does not support valid gates in a layout. ".to_owned()
                    + "The valid gates are: "
                    + &ALLOWED_NATIVE_SINGLE_QUBIT_GATES.join(", ")
                    + ", "
                    + &ALLOWED_NATIVE_TWO_QUBIT_GATES.join(", ")
                    + ", "
                    + &ALLOWED_NATIVE_THREE_QUBIT_GATES.join(", ")
                    + ".",
            ));
        }
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize TweezerDevice to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a TweezerDevice to a TweezerDevice.
    ///
    /// If a default_layout is found in the input, a layout switch is executed.
    /// Additionally, a gate set check is performed.
    ///
    /// Args:
    ///     input (str): The serialized TweezerDevice in json form.
    ///
    /// Returns:
    ///     TweezerDevice: The deserialized TweezerDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to TweezerDevice  or
    ///         the device does not have valid QRyd gates available.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    fn from_json(input: &str) -> PyResult<TweezerDeviceWrapper> {
        let mut internal: TweezerDevice = serde_json::from_str(input)
            .map_err(|_| PyValueError::new_err("Input cannot be deserialized to TweezerDevice"))?;
        let mut all_gates_names: HashSet<&str> = HashSet::new();
        for layout in internal.available_layouts() {
            all_gates_names.extend(
                &internal
                    .get_available_gates_names(Some(layout.to_string()))
                    .unwrap(),
            );
        }
        if all_gates_names.iter().any(|name| {
            !ALLOWED_NATIVE_SINGLE_QUBIT_GATES.contains(name)
                && !ALLOWED_NATIVE_TWO_QUBIT_GATES.contains(name)
                && !ALLOWED_NATIVE_THREE_QUBIT_GATES.contains(name)
        }) || all_gates_names.is_empty()
        {
            return Err(PyValueError::new_err(
                "The device does not support valid gates in a layout. ".to_owned()
                    + "The valid gates are: "
                    + &ALLOWED_NATIVE_SINGLE_QUBIT_GATES.join(", ")
                    + ", "
                    + &ALLOWED_NATIVE_TWO_QUBIT_GATES.join(", ")
                    + ", "
                    + &ALLOWED_NATIVE_THREE_QUBIT_GATES.join(", ")
                    + ".",
            ));
        }
        if let Some(layout) = &internal.default_layout {
            let _ = internal
                .switch_layout(&layout.to_string(), None)
                .map_err(|err| PyValueError::new_err(format!("{:}", err)));
        }
        Ok(TweezerDeviceWrapper { internal })
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
    /// Args:
    ///     layout_name (Optional[str]): The name of the layout to reference. Defaults to the current layout.
    ///
    /// Returns:
    ///     int: The number of tweezer positions in the device.
    #[pyo3(text_signature = "(layout_name, /)")]
    pub fn number_tweezer_positions(&self, layout_name: Option<String>) -> PyResult<usize> {
        self.internal
            .number_tweezer_positions(layout_name)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
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

    /// Returns the backend associated with the device.
    pub fn qrydbackend(&self) -> String {
        self.internal.qrydbackend()
    }

    /// Returns the seed usized for the API.
    pub fn seed(&self) -> Option<usize> {
        self.internal.seed()
    }

    /// Return the bincode representation of the Enum variant of the Device.
    ///
    /// Only used for internal interfacing.
    ///
    /// Returns:
    ///     ByteArray: The serialized TweezerDevice (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize Device to bytes.
    pub fn _enum_to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let qryd_enum: QRydAPIDevice = (&self.internal).into();
        let serialized = bincode::serialize(&qryd_enum)
            .map_err(|_| PyValueError::new_err("Cannot serialize TweezerDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new_bound(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Creates a graph representing a TweezerDevice.
    ///
    /// Args:
    ///     draw_shifts (Optional[bool]): Whether to draw shifts or not. Default: false
    ///     pixel_per_point (Optional[float]): The quality of the image.
    ///     file_save_path (Optional[str]): Path to save the image to. Default: output the image with the display method.
    ///
    /// Raises:
    ///     PyValueError - if there is no layout, an error occurred during the compilation or and invalid path was provided.
    ///
    #[pyo3(text_signature = "(draw_shifts, pixel_per_point, file_save_path, /)")]
    pub fn draw(
        &self,
        draw_shifts: Option<bool>,
        pixel_per_point: Option<f32>,
        file_save_path: Option<String>,
    ) -> PyResult<()> {
        let display_image = file_save_path.is_none();
        let image = self
            .internal
            .draw(
                pixel_per_point,
                draw_shifts.unwrap_or(false),
                &file_save_path,
            )
            .map_err(|x| PyValueError::new_err(format!("Error during Circuit drawing: {x:?}")))?;

        if display_image {
            let mut buffer = Cursor::new(Vec::new());
            image
                .write_to(&mut buffer, image::ImageFormat::Png)
                .map_err(|x| {
                    PyValueError::new_err(format!(
                        "Error during the generation of the Png file: {x:?}"
                    ))
                })?;
            Python::with_gil(|py| {
                let pil = PyModule::import_bound(py, "PIL.Image").unwrap();
                let io = PyModule::import_bound(py, "io").unwrap();
                let display = PyModule::import_bound(py, "IPython.display").unwrap();
                let builtins = PyModule::import_bound(py, "builtins").unwrap();

                let bytes_image_data = builtins
                    .call_method1("bytes", (buffer.clone().into_inner(),))
                    .unwrap();
                let bytes_io = io.call_method1("BytesIO", (bytes_image_data,)).unwrap();
                let image = pil.call_method1("open", (bytes_io,)).unwrap();

                display.call_method1("display", (image,)).unwrap();
            });
        }
        Ok(())
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
    pub internal: TweezerDevice,
}

#[pymethods]
impl TweezerMutableDeviceWrapper {
    /// Creates a new TweezerMutableDevice instance.
    ///
    /// Args:
    ///     seed (int): Optional seed, for simulation purposes.
    ///     controlled_z_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledZ gate.
    ///     controlled_phase_phase_relation (Optional[Union[str, float]]): The relation to use for the PhaseShiftedControlledPhase gate.
    ///
    /// Returns:
    ///     TweezerMutableDevice: The new TweezerMutableDevice instance.
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
            internal: TweezerDevice::new(seed, czpr, cppr),
        }
    }

    /// Get the name of the current layout.
    ///
    /// Returns:
    ///     str: The name of the current layout.
    pub fn current_layout(&self) -> &str {
        self.internal
            .current_layout
            .as_ref()
            .expect("None")
            .as_str()
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

    /// Switch to a different pre-defined Layout.
    ///
    /// It is updated only if the given Layout name is present in the device's
    /// Layout register. If the qubit -> tweezer mapping is empty, it is
    /// trivially populated by default.
    ///
    /// Args:
    ///     layout_number (str): The number index of the new Layout.
    ///     with_trivial_map (bool): Whether the qubit -> tweezer mapping should be trivially populated. Defaults to true.
    ///
    /// Raises:
    ///     PyValueError
    #[pyo3(text_signature = "(name, with_trivial_map, /)")]
    pub fn switch_layout(&mut self, name: &str, with_trivial_map: Option<bool>) -> PyResult<()> {
        self.internal
            .switch_layout(name, with_trivial_map)
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
                Ok(mapping) => Ok(mapping.into_py_dict_bound(py).into()),
                Err(err) => Err(PyValueError::new_err(format!("{:}", err))),
            }
        })
    }

    /// Get the qubit -> tweezer mapping of the device.
    ///
    /// Returns:
    ///     dict[int, int]: The qubit -> tweezer mapping.
    ///     None: The mapping is empty.
    pub fn get_qubit_to_tweezer_mapping(&self) -> Option<PyObject> {
        Python::with_gil(|py| -> Option<PyObject> {
            self.internal
                .qubit_to_tweezer
                .as_ref()
                .map(|mapping| mapping.into_py_dict_bound(py).into())
        })
    }

    /// Get the names of the available gates in the given layout.
    ///
    /// Args:
    ///     layout_name (Optional[str]): The name of the layout. Defaults to the current Layout.
    ///
    /// Returns:
    ///     list[str]: List of the names of the available gates in the given layout.
    ///
    /// Raises:
    ///     ValueError: No layout name provided and no current layout set.
    #[pyo3(text_signature = "(layout_name, /)")]
    pub fn get_available_gates_names(&self, layout_name: Option<String>) -> PyResult<Vec<&str>> {
        self.internal
            .get_available_gates_names(layout_name)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Get whether the device allows PragmaActiveReset operations or not.
    ///
    /// Returns:
    ///     bool: Whether the device allows PragmaActiveReset operations or not.
    pub fn get_allow_reset(&self) -> bool {
        self.internal.allow_reset
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
    ///     float: The gate time.
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
    ///     float: The gate time.
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
    ///     float: The gate time.
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
    ///     float: The gate time.
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
            PyByteArray::new_bound(py, &serialized[..]).into()
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
    pub fn from_bincode(input: &Bound<PyAny>) -> PyResult<TweezerMutableDeviceWrapper> {
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
    /// Additionally, a gate set check is performed.
    ///
    /// Returns:
    ///     str: The serialized form of TweezerMutableDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize TweezerMutableDevice to json or
    ///         the device does not have valid QRyd gates available.
    fn to_json(&self) -> PyResult<String> {
        let mut all_gates_names: HashSet<&str> = HashSet::new();
        for layout in self.internal.available_layouts() {
            all_gates_names.extend(
                &self
                    .internal
                    .get_available_gates_names(Some(layout.to_string()))
                    .unwrap(),
            );
        }
        if all_gates_names.iter().any(|name| {
            !ALLOWED_NATIVE_SINGLE_QUBIT_GATES.contains(name)
                && !ALLOWED_NATIVE_TWO_QUBIT_GATES.contains(name)
                && !ALLOWED_NATIVE_THREE_QUBIT_GATES.contains(name)
        }) || all_gates_names.is_empty()
        {
            return Err(PyValueError::new_err(
                "The device does not support valid gates in a layout. ".to_owned()
                    + "The valid gates are: "
                    + &ALLOWED_NATIVE_SINGLE_QUBIT_GATES.join(", ")
                    + ", "
                    + &ALLOWED_NATIVE_TWO_QUBIT_GATES.join(", ")
                    + ", "
                    + &ALLOWED_NATIVE_THREE_QUBIT_GATES.join(", ")
                    + ".",
            ));
        }
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize TweezerMutableDevice to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a TweezerMutableDevice to an TweezerMutableDevice.
    ///
    /// Additionally, a gate set check is performed.
    ///
    /// Args:
    ///     input (str): The serialized TweezerMutableDevice in json form.
    ///
    /// Returns:
    ///     TweezerMutableDevice: The deserialized TweezerMutableDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to TweezerMutableDevice or
    ///         the device does not have valid QRyd gates available.
    #[staticmethod]
    #[pyo3(text_signature = "(input, /)")]
    fn from_json(input: &str) -> PyResult<TweezerMutableDeviceWrapper> {
        let internal: TweezerDevice = serde_json::from_str(input).map_err(|_| {
            PyValueError::new_err("Input cannot be deserialized to TweezerMutableDevice")
        })?;
        let mut all_gates_names: HashSet<&str> = HashSet::new();
        for layout in internal.available_layouts() {
            all_gates_names.extend(
                &internal
                    .get_available_gates_names(Some(layout.to_string()))
                    .unwrap(),
            );
        }
        if all_gates_names.iter().any(|name| {
            !ALLOWED_NATIVE_SINGLE_QUBIT_GATES.contains(name)
                && !ALLOWED_NATIVE_TWO_QUBIT_GATES.contains(name)
                && !ALLOWED_NATIVE_THREE_QUBIT_GATES.contains(name)
        }) || all_gates_names.is_empty()
        {
            return Err(PyValueError::new_err(
                "The device does not support valid gates in a layout. ".to_owned()
                    + "The valid gates are: "
                    + &ALLOWED_NATIVE_SINGLE_QUBIT_GATES.join(", ")
                    + ", "
                    + &ALLOWED_NATIVE_TWO_QUBIT_GATES.join(", ")
                    + ", "
                    + &ALLOWED_NATIVE_THREE_QUBIT_GATES.join(", ")
                    + ".",
            ));
        }
        Ok(TweezerMutableDeviceWrapper { internal })
    }

    /// Return number of qubits in device.
    ///
    /// Returns:
    ///     int: The number of qubits.
    ///
    pub fn number_qubits(&self) -> usize {
        self.internal.number_qubits()
    }

    /// Returns the number of total tweezer positions in the device.
    ///
    /// Args:
    ///     layout_name (Optional[str]): The name of the layout to reference. Defaults to the current layout.
    ///
    /// Returns:
    ///     int: The number of tweezer positions in the device.
    #[pyo3(text_signature = "(layout_name, /)")]
    pub fn number_tweezer_positions(&self, layout_name: Option<String>) -> PyResult<usize> {
        self.internal
            .number_tweezer_positions(layout_name)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
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

    /// Returns the backend associated with the device.
    pub fn qrydbackend(&self) -> String {
        self.internal.qrydbackend()
    }

    /// Returns the seed usized for the API.
    pub fn seed(&self) -> Option<usize> {
        self.internal.seed()
    }

    /// Return the bincode representation of the Enum variant of the Device.
    ///
    /// Only used for internal interfacing.
    ///
    /// Returns:
    ///     ByteArray: The serialized TweezerMutableDevice (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize Device to bytes.
    pub fn _enum_to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let qryd_enum: QRydAPIDevice = (&self.internal).into();
        let serialized = bincode::serialize(&qryd_enum)
            .map_err(|_| PyValueError::new_err("Cannot serialize TweezerMutableDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new_bound(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Set the time of a single-qubit gate for a tweezer in a given Layout.
    ///
    /// Args:
    ///     hqslang (str): The hqslang name of a single-qubit gate.
    ///     tweezer (int): The index of the tweezer.
    ///     gate_time (float): The the gate time for the given gate.
    ///     layout_name (Optional[str]): The name of the Layout to apply the gate time in.
    ///         Defaults to the current Layout.
    ///
    /// Raises:
    ///     ValueError: No layout name provided and no current layout set.
    #[pyo3(text_signature = "(hqslang, tweezer, gate_time, layout_name, /)")]
    pub fn set_tweezer_single_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) -> PyResult<()> {
        self.internal
            .set_tweezer_single_qubit_gate_time(hqslang, tweezer, gate_time, layout_name)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Set the time of a two-qubit gate for a tweezer couple in a given Layout.
    ///
    /// Args:
    ///     hqslang (str): The hqslang name of a single-qubit gate.
    ///     tweezer0 (int): The index of the first tweezer.
    ///     tweezer1 (int): The index of the second tweezer.
    ///     gate_time (float): The the gate time for the given gate.
    ///     layout_name (Optional[str]): The name of the Layout to apply the gate time in.
    ///         Defaults to the current Layout.
    ///
    /// Raises:
    ///     ValueError: No layout name provided and no current layout set.
    #[pyo3(text_signature = "(hqslang, tweezer0, tweezer1, gate_time, layout_name, /)")]
    pub fn set_tweezer_two_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer0: usize,
        tweezer1: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) -> PyResult<()> {
        self.internal
            .set_tweezer_two_qubit_gate_time(hqslang, tweezer0, tweezer1, gate_time, layout_name)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Set the time of a three-qubit gate for a tweezer trio in a given Layout.
    ///
    /// Args:
    ///     hqslang (str): The hqslang name of a three-qubit gate.
    ///     tweezer0 (int): The index of the first tweezer.
    ///     tweezer1 (int): The index of the second tweezer.
    ///     tweezer2 (int): The index of the third tweezer.
    ///     gate_time (float): The the gate time for the given gate.
    ///     layout_name (Optional[str]): The name of the Layout to apply the gate time in.
    ///         Defaults to the current Layout.
    ///
    /// Raises:
    ///     ValueError: No layout name provided and no current layout set.
    #[pyo3(text_signature = "(hqslang, tweezer0, tweezer1, tweezer2, gate_time, layout_name, /)")]
    pub fn set_tweezer_three_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezer0: usize,
        tweezer1: usize,
        tweezer2: usize,
        gate_time: f64,
        layout_name: Option<String>,
    ) -> PyResult<()> {
        self.internal
            .set_tweezer_three_qubit_gate_time(
                hqslang,
                tweezer0,
                tweezer1,
                tweezer2,
                gate_time,
                layout_name,
            )
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Set the time of a multi-qubit gate for a list of tweezers in a given Layout.
    ///
    /// Args:
    ///     hqslang (str): The hqslang name of a multi-qubit gate.
    ///     tweezers (List[int]): The list of tweezer indexes.
    ///     gate_time (float): The the gate time for the given gate.
    ///     layout_name (Optional[str]): The name of the Layout to apply the gate time in.
    ///         Defaults to the current Layout.
    ///
    /// Raises:
    ///     ValueError: No layout name provided and no current layout set.
    #[pyo3(text_signature = "(hqslang, tweezers, gate_time, layout_name, /)")]
    pub fn set_tweezer_multi_qubit_gate_time(
        &mut self,
        hqslang: &str,
        tweezers: Vec<usize>,
        gate_time: f64,
        layout_name: Option<String>,
    ) -> PyResult<()> {
        self.internal
            .set_tweezer_multi_qubit_gate_time(hqslang, &tweezers, gate_time, layout_name)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
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
    ///     allowed_shifts (list[list[int]]): The allowed tweezer shifts.
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
    ///     row_shifts (list[list[int]]): A list of lists, each representing a row of tweezers.
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

    /// Set the tweezer per row value for a given Layout.
    ///
    /// This is needed for dynamically switching layouts during circuit execution.
    /// Only switching between layouts having the same tweezer per row value is supported.
    ///
    /// Args:
    ///     tweezers_per_row(List[int]): Vector containing the number of tweezers per row to set.
    ///     layout_name(Optional[str]): The name of the Layout to set the tweezer per row for. Defaults to the current Layout.
    ///
    /// Raises:
    ///     ValueError: No layout name provided and no current layout set.
    #[pyo3(text_signature = "(tweezers_per_row, layout_name, /)")]
    pub fn set_tweezers_per_row(
        &mut self,
        tweezers_per_row: Vec<usize>,
        layout_name: Option<String>,
    ) -> PyResult<()> {
        self.internal
            .set_tweezers_per_row(tweezers_per_row, layout_name)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Set whether the device allows PragmaActiveReset operations or not.
    ///
    /// Args:
    ///     allow_reset (bool): Whether the device should allow PragmaActiveReset operations or not.
    ///
    /// Raises:
    ///     ValueError: The device isn't compatible with PragmaActiveReset.
    #[pyo3(text_signature = "(allow_reset, /)")]
    pub fn set_allow_reset(&mut self, allow_reset: bool) -> PyResult<()> {
        self.internal
            .set_allow_reset(allow_reset)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Set the name of the default layout to use and switch to it.
    ///
    /// Args:
    ///     layout (str): The name of the layout to use.
    ///
    /// Raises:
    ///     ValueError: The given layout name is not present in the layout register.
    #[pyo3(text_signature = "(layout, /)")]
    pub fn set_default_layout(&mut self, layout: &str) -> PyResult<()> {
        self.internal
            .set_default_layout(layout)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Creates a graph representing a TweezerDevice.
    ///
    /// Args:
    ///     draw_shifts (Optional[bool]): Whether to draw shifts or not. Default: false
    ///     pixel_per_point (Optional[float]): The quality of the image.
    ///     file_save_path (Optional[str]): Path to save the image to. Default: output the image with the display method.
    ///
    /// Raises:
    ///     PyValueError - if there is no layout, an error occurred during the compilation or and invalid path was provided.
    ///
    #[pyo3(text_signature = "(draw_shifts, pixel_per_point, file_save_path, /)")]
    pub fn draw(
        &self,
        draw_shifts: Option<bool>,
        pixel_per_point: Option<f32>,
        file_save_path: Option<String>,
    ) -> PyResult<()> {
        let image = self
            .internal
            .draw(
                pixel_per_point,
                draw_shifts.unwrap_or(false),
                &file_save_path,
            )
            .map_err(|x| PyValueError::new_err(format!("Error during Circuit drawing: {x:?}")))?;

        if file_save_path.is_none() {
            let mut buffer = Cursor::new(Vec::new());
            image
                .write_to(&mut buffer, image::ImageFormat::Png)
                .map_err(|x| {
                    PyValueError::new_err(format!(
                        "Error during the generation of the Png file: {x:?}"
                    ))
                })?;
            Python::with_gil(|py| {
                let pil = PyModule::import_bound(py, "PIL.Image").unwrap();
                let io = PyModule::import_bound(py, "io").unwrap();
                let display = PyModule::import_bound(py, "IPython.display").unwrap();
                let builtins = PyModule::import_bound(py, "builtins").unwrap();

                let bytes_image_data = builtins
                    .call_method1("bytes", (buffer.clone().into_inner(),))
                    .unwrap();
                let bytes_io = io.call_method1("BytesIO", (bytes_image_data,)).unwrap();
                let image = pil.call_method1("open", (bytes_io,)).unwrap();

                display.call_method1("display", (image,)).unwrap();
            });
        }
        Ok(())
    }
}

impl TweezerMutableDeviceWrapper {
    /// Extracts a TweezerDevice from a TweezerDeviceWrapper python object.
    ///
    /// # Arguments:
    ///
    /// `input` - The Python object that should be casted to a [roqoqo_qryd::TweezerDevice]
    pub fn from_pyany(input: Py<PyAny>) -> PyResult<TweezerDevice> {
        Python::with_gil(|py| -> PyResult<TweezerDevice> {
            let input = input.bind(py);
            if let Ok(try_downcast) = input.extract::<TweezerMutableDeviceWrapper>() {
                Ok(try_downcast.internal)
            } else {
                Err(PyTypeError::new_err(
                    "Input cannot be converted to a TweezerMutableDevice instance.",
                ))
            }
        })
    }
}

impl From<TweezerMutableDeviceWrapper> for TweezerDeviceWrapper {
    fn from(mutable: TweezerMutableDeviceWrapper) -> Self {
        Self {
            internal: mutable.internal,
        }
    }
}

/// Convert generic python object to [roqoqo_qryd::TweezerDevice].
///
/// Fallible conversion of generic python object to [roqoqo_qryd::TweezerDevice].
pub fn convert_into_device(input: &Bound<PyAny>) -> Result<TweezerDevice, QoqoBackendError> {
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
pub fn tweezer_devices(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<TweezerDeviceWrapper>()?;
    m.add_class::<TweezerMutableDeviceWrapper>()?;
    Ok(())
}
