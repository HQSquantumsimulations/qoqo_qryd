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

use std::collections::HashMap;

use roqoqo::measurements::{Cheated, CheatedInput, ClassicalRegister};
use roqoqo::measurements::{PauliZProduct, PauliZProductInput};
use roqoqo::operations;
use roqoqo::Circuit;
use roqoqo::QuantumProgram;
use roqoqo::RoqoqoBackendError;
use roqoqo_qryd::api_devices::{QRydAPIDevice, QrydEmuSquareDevice, QrydEmuTriangularDevice};
// use roqoqo_qryd::downconvert_roqoqo_version;
use roqoqo_qryd::{APIBackend, QRydJobResult, QRydJobStatus, ResultCounts};

use qoqo_calculator::CalculatorFloat;

use mockito::Server;

use std::{env, thread, time};

// Test submitting a valid circuit
#[test]
fn api_backend() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        let number_qubits = 6;
        let device = QrydEmuSquareDevice::new(Some(2), None, None);
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(qryd_device, None, None, None).unwrap();
        let mut circuit = Circuit::new();
        circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
        circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
        circuit += operations::RotateY::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::RotateZ::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::PauliX::new(2);
        circuit += operations::PauliY::new(2);
        circuit += operations::PauliZ::new(2);
        circuit += operations::Hadamard::new(3);
        circuit += operations::SqrtPauliX::new(5);
        circuit += operations::InvSqrtPauliX::new(5);
        circuit += operations::PhaseShiftState1::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::RotateXY::new(
            4,
            std::f64::consts::FRAC_PI_2.into(),
            std::f64::consts::FRAC_PI_4.into(),
        );
        circuit += operations::CNOT::new(1, 2);
        circuit += operations::SWAP::new(1, 2);
        circuit += operations::ISwap::new(1, 2);
        circuit += operations::ControlledPauliY::new(1, 2);
        circuit += operations::ControlledPauliZ::new(1, 2);
        circuit += operations::ControlledPhaseShift::new(1, 2, std::f64::consts::FRAC_PI_4.into());
        circuit += operations::PragmaControlledCircuit::new(1, Circuit::new());
        circuit += operations::ControlledControlledPauliZ::new(1, 2, 3);
        circuit += operations::ControlledControlledPhaseShift::new(
            1,
            2,
            3,
            std::f64::consts::FRAC_PI_4.into(),
        );

        // circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
        for i in 0..number_qubits {
            circuit += operations::MeasureQubit::new(i, "ro".to_string(), number_qubits - i - 1);
        }
        circuit += operations::PragmaSetNumberOfMeasurements::new(40, "ro".to_string()); // assert!(api_backend_new.is_ok());
        let measurement = ClassicalRegister {
            constant_circuit: None,
            circuits: vec![circuit.clone()],
        };
        let program = QuantumProgram::ClassicalRegister {
            measurement,
            input_parameter_names: vec![],
        };
        let job_loc = api_backend_new.post_job(program).unwrap();

        let fifteen = time::Duration::from_secs(1);

        let mut test_counter = 0;
        let mut status = "".to_string();
        let mut job_result = QRydJobResult::default();
        while test_counter < 20 && status != "completed" {
            test_counter += 1;
            let job_status = api_backend_new.get_job_status(job_loc.clone()).unwrap();
            status = job_status.status.clone();
            thread::sleep(fifteen);

            if status == *"completed" {
                assert_eq!(job_status.status, "completed");
                job_result = api_backend_new.get_job_result(job_loc.clone()).unwrap();
            }
        }
        let (bits, _, _) =
            APIBackend::counts_to_result(job_result.data, "ro".to_string(), number_qubits).unwrap();
        assert!(!bits.is_empty());
    } else {
        let mut server = Server::new();
        let port = server
            .url()
            .chars()
            .rev()
            .take(5)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
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
        let mock_post = server
            .mock("POST", mockito::Matcher::Any)
            .with_status(201)
            .with_header("Location", &format!("{}/DummyLocation", server.url()))
            .create();
        let mock_status0 = server
            .mock("GET", "/DummyLocation/status")
            .with_status(200)
            .with_body(
                serde_json::to_string(&qryd_job_status_in_progress)
                    .unwrap()
                    .into_bytes(),
            )
            .expect(20)
            .create();
        let mock_result = server
            .mock("GET", "/DummyLocation/result")
            .with_status(200)
            .with_body(
                serde_json::to_string(&qryd_job_result_completed)
                    .unwrap()
                    .into_bytes(),
            )
            .create();

        let number_qubits = 6;
        let device = QrydEmuSquareDevice::new(Some(2), None, None);
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(qryd_device, None, None, Some(port)).unwrap();
        let mut circuit = Circuit::new();
        circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
        circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
        circuit += operations::RotateY::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::RotateZ::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::PauliX::new(2);
        circuit += operations::PauliY::new(2);
        circuit += operations::PauliZ::new(2);
        circuit += operations::Hadamard::new(3);
        circuit += operations::SqrtPauliX::new(5);
        circuit += operations::InvSqrtPauliX::new(5);
        circuit += operations::PhaseShiftState1::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::RotateXY::new(
            4,
            std::f64::consts::FRAC_PI_2.into(),
            std::f64::consts::FRAC_PI_4.into(),
        );
        circuit += operations::CNOT::new(1, 2);
        circuit += operations::SWAP::new(1, 2);
        circuit += operations::ISwap::new(1, 2);
        circuit += operations::ControlledPauliY::new(1, 2);
        circuit += operations::ControlledPauliZ::new(1, 2);
        circuit += operations::ControlledPhaseShift::new(1, 2, std::f64::consts::FRAC_PI_4.into());
        circuit += operations::PragmaControlledCircuit::new(1, Circuit::new());
        circuit += operations::ControlledControlledPauliZ::new(1, 2, 3);
        circuit += operations::ControlledControlledPhaseShift::new(
            1,
            2,
            3,
            std::f64::consts::FRAC_PI_4.into(),
        );
        circuit += operations::MeasureQubit::new(0, "ro".to_string(), 0);
        circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());

        let measurement = ClassicalRegister {
            constant_circuit: None,
            circuits: vec![circuit.clone()],
        };
        let program = QuantumProgram::ClassicalRegister {
            measurement,
            input_parameter_names: vec![],
        };
        let job_loc = api_backend_new.post_job(program).unwrap();

        let fifteen = time::Duration::from_millis(50);

        let mut test_counter = 0;
        let mut status = "".to_string();
        while test_counter < 20 && status != "completed" {
            test_counter += 1;
            let job_status = api_backend_new.get_job_status(job_loc.clone()).unwrap();
            status = job_status.status.clone();
            assert_eq!(job_status.status, "in progress");
            thread::sleep(fifteen);
        }
        mock_status0.assert();
        mock_status0.remove();

        let mock_status1 = server
            .mock("GET", "/DummyLocation/status")
            .with_status(200)
            .with_body(
                serde_json::to_string(&qryd_job_status_completed)
                    .unwrap()
                    .into_bytes(),
            )
            .create();

        let job_status = api_backend_new.get_job_status(job_loc.clone()).unwrap();

        assert_eq!(job_status.status, "completed");

        let job_result = api_backend_new.get_job_result(job_loc).unwrap();
        let (bits, _, _) =
            APIBackend::counts_to_result(job_result.data, "ro".to_string(), number_qubits).unwrap();
        assert!(!bits.is_empty());

        mock_post.assert();
        mock_status1.assert();
        mock_result.assert();
    }
}

