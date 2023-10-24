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

use roqoqo::devices::Device;
use roqoqo_qryd::api_devices::{QRydAPIDevice, QrydEmuSquareDevice, QrydEmuTriangularDevice};
use roqoqo_qryd::{phi_theta_relation, TweezerDevice};

use ndarray::Array2;

// Test the new function of the square device emulator
#[test]
fn test_new_square() {
    let device = QrydEmuSquareDevice::new(None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(device.seed(), 0);
    assert_eq!(device.seed(), apidevice.seed().unwrap());
    assert_eq!(device.qrydbackend(), "qryd_emu_cloudcomp_square");
    assert_eq!(device.qrydbackend(), apidevice.qrydbackend());
}

// Test the new function of the triangular device emulator
#[test]
fn test_new_triangular() {
    let device = QrydEmuTriangularDevice::new(Some(1), None, None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(device.seed(), 1);
    assert_eq!(device.seed(), apidevice.seed().unwrap());
    assert_eq!(device.qrydbackend(), "qryd_emu_cloudcomp_triangle");
    assert_eq!(device.qrydbackend(), apidevice.qrydbackend());
}

// Test the new function of the tweezer device
#[test]
fn test_new_tweezer() {
    let device = TweezerDevice::new(Some(1), None, None);
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(device.seed(), Some(1));
    assert_eq!(device.seed(), apidevice.seed());
    assert_eq!(device.qrydbackend(), "testdevice");
    assert_eq!(device.qrydbackend(), apidevice.qrydbackend());
}

// Test the functions from device trait of the square device emulator
#[test]
fn test_numberqubits_square() {
    let device = QrydEmuSquareDevice::new(None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(device.number_qubits(), 30);
    assert_eq!(apidevice.number_qubits(), device.number_qubits());
}

// Test the functions from device trait of the square device emulator
#[test]
fn test_decoherencerates_square() {
    let device = QrydEmuSquareDevice::new(None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(
        device.qubit_decoherence_rates(&0),
        Some(Array2::zeros((3, 3).to_owned()))
    );
    assert_eq!(
        apidevice.qubit_decoherence_rates(&0),
        device.qubit_decoherence_rates(&0)
    );
}

// Test the functions from device trait of the triangular device emulator
#[test]
fn test_numberqubits_triangular() {
    let device = QrydEmuTriangularDevice::new(None, None, None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(device.number_qubits(), 30);
    assert_eq!(apidevice.number_qubits(), device.number_qubits());
}

// Test the functions from device trait of the triangular device emulator
#[test]
fn test_decoherencerates_triangular() {
    let device = QrydEmuTriangularDevice::new(None, None, None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(
        device.qubit_decoherence_rates(&0),
        Some(Array2::zeros((3, 3).to_owned()))
    );
    assert_eq!(
        apidevice.qubit_decoherence_rates(&0),
        device.qubit_decoherence_rates(&0)
    );
}

// Test the functions from device trait of the tweezer device
#[test]
fn test_numberqubits_tweezer() {
    let device = TweezerDevice::new(None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(device.number_qubits(), 0);
    assert_eq!(apidevice.number_qubits(), device.number_qubits());
}

// Test the functions from device trait of the tweezer device
#[test]
fn test_decoherencerates_tweezer() {
    let device = TweezerDevice::new(None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(
        device.qubit_decoherence_rates(&0),
        Some(Array2::zeros((3, 3).to_owned()))
    );
    assert_eq!(
        apidevice.qubit_decoherence_rates(&0),
        device.qubit_decoherence_rates(&0)
    );
}

// Test the functions from device trait of the square device emulator
#[test]
fn test_gatetimes_square() {
    let device = QrydEmuSquareDevice::new(None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    // single qubit gates
    assert_eq!(device.single_qubit_gate_time("RotateXY", &0), Some(1e-6));
    assert_eq!(
        apidevice.single_qubit_gate_time("RotateXY", &0),
        device.single_qubit_gate_time("RotateXY", &0)
    );
    assert_eq!(device.single_qubit_gate_time("RotateXY", &31), None);
    assert_eq!(
        apidevice.single_qubit_gate_time("RotateXY", &31),
        device.single_qubit_gate_time("RotateXY", &31)
    );
    assert_eq!(device.single_qubit_gate_time("Hadamard", &0), None);
    assert_eq!(
        apidevice.single_qubit_gate_time("Hadamard", &0),
        device.single_qubit_gate_time("Hadamard", &0)
    );
    assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(1e-6));
    assert_eq!(
        apidevice.single_qubit_gate_time("RotateX", &0),
        device.single_qubit_gate_time("RotateX", &0)
    );
    assert_eq!(device.single_qubit_gate_time("RotateY", &0), Some(1e-6));
    assert_eq!(
        apidevice.single_qubit_gate_time("RotateY", &0),
        device.single_qubit_gate_time("RotateY", &0)
    );
    assert_eq!(
        device.single_qubit_gate_time("PhaseShiftState1", &0),
        Some(1e-6)
    );
    assert_eq!(
        apidevice.single_qubit_gate_time("PhaseShiftState1", &0),
        device.single_qubit_gate_time("PhaseShiftState1", &0)
    );
    assert_eq!(device.single_qubit_gate_time("PhaseShiftState2", &0), None);
    assert_eq!(
        apidevice.single_qubit_gate_time("PhaseShiftState2", &0),
        device.single_qubit_gate_time("PhaseShiftState2", &0)
    );
    // two qubit gates
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &1),
        Some(1e-6)
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &1),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &1)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &5),
        Some(1e-6)
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &5),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &5)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &5, &0),
        Some(1e-6)
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &5, &0),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &5, &0)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &0),
        None
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &0),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &0)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &29, &30),
        None
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &29, &30),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &29, &30)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &31),
        None
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &31),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &31)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &29),
        None
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &29),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &29)
    );
    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &5), None);
    assert_eq!(
        apidevice.two_qubit_gate_time("CNOT", &0, &5),
        device.two_qubit_gate_time("CNOT", &0, &5)
    );
    // three qubit gates
    assert_eq!(device.three_qubit_gate_time("Toffoli", &0, &1, &2), None);
    // multi qubit gates
    assert_eq!(device.multi_qubit_gate_time("MultiQubitMS", &[0, 1]), None);
    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMS", &[0, 1]),
        apidevice.multi_qubit_gate_time("MultiQubitMS", &[0, 1])
    );
}

