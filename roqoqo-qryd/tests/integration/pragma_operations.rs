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

use bincode::serialize;
use qoqo_calculator::Calculator;
use roqoqo::operations::{InvolveQubits, InvolvedQubits, Operate, PragmaChangeDevice, Substitute};
use roqoqo_qryd::pragma_operations::{
    PragmaChangeQRydLayout, PragmaDeactivateQRydQubit, PragmaShiftQRydQubit,
    PragmaShiftQubitsTweezers, PragmaSwitchDeviceLayout,
};
use serde_test::{assert_tokens, Configure, Token};
use std::collections::HashMap;

/// Test PragmaChangeQRydLayout inputs and involved qubits
#[test]
fn pragma_change_qryd_layout_inputs_qubits() {
    let pragma = PragmaChangeQRydLayout::new(1);

    // Test inputs are correct
    assert_eq!(pragma.new_layout(), &1_usize);

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaChangeQRydLayout to_pragma_change_device function
#[test]
fn pragma_change_qryd_layout_change() {
    let pragma = PragmaChangeQRydLayout::new(1);

    // Test inputs are correct
    let result = PragmaChangeDevice {
        wrapped_tags: vec![
            "Operation".to_string(),
            "PragmaOperation".to_string(),
            "PragmaChangeQRydLayout".to_string(),
        ],
        wrapped_hqslang: "PragmaChangeQRydLayout".to_string(),
        wrapped_operation: serialize(&pragma).unwrap(),
    };
    assert_eq!(pragma.to_pragma_change_device().unwrap(), result);
}

/// Test PragmaChangeQRydLayout standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_change_qryd_layout_simple_traits() {
    let pragma = PragmaChangeQRydLayout::new(1);
    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaChangeQRydLayout { new_layout: 1 }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaChangeQRydLayout::new(1);
    let pragma_1 = PragmaChangeQRydLayout::new(2);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaChangeQRydLayout Operate trait
#[test]
fn pragma_change_qryd_layout_operate_trait() {
    let pragma = PragmaChangeQRydLayout::new(1);

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaChangeQRydLayout"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaChangeQRydLayout"));

    // (3) Test is_parametrized function
    assert!(!pragma.is_parametrized());
}

/// Test PragmaChangeQRydLayout Substitute trait
#[test]
fn pragma_change_qryd_layout_substitute_trait() {
    let pragma = PragmaChangeQRydLayout::new(1);
    let pragma_test = PragmaChangeQRydLayout::new(1);
    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);
    qubit_mapping_test.insert(0, 2);
    let result = pragma_test.remap_qubits(&qubit_mapping_test);
    assert!(result.is_err());
}

/// Test PragmaChangeQRydLayout Serialization and Deserialization traits (readable)
#[test]
fn pragma_change_qryd_layout_serde_readable() {
    let pragma_serialization = PragmaChangeQRydLayout::new(1);
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaChangeQRydLayout",
                len: 1,
            },
            Token::Str("new_layout"),
            Token::U64(1),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaChangeQRydLayout Serialization and Deserialization traits (compact)
#[test]
fn pragma_change_qryd_layout_serde_compact() {
    let pragma_serialization = PragmaChangeQRydLayout::new(1);
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaChangeQRydLayout",
                len: 1,
            },
            Token::Str("new_layout"),
            Token::U64(1),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaShiftQRydQubit inputs and involved qubits
#[test]
fn pragma_shift_qryd_qubit_inputs_qubits() {
    let mut new_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    new_positions.insert(1, (0, 0));
    new_positions.insert(0, (0, 1));
    let pragma = PragmaShiftQRydQubit::new(new_positions.clone());

    // Test inputs are correct
    assert_eq!(pragma.new_positions(), &new_positions);

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaShiftQRydQubit to_pragma_change_device function
#[test]
fn pragma_shift_qryd_qubit_change() {
    let mut new_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    new_positions.insert(1, (0, 0));
    new_positions.insert(0, (0, 1));
    let pragma = PragmaShiftQRydQubit::new(new_positions);

    // Test inputs are correct
    let result = PragmaChangeDevice {
        wrapped_tags: vec![
            "Operation".to_string(),
            "PragmaOperation".to_string(),
            "PragmaShiftQRydQubit".to_string(),
        ],
        wrapped_hqslang: "PragmaShiftQRydQubit".to_string(),
        wrapped_operation: serialize(&pragma).unwrap(),
    };
    assert_eq!(pragma.to_pragma_change_device().unwrap(), result);
}

/// Test PragmaShiftQRydQubit standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_shift_qryd_qubit_simple_traits() {
    let mut new_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    new_positions.insert(1, (0, 0));
    new_positions.insert(0, (0, 1));
    let pragma = PragmaShiftQRydQubit::new(new_positions.clone());
    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        format!(
            "PragmaShiftQRydQubit {{ new_positions: {:?} }}",
            new_positions.clone()
        )
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaShiftQRydQubit::new(new_positions.clone());
    let pragma_1 = PragmaShiftQRydQubit::new(HashMap::new());
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaShiftQRydQubit Operate trait
#[test]
fn pragma_shift_qryd_qubit_operate_trait() {
    let mut new_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    new_positions.insert(1, (0, 0));
    new_positions.insert(0, (0, 1));
    let pragma = PragmaShiftQRydQubit::new(new_positions.clone());

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaShiftQRydQubit"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaShiftQRydQubit"));

    // (3) Test is_parametrized function
    assert!(!pragma.is_parametrized());
}

