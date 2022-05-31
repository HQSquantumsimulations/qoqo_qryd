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

//! Provides QRyd WebAPI Backend.

use crate::api_devices::convert_into_device;
use bincode::{deserialize, serialize};
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyDict};
use qoqo::QoqoBackendError;
use qoqo::{convert_into_circuit, convert_into_quantum_program};
use roqoqo::prelude::*;
use roqoqo::registers::{BitOutputRegister, ComplexOutputRegister, FloatOutputRegister};
use roqoqo::Circuit;
use roqoqo_qryd::APIBackend;
use roqoqo_qryd::QRydAPIDevice;
use std::collections::HashMap;

/// Qoqo backend interfacing QRydDemo WebAPI.
///
/// The WebAPI Backend implements methods available in the QRyd Web API.
/// Furthermore, QRyd quantum computer only allows gate operations
/// that are available on a device model of a QRyd device (stored in a [crate::QRydDevice]).
/// This limitation is introduced by design to check the compatability of quantum programs with a model of the QRyd hardware.
/// For simulations of the QRyd quantum computer use the Backend simulator [crate::Backend].
///
#[pyclass(name = "APIBackend", module = "qoqo_qryd")]
#[derive(Clone, Debug, PartialEq)]
#[pyo3(text_signature = "(device, access_token, timeout)")]
pub struct APIBackendWrapper {
    /// Internal storage of [roqoqo_qryd::APIBackend]
    pub internal: APIBackend,
}

/// Type of registers returned from a run of a Circuit.
pub type Registers = (
    HashMap<String, BitOutputRegister>,
    HashMap<String, FloatOutputRegister>,
    HashMap<String, ComplexOutputRegister>,
);

#[pymethods]
impl APIBackendWrapper {
    /// Create a new QRyd APIBackend.
    ///
    /// Args:
    ///     device (Device): QRydAPIDevice providing information about the endpoint running Circuits.
    ///     access_token (Optional[str]): Optional access token to QRyd endpoints.
    ///                                   When None access token is read from QRYD_API_TOKEN environmental variable.
    ///     timeout (Optional[int]) - Timeout for synchronous EvaluatingBackend trait. In the evaluating trait.
    ///               In synchronous operation the WebAPI is queried every 30 seconds until it has
    ///               been queried `timeout` times.
    ///
    /// Raises:
    ///     TypeError: Device Parameter is not QRydAPIDevice
    ///     RuntimeError: No access token found
    #[new]
    pub fn new(
        device: &PyAny,
        access_token: Option<String>,
        timeout: Option<usize>,
    ) -> PyResult<Self> {
        let device: QRydAPIDevice = convert_into_device(device).map_err(|err| {
            PyTypeError::new_err(format!("Device Parameter is not QRydAPIDevice {:?}", err))
        })?;
        Ok(Self {
            internal: APIBackend::new(device, access_token, timeout).map_err(|err| {
                PyRuntimeError::new_err(format!("No access token found {:?}", err))
            })?,
        })
    }

    /// Post to add a new job to be run on the backend and return the location of the job.
    ///
    /// Other free parameters of the job (`seed`, `pcz_theta` etc.)
    /// are provided by the device given during the initializing of the backend.
    ///
    /// The returned location is the URL of the job in String form
    /// that can be used to query the job status and result
    /// or to delete the job.
    ///
    /// Args:
    ///     quantumprogram (qoqo.QuantumProgram): qoqo QuantumProgram to be executed.
    ///
    /// Returns:
    ///     str: URL of the location of the job.
    #[pyo3(text_signature = "($self, quantumprogram)")]
    pub fn post_job(&self, quantumprogram: &PyAny) -> PyResult<String> {
        let program = convert_into_quantum_program(quantumprogram).map_err(|err| {
            PyTypeError::new_err(format!(
                "quantumprogram is not of type qoqo.QuantumProgram {}",
                err
            ))
        })?;
        let job_location = self
            .internal
            .post_job(program)
            .map_err(|err| PyRuntimeError::new_err(format!("Error posting job: {}", err)))?;
        Ok(job_location)
    }