// Test the functions from device trait of the triangular device emulator
#[test]
fn test_gatetimes_triangular() {
    let device = QrydEmuTriangularDevice::new(None, None, None, Some(true), Some(true));
    let no_3qbt_device = QrydEmuTriangularDevice::new(None, None, None, Some(false), Some(false));
    let apidevice = QRydAPIDevice::from(&device);
    // single qubit gates
    assert_eq!(device.single_qubit_gate_time("RotateXY", &0), Some(1e-6));
    assert_eq!(
        apidevice.single_qubit_gate_time("RotateXY", &0),
        device.single_qubit_gate_time("RotateXY", &0)
    );
    assert_eq!(device.single_qubit_gate_time("RotateXY", &31), None);
    assert_eq!(
        apidevice.single_qubit_gate_time("RotateXY", &31),
        device.single_qubit_gate_time("RotateXY", &31)
    );
    assert_eq!(device.single_qubit_gate_time("Hadamard", &0), None);
    assert_eq!(
        apidevice.single_qubit_gate_time("Hadamard", &0),
        device.single_qubit_gate_time("Hadamard", &0)
    );
    assert_eq!(device.single_qubit_gate_time("RotateX", &0), Some(1e-6));
    assert_eq!(
        apidevice.single_qubit_gate_time("RotateX", &0),
        device.single_qubit_gate_time("RotateX", &0)
    );
    assert_eq!(device.single_qubit_gate_time("RotateY", &0), Some(1e-6));
    assert_eq!(
        apidevice.single_qubit_gate_time("RotateY", &0),
        device.single_qubit_gate_time("RotateY", &0)
    );
    assert_eq!(
        device.single_qubit_gate_time("PhaseShiftState1", &0),
        Some(1e-6)
    );
    assert_eq!(
        apidevice.single_qubit_gate_time("PhaseShiftState1", &0),
        device.single_qubit_gate_time("PhaseShiftState1", &0)
    );
    assert_eq!(device.single_qubit_gate_time("PhaseShiftState2", &0), None);
    assert_eq!(
        apidevice.single_qubit_gate_time("PhaseShiftState2", &0),
        device.single_qubit_gate_time("PhaseShiftState2", &0)
    );
    // two qubit gates
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &1),
        Some(1e-6)
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &1),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &1)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &5),
        Some(1e-6)
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &5),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &5)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &5, &0),
        Some(1e-6)
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &5, &0),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &5, &0)
    );
    // -- this combination of qubits is allowed only for the triangular device
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &6),
        Some(1e-6)
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &6),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &6)
    );
    // --
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &0),
        None
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &0),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &0, &0)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &29, &30),
        None
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &29, &30),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &29, &30)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &31),
        None
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &31),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &31)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &29),
        None
    );
    assert_eq!(
        apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &29),
        device.two_qubit_gate_time("PhaseShiftedControlledZ", &30, &29)
    );
    assert_eq!(device.two_qubit_gate_time("CNOT", &0, &5), None);
    assert_eq!(
        apidevice.two_qubit_gate_time("CNOT", &0, &5),
        device.two_qubit_gate_time("CNOT", &0, &5)
    );
    // three qubit gates
    assert_eq!(device.three_qubit_gate_time("Toffoli", &0, &1, &2), None);
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPauliZ", &0, &5, &6),
        Some(1e-6)
    );
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPauliZ", &0, &5, &6),
        apidevice.three_qubit_gate_time("ControlledControlledPauliZ", &0, &5, &6)
    );
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPhaseShift", &0, &5, &6),
        Some(1e-6)
    );
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPhaseShift", &0, &5, &6),
        apidevice.three_qubit_gate_time("ControlledControlledPhaseShift", &0, &5, &6)
    );
    assert_eq!(device.three_qubit_gate_time("Toffoli", &43, &1, &2), None);
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPauliZ", &0, &5, &62),
        None
    );
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPhaseShift", &0, &51, &6),
        None
    );
    assert_eq!(
        no_3qbt_device.three_qubit_gate_time("ControlledControlledPauliZ", &0, &5, &6),
        None
    );
    assert_eq!(
        no_3qbt_device.three_qubit_gate_time("ControlledControlledPhaseShift", &0, &5, &6),
        None
    );
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPauliZ", &0, &5, &12),
        None
    );
    assert_eq!(
        device.three_qubit_gate_time("ControlledControlledPhaseShift", &0, &5, &12),
        None
    );
    // multi qubit gates
    assert_eq!(device.multi_qubit_gate_time("MultiQubitMS", &[0, 1]), None);
    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitMS", &[0, 1]),
        apidevice.multi_qubit_gate_time("MultiQubitMS", &[0, 1])
    );
}