// Test submitting an invalid circuit
#[test]
fn api_backend_failing() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        let number_qubits = 6;
        let device = QrydEmuSquareDevice::new(Some(2), None, None);
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(qryd_device, None, None, None).unwrap();
        // // CAUTION: environment variable QRYD_API_TOKEN needs to be set on the terminal to pass this test!
        let mut circuit = Circuit::new();
        circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);

        circuit += operations::ControlledPhaseShift::new(1, 2, std::f64::consts::FRAC_PI_4.into());
        circuit += operations::PragmaRepeatedMeasurement::new("ro".to_string(), 20, None);
        // circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
        // for i in 0..number_qubits {
        //     circuit += operations::MeasureQubit::new(i, "ro".to_string(), number_qubits - i - 1);
        // }
        // circuit += operations::PragmaSetNumberOfMeasurements::new(40, "ro".to_string()); // assert!(api_backend_new.is_ok());
        let measurement = ClassicalRegister {
            constant_circuit: None,
            circuits: vec![circuit.clone()],
        };
        let program = QuantumProgram::ClassicalRegister {
            measurement,
            input_parameter_names: vec![],
        };
        let program_result = program.run(api_backend_new, &[]);
        assert!(program_result.is_err());
    }
}

#[test]
fn api_backend_with_constant_circuit() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        let number_qubits = 6;
        let device = QrydEmuSquareDevice::new(Some(2), None, None);
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(qryd_device, None, None, None).unwrap();
        // // CAUTION: environment variable QRYD_API_TOKEN needs to be set on the terminal to pass this test!
        let mut circuit = Circuit::new();
        circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
        circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
        circuit += operations::RotateY::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::RotateZ::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::PauliX::new(2);
        circuit += operations::PauliY::new(2);
        circuit += operations::PauliZ::new(2);
        circuit += operations::Hadamard::new(3);
        circuit += operations::SqrtPauliX::new(5);
        circuit += operations::InvSqrtPauliX::new(5);
        circuit += operations::PhaseShiftState1::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::RotateXY::new(
            4,
            std::f64::consts::FRAC_PI_2.into(),
            std::f64::consts::FRAC_PI_4.into(),
        );
        circuit += operations::CNOT::new(1, 2);
        circuit += operations::SWAP::new(1, 2);
        circuit += operations::ISwap::new(1, 2);
        circuit += operations::ControlledPauliY::new(1, 2);
        circuit += operations::ControlledPauliZ::new(1, 2);
        circuit += operations::ControlledPhaseShift::new(1, 2, std::f64::consts::FRAC_PI_4.into());

        // circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
        for i in 0..number_qubits {
            circuit += operations::MeasureQubit::new(i, "ro".to_string(), number_qubits - i - 1);
        }
        circuit += operations::PragmaSetNumberOfMeasurements::new(40, "ro".to_string()); // assert!(api_backend_new.is_ok());
        let mut constant_circuit = Circuit::new();

        constant_circuit += operations::PauliX::new(0);
        constant_circuit += operations::Hadamard::new(1);

        let measurement = ClassicalRegister {
            constant_circuit: Some(constant_circuit),
            circuits: vec![circuit.clone()],
        };

        let program = QuantumProgram::ClassicalRegister {
            measurement,
            input_parameter_names: vec![],
        };
        let job_loc = api_backend_new
            .post_job(
                // "qryd_emu_localcomp_square".to_string(),
                // Some(0),
                // Some(0.23),
                program,
            )
            .unwrap();

        let fifteen = time::Duration::from_secs(1);

        let mut test_counter = 0;
        let mut status = "".to_string();
        let mut job_result = QRydJobResult::default();
        while test_counter < 20 && status != "completed" {
            test_counter += 1;
            let job_status = api_backend_new.get_job_status(job_loc.clone()).unwrap();
            status = job_status.status.clone();
            thread::sleep(fifteen);

            if status == *"completed" {
                assert_eq!(job_status.status, "completed");
                job_result = api_backend_new.get_job_result(job_loc.clone()).unwrap();
            }
        }
        let (bits, _, _) =
            APIBackend::counts_to_result(job_result.data, "ro".to_string(), number_qubits).unwrap();
        assert!(!bits.is_empty());
    }
}

