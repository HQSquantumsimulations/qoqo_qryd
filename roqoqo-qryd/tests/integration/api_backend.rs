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

use roqoqo::measurements::ClassicalRegister;
use roqoqo::measurements::{PauliZProduct, PauliZProductInput};
use roqoqo::operations;
use roqoqo::Circuit;
use roqoqo::QuantumProgram;
use roqoqo_qryd::api_devices::{QRydAPIDevice, QrydEmuSquareDevice, QrydEmuTriangularDevice};
use roqoqo_qryd::APIBackend;
use roqoqo_qryd::QRydJobResult;

use std::{env, thread, time};

// Test the new function
#[test]
fn api_backend() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        let number_qubits = 6;
        let device = QrydEmuSquareDevice::new(Some(2), Some(0.23)).unwrap();
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(qryd_device, None, None).unwrap();
        // // CAUTION: environment variable QRYD_API_TOKEN needs to be set on the terminal to pass this test!
        let qubit_mapping: HashMap<usize, usize> =
            (0..number_qubits).into_iter().map(|x| (x, x)).collect();
        let mut circuit = Circuit::new();
        circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
        circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
        circuit += operations::RotateX::new(4, std::f64::consts::FRAC_PI_2.into());
        // circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
        circuit +=
            operations::PragmaRepeatedMeasurement::new("ro".to_string(), 40, Some(qubit_mapping)); // assert!(api_backend_new.is_ok());
        let measurement = ClassicalRegister {
            constant_circuit: None,
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
        println!("Job location {}", job_loc);

        let fifteen = time::Duration::from_secs(1);

        let mut test_counter = 0;
        let mut status = "".to_string();
        let mut job_result = QRydJobResult::default();
        while test_counter < 20 && status != "completed" {
            test_counter += 1;
            let job_status = api_backend_new.get_job_status(job_loc.clone()).unwrap();
            status = job_status.status.clone();
            thread::sleep(fifteen);

            println!("Job status {:?}", job_status);
            if status == *"completed" {
                assert_eq!(job_status.status, "completed");
                job_result = api_backend_new.get_job_result(job_loc.clone()).unwrap();
                println!("Job result {:?}", job_result.clone());
            }
        }
        let (bits, _, _) =
            APIBackend::counts_to_result(job_result.data, "ro".to_string(), number_qubits).unwrap();
        assert!(!bits.is_empty());
        for line in bits["ro"].iter() {
            println!("{:?}", line);
        }
    }
}

#[test]
fn api_triangular() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        let number_qubits = 6;
        let device = QrydEmuTriangularDevice::new(Some(2), Some(0.23)).unwrap();
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(qryd_device, None, None).unwrap();
        // // CAUTION: environment variable QRYD_API_TOKEN needs to be set on the terminal to pass this test!
        let qubit_mapping: HashMap<usize, usize> =
            (0..number_qubits).into_iter().map(|x| (x, x)).collect();
        let mut circuit = Circuit::new();
        circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
        circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
        circuit += operations::RotateX::new(4, std::f64::consts::FRAC_PI_2.into());
        // circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
        circuit +=
            operations::PragmaRepeatedMeasurement::new("ro".to_string(), 40, Some(qubit_mapping)); // assert!(api_backend_new.is_ok());
        let measurement = ClassicalRegister {
            constant_circuit: None,
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
        println!("Job location {}", job_loc);
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

            println!("Job status {:?}", job_status);
            if status == *"completed" {
                assert_eq!(job_status.status, "completed");
                job_result = api_backend_new.get_job_result(job_loc.clone()).unwrap();
                println!("Job result {:?}", job_result.clone());
            }
        }
        let (bits, _, _) =
            APIBackend::counts_to_result(job_result.data, "ro".to_string(), number_qubits).unwrap();
        assert!(!bits.is_empty());
        for line in bits["ro"].iter() {
            println!("{:?}", line);
        }
    }
}

#[test]
fn evaluating_backend() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        let number_qubits = 6;
        let device = QrydEmuSquareDevice::new(Some(2), Some(0.23)).unwrap();
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(qryd_device, None, Some(20)).unwrap();
        // // CAUTION: environment variable QRYD_API_TOKEN needs to be set on the terminal to pass this test!
        let qubit_mapping: HashMap<usize, usize> =
            (0..number_qubits).into_iter().map(|x| (x, x)).collect();
        let mut circuit = Circuit::new();
        circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
        circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
        circuit += operations::RotateX::new(4, std::f64::consts::PI.into());
        circuit += operations::RotateX::new(2, std::f64::consts::PI.into());
        circuit +=
            operations::PragmaRepeatedMeasurement::new("ro".to_string(), 40, Some(qubit_mapping)); // assert!(api_backend_new.is_ok());
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
        let program_result = program.run(api_backend_new, &[]).unwrap().unwrap();
        println!("{:?}", program_result);
        assert_eq!(program_result.get("test"), Some(&-3.0));
    }
}