// Test the functions from device trait of the tweezer device
#[test]
fn test_gatetimes_tweezer() {
    let mut device = TweezerDevice::new(None, None, None);
    device.set_tweezer_single_qubit_gate_time("PhaseShiftState1", 0, 0.34, None);
    device.set_tweezer_single_qubit_gate_time("PhaseShiftState1", 1, 0.34, None);
    device.set_tweezer_single_qubit_gate_time("PhaseShiftState1", 2, 0.34, None);
    device.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 0, 1, 0.34, None);
    device.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 1, 2, 0.34, None);
    device.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 2, 0, 0.34, None);
    device.set_tweezer_three_qubit_gate_time("Toffoli", 0, 1, 2, 0.34, None);
    device.set_tweezer_multi_qubit_gate_time("MultiQubitZZ", &[0, 1, 2], 0.23, None);
    device.add_qubit_tweezer_mapping(0, 0).unwrap();
    device.add_qubit_tweezer_mapping(1, 1).unwrap();
    device.add_qubit_tweezer_mapping(2, 2).unwrap();
    let apidevice = QRydAPIDevice::from(&device);

    assert_eq!(
        device.single_qubit_gate_time("PhaseShiftState1", &0),
        apidevice.single_qubit_gate_time("PhaseShiftState1", &0)
    );
    assert_eq!(
        device.single_qubit_gate_time("PhaseShiftState1", &1),
        apidevice.single_qubit_gate_time("PhaseShiftState1", &1)
    );
    assert_eq!(
        device.single_qubit_gate_time("PhaseShiftState1", &2),
        apidevice.single_qubit_gate_time("PhaseShiftState1", &2)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledPhase", &0, &1),
        apidevice.two_qubit_gate_time("PhaseShiftedControlledPhase", &0, &1)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledPhase", &1, &2),
        apidevice.two_qubit_gate_time("PhaseShiftedControlledPhase", &1, &2)
    );
    assert_eq!(
        device.two_qubit_gate_time("PhaseShiftedControlledPhase", &2, &0),
        apidevice.two_qubit_gate_time("PhaseShiftedControlledPhase", &2, &0)
    );
    assert_eq!(
        device.three_qubit_gate_time("Toffoli", &0, &1, &2),
        apidevice.three_qubit_gate_time("Toffoli", &0, &1, &2)
    );
    assert_eq!(
        device.multi_qubit_gate_time("MultiQubitZZ", &[0, 1, 2]),
        apidevice.multi_qubit_gate_time("MultiQubitZZ", &[0, 1, 2])
    );
}

