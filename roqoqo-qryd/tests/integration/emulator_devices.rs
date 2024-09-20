// Copyright © 2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use std::collections::HashMap;

use roqoqo::devices::Device;

use roqoqo_qryd::{phi_theta_relation, EmulatorDevice, PragmaDeactivateQRydQubit};
use roqoqo_qryd::{
    PragmaChangeQRydLayout, PragmaShiftQRydQubit, PragmaShiftQubitsTweezers,
    PragmaSwitchDeviceLayout,
};
/// Test EmulatorDevice new()
#[test]
fn test_new() {
    let device = EmulatorDevice::new(Some(2), None, None);

    assert!(device.internal.current_layout.is_none());
    assert!(device.internal.qubit_to_tweezer.is_none());
    assert!(device.internal.layout_register.is_none());
    assert_eq!(device.internal.seed(), Some(2));
    assert_eq!(device.internal.qrydbackend(), "qryd_tweezer_device");

    let device_emp = EmulatorDevice::new(None, None, None);

    assert_eq!(device_emp.seed(), None);
}

// Test EmulatorDevice add_qubit_tweezer_mapping(), get_tweezer_from_qubit() methods
#[test]
fn test_qubit_tweezer_mapping() {
    let mut device = EmulatorDevice::new(None, None, None);

    assert!(device.internal.qubit_to_tweezer.is_none());

    let res = device.add_qubit_tweezer_mapping(0, 0);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), vec![(0, 0)].into_iter().collect());
    assert!(device.get_tweezer_from_qubit(&0).is_ok());
    assert_eq!(device.get_tweezer_from_qubit(&0).unwrap(), 0);

    assert!(device.add_qubit_tweezer_mapping(2, 3).is_ok());

    assert_eq!(device.get_tweezer_from_qubit(&0).unwrap(), 0);
    assert_eq!(device.get_tweezer_from_qubit(&2).unwrap(), 3);
    assert!(device.get_tweezer_from_qubit(&4).is_err());

    let add_01 = device.add_qubit_tweezer_mapping(0, 1);
    assert!(add_01.is_ok());
    assert_eq!(add_01.unwrap(), vec![(0, 1), (2, 3)].into_iter().collect());
}

/// Test EmulatorDevice allow_reset field
#[test]
fn test_allow_reset() {
    let mut device = EmulatorDevice::new(None, None, None);
    assert!(!device.internal.allow_reset);
    assert!(device.set_allow_reset(true).is_ok());
    assert!(device.internal.allow_reset);
}

/// Test EmulatorDevice deactivate_qubit()
#[test]
fn test_deactivate_qubit() {
    let mut device = EmulatorDevice::new(None, None, None);

    assert!(device.deactivate_qubit(0).is_err());

    device.add_qubit_tweezer_mapping(0, 1).unwrap();

    assert!(device.deactivate_qubit(0).is_ok());
    assert!(device.deactivate_qubit(0).is_err());
}

/// Test EmulatorDevice phase_shift_controlled_...() and gate_time_controlled_...()  methods
#[test]
fn test_phi_theta_relation() {
    let mut device = EmulatorDevice::new(None, None, None);
    let device_f = EmulatorDevice::new(None, Some(2.13.to_string()), Some(2.15.to_string()));

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

    device.add_qubit_tweezer_mapping(0, 0).unwrap();
    device.add_qubit_tweezer_mapping(1, 1).unwrap();

    assert!(device
        .gate_time_controlled_z(&0, &1, device.phase_shift_controlled_z().unwrap())
        .is_some());
    assert!(device
        .gate_time_controlled_z(&0, &7, device.phase_shift_controlled_z().unwrap())
        .is_some());
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
        .is_some());
}

/// Test EmulatorDevice number_tweezer_positions()
#[test]
fn test_number_tweezer_positions() {
    let mut device = EmulatorDevice::new(None, None, None);

    assert_eq!(device.number_tweezer_positions(), 0);

    device.add_qubit_tweezer_mapping(0, 0).unwrap();
    device.add_qubit_tweezer_mapping(1, 1).unwrap();

    assert_eq!(device.number_tweezer_positions(), 2)
}

/// Test EmulatorDevice ..._qubit_gate_time() methods
#[test]
fn test_qubit_times() {
    let device = EmulatorDevice::new(None, None, None);

    assert!(device.single_qubit_gate_time("RotateX", &0).is_some());
    assert!(device
        .two_qubit_gate_time("PhaseShiftedControlledPhase", &0, &7)
        .is_some());
    assert!(device
        .three_qubit_gate_time("ControlledControlledPhaseShift", &12, &1, &3)
        .is_some());
    assert!(device
        .multi_qubit_gate_time("MultiQubitZZ", &[6, 2, 3, 4])
        .is_some());
}

