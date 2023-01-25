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
use ndarray::{array, Array2};
use roqoqo::devices::Device;
use roqoqo::RoqoqoBackendError;
use roqoqo_qryd::{
    phi_theta_relation,
    pragma_operations::{PragmaChangeQRydLayout, PragmaShiftQRydQubit},
    qryd_devices::{FirstDevice, QRydDevice},
};
// use serde_test::{assert_tokens, Configure, Token};
use std::collections::HashMap;
use std::convert::From;

fn create_simple_qubit_positions(
    pairs: &[(usize, (usize, usize))],
) -> HashMap<usize, (usize, usize)> {
    let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
    for (key, val) in pairs.iter() {
        map.insert(*key, *val);
    }
    map
}

#[test]
fn test_new_no_errors() {
    let device = FirstDevice::new(1, 1, &[1], 0.0, array![[0.0,],], None, None).unwrap();
    assert_eq!(device.number_rows(), 1);
    assert_eq!(device.number_columns(), 1);
    assert_eq!(device.number_qubits(), 1);
    assert_eq!(
        device.qubit_positions(),
        &create_simple_qubit_positions(&[(0_usize, (0_usize, 0_usize))])
    );

    let qryd_device: QRydDevice = QRydDevice::from(&device);
    assert!(qryd_device == QRydDevice::FirstDevice(device));
}

#[test]
fn test_new_error_rows() {
    let device = FirstDevice::new(
        1,
        1,
        &[2, 1],
        0.0,
        array![[0.0, 1.0], [0.0, 1.0]],
        None,
        None,
    );

    assert!(device.is_err());
    assert_eq!(
        device,
        Err(RoqoqoBackendError::GenericError {
            msg: "Device has 1 rows but for 2 rows qubit numbers have been specified".to_string()
        })
    );
}

#[test]
fn test_new_error_columns() {
    let device = FirstDevice::new(
        2,
        1,
        &[2, 1],
        0.0,
        array![[0.0, 1.0], [0.0, 1.0]],
        None,
        None,
    );

    assert!(device.is_err());
    assert_eq!(
        device,
        Err(RoqoqoBackendError::GenericError {
            msg: "Device has 1 columns but for column 0, 2 qubit numbers have been specified"
                .to_string()
        })
    );
}

#[test]
fn test_new_large() {
    let device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();

    let qubits = [
        (0_usize, (0_usize, 0_usize)),
        (1_usize, (0_usize, 1_usize)),
        (2_usize, (0_usize, 2_usize)),
        (3_usize, (1_usize, 0_usize)),
        (4_usize, (1_usize, 1_usize)),
    ];

    assert_eq!(device.number_rows(), 2);
    assert_eq!(device.number_columns(), 3);
    assert_eq!(device.number_qubits(), 5);
    assert_eq!(
        device.qubit_positions(),
        &create_simple_qubit_positions(&qubits)
    );

    let qryd_device: QRydDevice = QRydDevice::from(&device);
    assert!(qryd_device == QRydDevice::FirstDevice(device));
}

#[test]
fn test_phi_theta_relation() {
    let mut device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    device = device
        .add_layout(1, array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]])
        .unwrap();
    device.switch_layout(&1).unwrap();

    assert_eq!(
        device.phase_shift_controlled_z().unwrap(),
        phi_theta_relation("DefaultRelation", std::f64::consts::PI).unwrap()
    );
    assert_eq!(
        device.phase_shift_controlled_phase(1.2).unwrap(),
        phi_theta_relation("DefaultRelation", 1.2).unwrap()
    );

    assert!(device.gate_time_controlled_z(&0, &1, 1.4).is_none());
    assert!(device
        .gate_time_controlled_phase(&0, &1, 1.4, 2.4)
        .is_none());
    assert!(device.gate_time_controlled_z(&0, &7, 1.4).is_none());
    assert!(device
        .gate_time_controlled_phase(&0, &7, 1.4, 2.3)
        .is_none());
    assert!(device.gate_time_controlled_z(&0, &1, device.phase_shift_controlled_z().unwrap()).is_some());
    assert!(device.gate_time_controlled_z(&0, &7, device.phase_shift_controlled_z().unwrap()).is_none());
    assert!(device.gate_time_controlled_phase(&0, &1, device.phase_shift_controlled_phase(0.1).unwrap(), 0.1).is_some());
    assert!(device.gate_time_controlled_phase(&0, &7, device.phase_shift_controlled_phase(0.1).unwrap(), 0.1).is_none());

    let d = QRydDevice::FirstDevice(device);
    assert_eq!(
        d.phase_shift_controlled_z().unwrap(),
        phi_theta_relation("DefaultRelation", std::f64::consts::PI).unwrap()
    );
    assert_eq!(
        d.phase_shift_controlled_phase(1.2).unwrap(),
        phi_theta_relation("DefaultRelation", 1.2).unwrap()
    );
}

