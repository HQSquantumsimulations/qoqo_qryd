// Copyright © 2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use bincode::serialize;
use ndarray::Array2;

use roqoqo::{devices::Device, RoqoqoBackendError};
use roqoqo_qryd::{phi_theta_relation, PragmaChangeQRydLayout, TweezerDevice};

use mockito::Server;

/// Test TweezerDevice new()
#[test]
fn test_new() {
    let device = TweezerDevice::new(None, None);

    assert_eq!(device.current_layout, "Default");
    assert!(device.qubit_to_tweezer.is_none());
    assert_eq!(device.layout_register.len(), 1);
    assert!(device.layout_register.get("Default").is_some());
}

// Test TweezerDevice add_layout(), switch_layout() methods
#[test]
fn test_layouts() {
    let mut device = TweezerDevice::new(None, None);

    assert!(device.available_layouts().contains(&"Default"));

    device.add_layout("Test").unwrap();

    assert!(device.add_layout("Test").is_err());

    assert_eq!(device.layout_register.len(), 2);
    assert!(device.layout_register.contains_key("Default"));
    assert!(device.layout_register.contains_key("Test"));

    device.set_tweezer_single_qubit_gate_time("RotateX", 0, 0.23, None);
    device.set_tweezer_single_qubit_gate_time("RotateZ", 1, 0.23, Some("Test".to_string()));
    device.set_tweezer_single_qubit_gate_time("RotateY", 2, 0.23, Some("Test".to_string()));
    device.set_tweezer_two_qubit_gate_time("CNOT", 0, 1, 0.23, None);
    device.set_tweezer_two_qubit_gate_time("CNOT", 0, 1, 0.23, Some("Test".to_string()));
    device.set_tweezer_three_qubit_gate_time("Toffoli", 0, 1, 2, 0.34, None);
    device.set_tweezer_three_qubit_gate_time("Toffoli", 0, 1, 2, 0.34, Some("Test".to_string()));
    device.set_tweezer_multi_qubit_gate_time("MultiQubitZZ", &[0, 1, 2, 3], 0.13, None);
    device.set_tweezer_multi_qubit_gate_time(
        "MultiQubitZZ",
        &[0, 1, 2, 3],
        0.13,
        Some("Test".to_string()),
    );

    let default_layout = device.layout_register.get("Default").unwrap();
    let test_layout = device.layout_register.get("Test").unwrap();
    assert!(default_layout
        .tweezer_single_qubit_gate_times
        .contains_key("RotateX"));
    assert!(default_layout
        .tweezer_single_qubit_gate_times
        .get("RotateX")
        .unwrap()
        .get(&0)
        .is_some());
    assert_eq!(
        *default_layout
            .tweezer_single_qubit_gate_times
            .get("RotateX")
            .unwrap()
            .get(&0)
            .unwrap(),
        0.23
    );
    assert_eq!(test_layout.tweezer_single_qubit_gate_times.len(), 2);
    assert_eq!(
        *test_layout
            .tweezer_single_qubit_gate_times
            .get("RotateZ")
            .unwrap()
            .get(&1)
            .unwrap(),
        0.23
    );
    assert_eq!(
        *test_layout
            .tweezer_single_qubit_gate_times
            .get("RotateY")
            .unwrap()
            .get(&2)
            .unwrap(),
        0.23
    );

    assert_eq!(
        *default_layout
            .tweezer_two_qubit_gate_times
            .get("CNOT")
            .unwrap()
            .get(&(0, 1))
            .unwrap(),
        0.23
    );
    assert_eq!(
        *test_layout
            .tweezer_two_qubit_gate_times
            .get("CNOT")
            .unwrap()
            .get(&(0, 1))
            .unwrap(),
        0.23
    );

    assert_eq!(
        *default_layout
            .tweezer_three_qubit_gate_times
            .get("Toffoli")
            .unwrap()
            .get(&(0, 1, 2))
            .unwrap(),
        0.34
    );
    assert_eq!(
        *test_layout
            .tweezer_three_qubit_gate_times
            .get("Toffoli")
            .unwrap()
            .get(&(0, 1, 2))
            .unwrap(),
        0.34
    );

    assert_eq!(
        *default_layout
            .tweezer_multi_qubit_gate_times
            .get("MultiQubitZZ")
            .unwrap()
            .get(&[0, 1, 2, 3].to_vec())
            .unwrap(),
        0.13
    );
    assert_eq!(
        *test_layout
            .tweezer_multi_qubit_gate_times
            .get("MultiQubitZZ")
            .unwrap()
            .get(&[0, 1, 2, 3].to_vec())
            .unwrap(),
        0.13
    );

    assert_eq!(device.current_layout, "Default");
    assert!(device.qubit_to_tweezer.is_none());

    device.switch_layout("Test").unwrap();
    assert_eq!(device.current_layout, "Test");
    assert!(device.qubit_to_tweezer.is_some());
    assert_eq!(device.qubit_to_tweezer.clone().unwrap().len(), 4);

    assert!(device.switch_layout("Error").is_err());

    assert!(device.available_layouts().contains(&"Default"));
    assert!(device.available_layouts().contains(&"Test"));
}