/// Test PragmaShiftQRydQubit Substitute trait
#[test]
fn pragma_shift_qryd_qubit_substitute_trait() {
    let mut new_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    new_positions.insert(1, (0, 0));
    new_positions.insert(0, (0, 1));
    let pragma = PragmaShiftQRydQubit::new(new_positions.clone());
    let mut new_new_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    new_new_positions.insert(1, (0, 0));
    new_new_positions.insert(2, (0, 1));
    let pragma_test = PragmaShiftQRydQubit::new(new_new_positions.clone());
    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    qubit_mapping_test.insert(2, 0);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);
}

/// Test PragmaShiftQRydQubit Serialization and Deserialization traits (readable)
#[test]
fn pragma_shift_qryd_qubit_serde_readable() {
    let mut new_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    new_positions.insert(0, (0, 0));
    let pragma_serialization = PragmaShiftQRydQubit::new(new_positions.clone());
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaShiftQRydQubit",
                len: 1,
            },
            Token::Str("new_positions"),
            Token::Map { len: Some(1) },
            Token::U64(0),
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::MapEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaShiftQRydQubit Serialization and Deserialization traits (compact)
#[test]
fn pragma_shift_qryd_qubit_serde_compact() {
    let mut new_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    new_positions.insert(0, (0, 0));
    let pragma_serialization = PragmaShiftQRydQubit::new(new_positions.clone());
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaShiftQRydQubit",
                len: 1,
            },
            Token::Str("new_positions"),
            Token::Map { len: Some(1) },
            Token::U64(0),
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::U64(0),
            Token::TupleEnd,
            Token::MapEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaDeactivateQRydQubit inputs and involved qubits
#[test]
fn pragma_deactivate_qryd_qubit_inputs_qubits() {
    let qubit = 0;
    let pragma = PragmaDeactivateQRydQubit::new(qubit);

    // Test inputs are correct
    assert_eq!(pragma.qubit, qubit);

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaDeactivateQRydQubit to_pragma_change_device function
#[test]
fn pragma_deactivate_qryd_qubit_change() {
    let qubit = 0;
    let pragma = PragmaDeactivateQRydQubit::new(qubit);

    // Test inputs are correct
    let result = PragmaChangeDevice {
        wrapped_tags: vec![
            "Operation".to_string(),
            "PragmaOperation".to_string(),
            "PragmaDeactivateQRydQubit".to_string(),
        ],
        wrapped_hqslang: "PragmaDeactivateQRydQubit".to_string(),
        wrapped_operation: serialize(&pragma).unwrap(),
    };
    assert_eq!(pragma.to_pragma_change_device().unwrap(), result);
}

/// Test PragmaDeactivateQRydQubit standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_deactivate_qryd_qubit_simple_traits() {
    let qubit = 0;
    let pragma = PragmaDeactivateQRydQubit::new(qubit);

    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        format!("PragmaDeactivateQRydQubit {{ qubit: {:?} }}", qubit)
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaDeactivateQRydQubit::new(qubit);
    let pragma_1 = PragmaDeactivateQRydQubit::new(1);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaDeactivateQRydQubit Operate trait
#[test]
fn pragma_deactivate_qryd_qubit_operate_trait() {
    let qubit = 0;
    let pragma = PragmaDeactivateQRydQubit::new(qubit);

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaDeactivateQRydQubit"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaDeactivateQRydQubit"));

    // (3) Test is_parametrized function
    assert!(!pragma.is_parametrized());
}

/// Test PragmaDeactivateQRydQubit Substitute trait
#[test]
fn pragma_deactivate_qryd_qubit_substitute_trait() {
    let qubit = 0;
    let pragma = PragmaDeactivateQRydQubit::new(qubit);
    let pragma_test = PragmaDeactivateQRydQubit::new(qubit);

    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(0, 2);
    qubit_mapping_test.insert(2, 0);
    let result = pragma_test.remap_qubits(&qubit_mapping_test);
    assert!(result.is_err());
}