#[test]
fn test_add_layout() {
    let device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    let new_layout: Array2<f64> = array![[0.5, 0.5, 0.5], [0.4, 0.4, 0.3]];
    let updated_device = device.add_layout(1, new_layout).unwrap();

    let qubits = [
        (0_usize, (0_usize, 0_usize)),
        (1_usize, (0_usize, 1_usize)),
        (2_usize, (0_usize, 2_usize)),
        (3_usize, (1_usize, 0_usize)),
        (4_usize, (1_usize, 1_usize)),
    ];

    assert_eq!(updated_device.number_rows(), 2);
    assert_eq!(updated_device.number_columns(), 3);
    assert_eq!(updated_device.number_qubits(), 5);
    assert_eq!(
        updated_device.qubit_positions(),
        &create_simple_qubit_positions(&qubits)
    );

    let qryd_device: QRydDevice = QRydDevice::from(&updated_device);
    assert!(qryd_device == QRydDevice::FirstDevice(updated_device));
}

#[test]
fn test_add_layout_error_key() {
    let device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    let new_layout: Array2<f64> = array![[0.5, 0.5, 0.5], [0.4, 0.4, 0.3]];
    let updated_device = device.add_layout(1, new_layout.clone()).unwrap();
    let error_device = updated_device.add_layout(1, new_layout.clone());

    assert!(error_device.is_err());
    assert_eq!(
        error_device,
        Err(RoqoqoBackendError::GenericError {
            msg: format!(
                "Error adding layout to QRyd device layout key 1 is already used for layout {:?}",
                Some(new_layout)
            )
        })
    );
}

#[test]
fn test_add_layout_error_rows() {
    let device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    let new_layout: Array2<f64> = array![[0.5, 0.5, 0.5], [0.4, 0.4, 0.4], [0.3, 0.3, 0.3]];
    let error_device = device.add_layout(1, new_layout);

    assert!(error_device.is_err());
    assert_eq!(
        error_device,
        Err(RoqoqoBackendError::GenericError {
            msg: "Error adding layout to QRyd device new layout 2 rows and 3 columns required"
                .to_string()
        })
    );
}

#[test]
fn test_add_layout_error_columns() {
    let device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    let new_layout: Array2<f64> = array![[0.5, 0.5], [0.4, 0.4]];
    let error_device = device.add_layout(1, new_layout);

    assert!(error_device.is_err());
    assert_eq!(
        error_device,
        Err(RoqoqoBackendError::GenericError {
            msg: "Error adding layout to QRyd device new layout 2 rows and 3 columns required"
                .to_string()
        })
    );
}

#[test]
fn test_switch_layout() {
    let device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    let qryd_device: QRydDevice = QRydDevice::from(&device);

    let new_layout: Array2<f64> = array![[0.5, 0.5, 0.5], [0.4, 0.4, 0.3]];
    let mut updated_device = device.add_layout(1, new_layout.clone()).unwrap();
    let _ = updated_device.switch_layout(&1);

    let mut updated_qryd_device = qryd_device.add_layout(1, new_layout).unwrap();
    let _ = updated_qryd_device.switch_layout(&1);

    let qubits = [
        (0_usize, (0_usize, 0_usize)),
        (1_usize, (0_usize, 1_usize)),
        (2_usize, (0_usize, 2_usize)),
        (3_usize, (1_usize, 0_usize)),
        (4_usize, (1_usize, 1_usize)),
    ];

    assert_eq!(updated_device.number_rows(), 2);
    assert_eq!(updated_device.number_columns(), 3);
    assert_eq!(updated_device.number_qubits(), 5);
    assert_eq!(
        updated_device.qubit_positions(),
        &create_simple_qubit_positions(&qubits)
    );

    assert!(updated_qryd_device == QRydDevice::FirstDevice(updated_device));

    assert_eq!(updated_qryd_device.number_rows(), 2);
    assert_eq!(updated_qryd_device.number_columns(), 3);
    assert_eq!(updated_qryd_device.number_qubits(), 5);
    assert_eq!(
        updated_qryd_device.qubit_positions(),
        &create_simple_qubit_positions(&qubits)
    );
}

