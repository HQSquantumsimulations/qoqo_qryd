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

use ndarray::{array, Array2};
use roqoqo::prelude::*;
use roqoqo::{operations::*, Circuit};
use roqoqo_qryd::qryd_devices::FirstDevice;
use roqoqo_qryd::SimulatorBackend;
use roqoqo_test::prepare_monte_carlo_gate_test;

#[test]
#[cfg(feature = "simulator")]
fn init_backend() {
    let device = FirstDevice::new(
        2,
        2,
        &[1, 1],
        3.0,
        array![[0.0, 1.0], [0.0, 1.0]],
        None,
        None,
        None,
        None,
    )
    .unwrap();
    let _backend = SimulatorBackend::new(device.into());
}

#[test]
fn test_to_qryd_json() {}

/// Test SimulatorBackend standard derived traits (Debug, Clone, PartialEq)
#[test]
fn pragma_shift_qryd_qubit_simple_traits() {
    let layout: Array2<f64> = array![[0.0, 1.0], [0.0, 1.0]];
    let device = FirstDevice::new(
        2,
        2,
        &[1, 1],
        3.0,
        array![[0.0, 1.0], [0.0, 1.0]],
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .add_layout(1, layout.clone())
    .unwrap();
    let backend = SimulatorBackend::new(device.clone().into());

    // Test Debug trait
    assert_eq!(
        format!("{:?}", backend),
        format!("SimulatorBackend {{ device: FirstDevice({:?}) }}", device)
    );

    // Test Clone trait
    assert_eq!(backend.clone(), backend);

    // Test PartialEq trait
    let device_0 = FirstDevice::new(
        2,
        2,
        &[1, 1],
        3.0,
        array![[0.0, 1.0], [0.0, 1.0]],
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .add_layout(1, layout.clone())
    .unwrap();
    let backend_0 = SimulatorBackend::new(device_0.into());
    let device_1 = FirstDevice::new(
        2,
        2,
        &[1, 1],
        2.0,
        array![[0.0, 1.0], [0.0, 1.0]],
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .add_layout(1, layout)
    .unwrap();
    let backend_1 = SimulatorBackend::new(device_1.into());
    assert!(backend_0 == backend);
    assert!(backend == backend_0);
    assert!(backend_1 != backend);
    assert!(backend != backend_1);
}

#[test]
#[cfg(feature = "simulator")]
fn run_simple_circuit() {
    let layout: Array2<f64> = array![[0.0, 1.0], [0.0, 1.0]];
    let mut device = FirstDevice::new(
        2,
        2,
        &[1, 1],
        3.0,
        array![[0.0, 1.0], [0.0, 1.0]],
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .add_layout(1, layout)
    .unwrap();
    device.switch_layout(&1).unwrap();
    let backend = SimulatorBackend::new(device.into());
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, true);
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += RotateX::new(1, std::f64::consts::FRAC_PI_2.into());
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 20, None);
    let (bit_registers, _float_registers, _complex_registers) =
        backend.run_circuit(&circuit).unwrap();
    assert!(bit_registers.contains_key("ro"));
    let out_reg = bit_registers.get("ro").unwrap();
    assert_eq!(out_reg.len(), 20);
    for reg in out_reg.iter() {
        assert_eq!(reg.len(), 2);
    }
}

/// Simply test measurement process, not that gate is translated correclty
#[test]
#[cfg(feature = "simulator")]
fn test_measurement() {
    let gate: GateOperation = PhaseShiftState1::new(0, std::f64::consts::FRAC_PI_2.into()).into();
    let preparation_gates: Vec<SingleQubitGateOperation> =
        vec![RotateX::new(0, std::f64::consts::FRAC_PI_2.into()).into()];
    let basis_rotation_gates: Vec<SingleQubitGateOperation> =
        vec![RotateY::new(0, std::f64::consts::FRAC_PI_2.into()).into()];
    let (measurement, exp_vals) =
        prepare_monte_carlo_gate_test(gate, preparation_gates, basis_rotation_gates, None, 1, 200);
    let device = FirstDevice::new(1, 1, &[1], 3.0, array![[0.0],], None, None, None, None).unwrap();
    let backend = SimulatorBackend::new(device.into());
    let measured_exp_vals = backend.run_measurement(&measurement).unwrap().unwrap();
    for (key, val) in exp_vals.iter() {
        assert!((val - measured_exp_vals.get(key).unwrap()).abs() < 1.0);
    }
}

/// Test full gate with stochastic application of gates, ignore normally because of length and load
#[test]
fn test_full_simple_gate() {
    let gate: GateOperation = RotateX::new(0, std::f64::consts::FRAC_PI_2.into()).into();
    let preparation_gates: Vec<SingleQubitGateOperation> = vec![
        RotateX::new(0, std::f64::consts::FRAC_PI_2.into()).into(),
        RotateY::new(0, std::f64::consts::FRAC_PI_2.into()).into(),
        PhaseShiftState1::new(0, std::f64::consts::FRAC_PI_2.into()).into(),
    ];
    let basis_rotation_gates: Vec<SingleQubitGateOperation> = vec![
        RotateX::new(0, std::f64::consts::FRAC_PI_2.into()).into(),
        RotateX::new(0, std::f64::consts::FRAC_PI_2.into()).into(),
        PhaseShiftState1::new(0, std::f64::consts::FRAC_PI_2.into()).into(),
    ];
    let (measurement, exp_vals) =
        prepare_monte_carlo_gate_test(gate, preparation_gates, basis_rotation_gates, None, 5, 200);

    let device =
        FirstDevice::new(1, 1, &[1], 3.0, array![[0.0,],], None, None, None, None).unwrap();
    let backend = SimulatorBackend::new(device.into());
    let measured_exp_vals = backend.run_measurement(&measurement).unwrap().unwrap();
    for (key, val) in exp_vals.iter() {
        assert!((val - measured_exp_vals.get(key).unwrap()).abs() < 1.0);
    }
}