#[test]
fn api_triangular() {
    let number_qubits = 6;
    let device = QrydEmuTriangularDevice::new(Some(2), None, None, None, None);
    let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
    circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
    circuit += operations::RotateY::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::RotateZ::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::PauliX::new(2);
    circuit += operations::PauliY::new(2);
    circuit += operations::PauliZ::new(2);
    circuit += operations::Hadamard::new(3);
    circuit += operations::SqrtPauliX::new(5);
    circuit += operations::InvSqrtPauliX::new(5);
    circuit += operations::PhaseShiftState1::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::RotateXY::new(
        4,
        std::f64::consts::FRAC_PI_2.into(),
        std::f64::consts::FRAC_PI_4.into(),
    );
    circuit += operations::CNOT::new(1, 2);
    circuit += operations::SWAP::new(1, 2);
    circuit += operations::ISwap::new(1, 2);
    circuit += operations::ControlledPauliY::new(1, 2);
    circuit += operations::ControlledPauliZ::new(1, 2);
    circuit += operations::ControlledPhaseShift::new(1, 2, std::f64::consts::FRAC_PI_4.into());

    // circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
    for i in 0..number_qubits {
        circuit += operations::MeasureQubit::new(i, "ro".to_string(), number_qubits - i - 1);
    }
    circuit += operations::PragmaSetNumberOfMeasurements::new(40, "ro".to_string()); // assert!(api_backend_new.is_ok());
    let measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![circuit.clone()],
    };
    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };

    if env::var("QRYD_API_TOKEN").is_ok() {
        let api_backend_new = APIBackend::new(qryd_device, None, None, None).unwrap();

        let job_loc = api_backend_new.post_job(program).unwrap();
        assert!(!job_loc.is_empty());

        let fifteen = time::Duration::from_secs(1);

        let mut test_counter = 0;
        let mut status = "".to_string();
        let mut job_result = QRydJobResult::default();
        while test_counter < 20 && status != "completed" {
            test_counter += 1;
            let job_status = api_backend_new.get_job_status(job_loc.clone()).unwrap();
            status = job_status.status.clone();
            thread::sleep(fifteen);
            assert!(!job_status.status.clone().is_empty());

            if status == *"completed" {
                assert_eq!(job_status.status, "completed");
                job_result = api_backend_new.get_job_result(job_loc.clone()).unwrap();
            }
        }
        let (bits, _, _) =
            APIBackend::counts_to_result(job_result.data, "ro".to_string(), number_qubits).unwrap();
        assert!(!bits.is_empty());
    } else {
        let mut server = Server::new();
        let port = server
            .url()
            .chars()
            .rev()
            .take(5)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
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
            device: "QrydEmuTriangularDevice".to_string(),
            num_qubits: 4,
            num_clbits: 4,
            fusion_max_qubits: 4,
            fusion_avg_qubits: 4.0,
            fusion_generated_gates: 100,
            executed_single_qubit_gates: 50,
            executed_two_qubit_gates: 50,
        };
        let mock_post = server
            .mock("POST", mockito::Matcher::Any)
            .with_status(201)
            .with_header("Location", &format!("{}/DummyLocation", server.url()))
            .create();
        let mock_status = server
            .mock("GET", "/DummyLocation/status")
            .with_status(200)
            .with_body(
                serde_json::to_string(&qryd_job_status_completed)
                    .unwrap()
                    .into_bytes(),
            )
            .create();
        let mock_result = server
            .mock("GET", "/DummyLocation/result")
            .with_status(200)
            .with_body(
                serde_json::to_string(&qryd_job_result_completed)
                    .unwrap()
                    .into_bytes(),
            )
            .create();

        let api_backend_new = APIBackend::new(qryd_device, None, None, Some(port)).unwrap();

        let job_loc = api_backend_new.post_job(program).unwrap();
        assert!(!job_loc.is_empty());

        let job_status = api_backend_new.get_job_status(job_loc.clone()).unwrap();
        let job_result = api_backend_new.get_job_result(job_loc).unwrap();

        assert_eq!(job_status.status, "completed");

        let (bits, _, _) =
            APIBackend::counts_to_result(job_result.data, "ro".to_string(), number_qubits).unwrap();
        assert!(!bits.is_empty());

        mock_post.assert();
        mock_status.assert();
        mock_result.assert();
    }
}
#[test]
fn evaluating_backend() {
    let number_qubits = 6;
    let device = QrydEmuSquareDevice::new(Some(2), None, None);
    let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
    circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
    circuit += operations::RotateX::new(4, std::f64::consts::PI.into());
    circuit += operations::RotateX::new(2, std::f64::consts::PI.into());
    for i in 0..number_qubits {
        circuit += operations::MeasureQubit::new(i, "ro".to_string(), i);
    }
    circuit += operations::PragmaSetNumberOfMeasurements::new(40, "ro".to_string()); // assert!(api_backend_new.is_ok());

    let mut input = PauliZProductInput::new(6, false);
    let index = input
        .add_pauliz_product("ro".to_string(), vec![0, 2, 4])
        .unwrap();
    let mut linear: HashMap<usize, f64> = HashMap::new();
    linear.insert(index, 3.0);
    input
        .add_linear_exp_val("test".to_string(), linear)
        .unwrap();
    let measurement = PauliZProduct {
        input,
        constant_circuit: None,
        circuits: vec![circuit.clone()],
    };
    let program = QuantumProgram::PauliZProduct {
        measurement,
        input_parameter_names: vec![],
    };

    if env::var("QRYD_API_TOKEN").is_ok() {
        let api_backend_new = APIBackend::new(qryd_device, None, Some(20), None).unwrap();

        let program_result = program.run(api_backend_new, &[]).unwrap().unwrap();
        assert_eq!(program_result.get("test"), Some(&-3.0));
    } else {
        let mut server = Server::new();
        let port = server
            .url()
            .chars()
            .rev()
            .take(5)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let qryd_job_status_completed = QRydJobStatus {
            status: "completed".to_string(),
            msg: "the job has been completed".to_string(),
        };
        let qryd_job_status_error = QRydJobStatus {
            status: "error".to_string(),
            msg: "an error as occured".to_string(),
        };
        let qryd_job_status_cancelled = QRydJobStatus {
            status: "cancelled".to_string(),
            msg: "the job has been cancelled".to_string(),
        };
        let mock_post = server
            .mock("POST", mockito::Matcher::Any)
            .with_status(201)
            .with_header("Location", &format!("{}/DummyLocation", server.url()))
            .create();
        let mock_status0 = server
            .mock("GET", "/DummyLocation/status")
            .with_status(200)
            .with_body(
                serde_json::to_string(&qryd_job_status_completed)
                    .unwrap()
                    .into_bytes(),
            )
            .create();
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
        let mock_result0 = server
            .mock("GET", "/DummyLocation/result")
            .with_status(200)
            .with_body(
                serde_json::to_string(&qryd_job_result_completed)
                    .unwrap()
                    .into_bytes(),
            )
            .create();

        let api_backend_new = APIBackend::new(qryd_device, None, Some(20), Some(port)).unwrap();

        let program_result = program.run(api_backend_new.clone(), &[]).unwrap().unwrap();

        assert_eq!(program_result.get("test"), Some(&-3.0));
        mock_post.assert();
        mock_status0.assert();
        mock_result0.assert();

        let mock_status1 = server
            .mock("GET", "/DummyLocation/status")
            .with_status(200)
            .with_body(
                serde_json::to_string(&qryd_job_status_error)
                    .unwrap()
                    .into_bytes(),
            )
            .expect(20)
            .create();
        let program_result = program.run(api_backend_new.clone(), &[]);

        assert!(program_result.is_err());
        assert_eq!(
            program_result.unwrap_err(),
            RoqoqoBackendError::GenericError {
                msg: format!(
                    "WebAPI returned an error status for the job {}/DummyLocation.",
                    server.url()
                )
            }
        );
        mock_status1.assert();
        mock_status1.remove();

        let mock_status2 = server
            .mock("GET", "/DummyLocation/status")
            .with_status(200)
            .with_body(
                serde_json::to_string(&qryd_job_status_cancelled)
                    .unwrap()
                    .into_bytes(),
            )
            .expect(20)
            .create();

        let program_result = program.run(api_backend_new.clone(), &[]);
        assert!(program_result.is_err());
        assert_eq!(
            program_result.unwrap_err(),
            RoqoqoBackendError::GenericError {
                msg: format!("Job {}/DummyLocation got cancelled.", server.url())
            }
        );
        mock_status2.assert();
        mock_status2.remove();

        let unknown_status = QRydJobStatus {
            status: "unknown".to_string(),
            msg: "".to_string(),
        };
        let mock_status3 = server
            .mock("GET", "/DummyLocation/status")
            .with_status(200)
            .with_body(serde_json::to_string(&unknown_status).unwrap().into_bytes())
            .expect(20)
            .create();
        let program_result = program.run(api_backend_new, &[]);
        assert!(program_result.is_err());
        assert_eq!(
            program_result.unwrap_err(),
            RoqoqoBackendError::GenericError {
                msg: "WebAPI did not return finished result in timeout: 20 * 30s".to_string(),
            }
        );
        mock_status3.assert();
    }
}

