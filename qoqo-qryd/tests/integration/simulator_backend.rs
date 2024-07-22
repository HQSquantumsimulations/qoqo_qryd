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

//! Integration test for public API of Basis rotation measurement

use pyo3::prelude::*;
use pyo3::Python;
use qoqo::measurements::{ClassicalRegisterWrapper, PauliZProductWrapper};
use qoqo::CircuitWrapper;
use qoqo_qryd::simulator_backend::{convert_into_backend, SimulatorBackendWrapper};
use qoqo_qryd::{TweezerDeviceWrapper, TweezerMutableDeviceWrapper};
use roqoqo::measurements::{ClassicalRegister, PauliZProduct, PauliZProductInput};
use roqoqo::operations;
use roqoqo::Circuit;
use roqoqo_qryd::SimulatorBackend;
use roqoqo_qryd::TweezerDevice;

#[test]
fn test_creating_backend() {
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerDeviceWrapper>();
        let device = device_type.call0().unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let _backend = backend_type
            .call1((device.downcast::<TweezerDeviceWrapper>().unwrap(),))
            .unwrap()
            .downcast::<SimulatorBackendWrapper>()
            .unwrap();
    });
}

#[test]
fn test_creating_backend_error() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let backend = backend_type.call1((vec!["fails"],));
        assert!(backend.is_err());
    });
}

#[test]
fn test_running_circuit() {
    pyo3::prepare_freethreaded_python();
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("readout".to_string(), 3, true);
    circuit += operations::RotateX::new(0, 0.0.into());
    circuit += operations::RotateX::new(0, 1.0.into());
    circuit += operations::RotateX::new(0, 2.0.into());
    circuit += operations::PhaseShiftedControlledZ::new(0, 1, 1.0.into());
    circuit += operations::PragmaRepeatedMeasurement::new("readout".to_string(), 100, None);
    let circuit_wrapper = CircuitWrapper { internal: circuit };
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device_tw = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device_tw.call_method1("add_layout", ("test",)).unwrap();
        device_tw
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device_tw
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device_tw.call_method1("switch_layout", ("test",)).unwrap();

        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let backend_tw = backend_type.call1((device_tw,)).unwrap();
        assert!(backend_tw
            .downcast::<SimulatorBackendWrapper>()
            .unwrap()
            .call_method1("run_circuit", (circuit_wrapper,))
            .is_ok());
    })
}

#[test]
fn test_running_circuit_error() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device.call_method1("add_layout", ("test",)).unwrap();
        device
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device.call_method1("switch_layout", ("test",)).unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let backend = backend_type.call1((device,)).unwrap();
        let error = backend
            .downcast::<SimulatorBackendWrapper>()
            .unwrap()
            .call_method1("run_circuit", (vec!["error"],));
        assert!(error.is_err());
    })
}

#[test]
fn test_running_measurement_registers() {
    pyo3::prepare_freethreaded_python();
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("readout".to_string(), 3, true);
    circuit += operations::RotateX::new(0, 0.0.into());
    circuit += operations::RotateX::new(0, 1.0.into());
    circuit += operations::RotateX::new(0, 2.0.into());
    circuit += operations::PhaseShiftedControlledZ::new(0, 1, 1.0.into());
    circuit += operations::PragmaRepeatedMeasurement::new("readout".to_string(), 100, None);
    let cr_measurement = ClassicalRegister {
        constant_circuit: None,
        circuits: vec![circuit],
    };
    let crm_wrapper = ClassicalRegisterWrapper {
        internal: cr_measurement,
    };
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device.call_method1("add_layout", ("test",)).unwrap();
        device
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device.call_method1("switch_layout", ("test",)).unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let backend = backend_type.call1((device,)).unwrap();
        let _ = backend
            .downcast::<SimulatorBackendWrapper>()
            .unwrap()
            .call_method1("run_measurement_registers", (crm_wrapper,))
            .unwrap();
    })
}

#[test]
fn test_running_measurement_registers_error_1() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device.call_method1("add_layout", ("test",)).unwrap();
        device
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device.call_method1("switch_layout", ("test",)).unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let backend = backend_type.call1((device,)).unwrap();
        let error = backend
            .downcast::<SimulatorBackendWrapper>()
            .unwrap()
            .call_method1("run_measurement_registers", (vec!["error"],));
        assert!(error.is_err());
    })
}