// Test gatetime gate category
#[test]
fn test_gatetime_type() {
    let sq_device = QrydEmuSquareDevice::new(None, None, None);
    let tr_device = QrydEmuTriangularDevice::new(None, None, None, None, None);

    assert!(sq_device
        .single_qubit_gate_time("PhaseShiftState1", &0)
        .is_some());
    assert!(sq_device.single_qubit_gate_time("RotateX", &0).is_some());
    assert!(sq_device.single_qubit_gate_time("RotateY", &0).is_some());
    assert!(sq_device.single_qubit_gate_time("RotateZ", &0).is_some());
    assert!(sq_device.single_qubit_gate_time("RotateXY", &0).is_some());
    assert!(sq_device.single_qubit_gate_time("PauliX", &0).is_some());
    assert!(sq_device.single_qubit_gate_time("PauliY", &0).is_some());
    assert!(sq_device.single_qubit_gate_time("PauliZ", &0).is_some());
    assert!(sq_device.single_qubit_gate_time("SqrtPauliX", &0).is_some());
    assert!(sq_device
        .single_qubit_gate_time("InvSqrtPauliX", &0)
        .is_some());

    assert!(tr_device
        .single_qubit_gate_time("PhaseShiftState1", &0)
        .is_some());
    assert!(tr_device.single_qubit_gate_time("RotateX", &0).is_some());
    assert!(tr_device.single_qubit_gate_time("RotateY", &0).is_some());
    assert!(tr_device.single_qubit_gate_time("RotateZ", &0).is_some());
    assert!(tr_device.single_qubit_gate_time("RotateXY", &0).is_some());
    assert!(tr_device.single_qubit_gate_time("PauliX", &0).is_some());
    assert!(tr_device.single_qubit_gate_time("PauliY", &0).is_some());
    assert!(tr_device.single_qubit_gate_time("PauliZ", &0).is_some());
    assert!(tr_device.single_qubit_gate_time("SqrtPauliX", &0).is_some());
    assert!(tr_device
        .single_qubit_gate_time("InvSqrtPauliX", &0)
        .is_some());

    assert!(sq_device
        .two_qubit_gate_time("PhaseShiftedControlledZ", &0, &1)
        .is_some());
    assert!(sq_device
        .two_qubit_gate_time("PhaseShiftedControlledPhase", &0, &1)
        .is_some());

    assert!(tr_device
        .two_qubit_gate_time("PhaseShiftedControlledZ", &0, &1)
        .is_some());
    assert!(tr_device
        .two_qubit_gate_time("PhaseShiftedControlledPhase", &0, &1)
        .is_some());
}