/// Test api_delete successful functionality
#[test]
fn api_delete() {
    let device = QrydEmuSquareDevice::new(Some(1), None, None);
    let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
    let number_qubits = 6;
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
    circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
    circuit += operations::RotateY::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::RotateZ::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::PauliX::new(2);
    circuit += operations::PauliY::new(2);
    circuit += operations::PauliZ::new(2);
    circuit += operations::Hadamard::new(3);
    circuit += operations::SqrtPauliX::new(5);
    circuit += operations::InvSqrtPauliX::new(5);
    circuit += operations::PhaseShiftState1::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::RotateXY::new(
        4,
        std::f64::consts::FRAC_PI_2.into(),
        std::f64::consts::FRAC_PI_4.into(),
    );
    circuit += operations::CNOT::new(1, 2);
    circuit += operations::SWAP::new(1, 2);
    circuit += operations::ISwap::new(1, 2);
    circuit += operations::ControlledPauliY::new(1, 2);
    circuit += operations::ControlledPauliZ::new(1, 2);
    circuit += operations::ControlledPhaseShift::new(1, 2, std::f64::consts::FRAC_PI_4.into());
    for i in 0..number_qubits {
        circuit += operations::MeasureQubit::new(i, "ro".to_string(), number_qubits - i - 1);
    }
    circuit += operations::PragmaSetNumberOfMeasurements::new(40, "ro".to_string()); // assert!(api_backend_new.is_ok());
    let measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![circuit.clone()],
    };
    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };

    if env::var("QRYD_API_TOKEN").is_ok() {
        let api_backend_new = APIBackend::new(qryd_device, None, None, None).unwrap();

        let job_loc = api_backend_new
            .post_job(
                // "qryd_emu_localcomp_square".to_string(),
                // Some(0),
                // Some(0.23),
                program,
            )
            .unwrap();
        let delete_job = api_backend_new.delete_job(job_loc);
        assert!(delete_job.is_ok());
    } else {
        let mut server = Server::new();
        let port = server
            .url()
            .chars()
            .rev()
            .take(5)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let mock_post = server
            .mock("POST", mockito::Matcher::Any)
            .with_status(201)
            .with_header("Location", &format!("{}/DummyLocation", server.url()))
            .create();
        let mock_delete = server
            .mock("DELETE", mockito::Matcher::Any)
            .with_status(200)
            .create();
        let api_backend_new = APIBackend::new(qryd_device, None, None, Some(port)).unwrap();

        let job_loc = api_backend_new.post_job(program).unwrap();

        let delete_job = api_backend_new.delete_job(job_loc);
        assert!(delete_job.is_ok());

        mock_post.assert();
        mock_delete.assert();
    }
}