    /// Get status of a posted WebAPI job.
    ///
    /// Args:
    ///     job_location (str): location (url) of the job one is interested in.
    ///
    /// Returns:
    ///     QRydJobStatus(dict): status and message of the job.
    ///
    #[pyo3(text_signature = "($self, job_location)")]
    pub fn get_job_status(&self, job_location: String) -> PyResult<HashMap<&'static str, String>> {
        let status = self.internal.get_job_status(job_location).map_err(|err| {
            PyRuntimeError::new_err(format!("Error retrieving job status: {}", err))
        })?;
        let mut result = HashMap::new();
        result.insert("status", status.status);
        result.insert("msg", status.msg);
        Ok(result)
    }

    /// Get status of a completed WebAPI job.
    ///
    /// Args:
    ///     job_location (str): location (url) of the job one is interested in.
    ///
    /// Returns
    ///     dict: Result of the job.
    ///
    #[pyo3(text_signature = "($self, job_location)")]
    pub fn get_job_result(&self, job_location: String) -> PyResult<PyObject> {
        let job_result = self.internal.get_job_result(job_location).map_err(|err| {
            PyRuntimeError::new_err(format!("Error retrieving job result: {}", err))
        })?;
        Python::with_gil(|py| -> PyResult<PyObject> {
            let result = PyDict::new(py);
            let data = PyDict::new(py);
            data.set_item("counts", job_result.data.counts)?;
            result.set_item("data", data)?;
            result.set_item("time_taken", job_result.time_taken)?;
            result.set_item("noise", job_result.noise)?;
            result.set_item("method", job_result.method)?;
            result.set_item("device", job_result.device)?;
            result.set_item("num_qubits", job_result.num_qubits)?;
            result.set_item("num_clbits", job_result.num_clbits)?;
            result.set_item("fusion_max_qubits", job_result.fusion_max_qubits)?;
            result.set_item("fusion_avg_qubits", job_result.fusion_avg_qubits)?;
            result.set_item("fusion_generated_gates", job_result.fusion_generated_gates)?;
            result.set_item(
                "executed_single_qubit_gates",
                job_result.executed_single_qubit_gates,
            )?;
            result.set_item(
                "executed_two_qubit_gates",
                job_result.executed_two_qubit_gates,
            )?;
            Ok(result.to_object(py))
        })
    }

    /// Delete a posted WebAPI job
    ///
    /// Args:
    ///     job_location (str): location (url) of the job one is interested in.
    ///
    /// Raises:
    ///     RuntimeError: Could not delete job.
    ///
    #[pyo3(text_signature = "($self, job_location)")]
    pub fn delete_job(&self, job_location: String) -> PyResult<()> {
        self.internal
            .delete_job(job_location)
            .map_err(|err| PyRuntimeError::new_err(format!("Error deleting job: {}", err)))
    }

    /// Return a copy of the APIBackend.
    ///
    /// (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     APIBackend: A deep copy of self.
    pub fn __copy__(&self) -> APIBackendWrapper {
        self.clone()
    }

    /// Return a deep copy of the APIBackend.
    ///
    /// Returns:
    ///     APIBackend: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> APIBackendWrapper {
        self.clone()
    }

    /// Return the bincode representation of the APIBackend using the bincode crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized APIBackend (in bincode form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize APIBackend to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize APIBackend to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the APIBackend to a APIBackend using the bincode crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized APIBackend (in bincode form).
    ///
    /// Returns:
    ///     APIBackend: The deserialized APIBackend.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to APIBackend.
    #[staticmethod]
    #[pyo3(text_signature = "(input)")]
    pub fn from_bincode(input: &PyAny) -> PyResult<APIBackendWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(APIBackendWrapper {
            internal: deserialize(&bytes[..])
                .map_err(|_| PyValueError::new_err("Input cannot be deserialized to APIBackend"))?,
        })
    }

    /// Return the json representation of the APIBackend.
    ///
    /// Returns:
    ///     str: The serialized form of APIBackend.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize APIBackend to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize APIBackend to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a APIBackend to a APIBackend.
    ///
    /// Args:
    ///     input (str): The serialized APIBackend in json form.
    ///
    /// Returns:
    ///     APIBackend: The deserialized APIBackend.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to APIBackend.
    #[staticmethod]
    #[pyo3(text_signature = "(input)")]
    fn from_json(input: &str) -> PyResult<APIBackendWrapper> {
        Ok(APIBackendWrapper {
            internal: serde_json::from_str(input)
                .map_err(|_| PyValueError::new_err("Input cannot be deserialized to APIBackend"))?,
        })
    }

    /// Run a circuit with the QRyd APIBackend.
    ///
    /// A circuit is passed to the APIBackend and executed.
    /// During execution values are written to and read from classical registers
    /// (List[bool], List[float], List[complex]).
    /// To produce sufficient statistics for evaluating expectation values,
    /// circuits have to be run multiple times.
    /// The results of each repetition are concatenated in OutputRegisters
    /// (List[List[bool]], List[List[float]], List[List[complex]]).  
    ///
    ///
    /// Args:
    ///     circuit (Circuit): The circuit that is run on the APIBackend.
    ///
    /// Returns:
    ///     Tuple[Dict[str, List[List[bool]]], Dict[str, List[List[float]]]], Dict[str, List[List[complex]]]]: The output registers written by the evaluated circuits.
    ///
    /// Raises:
    ///     TypeError: Circuit argument cannot be converted to qoqo Circuit
    ///     RuntimeError: Running Circuit failed
    #[pyo3(text_signature = "($self, circuit)")]
    pub fn run_circuit(&self, circuit: &PyAny) -> PyResult<Registers> {
        let circuit = convert_into_circuit(circuit).map_err(|err| {
            PyTypeError::new_err(format!(
                "Circuit argument cannot be converted to qoqo Circuit {:?}",
                err
            ))
        })?;
        self.internal
            .run_circuit(&circuit)
            .map_err(|err| PyRuntimeError::new_err(format!("Running Circuit failed {:?}", err)))
    }

    /// Run all circuits corresponding to one measurement with the QRyd APIBackend.
    ///
    /// An expectation value measurement in general involves several circuits.
    /// Each circuit is passed to the APIBackend and executed separately.
    /// During execution values are written to and read from classical registers
    /// (List[bool], List[float], List[complex]).
    /// To produce sufficient statistics for evaluating expectation values,
    /// circuits have to be run multiple times.
    /// The results of each repetition are concatenated in OutputRegisters
    /// (List[List[bool]], List[List[float]], List[List[complex]]).  
    ///
    ///
    /// Args:
    ///     measurement (Measurement): The measurement that is run on the APIBackend.
    ///
    /// Returns:
    ///     Tuple[Dict[str, List[List[bool]]], Dict[str, List[List[float]]]], Dict[str, List[List[complex]]]]: The output registers written by the evaluated circuits.
    ///
    /// Raises:
    ///     TypeError: Circuit argument cannot be converted to qoqo Circuit
    ///     RuntimeError: Running Circuit failed
    #[pyo3(text_signature = "($self, measurement)")]
    pub fn run_measurement_registers(&self, measurement: &PyAny) -> PyResult<Registers> {
        let mut run_circuits: Vec<Circuit> = Vec::new();

        let get_constant_circuit = measurement
            .call_method0("constant_circuit")
            .map_err(|err| {
                PyTypeError::new_err(format!(
                    "Cannot extract constant circuit from measurement {:?}",
                    err
                ))
            })?;
        let const_circuit = get_constant_circuit
            .extract::<Option<&PyAny>>()
            .map_err(|err| {
                PyTypeError::new_err(format!(
                    "Cannot extract constant circuit from measurement {:?}",
                    err
                ))
            })?;

        let constant_circuit = match const_circuit {
            Some(x) => convert_into_circuit(x).map_err(|err| {
                PyTypeError::new_err(format!(
                    "Cannot extract constant circuit from measurement {:?}",
                    err
                ))
            })?,
            None => Circuit::new(),
        };

        let get_circuit_list = measurement.call_method0("circuits").map_err(|err| {
            PyTypeError::new_err(format!(
                "Cannot extract circuit list from measurement {:?}",
                err
            ))
        })?;
        let circuit_list = get_circuit_list.extract::<Vec<&PyAny>>().map_err(|err| {
            PyTypeError::new_err(format!(
                "Cannot extract circuit list from measurement {:?}",
                err
            ))
        })?;

        for c in circuit_list {
            run_circuits.push(
                constant_circuit.clone()
                    + convert_into_circuit(c).map_err(|err| {
                        PyTypeError::new_err(format!(
                            "Cannot extract circuit of circuit list from measurement {:?}",
                            err
                        ))
                    })?,
            )
        }

        let mut bit_registers: HashMap<String, BitOutputRegister> = HashMap::new();
        let mut float_registers: HashMap<String, FloatOutputRegister> = HashMap::new();
        let mut complex_registers: HashMap<String, ComplexOutputRegister> = HashMap::new();

        for circuit in run_circuits {
            let (tmp_bit_reg, tmp_float_reg, tmp_complex_reg) =
                self.internal.run_circuit(&circuit).map_err(|err| {
                    PyRuntimeError::new_err(format!("Running a circuit failed {:?}", err))
                })?;

            for (key, mut val) in tmp_bit_reg.into_iter() {
                if let Some(x) = bit_registers.get_mut(&key) {
                    x.append(&mut val);
                } else {
                    let _ = bit_registers.insert(key, val);
                }
            }
            for (key, mut val) in tmp_float_reg.into_iter() {
                if let Some(x) = float_registers.get_mut(&key) {
                    x.append(&mut val);
                } else {
                    let _ = float_registers.insert(key, val);
                }
            }
            for (key, mut val) in tmp_complex_reg.into_iter() {
                if let Some(x) = complex_registers.get_mut(&key) {
                    x.append(&mut val);
                } else {
                    let _ = complex_registers.insert(key, val);
                }
            }
        }
        Ok((bit_registers, float_registers, complex_registers))
    }

    /// Evaluates expectation values of a measurement with the APIBackend.
    ///
    /// Args:
    ///     measurement (Measurement): The measurement that is run on the APIBackend.
    ///
    /// Returns:
    ///     Optional[Dict[str, float]]: The  dictionary of expectation values.
    ///
    /// Raises:
    ///     TypeError: Measurement evaluate function could not be used
    ///     RuntimeError: Internal error measurement.evaluation returned unknown type
    #[pyo3(text_signature = "($self, measurement)")]
    pub fn run_measurement(&self, measurement: &PyAny) -> PyResult<Option<HashMap<String, f64>>> {
        let (bit_registers, float_registers, complex_registers) =
            self.run_measurement_registers(measurement)?;
        let get_expectation_values = measurement
            .call_method1(
                "evaluate",
                (bit_registers, float_registers, complex_registers),
            )
            .map_err(|err| {
                PyTypeError::new_err(format!(
                    "Measurement evaluate function could not be used: {:?}",
                    err
                ))
            })?;
        get_expectation_values
            .extract::<Option<HashMap<String, f64>>>()
            .map_err(|_| {
                PyRuntimeError::new_err(
                    "Internal error measurement.evaluation returned unknown type",
                )
            })
    }
}

/// Convert generic python object to [roqoqo_qryd::APIBackend].
///
/// Fallible conversion of generic python object to [roqoqo_qryd::APIBackend].
pub fn convert_into_backend(input: &PyAny) -> Result<APIBackend, QoqoBackendError> {
    if let Ok(try_downcast) = input.extract::<APIBackendWrapper>() {
        Ok(try_downcast.internal)
    } else {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use roqoqo_qryd::api_devices::*;
    #[test]
    fn debug_and_clone() {
        let device: QRydAPIDevice = QrydEmuSquareDevice::new(None, None).into();
        let backend = APIBackend::new(device.clone(), Some("".to_string()), Some(2)).unwrap();
        let wrapper = APIBackendWrapper { internal: backend };
        let a = format!("{:?}", wrapper);
        assert!(a.contains("QrydEmuSquareDevice"));
        let backend2 = APIBackend::new(device, Some("a".to_string()), Some(2)).unwrap();
        let wrapper2 = APIBackendWrapper { internal: backend2 };
        assert_eq!(wrapper.clone(), wrapper);
        assert_ne!(wrapper, wrapper2);
    }
}
