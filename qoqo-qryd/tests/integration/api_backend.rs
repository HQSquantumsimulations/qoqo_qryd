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

//! Integration test for public API of QRyd WebAPI backend

use core::time;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyType};
use pyo3::Python;
use std::collections::HashMap;
use std::{env, thread};

use qoqo::measurements::CheatedWrapper;
use qoqo::{CircuitWrapper, QuantumProgramWrapper};
use roqoqo::measurements::{Cheated, CheatedInput, ClassicalRegister};
use roqoqo::{operations, Circuit, QuantumProgram};

use qoqo_qryd::api_backend::{convert_into_backend, APIBackendWrapper, Registers};
use qoqo_qryd::api_devices::{QrydEmuSquareDeviceWrapper, QrydEmuTriangularDeviceWrapper};
use qoqo_qryd::tweezer_devices::TweezerDeviceWrapper;
use roqoqo_qryd::api_devices::{QRydAPIDevice, QrydEmuSquareDevice};
use roqoqo_qryd::{APIBackend, QRydJobResult, QRydJobStatus, ResultCounts};

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Helper function to create a python object of square device
fn create_backend_with_square_device(py: Python, seed: Option<usize>) -> Bound<APIBackendWrapper> {
    let device_type = py.get_type_bound::<QrydEmuSquareDeviceWrapper>();
    let device = &device_type.call1((seed,)).unwrap();

    let backend_type: &Bound<PyType> = &py.get_type_bound::<APIBackendWrapper>();
    let backend: Bound<APIBackendWrapper> = backend_type
        .call1((device.downcast::<QrydEmuSquareDeviceWrapper>().unwrap(), ""))
        .unwrap()
        .downcast::<APIBackendWrapper>()
        .unwrap()
        .to_owned();
    backend
}

fn create_valid_backend_with_tweezer_device(
    py: Python,
    seed: Option<usize>,
) -> Bound<APIBackendWrapper> {
    let device_type = py.get_type_bound::<TweezerDeviceWrapper>();
    let binding = device_type
        .call_method1(
            "from_api",
            (
                Option::<String>::None,
                Option::<String>::None,
                Option::<String>::None,
                seed,
                Some(env::var("QRYD_API_HQS").is_ok()),
            ),
        )
        .unwrap();
    let device = binding.downcast::<TweezerDeviceWrapper>().unwrap();

    let backend_type: &Bound<PyType> = &py.get_type_bound::<APIBackendWrapper>();
    let none_string: Option<String> = None;
    let binding = backend_type.call1((device, none_string)).unwrap();
    let backend: &Bound<APIBackendWrapper> = binding.downcast::<APIBackendWrapper>().unwrap();
    backend.to_owned()
}

fn create_valid_backend_with_square_device(
    py: Python,
    seed: Option<usize>,
) -> Bound<APIBackendWrapper> {
    let device_type = py.get_type_bound::<QrydEmuSquareDeviceWrapper>();
    let device = device_type.call1((seed,)).unwrap();

    let backend_type: &Bound<PyType> = &py.get_type_bound::<APIBackendWrapper>();
    let none_string: Option<String> = None;
    let backend: Bound<APIBackendWrapper> = backend_type
        .call1((
            device.downcast::<QrydEmuSquareDeviceWrapper>().unwrap(),
            none_string,
        ))
        .unwrap()
        .downcast::<APIBackendWrapper>()
        .unwrap()
        .to_owned();
    backend
}

fn create_valid_backend_with_square_device_mocked(
    py: Python,
    seed: Option<usize>,
    mock_port: String,
) -> Bound<APIBackendWrapper> {
    let device_type = py.get_type_bound::<QrydEmuSquareDeviceWrapper>();
    let device = device_type.call1((seed,)).unwrap();

    let backend_type: &Bound<PyType> = &py.get_type_bound::<APIBackendWrapper>();
    let none_string: Option<String> = None;
    let backend: Bound<APIBackendWrapper> = backend_type
        .call1((
            device.downcast::<QrydEmuSquareDeviceWrapper>().unwrap(),
            none_string,
            30,
            mock_port,
        ))
        .unwrap()
        .downcast::<APIBackendWrapper>()
        .unwrap()
        .to_owned();
    backend
}