// Test error cases. Case const: invalid constant_circuit
#[test]
fn api_backend_errorcase_const() {
    let number_qubits = 6;
    let device = QrydEmuSquareDevice::new(Some(2), None, None);
    let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
    let api_backend_new: APIBackend = if env::var("QRYD_API_TOKEN").is_ok() {
        APIBackend::new(qryd_device, None, None, None).unwrap()
    } else {
        let server = Server::new();
        let port = server
            .url()
            .chars()
            .rev()
            .take(5)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        APIBackend::new(qryd_device, None, None, Some(port)).unwrap()
    };
    // // CAUTION: environment variable QRYD_API_TOKEN needs to be set on the terminal to pass this test!
    let qubit_mapping: HashMap<usize, usize> = (0..number_qubits).map(|x| (x, x)).collect();
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
    circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
    circuit += operations::RotateX::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit +=
        operations::PragmaRepeatedMeasurement::new("ro".to_string(), 40, Some(qubit_mapping));
    let measurement = ClassicalRegister {
        constant_circuit: Some(circuit.clone()),
        circuits: vec![circuit.clone()],
    };
    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };
    let job_loc = api_backend_new.post_job(program);
    assert!(job_loc.is_err());
}

/// Test error cases. Case 3: invalid API TOKEN
#[test]
fn api_backend_errorcase3() {
    let number_qubits = 6;
    let device = QrydEmuSquareDevice::new(Some(2), None, None);
    let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);

    if env::var("QRYD_API_TOKEN").is_err() {
        let api_backend_err = APIBackend::new(qryd_device.clone(), None, None, None);
        assert!(api_backend_err.is_err());
        assert_eq!(
            api_backend_err.unwrap_err(),
            RoqoqoBackendError::MissingAuthentification {
                msg: "QRYD access token is missing".to_string()
            }
        );
    }
    let api_backend_new =
        APIBackend::new(qryd_device, Some("DummyString".to_string()), None, None).unwrap();

    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
    circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
    circuit += operations::RotateX::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::MeasureQubit::new(0, "ro".to_string(), 0);
    circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());
    let measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![circuit.clone()],
    };
    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };
    let job_loc = api_backend_new.post_job(program);
    assert!(job_loc.is_err());

    let job_loc_dummy: String = "DummyString".to_string();
    let job_status = api_backend_new.get_job_status(job_loc_dummy.clone());
    assert!(job_status.is_err());

    let job_result = api_backend_new.get_job_result(job_loc_dummy.clone());
    assert!(job_result.is_err());

    let job_delete = api_backend_new.delete_job(job_loc_dummy);
    assert!(job_delete.is_err());
}