#[test]
fn test_switch_layout_error() {
    let mut device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    let error_device = device.switch_layout(&2);

    assert!(error_device.is_err());
    assert_eq!(
        error_device,
        Err(RoqoqoBackendError::GenericError {
            msg: "Error switching layout of QRyd device. Layout 2 is not set".to_string()
        })
    );

    let new_layout: Array2<f64> = array![[0.5, 0.5, 0.5], [0.4, 0.4, 0.3]];
    let mut updated_device = device.add_layout(1, new_layout).unwrap();
    let error_device = updated_device.switch_layout(&2);

    assert!(error_device.is_err());
    assert_eq!(
        error_device,
        Err(RoqoqoBackendError::GenericError {
            msg: "Error switching layout of QRyd device. Layout 2 is not set".to_string()
        })
    );
}

#[test]
fn test_change_qubit_positions() {
    let mut device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    let mut qryd_device: QRydDevice = QRydDevice::from(&device);
    let qubits = [
        (2_usize, (0_usize, 0_usize)),
        (0_usize, (0_usize, 1_usize)),
        (1_usize, (0_usize, 2_usize)),
        (4_usize, (1_usize, 0_usize)),
        (3_usize, (1_usize, 1_usize)),
    ];
    let new_pos = create_simple_qubit_positions(&qubits);
    device.change_qubit_positions(&new_pos).unwrap();
    qryd_device.change_qubit_positions(&new_pos).unwrap();

    assert_eq!(device.number_rows(), 2);
    assert_eq!(device.number_columns(), 3);
    assert_eq!(device.number_qubits(), 5);
    assert_eq!(device.qubit_positions(), &new_pos);

    assert!(qryd_device == QRydDevice::FirstDevice(device));
}

#[test]
fn test_change_qubit_positions_error_qubit() {
    let mut device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    let qubits = [
        (2_usize, (0_usize, 0_usize)),
        // (0_usize, (0_usize, 1_usize)),
        (1_usize, (0_usize, 2_usize)),
        (4_usize, (1_usize, 0_usize)),
        (3_usize, (1_usize, 1_usize)),
    ];
    let error_pos = create_simple_qubit_positions(&qubits);
    let error_device = device.change_qubit_positions(&error_pos);

    assert!(error_device.is_err());
    assert_eq!(
        error_device,
        Err(RoqoqoBackendError::GenericError {
            msg: "Qubit 0 is missing from new qubit positions".to_string()
        })
    );
}

#[test]
fn test_change_qubit_positions_error_row() {
    let mut device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    let qubits = [
        (0_usize, (0_usize, 0_usize)),
        (2_usize, (0_usize, 2_usize)),
        (3_usize, (1_usize, 0_usize)),
        (4_usize, (1_usize, 1_usize)),
        (1_usize, (1_usize, 2_usize)),
    ];
    let error_pos = create_simple_qubit_positions(&qubits);
    let error_device = device.change_qubit_positions(&error_pos);

    assert!(error_device.is_err());
    assert_eq!(
        error_device,
        Err(RoqoqoBackendError::GenericError {
            msg: "New qubit positions has a mismatch in rows for qubit 1 old row 0 new row 1"
                .to_string()
        })
    );
}

#[test]
fn test_change_qubit_positions_error_extra_qubits() {
    let mut device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    let qubits = [
        (2_usize, (0_usize, 0_usize)),
        (0_usize, (0_usize, 1_usize)),
        (1_usize, (0_usize, 2_usize)),
        (4_usize, (1_usize, 0_usize)),
        (3_usize, (1_usize, 1_usize)),
        (5_usize, (2_usize, 0_usize)),
    ];
    let error_pos = create_simple_qubit_positions(&qubits);
    let error_device = device.change_qubit_positions(&error_pos);

    assert!(error_device.is_err());
    assert_eq!(
        error_device,
        Err(RoqoqoBackendError::GenericError {
            msg: "There are additional keys in the new_positions input which do not exist in the old qubit positions"
                .to_string()
        })
    );
}