/// Test PragmaDeactivateQRydQubit Serialization and Deserialization traits (readable)
#[test]
fn pragma_deactivate_qryd_qubit_serde_readable() {
    let qubit = 0;
    let pragma_serialization = PragmaDeactivateQRydQubit::new(qubit);

    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaDeactivateQRydQubit",
                len: 1,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaDeactivateQRydQubit Serialization and Deserialization traits (compact)
#[test]
fn pragma_deactivate_qryd_qubit_serde_compact() {
    let qubit = 0;
    let pragma_serialization = PragmaDeactivateQRydQubit::new(qubit);

    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaDeactivateQRydQubit",
                len: 1,
            },
            Token::Str("qubit"),
            Token::U64(0),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaShiftQubitsTweezers inputs and involved qubits
#[test]
fn pragma_shift_qryd_qubit_tweezer_inputs_qubits() {
    let shifts: Vec<(usize, usize)> = vec![(0, 1), (3, 4)];
    let pragma = PragmaShiftQubitsTweezers::new(shifts.clone());

    // Test inputs are correct
    assert_eq!(pragma.shifts(), &shifts);

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaShiftQubitsTweezers to_pragma_change_device function
#[test]
fn pragma_shift_qryd_qubit_tweezer_change() {
    let shifts: Vec<(usize, usize)> = vec![(0, 1), (3, 4)];
    let pragma = PragmaShiftQubitsTweezers::new(shifts);

    // Test inputs are correct
    let result = PragmaChangeDevice {
        wrapped_tags: vec![
            "Operation".to_string(),
            "PragmaOperation".to_string(),
            "PragmaShiftQubitsTweezers".to_string(),
        ],
        wrapped_hqslang: "PragmaShiftQubitsTweezers".to_string(),
        wrapped_operation: serialize(&pragma).unwrap(),
    };
    assert_eq!(pragma.to_pragma_change_device().unwrap(), result);
}

/// Test PragmaShiftQubitsTweezers standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_shift_qryd_qubit_tweezer_simple_traits() {
    let shifts: Vec<(usize, usize)> = vec![(0, 1), (3, 4)];
    let pragma = PragmaShiftQubitsTweezers::new(shifts.clone());
    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        format!(
            "PragmaShiftQubitsTweezers {{ shifts: {:?} }}",
            shifts.clone()
        )
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaShiftQubitsTweezers::new(shifts.clone());
    let pragma_1 = PragmaShiftQubitsTweezers::new(vec![]);
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaShiftQubitsTweezers Operate trait
#[test]
fn pragma_shift_qryd_qubit_tweezer_operate_trait() {
    let shifts: Vec<(usize, usize)> = vec![(0, 1), (3, 4)];
    let pragma = PragmaShiftQubitsTweezers::new(shifts.clone());

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaShiftQubitsTweezers"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaShiftQubitsTweezers"));

    // (3) Test is_parametrized function
    assert!(!pragma.is_parametrized());
}

/// Test PragmaShiftQubitsTweezers Substitute trait
#[test]
fn pragma_shift_qryd_qubit_tweezer_substitute_trait() {
    let shifts: Vec<(usize, usize)> = vec![(0, 1), (3, 4)];
    let pragma = PragmaShiftQubitsTweezers::new(shifts.clone());
    let new_shifts: Vec<(usize, usize)> = vec![(0, 2), (3, 5)];
    let pragma_test = PragmaShiftQubitsTweezers::new(new_shifts.clone());
    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma.substitute_parameters(&substitution_dict).unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    qubit_mapping_test.insert(2, 1);
    qubit_mapping_test.insert(5, 4);
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);
}