/// Test error cases. Case 4: invalid job_id
#[test]
fn api_backend_errorcase4() {
    let device = QrydEmuSquareDevice::new(Some(2), None, None);
    let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);

    let api_backend_new: APIBackend = if env::var("QRYD_API_TOKEN").is_ok() {
        APIBackend::new(qryd_device, None, None, None).unwrap()
    } else {
        let server = Server::new();
        let port = server
            .url()
            .chars()
            .rev()
            .take(5)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        APIBackend::new(qryd_device, None, None, Some(port)).unwrap()
    };
    let job_loc: String = "DummyString".to_string();
    let job_status = api_backend_new.get_job_status(job_loc.clone());
    assert!(job_status.is_err());

    let job_result = api_backend_new.get_job_result(job_loc.clone());
    assert!(job_result.is_err());

    let job_delete = api_backend_new.delete_job(job_loc);
    assert!(job_delete.is_err());
}

/// Test error cases. Case 5: invalid QuantumProgram
#[test]
fn api_backend_errorcase5() {
    let device = QrydEmuSquareDevice::new(Some(2), None, None);
    let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
    let api_backend_new: APIBackend = if env::var("QRYD_API_TOKEN").is_ok() {
        APIBackend::new(qryd_device, None, None, None).unwrap()
    } else {
        let server = Server::new();
        let port = server
            .url()
            .chars()
            .rev()
            .take(5)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

        APIBackend::new(qryd_device, None, None, Some(port)).unwrap()
    };
    let measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![],
    };
    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };
    let job_loc0 = api_backend_new.post_job(program);
    assert!(job_loc0.is_err());
    assert_eq!(
        job_loc0.unwrap_err(),
        RoqoqoBackendError::GenericError {
            msg: "QRyd API Backend only supports posting ClassicalRegister with one circuit"
                .to_string()
        }
    );

    let mut circuit = Circuit::new();
    circuit += operations::RotateZ::new(0, CalculatorFloat::from("parametrized"));
    assert!(circuit.is_parametrized());
    let measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![circuit],
    };
    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };
    let job_loc1 = api_backend_new.post_job(program);
    assert!(job_loc1.is_err());
    assert_eq!(
        job_loc1.unwrap_err(),
        RoqoqoBackendError::GenericError {
            msg: "Qoqo circuit contains symbolic parameters. The QrydWebAPI does not support symbolic parameters."
                .to_string()
        }
    );

    let measurement = Cheated {
        constant_circuit: None,
        circuits: vec![],
        input: CheatedInput::new(4),
    };
    let program = QuantumProgram::Cheated {
        measurement,
        input_parameter_names: vec![],
    };
    let job_loc2 = api_backend_new.post_job(program);
    assert!(job_loc2.is_err());
    assert_eq!(
        job_loc2.unwrap_err(),
        RoqoqoBackendError::GenericError {
            msg: "QRyd API Backend only supports posting ClassicalRegister QuantumPrograms"
                .to_string()
        }
    );
}