#[test]
fn test_qubit_gate_times() {
    let mut device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    device = device
        .add_layout(1, array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]])
        .unwrap();
    device.switch_layout(&1).unwrap();

    assert_eq!(device.single_qubit_gate_time("RotateX", &7), None);
    assert_eq!(device.single_qubit_gate_time("PhaseShiftState0", &0), None);
    assert_eq!(
        device.single_qubit_gate_time("PhaseShiftState1", &0),
        Some(1e-6)
    );
    assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(1e-6));
    assert_eq!(device.single_qubit_gate_time("RotateY", &0), Some(1e-6));
    assert_eq!(device.single_qubit_gate_time("RotateXY", &0), Some(1e-6));

    assert_eq!(device.single_qubit_gate_time("PauliY", &0), None);
    assert_eq!(device.single_qubit_gate_time("PauliX", &0), None);

    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &7, &1),
        None
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &7),
        None
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &2),
        None
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledPhase", &0, &7),
        None
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledPhase", &0, &1),
        Some(2e-6)
    );
    assert_eq!(device.two_qubit_gate_time("ControlledPauliZ", &0, &1), None);

    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMSXX", &[0, 1, 2]),
        None
    );
    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMSZZ", &[0, 1, 2]),
        None
    );
    assert_eq!(
        device.qubit_decoherence_rates(&0),
        Some(Array2::zeros((3, 3).to_owned()))
    );
    assert_eq!(device.number_qubits(), 5);
    assert_eq!(device.two_qubit_edges(), vec![(0, 1), (1, 2), (3, 4)]);
}

/// Test the change_device function
#[test]
fn test_change_device() {
    let mut device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    device = device
        .add_layout(1, array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]])
        .unwrap();
    device.switch_layout(&1).unwrap();

    let pragma_c = PragmaChangeQRydLayout::new(1);
    let mut new_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    new_positions.insert(1, (0, 0));
    new_positions.insert(0, (0, 1));
    new_positions.insert(2, (0, 1));
    new_positions.insert(3, (1, 2));
    new_positions.insert(4, (1, 1));
    let pragma_s = PragmaShiftQRydQubit::new(new_positions);
    let operation: Vec<u8> = Vec::new();

    // PragmaChangeQRydLayout
    assert_eq!(
        device.change_device("PragmaChangeQRydLayout", &serialize(&pragma_c).unwrap()),
        Ok(())
    );
    assert_eq!(
        device.change_device("PragmaChangeQRydLayout", &operation),
        Err(RoqoqoBackendError::GenericError {
            msg: "Wrapped operation not supported in QRydDevice".to_string()
        })
    );
    assert_eq!(
        device.change_device("PragmaShiftQRydQubit", &serialize(&pragma_s).unwrap()),
        Ok(())
    );
    assert_eq!(
        device.change_device("PragmaShiftQRydQubit", &operation),
        Err(RoqoqoBackendError::GenericError {
            msg: "Wrapped operation not supported in QRydDevice".to_string()
        })
    );
    assert_eq!(
        device.change_device("Other", &serialize(&pragma_c).unwrap()),
        Err(RoqoqoBackendError::GenericError {
            msg: "Wrapped operation not supported in QRydDevice".to_string()
        })
    );
}

#[test]
fn test_qubit_edges() {
    let mut device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.0,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    device = device
        .add_layout(1, array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]])
        .unwrap();
    device.switch_layout(&1).unwrap();

    let edges = vec![(0, 1), (0, 3), (1, 2), (1, 4), (3, 4)];

    assert_eq!(device.two_qubit_edges(), edges);
}

