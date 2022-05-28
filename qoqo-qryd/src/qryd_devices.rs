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
use numpy::PyReadonlyArray2;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyType};
use qoqo::QoqoBackendError;
use roqoqo::devices::Device;
use roqoqo_qryd::qryd_devices::{FirstDevice, QRydDevice};
use std::collections::HashMap;

/// First example of a QRyd quantum device.
///
/// At the moment, it is only a prototype showcasing the fundamental design.
/// The device has a 2D grid of tweezer positions with a fixed number of rows and columns
/// Each row contains a `columns` tweezer positions.
/// The distance between neighbouring rows are fixed but in each row the tweezer positions can be changed.
///
/// Args:
///     number_rows (int): The fixed number of rows in device, needs to be the same for all layouts.
///     number_columns (int): Fixed number of tweezers in each row, needs to be the same for all layouts.
///     qubits_per_row (List[int]): Fixed number of occupied tweezer position in each row.
///                                 At the moment assumes that number of qubits in the traps is fixed. No loading/unloading once device is created.
///     row_distance (float): Fixed distance between rows.
///     initial_layout (np.ndarray): The starting layout (always had the index 0).
///     controlled_z_phase (Optional[float]): The phase shift in the native PhaseShiftedControlledZ gate. Defaults to pi/4.
///
/// Raises:
///     PyValueError
#[pyclass(name = "FirstDevice", module = "qoqo_qryd")]
#[derive(Clone, Debug, PartialEq)]
#[pyo3(
    text_signature = "(number_rows, number_columns, qubits_per_row, row_distance, initial_layour, /)"
)]
pub struct FirstDeviceWrapper {
    /// Internal storage of [roqoqo_qryd::FirstDevice]
    pub internal: FirstDevice,
}

#[pymethods]
impl FirstDeviceWrapper {
    /// Create new `First` QRyd device
    ///
    /// Args:
    ///     number_rows (int): The fixed number of rows in device, needs to be the same for all layouts.
    ///     number_columns (int): Fixed number of tweezers in each row, needs to be the same for all layouts.
    ///     qubits_per_row (List[int]): Fixed number of occupied tweezer position in each row.
    ///                                 At the moment assumes that number of qubits in the traps is fixed. No loading/unloading once device is created.
    ///     row_distance (float): Fixed distance between rows.
    ///     initial_layout (np.ndarray): The starting layout (always had the index 0).
    ///     controlled_z_phase (Optional[float]): The phase shift in the native PhaseShiftedControlledZ gate. Defaults to pi/4.
    /// Raises:
    ///     PyValueError
    #[new]
    pub fn new(
        number_rows: usize,
        number_columns: usize,
        qubits_per_row: Vec<usize>,
        row_distance: f64,
        initial_layout: PyReadonlyArray2<f64>,
        controlled_z_phase: Option<f64>,
    ) -> PyResult<Self> {
        Ok(Self {
            internal: FirstDevice::new(
                number_rows,
                number_columns,
                &qubits_per_row,
                row_distance,
                initial_layout.as_array().to_owned(),
                controlled_z_phase,
            )
            .map_err(|err| PyValueError::new_err(format!("{:?}", err)))?,
        })
    }

    /// Returns the phase shift in the native PhaseShiftedControlledZ gate.
    ///
    /// Returns:
    ///     Optional[float]
    pub fn controlled_z_phase(&self) -> f64 {
        self.internal.controlled_z_phase()
    }

    /// Return a copy of the FirstDevice (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     FirstDevice: A deep copy of self.
    pub fn __copy__(&self) -> FirstDeviceWrapper {
        self.clone()
    }

