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

use crate::api_devices::QRydAPIDevice;
use bitvec::prelude::*;
use num_complex::Complex64;
use reqwest::blocking::Client;
use roqoqo::backends::RegisterResult;
use roqoqo::measurements::ClassicalRegister;
use roqoqo::operations::Define;
use roqoqo::operations::Operation;
use roqoqo::operations::*;
use roqoqo::prelude::EvaluatingBackend;
use roqoqo::prelude::Operate;
use roqoqo::Circuit;
use roqoqo::QuantumProgram;
use roqoqo::RoqoqoBackendError;
// use roqoqo_1_0;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::env;
use std::{thread, time};

/// QRyd WebAPI backend.
///
/// The WebAPI backend implements methods available in the QRyd Web API.
/// Furthermore, QRyd quantum computer only allows gate operations
/// that are available on a device model of a QRyd device (stored in a [crate::QRydDevice]).
/// This limitation is introduced by design to check the compatability of quantum programs with a model of the QRyd hardware.
/// For simulations of the QRyd quantum computer use the backend simulator [crate::Backend].
///
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct APIBackend {
    /// Device representing the model of a QRyd device.
    pub device: QRydAPIDevice,
    /// Access token for identification with QRyd devices
    access_token: String,
    /// Timeout for synchronous EvaluatingBackend trait. In the evaluating trait.
    /// In synchronous operation the WebAPI is queried every 30 seconds until it has
    /// been queried `timeout` times.
    timeout: usize,
    /// The address of the Mock server, used for testing purposes.
    mock_port: Option<String>,
    /// Is develop version. Defaults to `false`.
    pub dev: bool,
    /// API version.
    api_version: String,
}

/// Local struct representing the body of the request message
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct QRydRunData {
    /// Format of the quantum program: qoqo
    format: String,
    /// The QRyd WebAPI Backend used to execute operations and circuits.
    /// At the moment limited to the string ('qryd_emulator')
    backend: String,
    /// Is develop version default: false
    dev: bool,
    /// Qubits that are fused in simulator default none
    fusion_max_qubits: usize,
    /// Random seed for the simulator default none
    seed_simulator: Option<usize>,
    /// Random seed for the compiler default none
    seed_compiler: Option<usize>,
    /// Use the extended set in SABRE routing
    /// default true
    use_extended_set: bool,
    /// Use back-and-forth SABRE runs to optimize initial qubit mapping
    /// default true
    use_reverse_traversal: bool,
    /// Number of back-and-forth iterations used
    reverse_traversal_iterations: usize,
    /// Size of the extended set if used, default 5
    extended_set_size: usize,
    /// Weight given to the extended set, default 0.5
    extended_set_weight: f64,
    /// Roqoqo QuantumProgram to be executed.
    program: QuantumProgram,
}

/// Local struct representing the body of a validation error message
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ValidationError {
    detail: ValidationTypes,
    body: Option<QRydRunData>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum LocTypes {
    LocStr(String),
    LocInt(i32),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum ValidationTypes {
    Simple(String),
    Detailed(Vec<ValidationErrorDetail>),
}

/// Local struct representing the body of a validation error message
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ValidationErrorDetail {
    #[serde(default)]
    loc: Vec<LocTypes>,
    #[serde(default)]
    msg: String,
    #[serde(alias = "type")]
    #[serde(default)]
    internal_type: String,
}

/// Struct to represent QRyd response when calling for the Job status.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct QRydJobStatus {
    /// status of the job, e.g. "pending"
    #[serde(default)] // for optional fields
    pub status: String,
    /// message, if any
    #[serde(default)]
    pub msg: String,
}

// /// Convert from new roqoqo 1.1.0 QuantumProgram to 1.0.0
// #[allow(unused)]
// pub fn downconvert_roqoqo_version(
//     program: QuantumProgram,
// ) -> Result<roqoqo_1_0::QuantumProgram, RoqoqoBackendError> {
//     let (measurement, input_parameter_names) = match program {
//         QuantumProgram::ClassicalRegister {
//             measurement,
//             input_parameter_names,
//         } => Ok((measurement, input_parameter_names)),
//         _ => Err(RoqoqoBackendError::GenericError {
//             msg:
//                 "Only ClassiclaRegister measurements are supported by the Qryd WebAPI at the moment"
//                     .to_string(),
//         }),
//     }?;
//     let mut downconverted_circuit = roqoqo_1_0::Circuit::new();
//     for op in measurement.circuits[0].iter() {
//         match op {
//             Operation::InputBit(_op) => {
//                 return Err(RoqoqoBackendError::GenericError {
//                     msg: "InputBit operation not compatible with roqoqo 1.0 and QRyd Web API v2_0"
//                         .to_string(),
//                 });
//             }
//             Operation::PragmaLoop(_op) => {
//                 return Err(RoqoqoBackendError::GenericError {
//                     msg:
//                         "PragmaLoop operation not compatible with roqoqo 1.0 and QRyd Web API v2_0"
//                             .to_string(),
//                 });
//             }
//             _ => {
//                 let serialized_op =
//                     serde_json::to_string(&op).map_err(|err| RoqoqoBackendError::GenericError {
//                         msg: format!("Internal error cannot serialize operation {}", err),
//                     })?;
//                 let new_op: roqoqo_1_0::operations::Operation = serde_json::from_str(&serialized_op).map_err(|err| RoqoqoBackendError::GenericError { msg: format!("Error could not convert Operation to roqoqo 1.0 compatible Operation. QRyd WebAPI only support roqoqo 1.0 compatible programs at the moment {}", err) })?;
//                 downconverted_circuit += new_op;
//             }
//         }
//     }

//     let downconverted_const_circuit = if let Some(const_circ) = measurement.constant_circuit {
//         let mut new_circuit = roqoqo_1_0::Circuit::new();
//         for op in const_circ.iter() {
//             match op {
//                 Operation::InputBit(_op) => {
//                     return Err(RoqoqoBackendError::GenericError {
//                     msg: "InputBit operation not compatible with roqoqo 1.0 and QRyd Web API v2_0"
//                         .to_string(),
//                 });
//                 }
//                 Operation::PragmaLoop(_op) => {
//                     return Err(RoqoqoBackendError::GenericError {
//                     msg:
//                         "PragmaLoop operation not compatible with roqoqo 1.0 and QRyd Web API v2_0"
//                             .to_string(),
//                 });
//                 }
//                 _ => {
//                     let serialized_op = serde_json::to_string(&op).map_err(|err| {
//                         RoqoqoBackendError::GenericError {
//                             msg: format!("Internal error cannot serialize operation {}", err),
//                         }
//                     })?;
//                     let new_op: roqoqo_1_0::operations::Operation = serde_json::from_str(&serialized_op).map_err(|err| RoqoqoBackendError::GenericError { msg: format!("Error could not convert Operation to roqoqo 1.0 compatible Operation. QRyd WebAPI only support roqoqo 1.0 compatible programs at the moment {}", err) })?;
//                     new_circuit += new_op;
//                 }
//             }
//         }
//         Some(new_circuit)
//     } else {
//         None
//     };
//     let downconverted_measurement = roqoqo_1_0::measurements::ClassicalRegister {
//         constant_circuit: downconverted_const_circuit,
//         circuits: vec![downconverted_circuit],
//     };
//     let downconverted_program = roqoqo_1_0::QuantumProgram::ClassicalRegister {
//         measurement: downconverted_measurement,
//         input_parameter_names,
//     };
//     Ok(downconverted_program)
// }

/// Struct to represent QRyd response on the result for the posted Job.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct QRydJobResult {
    /// The actual measured data
    #[serde(default)]
    pub data: ResultCounts,
    /// Time taken to run and return the result
    #[serde(default)]
    pub time_taken: f64,
    #[serde(default)]
    /// The noise that was used in the run
    pub noise: String,
    #[serde(default)]
    /// The method that was used for the run
    pub method: String,
    #[serde(default)]
    /// The device that was used for the run
    pub device: String,
    // #[serde(default)]
    // /// The used precision
    // pub precision: String,
    #[serde(default)]
    /// The number of qubits that were used in the run
    pub num_qubits: u32,
    /// Number of classical bits
    #[serde(default)]
    pub num_clbits: u32,
    #[serde(default)]
    /// Max qubits
    pub fusion_max_qubits: u32,
    #[serde(default)]
    /// Average qubits
    pub fusion_avg_qubits: f64,
    #[serde(default)]
    /// Number of gates generated by gate fusion
    pub fusion_generated_gates: u32,
    #[serde(default)]
    /// Number of single qubit gates actually executed in the circuit
    pub executed_single_qubit_gates: u32,
    #[serde(default)]
    /// Number of two qubit gates actually executed in the circuit
    pub executed_two_qubit_gates: u32,
    /// The time taken to compile the quantum program on the WebAPI
    #[serde(default)]
    pub compilation_time: f64,
}