#[test]
fn test_qubit_gate_times_with_layout() {
    let device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    let new_layout: Array2<f64> = array![[0.2, 5.5, 5.6], [0.1, 0.2, 0.3]];
    let mut updated_device = device.add_layout(1, new_layout).unwrap();
    let _ = updated_device.switch_layout(&1);
    updated_device.set_cutoff(2.1);

    assert_eq!(updated_device.single_qubit_gate_time("RotateX", &7), None);
    assert_eq!(
        updated_device.single_qubit_gate_time("PhaseShiftState0", &0),
        None
    );
    assert_eq!(
        updated_device.single_qubit_gate_time("PhaseShiftState1", &0),
        Some(1e-6)
    );
    assert_eq!(
        updated_device.single_qubit_gate_time("RotateX", &0),
        Some(1e-6)
    );
    assert_eq!(
        updated_device.single_qubit_gate_time("RotateY", &0),
        Some(1e-6)
    );
    assert_eq!(updated_device.single_qubit_gate_time("PauliY", &0), None);
    assert_eq!(
        updated_device.single_qubit_gate_time("RotateXY", &0),
        Some(1e-6)
    );

    assert_eq!(
        updated_device.two_qubit_gate_time("PhaseShiftedControlledZ", &7, &1),
        None
    );
    assert_eq!(
        updated_device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &7),
        None
    );
    assert_eq!(
        updated_device.two_qubit_gate_time("ControlledPauliZ", &0, &2),
        None
    );
    assert!(
        (updated_device
            .two_qubit_gate_time("PhaseShiftedControlledZ", &3, &4)
            .unwrap()
            - 2e-8)
            .abs()
            < 1e-9
    );
    assert_eq!(
        updated_device.two_qubit_gate_time("PhaseShiftedControlledZ", &1, &4),
        None
    ); // should be larger than cut-off
    assert_eq!(
        updated_device.multi_qubit_gate_time("MultiQubitMSXX", &[0, 1, 2]),
        None
    );
    assert_eq!(
        updated_device.qubit_decoherence_rates(&0),
        Some(Array2::zeros((3, 3).to_owned()))
    );

    // Repeat with QRydDevice:
    let qryd_device = QRydDevice::from(&updated_device);
    assert_eq!(qryd_device.single_qubit_gate_time("RotateX", &7), None);
    assert_eq!(
        qryd_device.single_qubit_gate_time("PhaseShiftState0", &0),
        None
    );
    assert_eq!(
        qryd_device.single_qubit_gate_time("PhaseShiftState1", &0),
        Some(1e-6)
    );
    assert_eq!(
        qryd_device.single_qubit_gate_time("RotateX", &0),
        Some(1e-6)
    );
    assert_eq!(
        qryd_device.single_qubit_gate_time("RotateY", &0),
        Some(1e-6)
    );

    assert_eq!(
        qryd_device.two_qubit_gate_time("PhaseShiftedControlledZ", &7, &1),
        None
    );
    assert_eq!(
        qryd_device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &7),
        None
    );
    assert_eq!(
        qryd_device.two_qubit_gate_time("ControlledPauliZ", &0, &1),
        None
    );
    assert!(
        (updated_device
            .two_qubit_gate_time("PhaseShiftedControlledZ", &0, &3)
            .unwrap()
            - 4.52e-6)
            .abs()
            < 1e-9
    );
    assert!(
        (updated_device
            .two_qubit_gate_time("PhaseShiftedControlledZ", &3, &4)
            .unwrap()
            - 2e-8)
            .abs()
            < 1e-9
    );
    assert_eq!(
        qryd_device.two_qubit_gate_time("PhaseShiftedControlledZ", &1, &4),
        None
    ); // should be larger than cut-off
       //
    assert_eq!(
        qryd_device.multi_qubit_gate_time("MultiQubitMSXX", &[0, 1, 2]),
        None
    );
    // MultiQubitZZ available but only for qubits from one row
    assert_eq!(
        qryd_device.multi_qubit_gate_time("MultiQubitZZ", &[0, 1, 4]),
        None
    );
    // MultiQubitZZ available but not for qubit outside of device
    assert_eq!(
        qryd_device.multi_qubit_gate_time("MultiQubitZZ", &[0, 1, 10]),
        None
    );
    // MultiQubitZZ available for all qubits in one row
    assert_eq!(
        qryd_device.multi_qubit_gate_time("MultiQubitZZ", &[0, 1]),
        Some(2e-5)
    );
    assert_eq!(
        qryd_device.qubit_decoherence_rates(&0),
        Some(Array2::zeros((3, 3).to_owned()))
    );
}