fn create_quantum_program(valid: bool) -> QuantumProgramWrapper {
    let number_qubits = 2;
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
    circuit += operations::RotateX::new(0, 0.0.into());
    circuit += operations::MeasureQubit::new(0, "ro".to_string(), 0);
    circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());
    let measurement = if valid {
        ClassicalRegister {
            constant_circuit: None,
            circuits: vec![circuit.clone()],
        }
    } else {
        ClassicalRegister {
            constant_circuit: Some(circuit.clone()),
            circuits: vec![circuit.clone()],
        }
    };
    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };
    QuantumProgramWrapper { internal: program }
}

fn create_cheated_measurement() -> CheatedWrapper {
    let number_qubits = 2;
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
    circuit += operations::RotateX::new(0, 0.0.into());
    // circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::MeasureQubit::new(0, "ro".to_string(), 0);
    circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());

    let cheated = Cheated {
        constant_circuit: None,
        circuits: vec![circuit],
        input: CheatedInput::new(2),
    };
    CheatedWrapper { internal: cheated }
}

// Test to create a new backend
#[test]
fn test_new_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let seed: Option<usize> = Some(11);
        let device_type = py.get_type_bound::<QrydEmuSquareDeviceWrapper>();
        let device = device_type.call1((seed,)).unwrap();

        let backend_type: &Bound<PyType> = &py.get_type_bound::<APIBackendWrapper>();
        let backend = backend_type
            .call1((device.downcast::<QrydEmuSquareDeviceWrapper>().unwrap(), ""))
            .unwrap();
        assert!(backend.downcast::<APIBackendWrapper>().is_ok());
    });
}

// Test to check a failed backend creation
#[test]
fn test_fail_new_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let seed: Option<usize> = Some(11);
        let device_type = py.get_type_bound::<QrydEmuSquareDeviceWrapper>();
        let device = device_type.call1((seed,)).unwrap();

        let backend_type: &Bound<PyType> = &py.get_type_bound::<APIBackendWrapper>();
        let backend = backend_type.call1((3_u32, ""));
        assert!(backend.is_err());
        if let Ok(old_token) = env::var("QRYD_API_TOKEN") {
            env::remove_var("QRYD_API_TOKEN");
            let backend =
                backend_type.call1((device.downcast::<QrydEmuSquareDeviceWrapper>().unwrap(),));
            assert!(backend.is_err());
            env::set_var("QRYD_API_TOKEN", old_token);
        } else {
            let backend =
                backend_type.call1((device.downcast::<QrydEmuSquareDeviceWrapper>().unwrap(),));
            assert!(backend.is_err());
        }
    });
}

// Test to create a new backend
#[test]
fn test_new_triangle() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let seed: Option<usize> = Some(11);
        let device_type = py.get_type_bound::<QrydEmuTriangularDeviceWrapper>();
        let device = device_type.call1((seed,)).unwrap();

        let backend_type: &Bound<PyType> = &py.get_type_bound::<APIBackendWrapper>();
        let backend = backend_type
            .call1((
                device.downcast::<QrydEmuTriangularDeviceWrapper>().unwrap(),
                "",
            ))
            .unwrap();
        assert!(backend.downcast::<APIBackendWrapper>().is_ok());
    });
}