// Test the functions from device trait of the triangular device emulator
// Changing the device is not allowed for the WebAPI emulators in the current version
#[test]
fn test_changedevice_square() {
    let mut device = QrydEmuSquareDevice::new(None, None, None);
    let mut apidevice = QRydAPIDevice::from(&device);
    assert!(device.change_device("", &[]).is_err());
    assert_eq!(
        apidevice.change_device("", &[]),
        device.change_device("", &[])
    );
}

// Test the functions from device trait of the triangular device emulator
// Changing the device is not allowed for the WebAPI emulators in the current version
#[test]
fn test_changedevice_triangular() {
    let mut device = QrydEmuTriangularDevice::new(None, None, None, None, None);
    let mut apidevice = QRydAPIDevice::from(&device);
    assert!(device.change_device("", &[]).is_err());
    assert_eq!(
        apidevice.change_device("", &[]),
        device.change_device("", &[])
    );
}

// Test the functions from device trait of the tweezer device
#[test]
fn test_changedevice_tweezer() {
    let mut device = TweezerDevice::new(None, None, None);
    let mut apidevice = QRydAPIDevice::from(&device);
    assert!(device.change_device("", &[]).is_err());
    assert_eq!(
        apidevice.change_device("", &[]),
        device.change_device("", &[])
    );
}

// Test the functions from device trait of the sqare device emulator
#[test]
fn test_twoqubitedges_square() {
    let device = QrydEmuSquareDevice::new(None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    let two_qubit_edges: Vec<(usize, usize)> = vec![
        (0, 1),
        (0, 5),
        (1, 2),
        (1, 6),
        (2, 3),
        (2, 7),
        (3, 4),
        (3, 8),
        (4, 9),
        (5, 6),
        (5, 10),
        (6, 7),
        (6, 11),
        (7, 8),
        (7, 12),
        (8, 9),
        (8, 13),
        (9, 14),
        (10, 11),
        (10, 15),
        (11, 12),
        (11, 16),
        (12, 13),
        (12, 17),
        (13, 14),
        (13, 18),
        (14, 19),
        (15, 16),
        (15, 20),
        (16, 17),
        (16, 21),
        (17, 18),
        (17, 22),
        (18, 19),
        (18, 23),
        (19, 24),
        (20, 21),
        (20, 25),
        (21, 22),
        (21, 26),
        (22, 23),
        (22, 27),
        (23, 24),
        (23, 28),
        (24, 29),
        (25, 26),
        (26, 27),
        (27, 28),
        (28, 29),
    ];
    assert_eq!(device.two_qubit_edges(), two_qubit_edges);
    assert_eq!(apidevice.two_qubit_edges(), device.two_qubit_edges());
}

// Test the functions from device trait of the triangular device emulator
#[test]
fn test_twoqubitedges_triangular() {
    let device = QrydEmuTriangularDevice::new(None, None, None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    let two_qubit_edges: Vec<(usize, usize)> = vec![
        (0, 1),
        (0, 5),
        (0, 6),
        (1, 2),
        (1, 6),
        (1, 7),
        (2, 3),
        (2, 7),
        (2, 8),
        (3, 4),
        (3, 8),
        (3, 9),
        (4, 9),
        (5, 6),
        (5, 10),
        (6, 7),
        (6, 10),
        (6, 11),
        (7, 8),
        (7, 11),
        (7, 12),
        (8, 9),
        (8, 12),
        (8, 13),
        (9, 13),
        (9, 14),
        (10, 11),
        (10, 15),
        (10, 16),
        (11, 12),
        (11, 16),
        (11, 17),
        (12, 13),
        (12, 17),
        (12, 18),
        (13, 14),
        (13, 18),
        (13, 19),
        (14, 19),
        (15, 16),
        (15, 20),
        (16, 17),
        (16, 20),
        (16, 21),
        (17, 18),
        (17, 21),
        (17, 22),
        (18, 19),
        (18, 22),
        (18, 23),
        (19, 23),
        (19, 24),
        (20, 21),
        (20, 25),
        (20, 26),
        (21, 22),
        (21, 26),
        (21, 27),
        (22, 23),
        (22, 27),
        (22, 28),
        (23, 24),
        (23, 28),
        (23, 29),
        (24, 29),
        (25, 26),
        (26, 27),
        (27, 28),
        (28, 29),
    ];
    assert_eq!(device.two_qubit_edges(), two_qubit_edges);
    assert_eq!(apidevice.two_qubit_edges(), device.two_qubit_edges());
}

#[test]
fn test_twoqubitedges_tweezer() {
    let mut device = TweezerDevice::new(None, None, None);
    device.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 0, 1, 0.34, None);
    device.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 1, 2, 0.34, None);
    device.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 2, 0, 0.34, None);
    device.add_qubit_tweezer_mapping(0, 0).unwrap();
    device.add_qubit_tweezer_mapping(1, 1).unwrap();
    device.add_qubit_tweezer_mapping(2, 2).unwrap();
    let apidevice = QRydAPIDevice::from(&device);
    let two_qubit_edges: Vec<(usize, usize)> = vec![(0, 1), (1, 2), (2, 0)];
    assert!(two_qubit_edges
        .iter()
        .all(|el| device.two_qubit_edges().contains(el)));
    assert!(apidevice
        .two_qubit_edges()
        .iter()
        .all(|el| device.two_qubit_edges().contains(el)));
}