#[test]
fn test_traits_firstdevice() {
    let device = FirstDevice::new(1, 1, &[1], 0.0, array![[0.0,],], None, None).unwrap();
    let device_1 = FirstDevice::new(1, 1, &[1], 0.0, array![[0.0,],], None, None).unwrap();
    let device_2 = FirstDevice::new(1, 2, &[1], 0.0, array![[0.0, 1.0],], None, None).unwrap();

    assert!(device == device_1);
    assert!(device_1 == device);
    assert!(device != device_2);
    assert!(device_2 != device);
    assert!(device == device.clone());
    assert!(device == device);

    let qubits = create_simple_qubit_positions(&[(0_usize, (0_usize, 0_usize))]);
    assert_eq!(
        format!("{:?}", device),
        format!("FirstDevice {{ number_rows: 1, number_columns: 1, qubit_positions: {:?}, row_distance: 0.0, layout_register: {{0: [[0.0]], shape=[1, 1], strides=[1, 1], layout=CFcf (0xf), const ndim=2}}, current_layout: 0, cutoff: 1.0, controlled_z_phase_relation: \"DefaultRelation\", controlled_phase_phase_relation: \"DefaultRelation\" }}", qubits) 
    );
}

#[test]
fn test_traits_qryddevice() {
    let device = FirstDevice::new(1, 1, &[1], 0.0, array![[0.0,],], None, None).unwrap();
    let device_1 = FirstDevice::new(1, 1, &[1], 0.0, array![[0.0,],], None, None).unwrap();
    let device_2 = FirstDevice::new(1, 2, &[1], 0.0, array![[0.0, 1.0],], None, None).unwrap();
    let qryd_device: QRydDevice = QRydDevice::from(&device);
    let qryd_device_1: QRydDevice = QRydDevice::from(&device_1);
    let qryd_device_2: QRydDevice = QRydDevice::from(&device_2);

    assert!(qryd_device == QRydDevice::FirstDevice(device.clone()));
    assert!(QRydDevice::FirstDevice(device) == qryd_device);
    assert!(qryd_device == qryd_device_1);
    assert!(qryd_device_1 == qryd_device);
    assert!(qryd_device != qryd_device_2);
    assert!(qryd_device_2 != qryd_device);
    assert!(qryd_device == qryd_device.clone());
    assert!(qryd_device == qryd_device);

    let qubits = create_simple_qubit_positions(&[(0_usize, (0_usize, 0_usize))]);
    assert_eq!(
        format!("{:?}", qryd_device),
        format!("FirstDevice(FirstDevice {{ number_rows: 1, number_columns: 1, qubit_positions: {:?}, row_distance: 0.0, layout_register: {{0: [[0.0]], shape=[1, 1], strides=[1, 1], layout=CFcf (0xf), const ndim=2}}, current_layout: 0, cutoff: 1.0, controlled_z_phase_relation: \"DefaultRelation\", controlled_phase_phase_relation: \"DefaultRelation\" }})", qubits) 
    );
}

#[test]
fn test_qryd_qubit_gate_times() {
    let mut device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    device = device
        .add_layout(1, array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]])
        .unwrap();
    device.switch_layout(&1).unwrap();
    let qryd_device: QRydDevice = QRydDevice::from(&device);

    assert_eq!(qryd_device.single_qubit_gate_time("RotateX", &7), None);
    assert_eq!(
        qryd_device.single_qubit_gate_time("PhaseShiftState0", &0),
        None
    );
    assert_eq!(
        qryd_device.single_qubit_gate_time("PhaseShiftState1", &0),
        Some(1e-6)
    );
    assert_eq!(
        qryd_device.single_qubit_gate_time("RotateY", &0),
        Some(1e-6)
    );
    assert_eq!(
        qryd_device.single_qubit_gate_time("RotateXY", &0),
        Some(1e-6)
    );
    assert_eq!(
        qryd_device.single_qubit_gate_time("RotateX", &0),
        Some(1e-6)
    );
    assert_eq!(
        qryd_device.two_qubit_gate_time("PhaseShiftedControlledZ", &7, &1),
        None
    );
    assert_eq!(
        qryd_device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &7),
        None
    );
    assert_eq!(
        qryd_device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &2),
        None
    );
    assert_eq!(
        qryd_device.two_qubit_gate_time("ControlledPauliZ", &0, &1),
        None
    );

    assert_eq!(
        qryd_device.multi_qubit_gate_time("MultiQubitMSXX", &[0, 1, 2]),
        None
    );
    assert_eq!(
        qryd_device.multi_qubit_gate_time("MultiQubitMSZZ", &[0, 1, 2]),
        None
    );
    assert_eq!(
        qryd_device.qubit_decoherence_rates(&0),
        Some(Array2::zeros((3, 3).to_owned()))
    );
    assert_eq!(qryd_device.number_qubits(), 5);
    assert_eq!(qryd_device.two_qubit_edges(), vec![(0, 1), (1, 2), (3, 4)]);
}