/// Test copy and deepcopy for api backend with square device
#[test]
fn test_copy_deepcopy_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = create_backend_with_square_device(py, Some(11));
        let backend2 = create_backend_with_square_device(py, Some(2));

        let copy_op = backend.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_op.extract::<APIBackendWrapper>().unwrap();
        let copy_op2 = backend2.call_method0("__copy__").unwrap();
        let copy_wrapper2 = copy_op2.extract::<APIBackendWrapper>().unwrap();
        let deepcopy_op = backend.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_op.extract::<APIBackendWrapper>().unwrap();

        let backend_wrapper = backend.extract::<APIBackendWrapper>().unwrap();
        assert_eq!(backend_wrapper, copy_wrapper);
        assert_ne!(copy_wrapper, copy_wrapper2);
        assert_eq!(backend_wrapper, deepcopy_wrapper);
    });
}

/// Test to and from json for api backend with square device
#[test]
fn test_json_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = create_backend_with_square_device(py, Some(11));

        let serialised = backend.call_method0("to_json").unwrap();
        let deserialised = backend.call_method1("from_json", (&serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = backend.call_method1("from_json", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<APIBackendWrapper>().unwrap();
        let backend_wrapper = backend.extract::<APIBackendWrapper>().unwrap();
        assert_eq!(backend_wrapper, serde_wrapper);
    });
}

#[test]
fn test_post_job_fail() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = create_backend_with_square_device(py, Some(11));

        let failed_post_job = backend.call_method1("post_job", (3_u32,));
        assert!(failed_post_job.is_err());

        let program = create_quantum_program(true);

        let failed_post_job = backend.call_method1("post_job", (program,));
        assert!(failed_post_job.is_err());

        let program = create_quantum_program(false);

        let failed_post_job = backend.call_method1("post_job", (program,));
        assert!(failed_post_job.is_err());
    });
}

#[test]
fn test_delete_job_fail() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = create_backend_with_square_device(py, Some(11));

        let failed_delete_job = backend.call_method1("delete_job", ("3",));
        assert!(failed_delete_job.is_err());
    });
}

#[test]
fn test_query_job_status_fail() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = create_backend_with_square_device(py, Some(11));

        let failed_status_job = backend.call_method1("get_job_status", ("3",));
        assert!(failed_status_job.is_err());
    });
}

#[test]
fn test_run_job() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let backend = create_valid_backend_with_tweezer_device(py, Some(11));
            let program = create_quantum_program(true);
            let job_loc = backend.call_method1("post_job", (program,)).unwrap();
            let fifteen = time::Duration::from_secs(1);

            let mut test_counter = 0;
            let mut status = "".to_string();
            while test_counter < 20 && status != "completed" {
                test_counter += 1;
                let status_report: HashMap<String, String> = backend
                    .call_method1("get_job_status", (&job_loc,))
                    .unwrap()
                    .extract()
                    .unwrap();
                let job_status = status_report.get("status").unwrap();
                status.clone_from(job_status);
                thread::sleep(fifteen);

                if status == *"completed" {
                    assert_eq!(status, "completed");
                    let _job_result = backend.call_method1("get_job_result", (&job_loc,)).unwrap();
                }
            }
        });
    }
}