// Test to_generic_device() for square device
#[test]
fn test_to_generic_device_square() {
    let device = QrydEmuSquareDevice::new(Some(0), None, None);
    let apidevice = QRydAPIDevice::from(&device);
    let genericdevice = apidevice.to_generic_device();

    assert_eq!(apidevice.number_qubits(), genericdevice.number_qubits());
    assert_eq!(device.number_qubits(), genericdevice.number_qubits());
    for gate_name in ["PhaseShiftState1", "RotateX", "RotateY", "RotateXY"] {
        for qubit in 0..genericdevice.number_qubits() {
            assert_eq!(
                genericdevice
                    .single_qubit_gate_time(gate_name, &qubit)
                    .unwrap(),
                apidevice.single_qubit_gate_time(gate_name, &qubit).unwrap()
            );
        }
    }
    for qubit in 0..genericdevice.number_qubits() {
        assert_eq!(
            genericdevice.qubit_decoherence_rates(&qubit),
            apidevice.qubit_decoherence_rates(&qubit)
        );
    }
    for row in 0..genericdevice.number_qubits() {
        for column in row + 1..genericdevice.number_qubits() {
            if genericdevice
                .two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                .is_some()
            {
                assert_eq!(
                    genericdevice.two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column),
                    apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                )
            }
        }
    }
}