/// Test the change_device function
#[test]
fn test_qryd_change_device() {
    let mut device = FirstDevice::new(
        2,
        3,
        &[3, 2],
        1.5,
        array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
        None,
        None,
    )
    .unwrap();
    device = device
        .add_layout(1, array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]])
        .unwrap();
    device.switch_layout(&1).unwrap();
    let mut qryd_device: QRydDevice = QRydDevice::from(&device);

    let pragma_c = PragmaChangeQRydLayout::new(1);
    let mut new_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    new_positions.insert(1, (0, 0));
    new_positions.insert(0, (0, 1));
    new_positions.insert(2, (0, 1));
    new_positions.insert(3, (1, 2));
    new_positions.insert(4, (1, 1));
    let pragma_s = PragmaShiftQRydQubit::new(new_positions);
    let operation: Vec<u8> = Vec::new();

    // PragmaChangeQRydLayout
    assert_eq!(
        qryd_device.change_device("PragmaChangeQRydLayout", &serialize(&pragma_c).unwrap()),
        Ok(())
    );
    assert_eq!(
        qryd_device.change_device("PragmaChangeQRydLayout", &operation),
        Err(RoqoqoBackendError::GenericError {
            msg: "Wrapped operation not supported in QRydDevice".to_string()
        })
    );
    assert_eq!(
        qryd_device.change_device("PragmaShiftQRydQubit", &serialize(&pragma_s).unwrap()),
        Ok(())
    );
    assert_eq!(
        qryd_device.change_device("PragmaShiftQRydQubit", &operation),
        Err(RoqoqoBackendError::GenericError {
            msg: "Wrapped operation not supported in QRydDevice".to_string()
        })
    );
    assert_eq!(
        qryd_device.change_device("Other", &serialize(&pragma_c).unwrap()),
        Err(RoqoqoBackendError::GenericError {
            msg: "Wrapped operation not supported in QRydDevice".to_string()
        })
    );
}

// #[test]
// fn test_pscp_phi_theta_relation() {
//     let correct_phi: f64 = 3.6150744773365036;
//     let correct_device = FirstDevice::new(
//         2,
//         3,
//         &[3, 2],
//         1.5,
//         array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
//         Some(correct_phi),
//     )
//     .unwrap();
//     let incorrect_device = FirstDevice::new(
//         2,
//         3,
//         &[3, 2],
//         1.5,
//         array![[0.0, 1.0, 2.0], [0.0, 1.0, 2.0]],
//         None,
//     )
//     .unwrap();

//     assert!(correct_device
//         .two_qubit_gate_time("PhaseShiftedControlledPhase", &0, &1)
//         .is_some());
//     assert!(incorrect_device
//         .two_qubit_gate_time("PhaseShiftedControlledPhase", &0, &1)
//         .is_none());
// }

// /// Test FirstDevice Serialization and Deserialization traits (readable)
// #[cfg(feature = "serialize")]
// #[test]
// fn serde_readable_first_device() {
//     let device = FirstDevice::new(1, 1, &[1], 0.0, array![[0.0,],]).unwrap();

//     assert_tokens(
//         &device.readable(),
//         &[
//             Token::Struct {
//                 name: "FirstDevice",
//                 len: 7,
//             },
//             Token::Str("number_rows"),
//             Token::U64(1),
//             Token::Str("number_columns"),
//             Token::U64(1),
//             Token::Str("qubit_positions"),
//             Token::Map { len: Some(1) },
//             Token::U64(0),
//             Token::Tuple { len: 2 },
//             Token::U64(0),
//             Token::U64(0),
//             Token::TupleEnd,
//             Token::MapEnd,
//             Token::Str("row_distance"),
//             Token::F64(0.0),
//             Token::Str("layout_register"),
//             Token::Map { len: Some(1) },
//             Token::MapEnd,
//             Token::Str("current_layout"),
//             Token::None,
//             Token::Str("cutoff"),
//             Token::F64(1.0),
//             Token::StructEnd,
//         ],
//     );
// }

