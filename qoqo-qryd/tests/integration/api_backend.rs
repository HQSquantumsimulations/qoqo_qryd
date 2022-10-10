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
use httpmock::MockServer;
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::Python;
use qoqo::measurements::CheatedWrapper;
use qoqo::{CircuitWrapper, QuantumProgramWrapper};
use qoqo_qryd::api_backend::{APIBackendWrapper, Registers};
use qoqo_qryd::api_devices::{QrydEmuSquareDeviceWrapper, QrydEmuTriangularDeviceWrapper};
use roqoqo::measurements::{Cheated, CheatedInput, ClassicalRegister};
use roqoqo::{operations, Circuit, QuantumProgram};
use roqoqo_qryd::{QRydJobResult, QRydJobStatus, ResultCounts};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::usize;
use std::{env, thread};

// Helper function to create a python object of square device
fn create_backend_with_square_device(
    py: Python,
    seed: Option<usize>,
) -> &PyCell<APIBackendWrapper> {
    let pcz_theta: f64 = PI / 4.0;
    let device_type = py.get_type::<QrydEmuSquareDeviceWrapper>();
    let device: &PyCell<QrydEmuSquareDeviceWrapper> = device_type
        .call1((seed, pcz_theta))
        .unwrap()
        .cast_as::<PyCell<QrydEmuSquareDeviceWrapper>>()
        .unwrap();

    let backend_type: &PyType = py.get_type::<APIBackendWrapper>();
    let backend: &PyCell<APIBackendWrapper> = backend_type
        .call1((device, ""))
        .unwrap()
        .cast_as::<PyCell<APIBackendWrapper>>()
        .unwrap();
    backend
}

// fn create_valid_backend_with_square_device(
//     py: Python,
//     seed: Option<usize>,
// ) -> &PyCell<APIBackendWrapper> {
//     let pcz_theta: f64 = PI / 4.0;
//     let device_type = py.get_type::<QrydEmuSquareDeviceWrapper>();
//     let device: &PyCell<QrydEmuSquareDeviceWrapper> = device_type
//         .call1((seed, pcz_theta))
//         .unwrap()
//         .cast_as::<PyCell<QrydEmuSquareDeviceWrapper>>()
//         .unwrap();

//     let backend_type: &PyType = py.get_type::<APIBackendWrapper>();
//     let none_string: Option<String> = None;
//     let backend: &PyCell<APIBackendWrapper> = backend_type
//         .call1((device, none_string))
//         .unwrap()
//         .cast_as::<PyCell<APIBackendWrapper>>()
//         .unwrap();
//     backend
// }

fn create_valid_backend_with_square_device_mocked(
    py: Python,
    seed: Option<usize>,
    mock_port: String,
) -> &PyCell<APIBackendWrapper> {
    let pcz_theta: f64 = PI / 4.0;
    let device_type = py.get_type::<QrydEmuSquareDeviceWrapper>();
    let device: &PyCell<QrydEmuSquareDeviceWrapper> = device_type
        .call1((seed, pcz_theta))
        .unwrap()
        .cast_as::<PyCell<QrydEmuSquareDeviceWrapper>>()
        .unwrap();

    let backend_type: &PyType = py.get_type::<APIBackendWrapper>();
    let none_string: Option<String> = None;
    let backend: &PyCell<APIBackendWrapper> = backend_type
        .call1((device, none_string, 30, mock_port))
        .unwrap()
        .cast_as::<PyCell<APIBackendWrapper>>()
        .unwrap();
    backend
}

fn create_quantum_program(valid: bool) -> QuantumProgramWrapper {
    let number_qubits = 2;
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
    circuit += operations::RotateX::new(0, 0.0.into());
    // circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::PragmaRepeatedMeasurement::new("ro".to_string(), 40, None); // assert!(api_backend_new.is_ok());
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
    circuit += operations::PragmaRepeatedMeasurement::new("ro".to_string(), 40, None); // assert!(api_backend_new.is_ok());

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
        let pcz_theta: f64 = PI / 4.0;
        let device_type = py.get_type::<QrydEmuSquareDeviceWrapper>();
        let device: &PyCell<QrydEmuSquareDeviceWrapper> = device_type
            .call1((seed, pcz_theta))
            .unwrap()
            .cast_as::<PyCell<QrydEmuSquareDeviceWrapper>>()
            .unwrap();

        let backend_type: &PyType = py.get_type::<APIBackendWrapper>();
        let backend = backend_type
            .call1((device, ""))
            .unwrap()
            .cast_as::<PyCell<APIBackendWrapper>>();
        assert!(backend.is_ok());
    });
}