#[tokio::test]
async fn async_test_run_job() {
    let wiremock_server = MockServer::start().await;
    let port = wiremock_server.address().port().to_string();
    let uri = wiremock_server.uri();
    let qryd_job_status_in_progress = QRydJobStatus {
        status: "in progress".to_string(),
        msg: "the job is still in progress".to_string(),
    };
    let qryd_job_status_completed = QRydJobStatus {
        status: "completed".to_string(),
        msg: "the job has been completed".to_string(),
    };
    let result_counts = ResultCounts {
        counts: HashMap::from([("0x1".to_string(), 100), ("0x4".to_string(), 20)]),
    };
    let qryd_job_result_completed = QRydJobResult {
        compilation_time: 1.0,
        data: result_counts,
        time_taken: 0.23,
        noise: "noise".to_string(),
        method: "method".to_string(),
        device: "QrydEmuSquareDevice".to_string(),
        num_qubits: 4,
        num_clbits: 4,
        fusion_max_qubits: 4,
        fusion_avg_qubits: 4.0,
        fusion_generated_gates: 100,
        executed_single_qubit_gates: 50,
        executed_two_qubit_gates: 50,
    };
    let _mock_post = Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(201).insert_header("Location", &format!("{}/DummyLocation", uri)),
        )
        .expect(1)
        .mount(&wiremock_server)
        .await;
    let _mock_status = Mock::given(method("GET"))
        .and(path("/DummyLocation/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&qryd_job_status_in_progress))
        .expect(20)
        .mount(&wiremock_server)
        .await;

    pyo3::prepare_freethreaded_python();
    let backend = Python::with_gil(|py| {
        create_valid_backend_with_square_device_mocked(py, Some(11), port).into_py(py)
    });
    let cloned_backend = backend.clone();
    let job_loc = tokio::task::spawn_blocking(move || {
        Python::with_gil(|py| {
            let program = create_quantum_program(true);
            cloned_backend.call_method1(py, "post_job", (program,))
        })
    })
    .await
    .unwrap()
    .unwrap();
    let fifteen = time::Duration::from_millis(50);

    let mut test_counter = 0;
    let mut status = "".to_string();
    while test_counter < 20 && status != "completed" {
        test_counter += 1;
        let cloned_backend = backend.clone();
        let cloned_job_loc = job_loc.clone();
        let job_status = tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                let status_report = cloned_backend
                    .call_method1(py, "get_job_status", (cloned_job_loc,))
                    .unwrap();
                let extracted: HashMap<String, String> = status_report.extract(py).unwrap();
                extracted.get("status").cloned().unwrap()
            })
        })
        .await
        .unwrap();
        status.clone_from(&job_status);
        assert_eq!(job_status, "in progress");
        thread::sleep(fifteen);
    }

    wiremock_server.verify().await;
    wiremock_server.reset().await;

    let _mock_status1 = Mock::given(method("GET"))
        .and(path("/DummyLocation/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&qryd_job_status_completed))
        .expect(1)
        .mount(&wiremock_server)
        .await;

    let cloned_backend = backend.clone();
    let cloned_job_loc = job_loc.clone();
    tokio::task::spawn_blocking(move || {
        Python::with_gil(|py| {
            let status_report: HashMap<String, String> = cloned_backend
                .call_method1(py, "get_job_status", (cloned_job_loc,))
                .unwrap()
                .extract(py)
                .unwrap();
            let job_status = status_report.get("status").unwrap();

            assert_eq!(job_status, "completed");
        });
    })
    .await
    .unwrap();

    let _mock_result = Mock::given(method("GET"))
        .and(path("/DummyLocation/result"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&qryd_job_result_completed))
        .expect(1)
        .mount(&wiremock_server)
        .await;

    tokio::task::spawn_blocking(move || {
        Python::with_gil(|py| {
            let _job_result = backend
                .call_method1(py, "get_job_result", (job_loc,))
                .unwrap();
        });
    })
    .await
    .unwrap();

    wiremock_server.verify().await;
}

#[test]
fn test_run_circuit() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        let mut circuit = Circuit::new();
        circuit += operations::DefinitionBit::new("ro".to_string(), 2, true);
        circuit += operations::RotateX::new(0, 0.0.into());
        circuit += operations::MeasureQubit::new(0, "ro".to_string(), 0);
        circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());
        let circuit_py = CircuitWrapper { internal: circuit };

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let backend: &Bound<APIBackendWrapper> =
                &create_valid_backend_with_tweezer_device(py, Some(11));

            let result = backend.call_method1("run_circuit", (3usize,));
            assert!(result.is_err());

            assert!(backend.call_method1("run_circuit", (circuit_py,)).is_ok());
        });
    }
}