// Test to_generic_device() for triangular device
#[test]
fn test_to_generic_device_triangular() {
    let device = QrydEmuTriangularDevice::new(Some(0), None, None, None, None);
    let apidevice = QRydAPIDevice::from(&device);
    let genericdevice = apidevice.to_generic_device();

    assert_eq!(apidevice.number_qubits(), genericdevice.number_qubits());
    assert_eq!(device.number_qubits(), genericdevice.number_qubits());
    for gate_name in ["PhaseShiftState1", "RotateX", "RotateY", "RotateXY"] {
        for qubit in 0..genericdevice.number_qubits() {
            assert_eq!(
                genericdevice
                    .single_qubit_gate_time(gate_name, &qubit)
                    .unwrap(),
                apidevice.single_qubit_gate_time(gate_name, &qubit).unwrap()
            );
        }
    }
    for qubit in 0..genericdevice.number_qubits() {
        assert_eq!(
            genericdevice.qubit_decoherence_rates(&qubit),
            apidevice.qubit_decoherence_rates(&qubit)
        );
    }
    for row in 0..genericdevice.number_qubits() {
        for column in row + 1..genericdevice.number_qubits() {
            if genericdevice
                .two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                .is_some()
            {
                assert_eq!(
                    genericdevice.two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column),
                    apidevice.two_qubit_gate_time("PhaseShiftedControlledZ", &row, &column)
                )
            }
        }
    }
}

// Test to_generic_device() for tweezer device
#[test]
fn test_to_generic_device_tweezer() {
    let mut device = TweezerDevice::new(None, None, None);
    device.set_tweezer_single_qubit_gate_time("PhaseShiftState1", 0, 0.34, None);
    device.set_tweezer_single_qubit_gate_time("PhaseShiftState1", 1, 0.34, None);
    device.set_tweezer_single_qubit_gate_time("PhaseShiftState1", 2, 0.34, None);
    device.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 0, 1, 0.34, None);
    device.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 1, 2, 0.34, None);
    device.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 2, 0, 0.34, None);
    device.add_qubit_tweezer_mapping(0, 0).unwrap();
    device.add_qubit_tweezer_mapping(1, 1).unwrap();
    device.add_qubit_tweezer_mapping(2, 2).unwrap();
    let apidevice = QRydAPIDevice::from(&device);
    let genericdevice = apidevice.to_generic_device();

    assert_eq!(apidevice.number_qubits(), genericdevice.number_qubits());
    assert_eq!(device.number_qubits(), genericdevice.number_qubits());
    for qubit in 0..genericdevice.number_qubits() {
        assert_eq!(
            genericdevice
                .single_qubit_gate_time("PhaseShiftState1", &qubit)
                .unwrap(),
            apidevice
                .single_qubit_gate_time("PhaseShiftState1", &qubit)
                .unwrap()
        );
    }
    for qubit in 0..genericdevice.number_qubits() {
        assert_eq!(
            genericdevice.qubit_decoherence_rates(&qubit),
            apidevice.qubit_decoherence_rates(&qubit)
        );
    }
    for row in 0..genericdevice.number_qubits() {
        for column in row + 1..genericdevice.number_qubits() {
            if genericdevice
                .two_qubit_gate_time("PhaseShiftedControlledPhase", &row, &column)
                .is_some()
            {
                assert_eq!(
                    genericdevice.two_qubit_gate_time("PhaseShiftedControlledPhase", &row, &column),
                    apidevice.two_qubit_gate_time("PhaseShiftedControlledPhase", &row, &column)
                )
            }
        }
    }
}