#[test]
fn test_running_measurement_registers_some() {
    pyo3::prepare_freethreaded_python();
    let cr_measurement = ClassicalRegister {
        constant_circuit: Some(Circuit::new()),
        circuits: vec![],
    };
    let crm_wrapper = ClassicalRegisterWrapper {
        internal: cr_measurement,
    };
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device.call_method1("add_layout", ("test",)).unwrap();
        device
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device.call_method1("switch_layout", ("test",)).unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let backend = backend_type.call1((device,)).unwrap();
        let _ = backend
            .downcast::<SimulatorBackendWrapper>()
            .unwrap()
            .call_method1("run_measurement_registers", (crm_wrapper,))
            .unwrap();
    })
}

#[test]
fn test_running_measurement_registers_all_registers() {
    pyo3::prepare_freethreaded_python();
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("readout".to_string(), 3, true);
    circuit += operations::DefinitionFloat::new("readout".to_string(), 3, true);
    circuit += operations::DefinitionComplex::new("readout".to_string(), 3, true);
    circuit += operations::DefinitionUsize::new("readout".to_string(), 3, true);
    circuit += operations::RotateX::new(0, 0.0.into());
    circuit += operations::RotateX::new(0, 1.0.into());
    circuit += operations::RotateX::new(0, 2.0.into());
    circuit += operations::PhaseShiftedControlledZ::new(0, 1, 1.0.into());
    circuit += operations::PragmaRepeatedMeasurement::new("readout".to_string(), 100, None);
    circuit += operations::PragmaGetDensityMatrix::new("readout".to_string(), None);
    circuit += operations::PragmaGetOccupationProbability::new("readout".to_string(), None);
    let cr_measurement = ClassicalRegister {
        constant_circuit: Some(Circuit::new()),
        circuits: vec![circuit],
    };
    let crm_wrapper = ClassicalRegisterWrapper {
        internal: cr_measurement,
    };
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device.call_method1("add_layout", ("test",)).unwrap();
        device
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device.call_method1("switch_layout", ("test",)).unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let backend = backend_type.call1((device,)).unwrap();
        let _ = backend
            .downcast::<SimulatorBackendWrapper>()
            .unwrap()
            .call_method1("run_measurement_registers", (crm_wrapper,))
            .unwrap();
    })
}

#[test]
fn test_running_measurement() {
    pyo3::prepare_freethreaded_python();
    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("ro".to_string(), 3, true);
    circuit += operations::RotateX::new(0, 0.0.into());
    circuit += operations::RotateX::new(0, 1.0.into());
    circuit += operations::RotateX::new(0, 2.0.into());
    circuit += operations::PhaseShiftedControlledZ::new(0, 1, 1.0.into());
    circuit += operations::PragmaRepeatedMeasurement::new("ro".to_string(), 100, None);

    let tmp_vec: Vec<usize> = Vec::new();
    let mut roqoqo_bri = PauliZProductInput::new(3, false);
    roqoqo_bri
        .add_pauliz_product("ro".to_string(), tmp_vec)
        .unwrap();
    let cr_measurement = PauliZProduct {
        constant_circuit: None,
        circuits: vec![circuit],
        input: roqoqo_bri,
    };
    let crm_wrapper = PauliZProductWrapper {
        internal: cr_measurement,
    };
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device.call_method1("add_layout", ("test",)).unwrap();
        device
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device.call_method1("switch_layout", ("test",)).unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let backend = backend_type.call1((device,)).unwrap();
        let _ = backend
            .downcast::<SimulatorBackendWrapper>()
            .unwrap()
            .call_method1("run_measurement", (crm_wrapper,))
            .unwrap();
    })
}