// Test TweezerDevice add_qubit_tweezer_mapping(), get_tweezer_from_qubit() methods
#[test]
fn test_qubit_tweezer_mapping() {
    let mut device = TweezerDevice::new(None, None);

    assert!(device.add_qubit_tweezer_mapping(0, 0).is_err());
    assert!(device.get_tweezer_from_qubit(&0).is_err());

    device.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.0, None);
    device.set_tweezer_multi_qubit_gate_time("MultiQubitZZ", &[1, 2, 3], 0.1, None);

    assert!(device.add_qubit_tweezer_mapping(0, 0).is_ok());
    assert!(device.add_qubit_tweezer_mapping(2, 3).is_ok());

    assert_eq!(device.get_tweezer_from_qubit(&0).unwrap(), 0);
    assert_eq!(device.get_tweezer_from_qubit(&2).unwrap(), 3);
    assert!(device.get_tweezer_from_qubit(&4).is_err());

    let add_01 = device.add_qubit_tweezer_mapping(0, 1);
    assert!(add_01.is_ok());
    assert_eq!(add_01.unwrap(), vec![(0, 1), (2, 3)].into_iter().collect());
}

/// Test TweezerDevice set_allowed_tweezer_shifts() method
#[test]
fn test_allowed_tweezer_shifts() {
    let mut device = TweezerDevice::new(None, None);
    device.add_layout("OtherLayout").unwrap();
    device.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.0, Some("OtherLayout".to_string()));
    device.set_tweezer_single_qubit_gate_time("PauliX", 1, 0.0, Some("OtherLayout".to_string()));
    device.set_tweezer_single_qubit_gate_time("PauliX", 2, 0.0, Some("OtherLayout".to_string()));
    device.switch_layout("OtherLayout").unwrap();

    assert!(device
        .set_allowed_tweezer_shifts(&0, &[&[1], &[2]], Some("OtherLayout".to_string()))
        .is_ok());

    let saved_shifts = &device
        .layout_register
        .get("OtherLayout")
        .unwrap()
        .allowed_tweezer_shifts;
    assert!(!saved_shifts.is_empty());
    assert!(saved_shifts.contains_key(&0));
    assert!(saved_shifts.get(&0).unwrap().contains(&vec![1]));
    assert!(saved_shifts.get(&0).unwrap().contains(&vec![2]));

    let incorrect_shift_list =
        device.set_allowed_tweezer_shifts(&0, &[&[0]], Some("OtherLayout".to_string()));
    assert!(incorrect_shift_list.is_err());
    assert_eq!(
        incorrect_shift_list.unwrap_err(),
        RoqoqoBackendError::GenericError {
            msg: "The allowed shifts contain the given tweezer.".to_string(),
        }
    );

    let incorrect_tweezer = device.set_allowed_tweezer_shifts(&3, &[&[0]], None);
    assert!(incorrect_tweezer.is_err());
    assert_eq!(
        incorrect_tweezer.unwrap_err(),
        RoqoqoBackendError::GenericError {
            msg:
                "The given tweezer, or shifts tweezers, are not present in the device Tweezer data."
                    .to_string(),
        }
    );

    let incorrect_shift_list_2 = device.set_allowed_tweezer_shifts(&2, &[&[5]], None);
    assert!(incorrect_shift_list_2.is_err());
    assert_eq!(
        incorrect_shift_list_2.unwrap_err(),
        RoqoqoBackendError::GenericError {
            msg:
                "The given tweezer, or shifts tweezers, are not present in the device Tweezer data."
                    .to_string(),
        }
    )
}