#[tokio::test]
async fn async_test_run_circuit() {
    let wiremock_server = MockServer::start().await;
    let port = wiremock_server.address().port().to_string();
    let uri = wiremock_server.uri();
    let qryd_job_status_completed = QRydJobStatus {
        status: "completed".to_string(),
        msg: "the job has been completed".to_string(),
    };
    let result_counts = ResultCounts {
        counts: HashMap::from([("0x0".to_string(), 40)]),
    };
    let qryd_job_result_completed = QRydJobResult {
        compilation_time: 1.0,
        data: result_counts,
        time_taken: 0.23,
        noise: "noise".to_string(),
        method: "method".to_string(),
        device: "QrydEmuSquareDevice".to_string(),
        num_qubits: 2,
        num_clbits: 2,
        fusion_max_qubits: 2,
        fusion_avg_qubits: 2.0,
        fusion_generated_gates: 100,
        executed_single_qubit_gates: 0,
        executed_two_qubit_gates: 0,
    };

    let _mock_post = Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(201).insert_header("Location", &format!("{}/DummyLocation", uri)),
        )
        .expect(1)
        .mount(&wiremock_server)
        .await;
    let _mock_status = Mock::given(method("GET"))
        .and(path("/DummyLocation/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&qryd_job_status_completed))
        .expect(1)
        .mount(&wiremock_server)
        .await;
    let _mock_result = Mock::given(method("GET"))
        .and(path("/DummyLocation/result"))
        .respond_with(ResponseTemplate::new(200).set_body_json(qryd_job_result_completed))
        .expect(1)
        .mount(&wiremock_server)
        .await;

    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), 2, true);
    circuit += operations::RotateX::new(0, 0.0.into());
    circuit += operations::MeasureQubit::new(0, "ro".to_string(), 0);
    circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());
    let circuit_py = CircuitWrapper { internal: circuit };

    pyo3::prepare_freethreaded_python();
    tokio::task::spawn_blocking(move || {
        Python::with_gil(|py| {
            let backend: &Bound<APIBackendWrapper> =
                &create_valid_backend_with_square_device_mocked(py, Some(11), port);

            let result = backend.call_method1("run_circuit", (3usize,));
            assert!(result.is_err());

            backend.call_method1("run_circuit", (circuit_py,)).unwrap();
        });
    })
    .await
    .unwrap();

    wiremock_server.verify().await;
}

#[test]
fn test_run_measurement_registers() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let backend: &Bound<APIBackendWrapper> =
                &create_valid_backend_with_tweezer_device(py, Some(11));

            let failed_result = backend.call_method1("run_measurement_registers", (3_u32,));
            assert!(failed_result.is_err());

            let failed_program = create_quantum_program(false);
            let measurement = failed_program.measurement();
            let failed_result = backend.call_method1("run_measurement_registers", (measurement,));
            assert!(failed_result.is_err());

            let program = create_quantum_program(true);
            let measurement = program.measurement();
            let (bits, floats, complex): Registers = backend
                .call_method1("run_measurement_registers", (measurement,))
                .unwrap()
                .extract()
                .unwrap();
            assert!(floats.is_empty());
            assert!(complex.is_empty());
            assert!(bits.contains_key("ro"));
            let bit = bits.get("ro").unwrap();
            assert_eq!(bit.len(), 10);
        });
    }
}