/// Represents the counts of measurements returned by QRyd API
///
/// Format corresponds to qiskit count format e.g.
///
/// ```python
/// counts = {'0x1': 100.0, '0x4': 20.0}
/// ```
///
/// where out of a total of 120 measurements 100 times
/// qubit 0 was measured in state |1> while the same measurement gave |0> for
/// qubits 1 and 2 and 20 times qubit 2 was measured in state |1>
/// with qubits 1 and 0 in state |0>
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ResultCounts {
    /// The dictionary of counts for each measured string
    pub counts: HashMap<String, u64>,
}

impl APIBackend {
    /// Creates a new QRyd WebAPI backend.
    ///
    /// # Arguments
    ///
    /// * `device` - The QRyd device the Backend uses to execute operations and circuits.
    ///                     At the moment limited to the QRyd emulator.
    /// * `access_token` - An access_token is required to access QRYD hardware and emulators.
    ///                                 The access_token can either be given as an argument here
    ///                                 or set via the environmental variable `$QRYD_API_TOKEN`
    /// * `timeout` - Timeout for synchronous EvaluatingBackend trait. In the evaluating trait.
    ///               In synchronous operation the WebAPI is queried every 30 seconds until it has
    ///               been queried `timeout` times.
    /// * `mock_port` - Server port to be used for testing purposes.
    /// * `dev` - The boolean to set the dev option to.
    /// * `api_version` - The version of the QRyd WebAPI to use. Defaults to "v5_2".
    ///
    pub fn new(
        device: QRydAPIDevice,
        access_token: Option<String>,
        timeout: Option<usize>,
        mock_port: Option<String>,
        dev: Option<bool>,
        api_version: Option<String>,
    ) -> Result<Self, RoqoqoBackendError> {
        if mock_port.is_some() {
            Ok(Self {
                device,
                access_token: "".to_string(),
                timeout: timeout.unwrap_or(30),
                mock_port,
                dev: false,
                api_version: api_version.unwrap_or("v5_2".to_string()),
            })
        } else {
            let access_token_internal: String = match access_token {
                Some(s) => s,
                None => env::var("QRYD_API_TOKEN").map_err(|_| {
                    RoqoqoBackendError::MissingAuthentication {
                        msg: "QRYD access token is missing".to_string(),
                    }
                })?,
            };

            Ok(Self {
                device,
                access_token: access_token_internal,
                timeout: timeout.unwrap_or(30),
                mock_port,
                dev: dev.unwrap_or(false),
                api_version: api_version.unwrap_or("v5_2".to_string()),
            })
        }
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
    /// # Arguments
    ///
    /// * `quantumprogram` - Roqoqo QuantumProgram to be executed.
    ///
    pub fn post_job(&self, quantumprogram: QuantumProgram) -> Result<String, RoqoqoBackendError> {
        // Prepare data that need to be passed to the WebAPI client
        let seed_param: Option<usize> = self.device.seed(); // seed.unwrap_or(0);
        let mut transform_pragma_repeated_measurement: bool = false;

        match &quantumprogram {
            QuantumProgram::ClassicalRegister { measurement, .. } => {
                if measurement.circuits.len() != 1 {
                    return Err(RoqoqoBackendError::GenericError { msg: "QRyd API Backend only supports posting ClassicalRegister with one circuit".to_string() });
                }
                if measurement.circuits[0].is_parametrized() {
                    return Err(RoqoqoBackendError::GenericError { msg: "Qoqo circuit contains symbolic parameters. The QrydWebAPI does not support symbolic parameters.".to_string() });
                }
                if measurement.circuits[0].count_occurences(&["PragmaRepeatedMeasurement"]) >= 1 {
                    transform_pragma_repeated_measurement = true;
                }
                if let Some(const_c) = &measurement.constant_circuit {
                    if const_c.count_occurences(&["PragmaRepeatedMeasurement"]) >= 1 {
                        transform_pragma_repeated_measurement = true;
                    }
                }
            }
            _ => {
                return Err(RoqoqoBackendError::GenericError {
                    msg: "QRyd API Backend only supports posting ClassicalRegister QuantumPrograms"
                        .to_string(),
                })
            }
        }

        self._check_for_api_compatability(&quantumprogram)?;

        // If a PragmaRepeatedMeasurement is present, substitute it with a set of MeasureQubit operations
        //  followed by a PragmaSetNumberOfMeasurements.
        // If not, take user's input directly.
        let filtered_qp: QuantumProgram = if transform_pragma_repeated_measurement {
            let (previous_circuit, previous_const_circuit) = match &quantumprogram {
                QuantumProgram::ClassicalRegister { measurement, .. } => (
                    measurement.circuits[0].clone(),
                    measurement.constant_circuit.clone(),
                ),
                _ => return Err(RoqoqoBackendError::GenericError {
                    msg: "QRyd API Backend only supports posting ClassicalRegister QuantumPrograms"
                        .to_string(),
                }),
            };

            let mut modified_circuit = Circuit::new();
            let mut modified_const_circuit: Option<Circuit> = None;

            let mut involved_set = HashSet::<usize>::new();
            for op in previous_circuit.iter() {
                match op {
                    Operation::PragmaRepeatedMeasurement(pragma) => {
                        modified_circuit += self
                            ._transform_pragma_repeated_measurements(pragma.clone(), &involved_set);
                    }
                    _ => {
                        match op.involved_qubits() {
                            InvolvedQubits::All => {}
                            InvolvedQubits::None => {}
                            InvolvedQubits::Set(op_set) => {
                                involved_set.extend(op_set);
                            }
                        }
                        modified_circuit.add_operation(op.clone());
                    }
                }
            }
            if let Some(const_circuit) = previous_const_circuit {
                let mut inner_const = Circuit::new();
                let mut involved_set = HashSet::<usize>::new();
                for op in const_circuit.iter() {
                    match op {
                        Operation::PragmaRepeatedMeasurement(pragma) => {
                            inner_const += self._transform_pragma_repeated_measurements(
                                pragma.clone(),
                                &involved_set,
                            );
                        }
                        _ => {
                            match op.involved_qubits() {
                                InvolvedQubits::All => {}
                                InvolvedQubits::None => {}
                                InvolvedQubits::Set(op_set) => {
                                    involved_set.extend(op_set);
                                }
                            }
                            inner_const.add_operation(op.clone());
                        }
                    }
                }
                modified_const_circuit = Some(inner_const);
            }
            QuantumProgram::ClassicalRegister {
                measurement: ClassicalRegister {
                    constant_circuit: modified_const_circuit,
                    circuits: vec![modified_circuit],
                },
                input_parameter_names: vec![],
            }
        } else {
            quantumprogram
        };

        // let quantumprogram: roqoqo_1_0::QuantumProgram =
        //     downconvert_roqoqo_version(quantumprogram)?;
        // dbg!(&serde_json::to_string(&quantumprogram).unwrap());
        let data = QRydRunData {
            format: "qoqo".to_string(),
            backend: self.device.qrydbackend(),
            program: filtered_qp,
            dev: self.dev,
            fusion_max_qubits: 4,
            seed_simulator: seed_param,
            seed_compiler: None,
            use_extended_set: true,
            use_reverse_traversal: true,
            extended_set_size: 5,
            extended_set_weight: 0.5,
            reverse_traversal_iterations: 3,
        };

        // Prepare WebAPI client
        let client: Client = if self.mock_port.is_some() {
            reqwest::blocking::Client::builder().build().map_err(|x| {
                RoqoqoBackendError::NetworkError {
                    msg: format!("could not create test client {:?}", x),
                }
            })?
        } else {
            reqwest::blocking::Client::builder()
                .https_only(true)
                .build()
                .map_err(|x| RoqoqoBackendError::NetworkError {
                    msg: format!("could not create https client {:?}", x),
                })?
        };
        let hqs_env_var = env::var("QRYD_API_HQS").is_ok();

        // Call WebAPI client
        // here: value for put() temporarily fixed.
        // needs to be derived dynamically based on the provided parameter 'qrydbackend'
        let resp = if let Some(mock_port) = &self.mock_port {
            client
                .post(format!("http://127.0.0.1:{}", mock_port))
                .json(&data)
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?
        } else {
            match (self.dev, hqs_env_var) {
                (true, true) => client
                    .post(format!(
                        "https://api.qryddemo.itp3.uni-stuttgart.de/{}/jobs",
                        self.api_version
                    ))
                    .header("X-API-KEY", self.access_token.clone())
                    .header("X-DEV", "?1")
                    .header("X-HQS", "?1")
                    .json(&data)
                    .send()
                    .map_err(|e| RoqoqoBackendError::NetworkError {
                        msg: format!("{:?}", e),
                    })?,
                (true, false) => client
                    .post(format!(
                        "https://api.qryddemo.itp3.uni-stuttgart.de/{}/jobs",
                        self.api_version
                    ))
                    .header("X-API-KEY", self.access_token.clone())
                    .header("X-DEV", "?1")
                    .json(&data)
                    .send()
                    .map_err(|e| RoqoqoBackendError::NetworkError {
                        msg: format!("{:?}", e),
                    })?,
                (false, true) => client
                    .post(format!(
                        "https://api.qryddemo.itp3.uni-stuttgart.de/{}/jobs",
                        self.api_version
                    ))
                    .header("X-API-KEY", self.access_token.clone())
                    .header("X-HQS", "?1")
                    .json(&data)
                    .send()
                    .map_err(|e| RoqoqoBackendError::NetworkError {
                        msg: format!("{:?}", e),
                    })?,
                (false, false) => client
                    .post(format!(
                        "https://api.qryddemo.itp3.uni-stuttgart.de/{}/jobs",
                        self.api_version
                    ))
                    .header("X-API-KEY", self.access_token.clone())
                    .json(&data)
                    .send()
                    .map_err(|e| RoqoqoBackendError::NetworkError {
                        msg: format!("{:?}", e),
                    })?,
            }
        };

        let status_code = resp.status();
        if status_code != reqwest::StatusCode::CREATED {
            if status_code == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
                let querry_response: ValidationError =
                    resp.json::<ValidationError>().map_err(|e| {
                        RoqoqoBackendError::NetworkError {
                            msg: format!("Error parsing ValidationError message {:?}", e),
                        }
                    })?;
                return Err(self._handle_validation_error(querry_response));
            }
            Err(RoqoqoBackendError::NetworkError {
                msg: format!(
                    "Request to server failed with HTTP status code {:?}",
                    status_code
                ),
            })
        } else {
            let resp_headers = resp.headers();
            if resp_headers.contains_key("Location") {
                Ok(resp_headers["Location"]
                    .to_str()
                    .map_err(|err| RoqoqoBackendError::NetworkError {
                        msg: format!("Server response missing the Location header {:?}", err),
                    })?
                    .to_string())
            } else {
                Err(RoqoqoBackendError::NetworkError {
                    msg: "Server response missing the Location header".to_string(),
                })
            }
        }
    }