/// Test TweezerDevice deactivate_qubit()
#[test]
fn test_deactivate_qubit() {
    let mut device = TweezerDevice::new(None, None);

    assert!(device.deactivate_qubit(0).is_err());

    device.set_tweezer_single_qubit_gate_time("PauliX", 1, 0.1, None);
    device.add_qubit_tweezer_mapping(0, 1).unwrap();

    assert!(device.deactivate_qubit(0).is_ok());
    assert!(device.deactivate_qubit(0).is_err());
}

/// Test TweezerDevice ..._qubit_gate_time() methods
#[test]
fn test_qubit_times() {
    let mut device = TweezerDevice::new(None, None);

    assert!(device.single_qubit_gate_time("PauliX", &0).is_none());

    // Testing missing qubits
    assert!(device.single_qubit_gate_time("PauliX", &5).is_none());
    assert!(device.two_qubit_gate_time("CNOT", &0, &7).is_none());
    assert!(device
        .three_qubit_gate_time("Toffoli", &12, &1, &3)
        .is_none());
    assert!(device
        .multi_qubit_gate_time("MultiQubitZZ", &[6, 2, 3, 4])
        .is_none());

    device.set_tweezer_single_qubit_gate_time("PauliX", 1, 0.23, None);
    device.set_tweezer_two_qubit_gate_time("CNOT", 0, 1, 0.45, None);
    device.set_tweezer_three_qubit_gate_time("Toffoli", 0, 1, 2, 0.65, None);
    device.set_tweezer_multi_qubit_gate_time("MultiQubitZZ", &[0, 1, 2, 3], 0.34, None);

    device.add_qubit_tweezer_mapping(0, 1).unwrap();
    device.add_qubit_tweezer_mapping(1, 2).unwrap();
    device.add_qubit_tweezer_mapping(2, 3).unwrap();
    device.add_qubit_tweezer_mapping(3, 0).unwrap();

    assert!(device.single_qubit_gate_time("PauliX", &0).is_some());
    assert_eq!(device.single_qubit_gate_time("PauliX", &0).unwrap(), 0.23);
    assert_eq!(device.two_qubit_gate_time("CNOT", &3, &0).unwrap(), 0.45);
    assert_eq!(
        device.three_qubit_gate_time("Toffoli", &3, &0, &1).unwrap(),
        0.65
    );
    assert_eq!(
        device
            .multi_qubit_gate_time("MultiQubitZZ", &[3, 0, 1, 2])
            .unwrap(),
        0.34
    );
}

/// Test TweezerDevice number_qubits() method
#[test]
fn test_number_qubits() {
    let mut device = TweezerDevice::new(None, None);

    assert_eq!(device.number_qubits(), 0);

    device.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.0, None);
    device.set_tweezer_single_qubit_gate_time("PauliX", 1, 0.0, None);

    assert_eq!(device.number_qubits(), 2);

    device.add_qubit_tweezer_mapping(0, 0).unwrap();
    device.add_qubit_tweezer_mapping(1, 1).unwrap();

    assert_eq!(device.number_qubits(), 2)
}

/// Test TweezerDevice to_generic_device() method method
#[test]
fn test_to_generic_device() {
    let mut device = TweezerDevice::new(None, None);
    device.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.23, None);
    device.set_tweezer_single_qubit_gate_time("PauliY", 1, 0.23, None);
    device.set_tweezer_two_qubit_gate_time("CNOT", 2, 3, 0.34, None);
    device.set_tweezer_two_qubit_gate_time("ControlledPauliZ", 1, 2, 0.34, None);
    device.add_qubit_tweezer_mapping(0, 0).unwrap();
    device.add_qubit_tweezer_mapping(1, 1).unwrap();
    device.add_qubit_tweezer_mapping(2, 2).unwrap();
    device.add_qubit_tweezer_mapping(3, 3).unwrap();

    let generic_device = device.to_generic_device();

    assert_eq!(
        generic_device
            .single_qubit_gates
            .get("PauliX")
            .unwrap()
            .get(&0)
            .unwrap(),
        &0.23
    );
    assert_eq!(
        generic_device
            .single_qubit_gates
            .get("PauliY")
            .unwrap()
            .get(&1)
            .unwrap(),
        &0.23
    );
    assert_eq!(
        generic_device
            .two_qubit_gates
            .get("CNOT")
            .unwrap()
            .get(&(2, 3))
            .unwrap(),
        &0.34
    );
    assert_eq!(
        generic_device
            .two_qubit_gates
            .get("ControlledPauliZ")
            .unwrap()
            .get(&(1, 2))
            .unwrap(),
        &0.34
    );
    assert_eq!(
        generic_device.qubit_decoherence_rates(&0),
        Some(Array2::zeros((3, 3).to_owned()))
    );
    assert_eq!(
        generic_device.qubit_decoherence_rates(&1),
        Some(Array2::zeros((3, 3).to_owned()))
    );
}