#[test]
fn test_phi_theta_relation() {
    let triangular = QrydEmuTriangularDevice::new(Some(0), None, None, None, None);
    let square = QrydEmuSquareDevice::new(Some(0), None, None);
    let mut tweezer = TweezerDevice::new(Some(0), None, None);
    tweezer.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledZ", 0, 1, 0.10, None);
    tweezer.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 0, 1, 0.10, None);
    tweezer.add_qubit_tweezer_mapping(0, 0).unwrap();
    tweezer.add_qubit_tweezer_mapping(1, 1).unwrap();
    let triangular_f =
        QrydEmuTriangularDevice::new(Some(0), Some("2.13".to_string()), None, None, None);
    let square_f = QrydEmuSquareDevice::new(Some(0), Some("2.13".to_string()), None);
    let tweezer_f = TweezerDevice::new(Some(0), Some("2.13".to_string()), None);

    assert_eq!(
        triangular.phase_shift_controlled_z().unwrap(),
        phi_theta_relation("DefaultRelation", std::f64::consts::PI).unwrap()
    );
    assert_eq!(
        square.phase_shift_controlled_z().unwrap(),
        phi_theta_relation("DefaultRelation", std::f64::consts::PI).unwrap()
    );
    assert_eq!(
        tweezer.phase_shift_controlled_z().unwrap(),
        phi_theta_relation("DefaultRelation", std::f64::consts::PI).unwrap()
    );
    assert_eq!(
        triangular.phase_shift_controlled_phase(1.2).unwrap(),
        phi_theta_relation("DefaultRelation", 1.2).unwrap()
    );
    assert_eq!(
        square.phase_shift_controlled_phase(1.2).unwrap(),
        phi_theta_relation("DefaultRelation", 1.2).unwrap()
    );
    assert_eq!(
        tweezer.phase_shift_controlled_phase(1.2).unwrap(),
        phi_theta_relation("DefaultRelation", 1.2).unwrap()
    );
    assert_eq!(triangular_f.phase_shift_controlled_z(), Some(2.13));
    assert_eq!(square_f.phase_shift_controlled_z(), Some(2.13));
    assert_eq!(tweezer_f.phase_shift_controlled_z(), Some(2.13));

    assert!(triangular.gate_time_controlled_z(&0, &13, 1.4).is_none());
    assert!(triangular
        .gate_time_controlled_phase(&0, &13, 0.6, 1.4)
        .is_none());
    assert!(square.gate_time_controlled_z(&0, &13, 1.4).is_none());
    assert!(square
        .gate_time_controlled_phase(&0, &13, 0.6, 1.4)
        .is_none());
    assert!(tweezer.gate_time_controlled_z(&0, &9, 1.3).is_none());
    assert!(tweezer
        .gate_time_controlled_phase(&0, &9, 1.3, 1.4)
        .is_none());

    assert!(triangular
        .gate_time_controlled_z(&0, &1, triangular.phase_shift_controlled_z().unwrap())
        .is_some());
    assert!(square
        .gate_time_controlled_z(&0, &1, square.phase_shift_controlled_z().unwrap())
        .is_some());
    assert!(tweezer
        .gate_time_controlled_z(&0, &1, triangular.phase_shift_controlled_z().unwrap())
        .is_some());
    assert!(triangular
        .gate_time_controlled_phase(
            &0,
            &1,
            triangular.phase_shift_controlled_phase(0.1).unwrap(),
            0.1
        )
        .is_some());
    assert!(square
        .gate_time_controlled_phase(
            &0,
            &1,
            square.phase_shift_controlled_phase(0.1).unwrap(),
            0.1
        )
        .is_some());
    assert!(tweezer
        .gate_time_controlled_phase(
            &0,
            &1,
            square.phase_shift_controlled_phase(0.1).unwrap(),
            0.1
        )
        .is_some());

    assert!(triangular
        .gate_time_controlled_z(&0, &1, triangular.phase_shift_controlled_z().unwrap() + 0.2)
        .is_none());
    assert!(square
        .gate_time_controlled_z(&0, &1, square.phase_shift_controlled_z().unwrap() + 0.2)
        .is_none());
    assert!(tweezer
        .gate_time_controlled_z(&0, &1, square.phase_shift_controlled_z().unwrap() + 0.2)
        .is_none());
    assert!(triangular
        .gate_time_controlled_phase(
            &0,
            &1,
            triangular.phase_shift_controlled_phase(0.1).unwrap() + 0.2,
            0.1
        )
        .is_none());
    assert!(square
        .gate_time_controlled_phase(
            &0,
            &1,
            square.phase_shift_controlled_phase(0.1).unwrap() + 0.2,
            0.1
        )
        .is_none());
    assert!(tweezer
        .gate_time_controlled_phase(
            &0,
            &1,
            square.phase_shift_controlled_phase(0.1).unwrap() + 0.2,
            0.1
        )
        .is_none());
}