/// Test error cases. Case 6: missing Location header
#[test]
fn api_backend_errorcase6() {
    let mut server = Server::new();
    let port = server
        .url()
        .chars()
        .rev()
        .take(5)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    let mock = server
        .mock("POST", mockito::Matcher::Any)
        .with_status(201)
        .create();
    let device = QrydEmuSquareDevice::new(Some(1), None, None);
    let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
    let api_backend_new = APIBackend::new(qryd_device, None, None, Some(port)).unwrap();
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), 6, true);
    circuit += operations::RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::RotateX::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::MeasureQubit::new(0, "ro".to_string(), 0);
    circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());
    let measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![circuit.clone()],
    };
    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };
    let job_loc = api_backend_new.post_job(program.clone());

    assert!(job_loc.is_err());
    assert_eq!(
        job_loc.unwrap_err(),
        RoqoqoBackendError::NetworkError {
            msg: "Server response missing the Location header".to_string()
        }
    );
    mock.assert();

    let mock = server
        .mock("POST", mockito::Matcher::Any)
        .with_status(201)
        .with_header("Location", "\n")
        .create();

    let job_loc = api_backend_new.post_job(program);

    assert!(job_loc.is_err());
    assert!(matches!(
        job_loc.unwrap_err(),
        RoqoqoBackendError::NetworkError { .. }
    ));
    mock.assert();
}

/// Test error case. Case 7: unreachable server
#[test]
fn api_backend_errorcase7() {
    let device = QrydEmuSquareDevice::new(Some(1), None, None);
    let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
    let api_backend_new =
        APIBackend::new(qryd_device, None, None, Some("12345".to_string())).unwrap();
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), 6, true);
    circuit += operations::RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::RotateX::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::MeasureQubit::new(0, "ro".to_string(), 0);
    circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());
    let measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![circuit.clone()],
    };
    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };

    let job_loc = api_backend_new.post_job(program);

    assert!(job_loc.is_err());
    assert!(matches!(
        job_loc.unwrap_err(),
        RoqoqoBackendError::NetworkError { .. }
    ));

    let job_status =
        api_backend_new.get_job_status("http://127.0.0.1:12345/DummyLocation".to_string());

    assert!(job_status.is_err());
    assert!(matches!(
        job_status.unwrap_err(),
        RoqoqoBackendError::NetworkError { .. }
    ));

    let job_result =
        api_backend_new.get_job_result("http://127.0.0.1:12345/DummyLocation".to_string());

    assert!(job_result.is_err());
    assert!(matches!(
        job_result.unwrap_err(),
        RoqoqoBackendError::NetworkError { .. }
    ));

    let job_delete = api_backend_new.delete_job("http://127.0.0.1:12345/DummyLocation".to_string());

    assert!(job_delete.is_err());
    assert!(matches!(
        job_delete.unwrap_err(),
        RoqoqoBackendError::NetworkError { .. }
    ));
}

/// Test error case. Case 8: unexpected status code
#[test]
fn api_backend_errorcase8() {
    let mut server = Server::new();
    let port = server
        .url()
        .chars()
        .rev()
        .take(5)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    let mock_post = server
        .mock("POST", mockito::Matcher::Any)
        .with_status(404)
        .create();
    let mock_status = server
        .mock("GET", "/DummyLocation/status")
        .with_status(404)
        .create();
    let mock_result = server
        .mock("GET", "/DummyLocation/result")
        .with_status(404)
        .create();
    let mock_delete = server
        .mock("DELETE", mockito::Matcher::Any)
        .with_status(404)
        .create();

    let device = QrydEmuSquareDevice::new(Some(1), None, None);
    let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
    let api_backend_new = APIBackend::new(qryd_device, None, None, Some(port)).unwrap();
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), 6, true);
    circuit += operations::RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::RotateX::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit += operations::MeasureQubit::new(0, "ro".to_string(), 0);
    circuit += operations::PragmaSetNumberOfMeasurements::new(10, "ro".to_string());
    let measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![circuit.clone()],
    };
    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };

    let job_loc = api_backend_new.post_job(program);

    mock_post.assert();
    assert!(job_loc.is_err());
    assert_eq!(
        job_loc.unwrap_err(),
        RoqoqoBackendError::NetworkError {
            msg: "Request to server failed with HTTP status code 404".to_string()
        }
    );

    let job_status = api_backend_new.get_job_status(format!("{}/DummyLocation", server.url()));

    mock_status.assert();
    assert!(job_status.is_err());
    assert_eq!(
        job_status.unwrap_err(),
        RoqoqoBackendError::NetworkError {
            msg: "Request to server failed with HTTP status code 404".to_string()
        }
    );

    let job_result = api_backend_new.get_job_result(format!("{}/DummyLocation", server.url()));

    mock_result.assert();
    assert!(job_result.is_err());
    assert_eq!(
        job_result.unwrap_err(),
        RoqoqoBackendError::NetworkError {
            msg: "Request to server failed with HTTP status code 404".to_string()
        }
    );

    let job_delete = api_backend_new.delete_job(format!("{}/DummyLocation", server.url()));

    mock_delete.assert();
    assert!(job_delete.is_err());
    assert_eq!(
        job_delete.unwrap_err(),
        RoqoqoBackendError::NetworkError {
            msg: "Request to server failed with HTTP status code 404".to_string()
        }
    );
}

