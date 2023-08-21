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

use roqoqo::devices::Device;
use roqoqo_qryd::ExperimentalDevice;

/// Test ExperimentalDevice new()
#[test]
fn test_new() {
    let device = ExperimentalDevice::new();

    assert_eq!(device.current_layout, "Default");
    assert!(device.qubit_to_tweezer.is_empty());
    assert_eq!(device.layout_register.len(), 1);
    assert!(device.layout_register.get("Default").is_some());
}

// Test ExperimentalDevice add_layout(), switch_layout() methods
#[test]
fn test_layouts() {
    let mut device = ExperimentalDevice::new();
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
    device.switch_layout("Test").unwrap();
    assert_eq!(device.current_layout, "Test");
    assert!(device.switch_layout("Error").is_err());
}

// Test ExperimentalDevice add_qubit_tweezer_mapping(), get_tweezer_from_qubit() methods
#[test]
fn test_qubit_tweezer_mapping() {
    let mut device = ExperimentalDevice::new();
    device.add_qubit_tweezer_mapping(0, 1);

    assert!(device.get_tweezer_from_qubit(&1).is_err());
    assert_eq!(device.get_tweezer_from_qubit(&0).unwrap(), 1);
}

/// Test ExperimentalDevice ..._qubit_gate_time() methods
#[test]
fn test_qubit_times() {
    let mut device = ExperimentalDevice::new();
    device.add_qubit_tweezer_mapping(0, 1);
    device.add_qubit_tweezer_mapping(1, 2);
    device.add_qubit_tweezer_mapping(2, 3);
    device.add_qubit_tweezer_mapping(3, 0);

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
    assert!(device.single_qubit_gate_time("PauliX", &0).is_some());
    assert_eq!(device.single_qubit_gate_time("PauliX", &0).unwrap(), 0.23);

    device.set_tweezer_two_qubit_gate_time("CNOT", 0, 1, 0.45, None);
    assert_eq!(device.two_qubit_gate_time("CNOT", &3, &0).unwrap(), 0.45);

    device.set_tweezer_three_qubit_gate_time("Toffoli", 0, 1, 2, 0.65, None);
    assert_eq!(
        device.three_qubit_gate_time("Toffoli", &3, &0, &1).unwrap(),
        0.65
    );

    device.set_tweezer_multi_qubit_gate_time("MultiQubitZZ", &[0, 1, 2, 3], 0.34, None);
    assert_eq!(
        device
            .multi_qubit_gate_time("MultiQubitZZ", &[3, 0, 1, 2])
            .unwrap(),
        0.34
    );
}