/// Test EmulatorDevice number_qubits() method
#[test]
fn test_number_qubits() {
    let mut device = EmulatorDevice::new(None, None, None);

    assert_eq!(device.number_qubits(), 0);

    device.add_qubit_tweezer_mapping(0, 0).unwrap();
    device.add_qubit_tweezer_mapping(1, 1).unwrap();

    assert_eq!(device.number_qubits(), 2)
}

/// Test EmulatorDevice change_device() method errors
#[test]
fn test_change_device_errors() {
    let mut device = EmulatorDevice::new(None, None, None);

    let pragma_change = PragmaChangeQRydLayout::new(0);
    let pragma_shift = PragmaSwitchDeviceLayout::new("error".to_string());
    let hm: HashMap<usize, (usize, usize)> = [(0, (1, 2))].into_iter().collect();
    let pragma_old_s = PragmaShiftQRydQubit::new(hm);

    assert!(device.change_device("Error", &Vec::<u8>::new()).is_err());
    assert!(device
        .change_device("PragmaChangeQRydLayout", &Vec::<u8>::new())
        .is_err());
    assert!(device
        .change_device(
            "PragmaChangeQRydLayout",
            &serialize(&pragma_change).unwrap()
        )
        .is_err());
    assert!(device
        .change_device(
            "PragmaSwitchDeviceLayout",
            &serialize(&pragma_shift).unwrap()
        )
        .is_err());

    assert!(device
        .change_device("PragmaShiftQRydQubit", &serialize(&pragma_old_s).unwrap())
        .is_err());
}

/// Test EmulatorDevice change_device() method
#[test]
fn test_change_device() {
    let mut device = EmulatorDevice::new(None, None, None);

    let pr_shift = PragmaShiftQubitsTweezers::new(vec![(2, 3)]);
    assert!(device
        .change_device("PragmaShiftQubitsTweezers", &serialize(&pr_shift).unwrap())
        .unwrap_err()
        .to_string()
        .contains("The device qubit -> tweezer mapping is empty"));

    device.add_qubit_tweezer_mapping(0, 0).unwrap();
    device.add_qubit_tweezer_mapping(1, 1).unwrap();
    device.add_qubit_tweezer_mapping(2, 2).unwrap();
    device.add_qubit_tweezer_mapping(3, 3).unwrap();

    let pr_deac = PragmaDeactivateQRydQubit::new(3);
    assert!(device
        .internal
        .qubit_to_tweezer
        .as_ref()
        .unwrap()
        .contains_key(&3));
    assert!(device
        .change_device("PragmaDeactivateQRydQubit", &serialize(&pr_deac).unwrap())
        .is_ok());
    assert!(!device
        .internal
        .qubit_to_tweezer
        .as_ref()
        .unwrap()
        .contains_key(&3));

    assert!(device
        .change_device("PragmaShiftQubitsTweezers", &serialize(&pr_shift).unwrap())
        .is_ok());
    assert_eq!(
        device.internal.qubit_to_tweezer.as_ref().unwrap().get(&0),
        Some(&0)
    );
    assert_eq!(
        device.internal.qubit_to_tweezer.as_ref().unwrap().get(&1),
        Some(&1)
    );
    assert_eq!(
        device.internal.qubit_to_tweezer.as_ref().unwrap().get(&2),
        Some(&3)
    );
}

/// Test EmulatorDevice to_generic_device() method
#[test]
fn test_to_generic_device() {
    let mut device = EmulatorDevice::new(None, None, None);
    device.add_qubit_tweezer_mapping(0, 0).unwrap();
    device.add_qubit_tweezer_mapping(1, 1).unwrap();
    device.add_qubit_tweezer_mapping(2, 2).unwrap();
    device.add_qubit_tweezer_mapping(3, 3).unwrap();

    let generic_device = device.to_generic_device();

    assert_eq!(
        generic_device
            .single_qubit_gates
            .get("RotateX")
            .unwrap()
            .get(&0)
            .unwrap(),
        &1.0
    );
    assert_eq!(
        generic_device
            .single_qubit_gates
            .get("RotateZ")
            .unwrap()
            .get(&1)
            .unwrap(),
        &1.0
    );
    assert_eq!(
        generic_device
            .two_qubit_gates
            .get("PhaseShiftedControlledPhase")
            .unwrap()
            .get(&(2, 3))
            .unwrap(),
        &1.0
    );
    assert_eq!(
        generic_device
            .two_qubit_gates
            .get("PhaseShiftedControlledZ")
            .unwrap()
            .get(&(1, 2))
            .unwrap(),
        &1.0
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