    /// Get status of a posted WebAPI job.
    ///
    /// # Arguments
    ///
    /// * `job_location` - location (url) of the job one is interested in.
    ///
    /// # Returns
    ///
    /// * QRydJobStatus - status and message of the job.
    /// * RoqoqoBackendError in case of a network failure.
    ///
    pub fn get_job_status(
        &self,
        job_location: String,
    ) -> Result<QRydJobStatus, RoqoqoBackendError> {
        // Prepare WebAPI client
        let client: Client = if self.mock_port.is_some() {
            reqwest::blocking::Client::builder().build().map_err(|x| {
                RoqoqoBackendError::NetworkError {
                    msg: format!("could not create test client {:?}", x),
                }
            })?
        } else {
            reqwest::blocking::Client::builder()
                .https_only(true)
                .build()
                .map_err(|x| RoqoqoBackendError::NetworkError {
                    msg: format!("could not create https client {:?}", x),
                })?
        };

        let url_string: String = job_location + "/status";
        let hqs_env_var = env::var("QRYD_API_HQS").is_ok();

        // Call WebAPI client
        let resp = match (self.dev, hqs_env_var) {
            (true, true) => client
                .get(url_string)
                .header("X-API-KEY", self.access_token.clone())
                .header("X-DEV", "?1")
                .header("X-HQS", "?1")
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
            (true, false) => client
                .get(url_string)
                .header("X-API-KEY", self.access_token.clone())
                .header("X-DEV", "?1")
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
            (false, true) => client
                .get(url_string)
                .header("X-API-KEY", self.access_token.clone())
                .header("X-HQS", "?1")
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
            (false, false) => client
                .get(url_string)
                .header("X-API-KEY", self.access_token.clone())
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
        };

        let status_code = resp.status();
        if status_code != reqwest::StatusCode::OK {
            if status_code == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
                let querry_response: ValidationError =
                    resp.json::<ValidationError>().map_err(|e| {
                        RoqoqoBackendError::NetworkError {
                            msg: format!("Error parsing ValidationError message {:?}", e),
                        }
                    })?;
                return Err(self._handle_validation_error(querry_response));
            }
            Err(RoqoqoBackendError::NetworkError {
                msg: format!(
                    "Request to server failed with HTTP status code {:?}",
                    status_code
                ),
            })
        } else {
            // response object includes the fields `status` and `msg` that can be accessed if required
            let response: Result<QRydJobStatus, RoqoqoBackendError> = resp
                .json::<QRydJobStatus>()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("second {:?}", e),
                });
            response
        }
    }

    /// Get status of a completed WebAPI job.
    ///
    /// # Arguments
    ///
    /// * `job_location` - location (url) of the job one is interested in.
    ///
    /// # Returns
    /// * Result of the job.
    /// * RoqoqoBackendError in case of a network failure.
    ///
    pub fn get_job_result(
        &self,
        job_location: String,
    ) -> Result<QRydJobResult, RoqoqoBackendError> {
        // Prepare WebAPI client
        let client: Client = if self.mock_port.is_some() {
            reqwest::blocking::Client::builder().build().map_err(|x| {
                RoqoqoBackendError::NetworkError {
                    msg: format!("could not create test client {:?}", x),
                }
            })?
        } else {
            reqwest::blocking::Client::builder()
                .https_only(true)
                .build()
                .map_err(|x| RoqoqoBackendError::NetworkError {
                    msg: format!("could not create https client {:?}", x),
                })?
        };

        // construct URL with {job_id} not required?
        let url_string: String = job_location + "/result";
        let hqs_env_var = env::var("QRYD_API_HQS").is_ok();

        // Call WebAPI client
        let resp = match (self.dev, hqs_env_var) {
            (true, true) => client
                .get(url_string)
                .header("X-API-KEY", self.access_token.clone())
                .header("X-DEV", "?1")
                .header("X-HQS", "?1")
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
            (true, false) => client
                .get(url_string)
                .header("X-API-KEY", self.access_token.clone())
                .header("X-DEV", "?1")
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
            (false, true) => client
                .get(url_string)
                .header("X-API-KEY", self.access_token.clone())
                .header("X-HQS", "?1")
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
            (false, false) => client
                .get(url_string)
                .header("X-API-KEY", self.access_token.clone())
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
        };

        let status_code = resp.status();
        if status_code != reqwest::StatusCode::OK {
            if status_code == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
                let querry_response: ValidationError =
                    resp.json::<ValidationError>().map_err(|e| {
                        RoqoqoBackendError::NetworkError {
                            msg: format!("Error parsing ValidationError message {:?}", e),
                        }
                    })?;
                return Err(self._handle_validation_error(querry_response));
            }
            Err(RoqoqoBackendError::NetworkError {
                msg: format!(
                    "Request to server failed with HTTP status code {:?}",
                    status_code
                ),
            })
        } else {
            // response object
            let response: Result<QRydJobResult, RoqoqoBackendError> = resp
                .json::<QRydJobResult>()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("Error parsing job status response {:?}", e),
                });
            response
        }
    }

    /// Delete a posted WebAPI job
    ///
    /// # Arguments
    ///
    /// * `job_location` - location (url) of the job one is interested in.
    ///
    /// # Returns
    /// * RoqoqoBackendError in case of a network failure.
    ///
    pub fn delete_job(&self, job_location: String) -> Result<(), RoqoqoBackendError> {
        // Prepare WebAPI client
        let client: Client = if self.mock_port.is_some() {
            reqwest::blocking::Client::builder().build().map_err(|x| {
                RoqoqoBackendError::NetworkError {
                    msg: format!("could not create test client {:?}", x),
                }
            })?
        } else {
            reqwest::blocking::Client::builder()
                .https_only(true)
                .build()
                .map_err(|x| RoqoqoBackendError::NetworkError {
                    msg: format!("could not create https client {:?}", x),
                })?
        };

        let hqs_env_var = env::var("QRYD_API_HQS").is_ok();

        // Call WebAPI client
        let resp = match (self.dev, hqs_env_var) {
            (true, true) => client
                .delete(job_location)
                .header("X-API-KEY", self.access_token.clone())
                .header("X-DEV", "?1")
                .header("X-HQS", "?1")
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
            (true, false) => client
                .delete(job_location)
                .header("X-API-KEY", self.access_token.clone())
                .header("X-DEV", "?1")
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
            (false, true) => client
                .delete(job_location)
                .header("X-API-KEY", self.access_token.clone())
                .header("X-HQS", "?1")
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
            (false, false) => client
                .delete(job_location)
                .header("X-API-KEY", self.access_token.clone())
                .send()
                .map_err(|e| RoqoqoBackendError::NetworkError {
                    msg: format!("{:?}", e),
                })?,
        };

        let status_code = resp.status();
        if status_code != reqwest::StatusCode::OK {
            if status_code == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
                let querry_response: ValidationError =
                    resp.json::<ValidationError>().map_err(|e| {
                        RoqoqoBackendError::NetworkError {
                            msg: format!("Error parsing ValidationError message {:?}", e),
                        }
                    })?;
                return Err(self._handle_validation_error(querry_response));
            }
            Err(RoqoqoBackendError::NetworkError {
                msg: format!(
                    "Request to server failed with HTTP status code {:?}",
                    status_code
                ),
            })
        } else {
            Ok(())
        }
    }

    /// Convert the counts returned from the QRyd WebAPI to Qoqo-style registers
    ///
    /// # Arguments
    ///
    /// `counts` - The counts returned from the Qryd WebAPI
    /// `readout` - The name of the readout register. Needs to be specified based on original circuit
    ///             cannont be extrected from returned result
    /// `number_qubits` - The number of measured qubits. Needs to be specified based on original circuit
    ///                   cannont be extrected from returned result
    ///
    pub fn counts_to_result(
        counts: ResultCounts,
        readout: String,
        number_qubits: usize,
    ) -> RegisterResult {
        let mut bit_map: HashMap<String, Vec<Vec<bool>>> = HashMap::new();
        let float_map: HashMap<String, Vec<Vec<f64>>> = HashMap::new();
        let complex_map: HashMap<String, Vec<Vec<Complex64>>> = HashMap::new();
        let mut measurement_record: Vec<Vec<bool>> = Vec::new();
        for (measurement, count) in counts.counts.into_iter() {
            let bit_representation: Vec<u8> = hex::decode(
                measurement
                    .strip_prefix("0x")
                    .map(|s| {
                        if s.len() % 2 == 0 {
                            s.to_string()
                        } else {
                            format!("0{}", s)
                        }
                    })
                    .ok_or(RoqoqoBackendError::GenericError {
                        msg: format!(
                            "Cannot parse a measurement result as bit representation {}",
                            measurement.clone()
                        ),
                    })?,
            )
            .map_err(|err| RoqoqoBackendError::GenericError {
                msg: format!(
                    "Cannot parse a measurement result as bit representation {:?}",
                    err
                ),
            })?;
            let qubit_results = bit_representation.view_bits::<Lsb0>();
            let mut tmp_vec: Vec<bool> = (0..number_qubits).map(|_| false).collect();
            // only iterating over qubits in number_qubits returns of larger qubits will be ignored
            for (mut_val, tmp_val) in (tmp_vec.iter_mut()).zip(qubit_results.iter()) {
                *mut_val = *tmp_val
            }
            for _ in 0..count {
                measurement_record.push(tmp_vec.clone())
            }
        }
        bit_map.insert(readout, measurement_record);
        Ok((bit_map, float_map, complex_map))
    }

    /// Setter for the dev option of the APIDevice.
    ///
    /// # Arguments
    ///
    /// * `dev` - The boolean to set the dev option to.
    ///
    pub fn set_dev(&mut self, dev: bool) {
        self.dev = dev;
    }

    fn _check_operation_compatability(&self, op: &Operation) -> Result<(), RoqoqoBackendError> {
        match op {
            Operation::MeasureQubit(_) => Ok(()),
            Operation::DefinitionBit(_) => Ok(()),
            Operation::PhaseShiftState1(_) => Ok(()),
            Operation::RotateXY(_) => Ok(()),
            Operation::RotateX(_) => Ok(()),
            Operation::RotateY(_) => Ok(()),
            Operation::RotateZ(_) => Ok(()),
            Operation::PhaseShiftedControlledZ(_) => Ok(()),
            Operation::PhaseShiftedControlledPhase(_) => Ok(()),
            Operation::Hadamard(_) => Ok(()),
            Operation::PauliX(_) => Ok(()),
            Operation::PauliY(_) => Ok(()),
            Operation::PauliZ(_) => Ok(()),
            Operation::SqrtPauliX(_) =>  Ok(()),
            Operation::InvSqrtPauliX(_) =>  Ok(()),
            Operation::CNOT(_) => Ok(()),
            Operation::ControlledPauliY(_) =>  Ok(()),
            Operation::ControlledPauliZ(_) =>  Ok(()),
            Operation::ControlledPhaseShift(_) => Ok(()),
            Operation::PragmaControlledCircuit(_) => Ok(()),
            Operation::ControlledControlledPauliZ(_) => Ok(()),
            Operation::ControlledControlledPhaseShift(_) => Ok(()),
            Operation::SWAP(_) =>  Ok(()),
            Operation::ISwap(_) => Ok(()),
            Operation::PragmaSetNumberOfMeasurements(_) => Ok(()),
            Operation::PragmaRepeatedMeasurement(_) => Ok(()),
            Operation::PragmaActiveReset(_) => {
                if self.device.qrydbackend() != "qiskit_emulator" {
                    Err(RoqoqoBackendError::GenericError { msg: "The device isn't qryd_emulator, PragmaActiveReset is not supported.".to_string() })
                } else {
                    Ok(())
                }
            },
            _ => Err(RoqoqoBackendError::GenericError {
                msg: format!("Operation {} is not supported by QRydDemo Web API backend.\n
                Use: MeasureQubit, PragmaSetNumberOfMeasurements, PragmaRepeatedMeasurement, PragmaActiveReset, PhaseShiftState1, RotateXY, RotateX, RotateY, RotateZ, RotateZ, Hadamard, PauliX, PauliY, PauliZ, SqrtPauliX, InvSqrtPauliX, PhaseShiftedControlledZ, PhaseShiftedControlledPhase, CNOT, ControlledPauliY, ControlledPauliZ, ControlledPhaseShift, PragmaControlledCircuit, ControlledControlledPauliZ, ControlledControlledPhaseShift, SWAP or ISwap instead.", op.hqslang())
            })
        }
    }

    fn _check_for_api_compatability(
        &self,
        program: &QuantumProgram,
    ) -> Result<(), RoqoqoBackendError> {
        let (measurement, _input_parameter_names) = match program {
            QuantumProgram::ClassicalRegister {
                measurement,
                input_parameter_names,
            } => Ok((measurement, input_parameter_names)),
            _ => Err(RoqoqoBackendError::GenericError {
                msg:
                    "Only ClassicalRegister measurements are supported by the Qryd WebAPI at the moment"
                        .to_string(),
            }),
        }?;
        for op in measurement.circuits[0].iter() {
            self._check_operation_compatability(op)?
        }

        if let Some(constant_circuit) = &measurement.constant_circuit {
            for op in constant_circuit.iter() {
                self._check_operation_compatability(op)?
            }
        }

        Ok(())
    }

    /// Transforms a PragmaRepeatedMeasurement operation into a set of
    /// MeasureQubit operations followed by a PragmaSetNumberOfMeasurements.
    ///
    fn _transform_pragma_repeated_measurements(
        &self,
        operation: PragmaRepeatedMeasurement,
        involved_qubits: &HashSet<usize>,
    ) -> Circuit {
        let involved_ordered = BTreeSet::from_iter(involved_qubits.iter().cloned());
        let mut equivalent_circuit = Circuit::new();
        for qbt in involved_ordered {
            equivalent_circuit += MeasureQubit::new(qbt, operation.readout().to_string(), qbt);
        }
        equivalent_circuit += PragmaSetNumberOfMeasurements::new(
            *operation.number_measurements(),
            operation.readout().to_string(),
        );
        equivalent_circuit
    }

    fn _handle_validation_error(&self, val_error: ValidationError) -> RoqoqoBackendError {
        let types = val_error.detail;
        match types {
            ValidationTypes::Simple(x) => RoqoqoBackendError::GenericError { msg: format!("QuantumProgram or metadata could not be parsed by QRyd Web-API Backend. msg: {}", x) },
            ValidationTypes::Detailed(x) => {
                let mut msg = "QuantumProgram or metadata could not be parsed by QRyd Web-API Backend. ".to_owned();
                msg.extend(x.iter().map(|detail| format!("[loc: {:?}, msg: {}, type: {:?}]", detail.loc, detail.msg, detail.internal_type)));
                msg.push('.');
                RoqoqoBackendError::GenericError { msg }
            },
        }
    }
}

