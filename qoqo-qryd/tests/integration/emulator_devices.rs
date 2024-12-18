// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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

//! Integration test for Emulator Devices

use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyDict, PyList},
};

use qoqo_qryd::{emulator_devices::convert_into_device, EmulatorDeviceWrapper};
use roqoqo_qryd::{phi_theta_relation, EmulatorDevice};

/// Test new instantiation of EmulatorDeviceWrapper
#[test]
fn test_new() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let res = device_type.call1((2,));

        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(
            res.call_method0("qrydbackend")
                .unwrap()
                .extract::<String>()
                .unwrap(),
            "qryd_tweezer_device"
        );
        assert_eq!(
            res.call_method0("seed")
                .unwrap()
                .extract::<Option<usize>>()
                .unwrap(),
            Some(2)
        );

        let res_emp = device_type.call0().unwrap();

        assert_eq!(
            res_emp
                .call_method0("seed")
                .unwrap()
                .extract::<Option<usize>>()
                .unwrap(),
            None
        );
    })
}

//Test EmulatorDevice available_layouts() method
#[test]
fn test_available_layouts() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        assert!(device
            .call_method0("available_layouts")
            .unwrap()
            .downcast::<PyList>()
            .unwrap()
            .is_empty());
    })
}

/// Test add_qubit_tweezer_mapping() get_tweezer_from_qubit(), methods of EmulatorDeviceWrapper
#[test]
fn test_qubit_tweezer_mapping() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        assert!(device
            .call_method0("get_qubit_to_tweezer_mapping")
            .unwrap()
            .is_none());

        assert!(device
            .call_method1("add_qubit_tweezer_mapping", (0, 0))
            .is_ok());

        assert!(device.call_method0("get_qubit_to_tweezer_mapping").is_ok());

        assert!(device
            .call_method1("add_qubit_tweezer_mapping", (1, 1))
            .is_ok());

        let ex_dict: &Bound<PyDict> = &[(0, 0), (1, 1)].into_py_dict_bound(py);
        assert!(device
            .call_method0("get_qubit_to_tweezer_mapping")
            .unwrap()
            .eq(ex_dict)
            .unwrap());

        assert_eq!(
            device
                .call_method1("add_qubit_tweezer_mapping", (3, 2))
                .unwrap()
                .extract::<&PyDict>()
                .unwrap()
                .len(),
            3
        );
    })
}

/// Test get_available_gates_names and add_available_gate for EmulatorDeviceWrapper
#[test]
fn test_available_gate_names() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        let res = device
            .call_method1("get_available_gates_names", ())
            .unwrap();
        assert!(res.is_empty().unwrap());

        device
            .call_method1("add_available_gate", ("RotateX",))
            .unwrap();
        device
            .call_method1("add_available_gate", ("SWAP",))
            .unwrap();
        device
            .call_method1("add_available_gate", ("Toffoli",))
            .unwrap();
        device
            .call_method1("add_available_gate", ("MultiQubitZZ",))
            .unwrap();

        let res = device
            .call_method1("get_available_gates_names", ())
            .unwrap();
        assert!(res.contains("RotateX".to_string()).unwrap());
        assert!(res.contains("SWAP".to_string()).unwrap());
        assert!(res.contains("Toffoli".to_string()).unwrap());
        assert!(res.contains("MultiQubitZZ".to_string()).unwrap());
    })
}

/// Test allow_reset for EmulatorDeviceWrapper
#[test]
fn test_allow_reset() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        assert!(!device
            .call_method0("get_allow_reset")
            .unwrap()
            .extract::<bool>()
            .unwrap());

        assert!(device.call_method1("set_allow_reset", (true,)).is_ok());

        assert!(device
            .call_method0("get_allow_reset")
            .unwrap()
            .extract::<bool>()
            .unwrap());
    })
}

/// Test deactivate_qubit function of EmulatorDeviceWrapper
#[test]
fn test_deactivate_qubit() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        assert!(device.call_method1("deactivate_qubit", (0,)).is_err());

        device
            .call_method1("add_qubit_tweezer_mapping", (0, 1))
            .unwrap();

        assert!(device.call_method1("deactivate_qubit", (0,)).is_ok());
        assert!(device.call_method1("deactivate_qubit", (0,)).is_err());
    })
}