// Test to check a failed backend creation
#[test]
fn test_fail_new_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let seed: Option<usize> = Some(11);
        let pcz_theta: f64 = PI / 4.0;
        let device_type = py.get_type::<QrydEmuSquareDeviceWrapper>();
        let device: &PyCell<QrydEmuSquareDeviceWrapper> = device_type
            .call1((seed, pcz_theta))
            .unwrap()
            .cast_as::<PyCell<QrydEmuSquareDeviceWrapper>>()
            .unwrap();

        let backend_type: &PyType = py.get_type::<APIBackendWrapper>();
        let backend = backend_type.call1((3_u32, ""));
        assert!(backend.is_err());
        if let Ok(old_token) = env::var("QRYD_API_TOKEN") {
            env::remove_var("QRYD_API_TOKEN");
            let backend = backend_type.call1((device,));
            assert!(backend.is_err());
            env::set_var("QRYD_API_TOKEN", old_token);
        } else {
            let backend = backend_type.call1((device,));
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
        let pcz_theta: f64 = PI / 4.0;
        let device_type = py.get_type::<QrydEmuTriangularDeviceWrapper>();
        let device: &PyCell<QrydEmuTriangularDeviceWrapper> = device_type
            .call1((seed, pcz_theta))
            .unwrap()
            .cast_as::<PyCell<QrydEmuTriangularDeviceWrapper>>()
            .unwrap();

        let backend_type: &PyType = py.get_type::<APIBackendWrapper>();
        let backend = backend_type
            .call1((device, ""))
            .unwrap()
            .cast_as::<PyCell<APIBackendWrapper>>();
        assert!(backend.is_ok());
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
        let deserialised = backend.call_method1("from_json", (serialised,)).unwrap();

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
    let server = MockServer::start();
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
    let mock_post = server.mock(|when, then| {
        when.method("POST");
        then.status(201).header(
            "Location",
            format!("http://127.0.0.1:{}/DummyLocation", server.port()),
        );
    });
    let mut mock_status0 = server.mock(|when, then| {
        when.method("GET").path("/DummyLocation/status");
        then.status(200).json_body_obj(&qryd_job_status_in_progress);
    });
    let mock_result = server.mock(|when, then| {
        when.method("GET").path("/DummyLocation/result");
        then.status(200).json_body_obj(&qryd_job_result_completed);
    });

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend =
            create_valid_backend_with_square_device_mocked(py, Some(11), server.port().to_string());
        let program = create_quantum_program(true);
        let job_loc = backend.call_method1("post_job", (program,)).unwrap();
        let fifteen = time::Duration::from_millis(50);

        let mut test_counter = 0;
        let mut status = "".to_string();
        while test_counter < 20 && status != "completed" {
            test_counter += 1;
            let status_report: HashMap<String, String> = backend
                .call_method1("get_job_status", (job_loc,))
                .unwrap()
                .extract()
                .unwrap();
            let job_status = status_report.get("status").unwrap();
            status = job_status.clone();
            assert_eq!(job_status, "in progress");
            thread::sleep(fifteen);
        }

        mock_status0.assert_hits(20);
        mock_status0.delete();
        let mock_status1 = server.mock(|when, then| {
            when.method("GET").path("/DummyLocation/status");
            then.status(200).json_body_obj(&qryd_job_status_completed);
        });

        let status_report: HashMap<String, String> = backend
            .call_method1("get_job_status", (job_loc,))
            .unwrap()
            .extract()
            .unwrap();
        let job_status = status_report.get("status").unwrap();

        assert_eq!(job_status, "completed");

        let _job_result = backend.call_method1("get_job_result", (job_loc,)).unwrap();

        mock_post.assert();
        mock_status1.assert();
        mock_result.assert();
    });
}