impl EvaluatingBackend for APIBackend {
    fn run_circuit_iterator<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation>,
    ) -> RegisterResult {
        let new_circ: Circuit = circuit.cloned().collect();

        let mut readout = "".to_string();
        let mut number_qubits = 0;

        for op in new_circ.iter() {
            if let Operation::DefinitionBit(x) = op {
                let new_readout = x.name().clone();
                if readout == *"" {
                    readout = new_readout;
                    number_qubits = *x.length();
                } else {
                    return Err(RoqoqoBackendError::GenericError {
                        msg: "QRydAPIBAckend does not support more than one readout register"
                            .to_string(),
                    });
                }
            }
        }

        let measurement = ClassicalRegister {
            constant_circuit: None,
            circuits: vec![new_circ],
        };
        let program = QuantumProgram::ClassicalRegister {
            measurement,
            input_parameter_names: vec![],
        };
        let job_loc = self.post_job(program)?;

        let mut test_counter = 0;
        let mut status = "".to_string();
        let mut job_result = QRydJobResult::default();
        let fifteen = time::Duration::from_millis(200);
        while test_counter < self.timeout && status != "completed" {
            test_counter += 1;
            let job_status = self.get_job_status(job_loc.clone()).unwrap();
            status.clone_from(&job_status.status);
            thread::sleep(fifteen);
            if status == *"completed" {
                job_result = self.get_job_result(job_loc.clone()).unwrap();
            }
        }

        if status == "completed" {
            APIBackend::counts_to_result(job_result.data, readout, number_qubits)
        } else if status == "error" {
            Err(RoqoqoBackendError::GenericError {
                msg: format!("WebAPI returned an error status for the job {}.", job_loc),
            })
        } else if status == "cancelled" {
            Err(RoqoqoBackendError::GenericError {
                msg: format!("Job {} got cancelled.", job_loc),
            })
        } else {
            Err(RoqoqoBackendError::GenericError {
                msg: format!(
                    "WebAPI did not return finished result in timeout: {} * 30s",
                    self.timeout
                ),
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api_devices::QrydEmuSquareDevice;
    use roqoqo::operations;
    use roqoqo::{Circuit, QuantumProgram};
    use serde_json::json;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Test Debug, Clone and PartialEq of ApiBackend
    #[test]
    fn debug_and_clone() {
        let device: QRydAPIDevice = QrydEmuSquareDevice::new(None, None, None).into();
        let backend = APIBackend::new(
            device.clone(),
            Some("".to_string()),
            Some(2),
            None,
            None,
            None,
        )
        .unwrap();
        let a = format!("{:?}", backend);
        assert!(a.contains("QrydEmuSquareDevice"));
        let backend2 =
            APIBackend::new(device, Some("a".to_string()), Some(2), None, None, None).unwrap();
        assert_eq!(backend.clone(), backend);
        assert_ne!(backend, backend2);
    }

    /// Test Debug of QRydRunData
    #[test]
    fn test_debug_qrydrundatastruct() {
        let circuit = Circuit::new();
        let measurement = ClassicalRegister {
            constant_circuit: None,
            circuits: vec![circuit],
        };
        let program = QuantumProgram::ClassicalRegister {
            measurement,
            input_parameter_names: vec!["test".to_string()],
        };
        // let program: roqoqo_1_0::QuantumProgram = downconvert_roqoqo_version(program).unwrap();

        let test = QRydRunData {
            format: "qoqo".to_string(),
            backend: "qryd_emu_cloudcomp_square".to_string(),
            program,
            dev: false,
            fusion_max_qubits: 4,
            seed_simulator: None,
            seed_compiler: None,
            use_extended_set: true,
            use_reverse_traversal: true,
            extended_set_size: 5,
            extended_set_weight: 0.5,
            reverse_traversal_iterations: 2,
        };
        assert_eq!(format!("{:?}", test), "QRydRunData { format: \"qoqo\", backend: \"qryd_emu_cloudcomp_square\", dev: false, fusion_max_qubits: 4, seed_simulator: None, seed_compiler: None, use_extended_set: true, use_reverse_traversal: true, reverse_traversal_iterations: 2, extended_set_size: 5, extended_set_weight: 0.5, program: ClassicalRegister { measurement: ClassicalRegister { constant_circuit: None, circuits: [Circuit { definitions: [], operations: [], _roqoqo_version: RoqoqoVersion }] }, input_parameter_names: [\"test\"] } }");
    }

    /// Test Debug of QRydJobResult
    #[test]
    fn test_debug_qrydjobresult() {
        let rescounts = ResultCounts {
            counts: HashMap::new(),
        };
        let result = QRydJobResult {
            data: rescounts,
            time_taken: 0.0,
            noise: "noise".to_string(),
            method: "method".to_string(),
            device: "device".to_string(),
            num_qubits: 2,
            num_clbits: 2,
            fusion_max_qubits: 0,
            fusion_avg_qubits: 0.0,
            fusion_generated_gates: 0,
            executed_single_qubit_gates: 0,
            executed_two_qubit_gates: 0,
            // precision: "single".to_string(),
            compilation_time: 1.0,
        };
        assert_eq!(format!("{:?}", result), "QRydJobResult { data: ResultCounts { counts: {} }, time_taken: 0.0, noise: \"noise\", method: \"method\", device: \"device\", num_qubits: 2, num_clbits: 2, fusion_max_qubits: 0, fusion_avg_qubits: 0.0, fusion_generated_gates: 0, executed_single_qubit_gates: 0, executed_two_qubit_gates: 0, compilation_time: 1.0 }");
    }

    /// Test Debug of QRydJobStatus
    #[test]
    fn test_debug_validation() {
        let status = QRydJobStatus {
            status: "in progress".to_string(),
            msg: "the job is still in progress".to_string(),
        };
        assert_eq!(
            format!("{:?}", status),
            "QRydJobStatus { status: \"in progress\", msg: \"the job is still in progress\" }"
        );
    }

    /// Test error cases. Case 1: UnprocessableEntity
    #[tokio::test]
    async fn async_api_backend_errorcase1() {
        let detail = ValidationErrorDetail {
            loc: vec![LocTypes::LocStr("DummyLoc".to_string())],
            msg: "DummyMsg".to_string(),
            internal_type: "DummyType".to_string(),
        };
        let error = ValidationError {
            detail: ValidationTypes::Detailed(vec![detail]),
            body: None,
        };
        let server_wiremock = MockServer::start().await;
        let uri = server_wiremock.uri();
        let _mock_status = Mock::given(method("GET"))
            .and(path("/DummyLocation/status"))
            .respond_with(ResponseTemplate::new(422).set_body_json(&error))
            .expect(1)
            .mount(&server_wiremock)
            .await;
        let _mock_result = Mock::given(method("GET"))
            .and(path("/DummyLocation/result"))
            .respond_with(ResponseTemplate::new(422).set_body_json(&error))
            .expect(1)
            .mount(&server_wiremock)
            .await;
        let _mock_delete = Mock::given(method("DELETE"))
            .respond_with(ResponseTemplate::new(422).set_body_json(&error))
            .expect(1)
            .mount(&server_wiremock)
            .await;
        let _mock_post = Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(422)
                    .insert_header(
                        "Location",
                        &format!("{}/DummyLocation", server_wiremock.uri()),
                    )
                    .set_body_json(&error),
            )
            .expect(1)
            .mount(&server_wiremock)
            .await;
        let number_qubits = 6;
        let device = QrydEmuSquareDevice::new(Some(2), None, None);
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(
            qryd_device,
            None,
            None,
            Some(server_wiremock.address().port().to_string()),
            None,
            None,
        )
        .unwrap();
        let mut circuit = Circuit::new();
        circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
        circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
        circuit += operations::RotateX::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::MeasureQubit::new(0, "ro".to_string(), 0);
        circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());
        let measurement = ClassicalRegister {
            constant_circuit: Some(circuit.clone()),
            circuits: vec![circuit.clone()],
        };
        let program = QuantumProgram::ClassicalRegister {
            measurement,
            input_parameter_names: vec![],
        };

        let api_backend_new_cloned = api_backend_new.clone();
        let job_loc = tokio::task::spawn_blocking(move || api_backend_new_cloned.post_job(program))
            .await
            .unwrap();
        assert!(job_loc.is_err());
        assert!(matches!(
            job_loc.unwrap_err(),
            RoqoqoBackendError::GenericError { .. }
        ));

        let api_backend_new_cloned = api_backend_new.clone();
        let uri_cloned = uri.clone();
        let job_status = tokio::task::spawn_blocking(move || {
            api_backend_new_cloned.get_job_status(format!("{}/DummyLocation", uri_cloned))
        })
        .await
        .unwrap();
        assert!(job_status.is_err());
        assert!(matches!(
            job_status.unwrap_err(),
            RoqoqoBackendError::GenericError { .. }
        ));

        let api_backend_new_cloned = api_backend_new.clone();
        let uri_cloned = uri.clone();
        let job_result = tokio::task::spawn_blocking(move || {
            api_backend_new_cloned.get_job_result(format!("{}/DummyLocation", uri_cloned))
        })
        .await
        .unwrap();
        assert!(job_result.is_err());
        assert!(matches!(
            job_result.unwrap_err(),
            RoqoqoBackendError::GenericError { .. }
        ));

        let api_backend_new_cloned = api_backend_new.clone();
        let uri_cloned = uri.clone();
        let job_delete = tokio::task::spawn_blocking(move || {
            api_backend_new_cloned.delete_job(format!("{}/DummyLocation", uri_cloned))
        })
        .await
        .unwrap();
        assert!(job_delete.is_err());
        assert!(matches!(
            job_delete.unwrap_err(),
            RoqoqoBackendError::GenericError { .. }
        ));

        server_wiremock.verify().await;
    }

    /// Test error cases. Case 2: ValidationError parsing error
    #[tokio::test]
    async fn async_api_backend_errorcase2() {
        let server_wiremock = MockServer::start().await;
        let uri = server_wiremock.uri();
        let _mock_status = Mock::given(method("GET"))
            .and(path("/DummyLocation/status"))
            .respond_with(ResponseTemplate::new(422))
            .expect(1)
            .mount(&server_wiremock)
            .await;
        let _mock_result = Mock::given(method("GET"))
            .and(path("/DummyLocation/result"))
            .respond_with(ResponseTemplate::new(422))
            .expect(1)
            .mount(&server_wiremock)
            .await;
        let _mock_delete = Mock::given(method("DELETE"))
            .respond_with(ResponseTemplate::new(422))
            .expect(1)
            .mount(&server_wiremock)
            .await;
        let _mock_post = Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(422).insert_header(
                "Location",
                &format!("{}/DummyLocation", server_wiremock.uri()),
            ))
            .expect(1)
            .mount(&server_wiremock)
            .await;
        let number_qubits = 6;
        let device = QrydEmuSquareDevice::new(Some(2), None, None);
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(
            qryd_device,
            None,
            None,
            Some(server_wiremock.address().port().to_string()),
            None,
            None,
        )
        .unwrap();
        let mut circuit = Circuit::new();
        circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
        circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
        circuit += operations::RotateX::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::MeasureQubit::new(0, "ro".to_string(), 0);
        circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());
        let measurement = ClassicalRegister {
            constant_circuit: Some(circuit.clone()),
            circuits: vec![circuit.clone()],
        };
        let program = QuantumProgram::ClassicalRegister {
            measurement,
            input_parameter_names: vec![],
        };

        let api_backend_new_cloned = api_backend_new.clone();
        let job_loc = tokio::task::spawn_blocking(move || api_backend_new_cloned.post_job(program))
            .await
            .unwrap();
        assert!(job_loc.is_err());
        assert!(matches!(
            job_loc.unwrap_err(),
            RoqoqoBackendError::NetworkError { .. }
        ));

        let api_backend_new_cloned = api_backend_new.clone();
        let uri_cloned = uri.clone();
        let job_status = tokio::task::spawn_blocking(move || {
            api_backend_new_cloned.get_job_status(format!("{}/DummyLocation", uri_cloned))
        })
        .await
        .unwrap();
        assert!(job_status.is_err());
        assert!(matches!(
            job_status.unwrap_err(),
            RoqoqoBackendError::NetworkError { .. }
        ));

        let api_backend_new_cloned = api_backend_new.clone();
        let uri_cloned = uri.clone();
        let job_result = tokio::task::spawn_blocking(move || {
            api_backend_new_cloned.get_job_result(format!("{}/DummyLocation", uri_cloned))
        })
        .await
        .unwrap();
        assert!(job_result.is_err());
        assert!(matches!(
            job_result.unwrap_err(),
            RoqoqoBackendError::NetworkError { .. }
        ));

        let api_backend_new_cloned = api_backend_new.clone();
        let uri_cloned = uri.clone();
        let job_delete = tokio::task::spawn_blocking(move || {
            api_backend_new_cloned.delete_job(format!("{}/DummyLocation", uri_cloned))
        })
        .await
        .unwrap();
        assert!(job_delete.is_err());
        assert!(matches!(
            job_delete.unwrap_err(),
            RoqoqoBackendError::NetworkError { .. }
        ));

        server_wiremock.verify().await;
    }

    // Test PragmaRepeatedMeasurement `.post_job()` transformation
    #[tokio::test]
    async fn async_api_backend_repeated_measurement() {
        let server_wiremock = MockServer::start().await;
        let device = QrydEmuSquareDevice::new(Some(1), None, None);
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(
            qryd_device,
            None,
            None,
            Some(server_wiremock.address().port().to_string()),
            None,
            None,
        )
        .unwrap();

        let mut input_circuit = Circuit::new();
        input_circuit += operations::DefinitionBit::new("ro".to_string(), 3, true);
        input_circuit += operations::RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
        input_circuit += operations::RotateX::new(1, std::f64::consts::FRAC_PI_2.into());
        input_circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
        input_circuit += operations::PragmaRepeatedMeasurement::new("ro".to_string(), 10, None);
        let mut const_input_circuit = Circuit::new();
        const_input_circuit += operations::DefinitionBit::new("ro".to_string(), 3, true);
        const_input_circuit += operations::RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
        const_input_circuit += operations::RotateX::new(1, std::f64::consts::FRAC_PI_2.into());
        const_input_circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
        const_input_circuit +=
            operations::PragmaRepeatedMeasurement::new("ro".to_string(), 10, None);
        let input_measurement = ClassicalRegister {
            constant_circuit: Some(const_input_circuit),
            circuits: vec![input_circuit.clone()],
        };
        let input_program = QuantumProgram::ClassicalRegister {
            measurement: input_measurement,
            input_parameter_names: vec![],
        };

        let mut output_circuit = Circuit::new();
        output_circuit += operations::DefinitionBit::new("ro".to_string(), 3, true);
        output_circuit += operations::RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
        output_circuit += operations::RotateX::new(1, std::f64::consts::FRAC_PI_2.into());
        output_circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
        for i in 0..3 {
            output_circuit += operations::MeasureQubit::new(i, "ro".to_string(), i);
        }
        output_circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());
        let mut const_output_circuit = Circuit::new();
        const_output_circuit += operations::DefinitionBit::new("ro".to_string(), 3, true);
        const_output_circuit += operations::RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
        const_output_circuit += operations::RotateX::new(1, std::f64::consts::FRAC_PI_2.into());
        const_output_circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
        for i in 0..3 {
            const_output_circuit += operations::MeasureQubit::new(i, "ro".to_string(), i);
        }
        const_output_circuit +=
            operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());
        let output_measurement = ClassicalRegister {
            constant_circuit: Some(const_output_circuit),
            circuits: vec![output_circuit.clone()],
        };
        let output_program = QuantumProgram::ClassicalRegister {
            measurement: output_measurement,
            input_parameter_names: vec![],
        };
        let data = QRydRunData {
            format: "qoqo".to_string(),
            backend: device.qrydbackend(),
            program: output_program,
            dev: false,
            fusion_max_qubits: 4,
            extended_set_size: 5,
            extended_set_weight: 0.5,
            seed_simulator: Some(1),
            seed_compiler: None,
            use_extended_set: true,
            use_reverse_traversal: true,
            reverse_traversal_iterations: 3,
        };

        let _mock = Mock::given(method("POST"))
            .and(body_json(json!(data)))
            .respond_with(ResponseTemplate::new("200"))
            .expect(1)
            .mount(&server_wiremock)
            .await;

        let _ = tokio::task::spawn_blocking(move || api_backend_new.post_job(input_program))
            .await
            .unwrap();

        server_wiremock.verify().await;
    }
}