/// Test phase_shift_controlled_... and gate_time_controlled_...  methods
#[test]
fn test_phi_theta_relations() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device_f = device_type
            .call1((Option::<usize>::None, "2.15", "2.13"))
            .unwrap();
        let device = device_type.call0().unwrap();

        assert_eq!(
            device_f
                .call_method0("phase_shift_controlled_z")
                .unwrap()
                .extract::<f64>()
                .unwrap(),
            2.15
        );
        assert_eq!(
            device_f
                .call_method1("phase_shift_controlled_phase", (0.2,))
                .unwrap()
                .extract::<f64>()
                .unwrap(),
            2.13
        );
        assert_eq!(
            device
                .call_method0("phase_shift_controlled_z")
                .unwrap()
                .extract::<f64>()
                .unwrap(),
            phi_theta_relation("DefaultRelation", std::f64::consts::PI).unwrap()
        );
        assert_eq!(
            device
                .call_method1("phase_shift_controlled_phase", (1.2,))
                .unwrap()
                .extract::<f64>()
                .unwrap(),
            phi_theta_relation("DefaultRelation", 1.2).unwrap()
        );

        device
            .call_method1("add_qubit_tweezer_mapping", (0, 0))
            .unwrap();
        device
            .call_method1("add_qubit_tweezer_mapping", (1, 1))
            .unwrap();

        let pscz_phase = device
            .call_method0("phase_shift_controlled_z")
            .unwrap()
            .extract::<f64>()
            .unwrap();
        let pscp_phase = device
            .call_method1("phase_shift_controlled_phase", (1.0,))
            .unwrap()
            .extract::<f64>()
            .unwrap();
        assert!(pscz_phase.is_finite());
        assert!(pscp_phase.is_finite());

        device
            .call_method1("add_available_gate", ("PhaseShiftedControlledZ",))
            .unwrap();
        device
            .call_method1("add_available_gate", ("PhaseShiftedControlledPhase",))
            .unwrap();

        let gtcz_err = device.call_method1("gate_time_controlled_z", (0, 1, 0.3));
        let gtcz_ok = device.call_method1("gate_time_controlled_z", (0, 1, pscz_phase));
        assert!(gtcz_err.is_err());
        assert!(gtcz_ok.is_ok());

        let gtcp_err = device.call_method1("gate_time_controlled_phase", (0, 1, 0.3, 0.7));
        let gtcp_ok = device.call_method1("gate_time_controlled_phase", (0, 1, pscp_phase, 1.0));
        assert!(gtcp_err.is_err());
        assert!(gtcp_ok.is_ok());
    })
}

/// Test _qubit_time functions of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_qubit_times() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        device
            .call_method1("add_available_gate", ("RotateX",))
            .unwrap();
        device
            .call_method1("add_available_gate", ("PhaseShiftedControlledZ",))
            .unwrap();
        device
            .call_method1("add_available_gate", ("ControlledControlledPauliZ",))
            .unwrap();

        assert!(device
            .call_method1("single_qubit_gate_time", ("RotateX", 3))
            .is_ok());

        assert!(device
            .call_method1("two_qubit_gate_time", ("PhaseShiftedControlledZ", 3, 0))
            .is_ok());

        assert!(device
            .call_method1(
                "three_qubit_gate_time",
                ("ControlledControlledPauliZ", 3, 0, 1)
            )
            .is_ok());
    })
}

/// Test number_tweezer_positions function of EmulatorDeviceWrapper
#[test]
fn test_number_tweezer_positions() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        assert_eq!(
            device
                .call_method0("number_tweezer_positions")
                .unwrap()
                .extract::<usize>()
                .unwrap(),
            0
        );

        device
            .call_method1("add_qubit_tweezer_mapping", (0, 0))
            .unwrap();
        device
            .call_method1("add_qubit_tweezer_mapping", (1, 1))
            .unwrap();

        assert_eq!(
            device
                .call_method0("number_tweezer_positions")
                .unwrap()
                .extract::<usize>()
                .unwrap(),
            2
        );
    })
}

/// Test number_qubits function of EmulatorDeviceWrapper
#[test]
fn test_number_qubits() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        assert_eq!(
            device
                .call_method0("number_qubits")
                .unwrap()
                .extract::<usize>()
                .unwrap(),
            0
        );

        device
            .call_method1("add_qubit_tweezer_mapping", (0, 0))
            .unwrap();
        device
            .call_method1("add_qubit_tweezer_mapping", (1, 1))
            .unwrap();

        assert_eq!(
            device
                .call_method0("number_qubits")
                .unwrap()
                .extract::<usize>()
                .unwrap(),
            2
        );
    })
}

/// Test to_generic_device functions of EmulatorDeviceWrapper
#[test]
fn test_generic_device() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        let gen_dev = device.call_method0("generic_device");

        assert!(gen_dev.is_ok());

        let num_gen = gen_dev
            .unwrap()
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        let num = device
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();

        assert_eq!(num_gen, num);
    })
}

/// Test copy and deepcopy functions of EmulatorDeviceWrapper
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        let copy_op = device.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_op.extract::<EmulatorDeviceWrapper>().unwrap();

        let deepcopy_op = device.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_op.extract::<EmulatorDeviceWrapper>().unwrap();

        let device_wrapper = device.extract::<EmulatorDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, copy_wrapper);
        assert_eq!(device_wrapper, deepcopy_wrapper);
    });
}

/// Test to_json and from_json functions of EmulatorDeviceWrapper
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        let serialised = device.call_method0("to_json").unwrap();
        let deserialised = device.call_method1("from_json", (&serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = device.call_method1("from_json", (vec.clone(),));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<EmulatorDeviceWrapper>().unwrap();
        let device_wrapper = device.extract::<EmulatorDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

/// Test to_ and from_bincode functions of EmulatorDeviceWrapper
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        let serialised = device.call_method0("to_bincode").unwrap();
        let deserialised = device.call_method1("from_bincode", (&serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = device.call_method1("from_bincode", (vec.clone(),));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<EmulatorDeviceWrapper>().unwrap();
        let device_wrapper = device.extract::<EmulatorDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

/// Test convert_into_device function
#[test]
fn test_convert_to_device() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type_bound::<EmulatorDeviceWrapper>();
        let device = device_type.call0().unwrap();

        let converted = convert_into_device(&device).unwrap();
        let rust_dev: EmulatorDevice = EmulatorDevice::new(None, None, None);

        assert_eq!(converted, rust_dev);
    });
}