#[tokio::test]
async fn async_test_run_measurement_registers() {
    let wiremock_server = MockServer::start().await;
    let port = wiremock_server.address().port().to_string();
    let uri = wiremock_server.uri();
    let qryd_job_status_completed = QRydJobStatus {
        status: "completed".to_string(),
        msg: "the job has been completed".to_string(),
    };
    let result_counts = ResultCounts {
        counts: HashMap::from([("0x0".to_string(), 10)]),
    };
    let qryd_job_result_completed = QRydJobResult {
        compilation_time: 1.0,
        data: result_counts,
        time_taken: 0.23,
        noise: "noise".to_string(),
        method: "method".to_string(),
        device: "QrydEmuSquareDevice".to_string(),
        num_qubits: 2,
        num_clbits: 2,
        fusion_max_qubits: 2,
        fusion_avg_qubits: 2.0,
        fusion_generated_gates: 100,
        executed_single_qubit_gates: 0,
        executed_two_qubit_gates: 0,
    };

    let _mock_post = Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(201).insert_header("Location", &format!("{}/DummyLocation", uri)),
        )
        .expect(1)
        .mount(&wiremock_server)
        .await;
    let _mock_status = Mock::given(method("GET"))
        .and(path("/DummyLocation/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&qryd_job_status_completed))
        .expect(1)
        .mount(&wiremock_server)
        .await;
    let _mock_result = Mock::given(method("GET"))
        .and(path("/DummyLocation/result"))
        .respond_with(ResponseTemplate::new(200).set_body_json(qryd_job_result_completed))
        .expect(1)
        .mount(&wiremock_server)
        .await;

    pyo3::prepare_freethreaded_python();
    tokio::task::spawn_blocking(move || {
        Python::with_gil(|py| {
            let backend: &Bound<APIBackendWrapper> =
                &create_valid_backend_with_square_device_mocked(py, Some(11), port);

            let failed_result = backend.call_method1("run_measurement_registers", (3_u32,));
            assert!(failed_result.is_err());

            let failed_program = create_quantum_program(false);
            let measurement = failed_program.measurement();
            let failed_result = backend.call_method1("run_measurement_registers", (measurement,));
            assert!(failed_result.is_err());

            let program = create_quantum_program(true);
            let measurement = program.measurement();
            let (bits, floats, complex): Registers = backend
                .call_method1("run_measurement_registers", (measurement,))
                .unwrap()
                .extract()
                .unwrap();
            assert!(floats.is_empty());
            assert!(complex.is_empty());
            assert!(bits.contains_key("ro"));
            let bit = bits.get("ro").unwrap();
            assert_eq!(bit.len(), 10);
        });
    })
    .await
    .unwrap();

    wiremock_server.verify().await;
}

#[test]
fn test_run_measurement() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let backend: &Bound<APIBackendWrapper> =
                &create_valid_backend_with_tweezer_device(py, Some(11));

            let cheated = create_cheated_measurement();

            let failed_result = backend.call_method1("run_measurement", (3_u32,));
            assert!(failed_result.is_err());

            let result: Option<HashMap<String, f64>> = backend
                .call_method1("run_measurement", (cheated,))
                .unwrap()
                .extract()
                .unwrap();

            assert!(result.is_some());
        });
    }
}

#[tokio::test]
async fn async_test_run_measurement() {
    let wiremock_server = MockServer::start().await;
    let port = wiremock_server.address().port().to_string();
    let uri = wiremock_server.uri();
    let qryd_job_status_completed = QRydJobStatus {
        status: "completed".to_string(),
        msg: "the job has been completed".to_string(),
    };
    let result_counts = ResultCounts {
        counts: HashMap::from([("0x0".to_string(), 40)]),
    };
    let qryd_job_result_completed = QRydJobResult {
        compilation_time: 1.0,
        data: result_counts,
        time_taken: 0.23,
        noise: "noise".to_string(),
        method: "method".to_string(),
        device: "QrydEmuSquareDevice".to_string(),
        num_qubits: 2,
        num_clbits: 2,
        fusion_max_qubits: 2,
        fusion_avg_qubits: 2.0,
        fusion_generated_gates: 100,
        executed_single_qubit_gates: 0,
        executed_two_qubit_gates: 0,
    };

    let _mock_post = Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(201).insert_header("Location", &format!("{}/DummyLocation", uri)),
        )
        .expect(1)
        .mount(&wiremock_server)
        .await;
    let _mock_status = Mock::given(method("GET"))
        .and(path("/DummyLocation/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&qryd_job_status_completed))
        .expect(1)
        .mount(&wiremock_server)
        .await;
    let _mock_result = Mock::given(method("GET"))
        .and(path("/DummyLocation/result"))
        .respond_with(ResponseTemplate::new(200).set_body_json(qryd_job_result_completed))
        .expect(1)
        .mount(&wiremock_server)
        .await;

    pyo3::prepare_freethreaded_python();
    tokio::task::spawn_blocking(|| {
        Python::with_gil(|py| {
            let backend: &Bound<APIBackendWrapper> =
                &create_valid_backend_with_square_device_mocked(py, Some(11), port);

            let cheated = create_cheated_measurement();

            let failed_result = backend.call_method1("run_measurement", (3_u32,));
            assert!(failed_result.is_err());

            let result: Option<HashMap<String, f64>> = backend
                .call_method1("run_measurement", (cheated,))
                .unwrap()
                .extract()
                .unwrap();

            assert!(result.is_some());
        });
    })
    .await
    .unwrap();

    wiremock_server.verify().await;
}

#[test]
fn test_query_result_fail() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = create_backend_with_square_device(py, Some(11));

        let failed_result_job = backend.call_method1("get_job_result", ("3",));
        assert!(failed_result_job.is_err());
    });
}