#[test]
fn api_delete() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        let device = QrydEmuSquareDevice::new(Some(1), Some(0.23)).unwrap();
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(qryd_device, None, None).unwrap();
        // // CAUTION: environment variable QRYD_API_TOKEN needs to be set on the terminal to pass this test!
        let qubit_mapping: HashMap<usize, usize> = (0..6).into_iter().map(|x| (x, x)).collect();
        let mut circuit = Circuit::new();
        circuit += operations::DefinitionBit::new("ro".to_string(), 6, true);
        circuit += operations::RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::RotateX::new(2, std::f64::consts::FRAC_PI_2.into());
        circuit += operations::RotateX::new(4, std::f64::consts::FRAC_PI_2.into());
        circuit +=
            operations::PragmaRepeatedMeasurement::new("ro".to_string(), 100, Some(qubit_mapping)); // assert!(api_backend_new.is_ok());
        let measurement = ClassicalRegister {
            constant_circuit: None,
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
        println!("Job location {}", job_loc);
        let delete_job = api_backend_new.delete_job(job_loc);
        assert!(delete_job.is_ok());
    }
}

// Test error cases. Case 1: constant_circuit != None
#[test]
fn api_backend_errorcase1() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        let number_qubits = 6;
        let device = QrydEmuSquareDevice::new(Some(2), Some(0.23)).unwrap();
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(qryd_device, None, None).unwrap();
        // // CAUTION: environment variable QRYD_API_TOKEN needs to be set on the terminal to pass this test!
        let qubit_mapping: HashMap<usize, usize> =
            (0..number_qubits).into_iter().map(|x| (x, x)).collect();
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
        let job_loc = api_backend_new
            .post_job(
                program,
            );
        
        assert!(job_loc.is_err());
    }
}

// Test error cases. Case 2: invalid API TOKEN
#[test]
fn api_backend_errorcase2() {
    let number_qubits = 6;
    let device = QrydEmuSquareDevice::new(Some(2), Some(0.23)).unwrap();
    let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
    let api_backend_new = APIBackend::new(qryd_device, Some("DummyString".to_string()), None).unwrap();
    let qubit_mapping: HashMap<usize, usize> =
        (0..number_qubits).into_iter().map(|x| (x, x)).collect();
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), number_qubits, true);
    circuit += operations::RotateX::new(0, std::f64::consts::PI.into());
    circuit += operations::RotateX::new(4, std::f64::consts::FRAC_PI_2.into());
    circuit +=
        operations::PragmaRepeatedMeasurement::new("ro".to_string(), 40, Some(qubit_mapping));
    let measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![circuit.clone()],
    };
    let program = QuantumProgram::ClassicalRegister {
        measurement,
        input_parameter_names: vec![],
    };
    let job_loc = api_backend_new
        .post_job(
            program,
        );
    assert!(job_loc.is_err());

    let job_loc_dummy: String = "DummyString".to_string();
    let job_status = api_backend_new.get_job_status(job_loc_dummy.clone());
    assert!(job_status.is_err());

    let job_result = api_backend_new.get_job_result(job_loc_dummy.clone());
    assert!(job_result.is_err());

    let job_delete = api_backend_new.delete_job(job_loc_dummy.clone());
    assert!(job_delete.is_err());
}

// Test error cases. Case 3: invalid job_id
#[test]
fn api_backend_errorcase3() {
    if env::var("QRYD_API_TOKEN").is_ok() {
        let device = QrydEmuSquareDevice::new(Some(2), Some(0.23)).unwrap();
        let qryd_device: QRydAPIDevice = QRydAPIDevice::from(&device);
        let api_backend_new = APIBackend::new(qryd_device, None, None).unwrap();
        // // CAUTION: environment variable QRYD_API_TOKEN needs to be set on the terminal to pass this test!
        let job_loc: String = "DummyString".to_string();
        let job_status = api_backend_new.get_job_status(job_loc.clone());
        assert!(job_status.is_err());

        let job_result = api_backend_new.get_job_result(job_loc.clone());
        assert!(job_result.is_err());

        let job_delete = api_backend_new.delete_job(job_loc.clone());
        assert!(job_delete.is_err());
    }
}