// /// Test FirstDevice Serialization and Deserialization traits (compact)
// #[cfg(feature = "serialize")]
// #[test]
// fn serde_compact_first_device() {
//     let device = FirstDevice::new(1, 1, &[1], 0.0, array![[0.0],]).unwrap();

//     assert_tokens(
//         &device.compact(),
//         &[
//             Token::Struct {
//                 name: "FirstDevice",
//                 len: 7,
//             },
//             Token::Str("number_rows"),
//             Token::U64(1),
//             Token::Str("number_columns"),
//             Token::U64(1),
//             Token::Str("qubit_positions"),
//             Token::Map { len: Some(1) },
//             Token::U64(0),
//             Token::Tuple { len: 2 },
//             Token::U64(0),
//             Token::U64(0),
//             Token::TupleEnd,
//             Token::MapEnd,
//             Token::Str("row_distance"),
//             Token::F64(0.0),
//             Token::Str("layout_register"),
//             Token::Map { len: Some(0) },
//             Token::MapEnd,
//             Token::Str("current_layout"),
//             Token::None,
//             Token::Str("cutoff"),
//             Token::F64(1.0),
//             Token::StructEnd,
//         ],
//     );
// }

// /// Test QRydDevice Serialization and Deserialization traits (readable)
// #[cfg(feature = "serialize")]
// #[test]
// fn serde_readable_qryd_device() {
//     let first_device = FirstDevice::new(1, 1, &[1], 0.0, array![[0.0,],]).unwrap();
//     let device = QRydDevice::from(&first_device);

//     assert_tokens(
//         &device.readable(),
//         &[
//             Token::NewtypeVariant {
//                 name: "QRydDevice",
//                 variant: "FirstDevice",
//             },
//             Token::Struct {
//                 name: "FirstDevice",
//                 len: 7,
//             },
//             Token::Str("number_rows"),
//             Token::U64(1),
//             Token::Str("number_columns"),
//             Token::U64(1),
//             Token::Str("qubit_positions"),
//             Token::Map { len: Some(1) },
//             Token::U64(0),
//             Token::Tuple { len: 2 },
//             Token::U64(0),
//             Token::U64(0),
//             Token::TupleEnd,
//             Token::MapEnd,
//             Token::Str("row_distance"),
//             Token::F64(0.0),
//             Token::Str("layout_register"),
//             Token::Map { len: Some(0) },
//             Token::MapEnd,
//             Token::Str("current_layout"),
//             Token::None,
//             Token::Str("cutoff"),
//             Token::F64(1.0),
//             Token::StructEnd,
//         ],
//     );
// }

// /// Test QRydDevice Serialization and Deserialization traits (compact)
// #[cfg(feature = "serialize")]
// #[test]
// fn serde_compact_qryd_device() {
//     let first_device = FirstDevice::new(1, 1, &[1], 0.0, array![[0.0,],]).unwrap();
//     let device = QRydDevice::from(&first_device);

//     assert_tokens(
//         &device.compact(),
//         &[
//             Token::NewtypeVariant {
//                 name: "QRydDevice",
//                 variant: "FirstDevice",
//             },
//             Token::Struct {
//                 name: "FirstDevice",
//                 len: 7,
//             },
//             Token::Str("number_rows"),
//             Token::U64(1),
//             Token::Str("number_columns"),
//             Token::U64(1),
//             Token::Str("qubit_positions"),
//             Token::Map { len: Some(1) },
//             Token::U64(0),
//             Token::Tuple { len: 2 },
//             Token::U64(0),
//             Token::U64(0),
//             Token::TupleEnd,
//             Token::MapEnd,
//             Token::Str("row_distance"),
//             Token::F64(0.0),
//             Token::Str("layout_register"),
//             Token::Map { len: Some(0) },
//             Token::MapEnd,
//             Token::Str("current_layout"),
//             Token::None,
//             Token::Str("cutoff"),
//             Token::F64(1.0),
//             Token::StructEnd,
//         ],
//     );
// }