    /// Return a deep copy of the FirstDevice.
    ///
    /// Returns:
    ///     FirstDevice: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> FirstDeviceWrapper {
        self.clone()
    }

    /// Return the bincode representation of the FirstDevice using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized FirstDevice (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize FirstDevice to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize FirstDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the FirstDevice to a FirstDevice using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized FirstDevice (in bincode form).
    ///
    /// Returns:
    ///     FirstDevice: The deserialized FirstDevice.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to FirstDevice.
    #[classmethod]
    pub fn from_bincode(_cls: &PyType, input: &PyAny) -> PyResult<FirstDeviceWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(FirstDeviceWrapper {
            internal: deserialize(&bytes[..]).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to FirstDevice")
            })?,
        })
    }

    /// Return the json representation of the FirstDevice.
    ///
    /// Returns:
    ///     str: The serialized form of FirstDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize FirstDevice to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize FirstDevice to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a FirstDevice to a FirstDevice.
    ///
    /// Args:
    ///     input (str): The serialized FirstDevice in json form.
    ///
    /// Returns:
    ///     FirstDevice: The deserialized FirstDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to FirstDevice.
    #[classmethod]
    fn from_json(_cls: &PyType, input: &str) -> PyResult<FirstDeviceWrapper> {
        Ok(FirstDeviceWrapper {
            internal: serde_json::from_str(input).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to FirstDevice")
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

    /// Return the number of rows of optical tweezers in the two-dimensional grid of potential qubit positions.
    ///
    /// Returns:
    ///     int: The number of rows.
    ///
    pub fn number_rows(&self) -> usize {
        self.internal.number_rows()
    }

    /// Return number of columns in device.
    ///
    /// Returns:
    ///     int: The number of columns.
    ///
    pub fn number_columns(&self) -> usize {
        self.internal.number_columns()
    }

    /// Return the position of each qubit in the row-column grid of tweezer positions.
    ///
    /// Returns:
    ///     Dict[int, (int, int)]: Map between qubit number and row-column position
    pub fn qubit_positions(&self) -> HashMap<usize, (usize, usize)> {
        self.internal.qubit_positions().clone()
    }

    /// Change the positions of the qubits in their rows.
    ///
    /// The occupation of the available tweezer positions can be changed.
    /// This allows us to change the positions of the qubits in each row.
    ///
    /// Args:
    ///     new_positions (Dict[int, (int, int)]): The new column positions of the qubits, given as a map between qubits and new positions.
    ///
    /// Raises:
    ///     ValueError: trying to change the number of qubits in one row
    #[pyo3(text_signature = "(new_positions, /)")]
    pub fn change_qubit_positions(
        &mut self,
        new_positions: HashMap<usize, (usize, usize)>,
    ) -> PyResult<()> {
        self.internal
            .change_qubit_positions(&new_positions)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Switch to a different pre-defined layout.
    ///
    /// Args:
    ///     layout_number (int): The number index of the new layout
    ///
    /// Raises:
    ///     PyValueError
    #[pyo3(text_signature = "(layout_number, /)")]
    pub fn switch_layout(&mut self, layout_number: usize) -> PyResult<()> {
        self.internal
            .switch_layout(&layout_number)
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))
    }

    /// Add a new layout to the device.
    ///
    /// A layout is a two-dimensional representation of the y-positions of the tweezers in each row.
    /// The x-position is fixed by the row-distance.
    ///
    /// Args:
    ///     layout_number (int): The number index that is assigned to the new layout
    ///     layout (List[float]): The new layout that is added
    ///
    /// Raises:
    ///     PyValueError: layout number is already in use
    #[pyo3(text_signature = "(layout_number, layout, /)")]
    pub fn add_layout(
        &self,
        layout_number: usize,
        layout: PyReadonlyArray2<f64>,
    ) -> PyResult<Self> {
        let new_internal = self
            .internal
            .add_layout(layout_number, layout.as_array().to_owned())
            .map_err(|err| PyValueError::new_err(format!("{:}", err)))?;
        Ok(Self {
            internal: new_internal,
        })
    }

    /// Set distance cutoff for two-qubit gate operations.
    ///
    /// In the FirstQryd device the availability of two-qubit operations
    /// is determined by the physical distance between the involved qubits.
    ///
    /// When the distance is larger than the cut-off the two-qubit gate is not available.
    /// The cutoff defaults to 1.0 but can be changed with the set_cutoff function.
    ///
    /// Args:
    ///     cutoff (float): The new cutoff for interaction distance
    #[pyo3(text_signature = "(cutoff, /)")]
    pub fn set_cutoff(&mut self, cutoff: f64) -> PyResult<()> {
        self.internal.set_cutoff(cutoff);
        Ok(())
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
        let qryd_enum: QRydDevice = (&self.internal).into();
        let serialized = serialize(&qryd_enum)
            .map_err(|_| PyValueError::new_err("Cannot serialize FirstDevice to bytes"))?;
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
    /// ... from qoqo_qryd import FirstDevice
    /// ...
    /// ... device = FirstDevice(number_rows=2,number_columns= 2 qubits_per_row=[2,2], row_distance=1.0)
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
}

/// Convert generic python object to [roqoqo_qryd::QrydDevice].
///
/// Fallible conversion of generic python object to [roqoqo::FirstDevice].
pub fn convert_into_device(input: &PyAny) -> Result<QRydDevice, QoqoBackendError> {
    // Everything that follows tries to extract the circuit when two separately
    // compiled python packages are involved
    let get_bytes = input
        .call_method0("_enum_to_bincode")
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    let bytes = get_bytes
        .extract::<Vec<u8>>()
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    deserialize(&bytes[..]).map_err(|_| QoqoBackendError::CannotExtractObject)
}

/// Prototype qoqo devices for Rydberg hardware
///
/// .. autosummary::
///    :toctree: generated/
///
///    FirstDevice
///
#[pymodule]
pub fn qryd_devices(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FirstDeviceWrapper>()?;
    Ok(())
}