/// Test PragmaShiftQubitsTweezers Serialization and Deserialization traits (readable)
#[test]
fn pragma_shift_qryd_qubit_tweezer_serde_readable() {
    let shifts: Vec<(usize, usize)> = vec![(0, 1)];
    let pragma_serialization = PragmaShiftQubitsTweezers::new(shifts.clone());
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaShiftQubitsTweezers",
                len: 1,
            },
            Token::Str("shifts"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::U64(1),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaShiftQubitsTweezers Serialization and Deserialization traits (compact)
#[test]
fn pragma_shift_qryd_qubit_tweezer_serde_compact() {
    let shifts: Vec<(usize, usize)> = vec![(0, 1)];
    let pragma_serialization = PragmaShiftQubitsTweezers::new(shifts.clone());
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaShiftQubitsTweezers",
                len: 1,
            },
            Token::Str("shifts"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::U64(0),
            Token::U64(1),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}

/// Test PragmaSwitchDeviceLayout inputs and involved qubits
#[test]
fn pragma_switch_device_layout_inputs_qubits() {
    let pragma = PragmaSwitchDeviceLayout::new("Square".to_string());

    // Test inputs are correct
    assert_eq!(pragma.new_layout(), "Square");

    // Test InvolveQubits trait
    assert_eq!(pragma.involved_qubits(), InvolvedQubits::All);
}

/// Test PragmaSwitchDeviceLayout to_pragma_change_device function
#[test]
fn pragma_switch_device_layout_change() {
    let pragma = PragmaSwitchDeviceLayout::new("Square".to_string());

    // Test inputs are correct
    let result = PragmaChangeDevice {
        wrapped_tags: vec![
            "Operation".to_string(),
            "PragmaOperation".to_string(),
            "PragmaSwitchDeviceLayout".to_string(),
        ],
        wrapped_hqslang: "PragmaSwitchDeviceLayout".to_string(),
        wrapped_operation: serialize(&pragma).unwrap(),
    };
    assert_eq!(pragma.to_pragma_change_device().unwrap(), result);
}

/// Test PragmaSwitchDeviceLayout standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_switch_device_layout_simple_traits() {
    let pragma = PragmaSwitchDeviceLayout::new("Square".to_string());
    // Test Debug trait
    assert_eq!(
        format!("{:?}", pragma),
        "PragmaSwitchDeviceLayout { new_layout: \"Square\" }"
    );

    // Test Clone trait
    assert_eq!(pragma.clone(), pragma);

    // Test PartialEq trait
    let pragma_0 = PragmaSwitchDeviceLayout::new("Square".to_string());
    let pragma_1 = PragmaSwitchDeviceLayout::new("Triangle".to_string());
    assert!(pragma_0 == pragma);
    assert!(pragma == pragma_0);
    assert!(pragma_1 != pragma);
    assert!(pragma != pragma_1);
}

/// Test PragmaSwitchDeviceLayout Operate trait
#[test]
fn pragma_switch_device_layout_operate_trait() {
    let pragma = PragmaSwitchDeviceLayout::new("Square".to_string());

    // (1) Test tags function
    let tags: &[&str; 3] = &["Operation", "PragmaOperation", "PragmaSwitchDeviceLayout"];
    assert_eq!(pragma.tags(), tags);

    // (2) Test hqslang function
    assert_eq!(pragma.hqslang(), String::from("PragmaSwitchDeviceLayout"));

    // (3) Test is_parametrized function
    assert!(!pragma.is_parametrized());
}

/// Test PragmaSwitchDeviceLayout Substitute trait
#[test]
fn pragma_switch_device_layout_substitute_trait() {
    let pragma = PragmaSwitchDeviceLayout::new("Square".to_string());
    let pragma_test = PragmaSwitchDeviceLayout::new("Square".to_string());
    // (1) Substitute parameters function
    let mut substitution_dict: Calculator = Calculator::new();
    substitution_dict.set_variable("ro", 0.0);
    let result = pragma_test
        .substitute_parameters(&substitution_dict)
        .unwrap();
    assert_eq!(result, pragma);

    // (2) Remap qubits function
    let mut qubit_mapping_test: HashMap<usize, usize> = HashMap::new();
    let result = pragma_test.remap_qubits(&qubit_mapping_test).unwrap();
    assert_eq!(result, pragma);
    qubit_mapping_test.insert(0, 2);
    let result = pragma_test.remap_qubits(&qubit_mapping_test);
    assert!(result.is_err());
}

/// Test PragmaSwitchDeviceLayout Serialization and Deserialization traits (readable)
#[test]
fn pragma_switch_device_layout_serde_readable() {
    let pragma_serialization = PragmaSwitchDeviceLayout::new("Square".to_string());
    assert_tokens(
        &pragma_serialization.readable(),
        &[
            Token::Struct {
                name: "PragmaSwitchDeviceLayout",
                len: 1,
            },
            Token::Str("new_layout"),
            Token::Str("Square"),
            Token::StructEnd,
        ],
    );
}

/// Test PragmaSwitchDeviceLayout Serialization and Deserialization traits (compact)
#[test]
fn pragma_switch_device_layout_serde_compact() {
    let pragma_serialization = PragmaSwitchDeviceLayout::new("Square".to_string());
    assert_tokens(
        &pragma_serialization.compact(),
        &[
            Token::Struct {
                name: "PragmaSwitchDeviceLayout",
                len: 1,
            },
            Token::Str("new_layout"),
            Token::Str("Square"),
            Token::StructEnd,
        ],
    );
}