/// Test TweezerDevice change_device() method
#[test]
fn test_change_device() {
    let mut device = TweezerDevice::new(None, None);
    device.add_layout("0").unwrap();
    device.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.23, Some("0".to_string()));
    device.set_tweezer_single_qubit_gate_time("PauliY", 1, 0.23, Some("0".to_string()));
    device.set_tweezer_two_qubit_gate_time("CNOT", 2, 3, 0.34, Some("0".to_string()));
    device.set_tweezer_two_qubit_gate_time("ControlledPauliZ", 1, 2, 0.34, Some("0".to_string()));
    let pragma_c = PragmaChangeQRydLayout::new(0);

    assert!(device.change_device("Error", &Vec::<u8>::new()).is_err());
    assert!(device
        .change_device("PragmaChangeQRydLayout", &Vec::<u8>::new())
        .is_err());
    assert_eq!(device.current_layout, "Default");

    assert!(device
        .change_device("PragmaChangeQRydLayout", &serialize(&pragma_c).unwrap())
        .is_ok());
    assert_eq!(device.current_layout, "0");
}

/// Test TweezerDevice from_api() method
#[test]
#[cfg(feature = "web-api")]
fn test_from_api() {
    let mut returned_device_default = TweezerDevice::new(None, None);
    returned_device_default.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.23, None);
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
    let mut mock = server
        .mock("POST", mockito::Matcher::Any)
        .with_status(200)
        .with_body(
            serde_json::to_string(&returned_device_default)
                .unwrap()
                .into_bytes(),
        )
        .create();

    let response = TweezerDevice::from_api(None, None, Some(port.clone()));
    mock.assert();
    assert!(response.is_ok());

    let device = response.unwrap();
    assert_eq!(device, returned_device_default);

    mock.remove();
    mock = server
        .mock("POST", mockito::Matcher::Any)
        .with_status(400)
        .create();

    let response = TweezerDevice::from_api(None, None, Some(port));
    mock.assert();
    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err(),
        RoqoqoBackendError::NetworkError {
            msg: format!("Request to server failed with HTTP status code {:?}.", 400),
        }
    );
}

/// Test TweezerDevice phase_shift_controlled_...() and gate_time_controlled_...()  methods
#[test]
fn test_phi_theta_relation() {
    let mut device = TweezerDevice::new(None, None);
    let device_f = TweezerDevice::new(Some(2.13.to_string()), Some(2.15.to_string()));

    assert_eq!(
        device.phase_shift_controlled_z().unwrap(),
        phi_theta_relation("DefaultRelation", std::f64::consts::PI).unwrap()
    );
    assert_eq!(
        device.phase_shift_controlled_phase(1.2).unwrap(),
        phi_theta_relation("DefaultRelation", 1.2).unwrap()
    );
    assert_eq!(device_f.phase_shift_controlled_z(), Some(2.13));
    assert_eq!(device_f.phase_shift_controlled_phase(0.3), Some(2.15));

    assert!(device.gate_time_controlled_z(&0, &1, 1.4).is_none());
    assert!(device
        .gate_time_controlled_phase(&0, &1, 1.4, 2.4)
        .is_none());
    assert!(device.gate_time_controlled_z(&0, &7, 1.4).is_none());
    assert!(device
        .gate_time_controlled_phase(&0, &7, 1.4, 2.3)
        .is_none());

    device.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledZ", 0, 1, 0.23, None);
    device.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 0, 1, 0.23, None);
    device.add_qubit_tweezer_mapping(0, 0).unwrap();
    device.add_qubit_tweezer_mapping(1, 1).unwrap();

    assert!(device
        .gate_time_controlled_z(&0, &1, device.phase_shift_controlled_z().unwrap())
        .is_some());
    assert!(device
        .gate_time_controlled_z(&0, &7, device.phase_shift_controlled_z().unwrap())
        .is_none());
    assert!(device
        .gate_time_controlled_phase(
            &0,
            &1,
            device.phase_shift_controlled_phase(0.1).unwrap(),
            0.1
        )
        .is_some());
    assert!(device
        .gate_time_controlled_phase(
            &0,
            &7,
            device.phase_shift_controlled_phase(0.1).unwrap(),
            0.1
        )
        .is_none());
}