// /// Test downcovert_roqoqo_version function
// #[test]
// fn test_downconvert_roqoqo_version() {
//     let measurement = Cheated {
//         constant_circuit: None,
//         circuits: vec![],
//         input: CheatedInput::new(4),
//     };
//     let program = QuantumProgram::Cheated {
//         measurement,
//         input_parameter_names: vec![],
//     };

//     let wrong_downconverted_quantumprogram = downconvert_roqoqo_version(program);
//     assert!(wrong_downconverted_quantumprogram.is_err());
//     assert_eq!(
//         wrong_downconverted_quantumprogram.unwrap_err(),
//         RoqoqoBackendError::GenericError {
//             msg:
//                 "Only ClassiclaRegister measurements are supported by the Qryd WebAPI at the moment"
//                     .to_string()
//         }
//     );

//     let mut circuit = Circuit::new();
//     circuit += operations::InputBit::new("ro".to_string(), 0, true);
//     let measurement = ClassicalRegister {
//         constant_circuit: None,
//         circuits: vec![circuit.clone()],
//     };
//     let program = QuantumProgram::ClassicalRegister {
//         measurement,
//         input_parameter_names: vec![],
//     };

//     let wrong_downconverted_quantumprogram = downconvert_roqoqo_version(program);
//     assert!(wrong_downconverted_quantumprogram.is_err());
//     assert_eq!(
//         wrong_downconverted_quantumprogram.unwrap_err(),
//         RoqoqoBackendError::GenericError {
//             msg: "InputBit operation not compatible with roqoqo 1.0 and QRyd Web API v2_0"
//                 .to_string(),
//         }
//     );

//     let mut circuit = Circuit::new();
//     circuit += operations::PragmaLoop::new(CalculatorFloat::Float(0.23), Circuit::new());
//     let measurement = ClassicalRegister {
//         constant_circuit: None,
//         circuits: vec![circuit.clone()],
//     };
//     let program = QuantumProgram::ClassicalRegister {
//         measurement,
//         input_parameter_names: vec![],
//     };

//     let wrong_downconverted_quantumprogram = downconvert_roqoqo_version(program);
//     assert!(wrong_downconverted_quantumprogram.is_err());
//     assert_eq!(
//         wrong_downconverted_quantumprogram.unwrap_err(),
//         RoqoqoBackendError::GenericError {
//             msg: "PragmaLoop operation not compatible with roqoqo 1.0 and QRyd Web API v2_0"
//                 .to_string(),
//         }
//     );

//     let mut circuit = Circuit::new();
//     circuit += operations::InputBit::new("ro".to_string(), 0, true);
//     let measurement = ClassicalRegister {
//         constant_circuit: Some(circuit.clone()),
//         circuits: vec![Circuit::new()],
//     };
//     let program = QuantumProgram::ClassicalRegister {
//         measurement,
//         input_parameter_names: vec![],
//     };

//     let wrong_downconverted_quantumprogram = downconvert_roqoqo_version(program);
//     assert!(wrong_downconverted_quantumprogram.is_err());
//     assert_eq!(
//         wrong_downconverted_quantumprogram.unwrap_err(),
//         RoqoqoBackendError::GenericError {
//             msg: "InputBit operation not compatible with roqoqo 1.0 and QRyd Web API v2_0"
//                 .to_string(),
//         }
//     );

//     let mut circuit = Circuit::new();
//     circuit += operations::PragmaLoop::new(CalculatorFloat::Float(0.23), Circuit::new());
//     let measurement = ClassicalRegister {
//         constant_circuit: Some(circuit.clone()),
//         circuits: vec![Circuit::new()],
//     };
//     let program = QuantumProgram::ClassicalRegister {
//         measurement,
//         input_parameter_names: vec![],
//     };

//     let wrong_downconverted_quantumprogram = downconvert_roqoqo_version(program);
//     assert!(wrong_downconverted_quantumprogram.is_err());
//     assert_eq!(
//         wrong_downconverted_quantumprogram.unwrap_err(),
//         RoqoqoBackendError::GenericError {
//             msg: "PragmaLoop operation not compatible with roqoqo 1.0 and QRyd Web API v2_0"
//                 .to_string(),
//         }
//     );

//     let mut circuit = Circuit::new();
//     circuit += operations::RotateX::new(0, CalculatorFloat::Float(0.23));
//     let measurement = ClassicalRegister {
//         constant_circuit: None,
//         circuits: vec![circuit.clone()],
//     };
//     let program = QuantumProgram::ClassicalRegister {
//         measurement,
//         input_parameter_names: vec![],
//     };

//     let correct_downconverted_quantum_program = downconvert_roqoqo_version(program);
//     assert!(correct_downconverted_quantum_program.is_ok());
// }