/// Test involved_qubits function for Pragmas with All
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device.call_method1("add_layout", ("test",)).unwrap();
        device
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device.call_method1("switch_layout", ("test",)).unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let binding = backend_type.call1((device,)).unwrap();
        let backend = binding.downcast::<SimulatorBackendWrapper>().unwrap();

        let copy_op = backend.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_op.extract::<SimulatorBackendWrapper>().unwrap();
        let deepcopy_op = backend.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_op.extract::<SimulatorBackendWrapper>().unwrap();

        let backend_wrapper = backend.extract::<SimulatorBackendWrapper>().unwrap();
        assert_eq!(backend_wrapper, copy_wrapper);
        assert_eq!(backend_wrapper, deepcopy_wrapper);
    });
}

/// Test to_ and from_bincode() functions of SimulatorBackendWrapper
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device.call_method1("add_layout", ("test",)).unwrap();
        device
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device.call_method1("switch_layout", ("test",)).unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let binding = backend_type.call1((device,)).unwrap();
        let backend = binding.downcast::<SimulatorBackendWrapper>().unwrap();

        let serialised = backend.call_method0("to_bincode").unwrap();
        let deserialised = backend
            .call_method1("from_bincode", (&serialised,))
            .unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = backend.call_method1("from_bincode", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<SimulatorBackendWrapper>().unwrap();
        let backend_wrapper = backend.extract::<SimulatorBackendWrapper>().unwrap();
        assert_eq!(backend_wrapper, serde_wrapper);
    });
}

/// Test to_ and from_json() functions of SimulatorBackendWrapper
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device.call_method1("add_layout", ("test",)).unwrap();
        device
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device.call_method1("switch_layout", ("test",)).unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let binding = backend_type.call1((device,)).unwrap();
        let backend = binding.downcast::<SimulatorBackendWrapper>().unwrap();

        let serialised = backend.call_method0("to_json").unwrap();
        // let new = backend.clone();
        let deserialised = backend.call_method1("from_json", (&serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = backend.call_method1("from_json", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<SimulatorBackendWrapper>().unwrap();
        let backend_wrapper = backend.extract::<SimulatorBackendWrapper>().unwrap();
        assert_eq!(backend_wrapper, serde_wrapper);
    });
}

/// Test convert_to_backend()
#[test]
fn test_convert_to_backend() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device.call_method1("add_layout", ("test",)).unwrap();
        device
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device.call_method1("switch_layout", ("test",)).unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let backend = backend_type.call1((device,)).unwrap();

        let converted =
            convert_into_backend(backend.downcast::<SimulatorBackendWrapper>().unwrap()).unwrap();
        let mut rust_dev = TweezerDevice::new(None, None, None);
        rust_dev.add_layout("test").unwrap();
        rust_dev
            .set_tweezer_single_qubit_gate_time("RotateX", 0, 1.0, Some("test".to_string()))
            .unwrap();
        rust_dev
            .set_tweezer_two_qubit_gate_time(
                "PhaseShiftedControlledZ",
                0,
                1,
                1.0,
                Some("test".to_string()),
            )
            .unwrap();
        rust_dev.switch_layout("test", None).unwrap();
        let rust_backend = SimulatorBackend::new(rust_dev, None);
        assert_eq!(converted, rust_backend);
        assert!(convert_into_backend(device).is_err());
    });
}

#[test]
fn test_pyo3_new_change_layout() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<TweezerMutableDeviceWrapper>();
        let binding = device_type.call0().unwrap();
        let device = binding.downcast::<TweezerMutableDeviceWrapper>().unwrap();
        device.call_method1("add_layout", ("test",)).unwrap();
        device
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 1.0, "test"),
            )
            .unwrap();
        device
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 1.0, "test"),
            )
            .unwrap();
        device.call_method1("switch_layout", ("test",)).unwrap();
        let backend_type = py.get_type_bound::<SimulatorBackendWrapper>();
        let backend = backend_type.call1((device,)).unwrap();

        let pragma_wrapper = backend
            .downcast::<SimulatorBackendWrapper>()
            .unwrap()
            .extract::<SimulatorBackendWrapper>()
            .unwrap();
        let device_diff = device_type.call0().unwrap();
        let new_op_diff = backend_type
            .call1((device_diff
                .downcast::<TweezerMutableDeviceWrapper>()
                .unwrap(),))
            .unwrap();
        let pragma_wrapper_diff = new_op_diff
            .downcast::<SimulatorBackendWrapper>()
            .unwrap()
            .extract::<SimulatorBackendWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);
    })
}