#[tokio::test]
async fn async_test_convert_into_backend() {
    let wiremock_server = MockServer::start().await;
    let port = wiremock_server.address().port().to_string();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let none_string: Option<String> = None;
        let initial: Bound<APIBackendWrapper> = if env::var("QRYD_API_TOKEN").is_ok() {
            create_valid_backend_with_square_device(py, Some(11))
        } else {
            create_valid_backend_with_square_device_mocked(py, Some(11), port.clone())
        };

        let converted = convert_into_backend(&initial).unwrap();

        let rust_dev: QrydEmuSquareDevice = QrydEmuSquareDevice::new(Some(11), None, None);
        let rust_api: QRydAPIDevice = QRydAPIDevice::from(rust_dev);
        let rust_backend: APIBackend = if env::var("QRYD_API_TOKEN").is_ok() {
            APIBackend::new(
                rust_api,
                none_string.clone(),
                Some(30),
                none_string,
                None,
                None,
            )
            .unwrap()
        } else {
            APIBackend::new(rust_api, none_string, Some(30), Some(port), None, None).unwrap()
        };

        assert_eq!(converted, rust_backend);

        let wrong_param: &Bound<PyAny> = &PyList::empty_bound(py);
        let wrong_convert = convert_into_backend(wrong_param);
        assert!(wrong_convert.is_err());
    });
}

/// Test to and from bincode for api backend with square device
#[test]
fn test_bincode_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = create_backend_with_square_device(py, Some(11));

        let serialised = backend.call_method0("to_bincode").unwrap();
        let deserialised = backend
            .call_method1("from_bincode", (&serialised,))
            .unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = backend.call_method1("from_bincode", (vec,));
        assert!(deserialised_error.is_err());

        let st: String = "test".to_string();
        let deserialised_error = backend.call_method1("from_bincode", (st,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<APIBackendWrapper>().unwrap();
        let backend_wrapper = backend.extract::<APIBackendWrapper>().unwrap();
        assert_eq!(backend_wrapper, serde_wrapper);
    });
}

#[tokio::test]
async fn test_dev() {
    let wiremock_server = MockServer::start().await;
    let port = wiremock_server.address().port().to_string();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<QrydEmuSquareDeviceWrapper>();
        let device = device_type.call1((11,)).unwrap();

        let backend_type: &Bound<PyType> = &py.get_type_bound::<APIBackendWrapper>();
        let binding = backend_type
            .call1((
                device.downcast::<QrydEmuSquareDeviceWrapper>().unwrap(),
                Option::<String>::None,
                Option::<usize>::None,
                port,
                false,
            ))
            .unwrap();
        let backend: &Bound<APIBackendWrapper> = binding.downcast::<APIBackendWrapper>().unwrap();

        assert!(backend.call_method1("set_dev", (true,)).is_ok());

        let internal = &backend.borrow().internal;
        assert!(internal.dev);
    });
}