#[test]
fn test_run_circuit() {
    let server = MockServer::start();
    let qryd_job_status_completed = QRydJobStatus {
        status: "completed".to_string(),
        msg: "the job has been completed".to_string(),
    };
    let result_counts = ResultCounts {
        counts: HashMap::from([("0x0".to_string(), 40)]),
    };
    let qryd_job_result_completed = QRydJobResult {
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

    let mock_post = server.mock(|when, then| {
        when.method("POST");
        then.status(201).header(
            "Location",
            format!("http://127.0.0.1:{}/DummyLocation", server.port()),
        );
    });
    let mock_status1 = server.mock(|when, then| {
        when.method("GET").path("/DummyLocation/status");
        then.status(200).json_body_obj(&qryd_job_status_completed);
    });
    let mock_result = server.mock(|when, then| {
        when.method("GET").path("/DummyLocation/result");
        then.status(200).json_body_obj(&qryd_job_result_completed);
    });

    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), 2, true);
    circuit += operations::RotateX::new(0, 0.0.into());
    circuit += operations::PragmaRepeatedMeasurement::new("ro".to_string(), 40, None);
    let circuit_py = CircuitWrapper { internal: circuit };

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend =
            create_valid_backend_with_square_device_mocked(py, Some(11), server.port().to_string());

        backend.call_method1("run_circuit", (circuit_py,)).unwrap();

        mock_post.assert();
        mock_status1.assert();
        mock_result.assert();
    });
}

#[test]
fn test_run_measurement_registers() {
    let server = MockServer::start();
    let qryd_job_status_completed = QRydJobStatus {
        status: "completed".to_string(),
        msg: "the job has been completed".to_string(),
    };
    let result_counts = ResultCounts {
        counts: HashMap::from([("0x0".to_string(), 40)]),
    };
    let qryd_job_result_completed = QRydJobResult {
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

    let mock_post = server.mock(|when, then| {
        when.method("POST");
        then.status(201).header(
            "Location",
            format!("http://127.0.0.1:{}/DummyLocation", server.port()),
        );
    });
    let mock_status1 = server.mock(|when, then| {
        when.method("GET").path("/DummyLocation/status");
        then.status(200).json_body_obj(&qryd_job_status_completed);
    });
    let mock_result = server.mock(|when, then| {
        when.method("GET").path("/DummyLocation/result");
        then.status(200).json_body_obj(&qryd_job_result_completed);
    });

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend =
            create_valid_backend_with_square_device_mocked(py, Some(11), server.port().to_string());
        let program = create_quantum_program(true);
        let failed_result = backend.call_method1("run_measurement_registers", (3_u32,));
        assert!(failed_result.is_err());
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
        assert_eq!(bit.len(), 40);
        mock_post.assert();
        mock_status1.assert();
        mock_result.assert();
    });
}

#[test]
fn test_run_measurement() {
    let server = MockServer::start();
    let qryd_job_status_completed = QRydJobStatus {
        status: "completed".to_string(),
        msg: "the job has been completed".to_string(),
    };
    let result_counts = ResultCounts {
        counts: HashMap::from([("0x0".to_string(), 40)]),
    };
    let qryd_job_result_completed = QRydJobResult {
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

    let mock_post = server.mock(|when, then| {
        when.method("POST");
        then.status(201).header(
            "Location",
            format!("http://127.0.0.1:{}/DummyLocation", server.port()),
        );
    });
    let mock_status1 = server.mock(|when, then| {
        when.method("GET").path("/DummyLocation/status");
        then.status(200).json_body_obj(&qryd_job_status_completed);
    });
    let mock_result = server.mock(|when, then| {
        when.method("GET").path("/DummyLocation/result");
        then.status(200).json_body_obj(&qryd_job_result_completed);
    });

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend =
            create_valid_backend_with_square_device_mocked(py, Some(11), server.port().to_string());
        let cheated = create_cheated_measurement();

        let failed_result = backend.call_method1("run_measurement", (3_u32,));
        assert!(failed_result.is_err());

        let (bits, floats, complex) = backend
            .call_method1("run_measurement", (cheated,))
            .unwrap();
        // TODO
        mock_post.assert();
        mock_status1.assert();
        mock_result.assert();
    });
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

/// Test to and from bincode for api backend with square device
#[test]
fn test_bincode_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = create_backend_with_square_device(py, Some(11));

        let serialised = backend.call_method0("to_bincode").unwrap();
        let deserialised = backend.call_method1("from_bincode", (serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = backend.call_method1("from_bincode", (vec,));
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
