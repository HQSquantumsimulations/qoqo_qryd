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

//! Integration test for Experimental Devices

use pyo3::{prelude::*, types::PyDict};

use qoqo_qryd::{ExperimentalDeviceWrapper, ExperimentalMutableDeviceWrapper};
use roqoqo_qryd::ExperimentalDevice;

/// Test new instantiation of ExperimentalDeviceWrapper and ExperimentalMutableDeviceWrapper
#[test]
fn test_new() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<ExperimentalDeviceWrapper>();
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let res = device_type.call0();
        let res_mut = device_type_mut.call0();

        assert!(res.is_ok());
        assert!(res_mut.is_ok());
    })
}

/// Test available_ switch_ and add_layout methods of ExperimentalDeviceWrapper and ExperimentalMutableDeviceWrapper
#[test]
fn test_layouts() {
    // Setup fake preconfigured device
    let mut exp = ExperimentalDevice::new();
    exp.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.23, None);
    exp.add_qubit_tweezer_mapping(0, 0).unwrap();
    exp.add_layout("OtherLayout").unwrap();
    exp.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.23, Some("OtherLayout".to_string()));
    let fake_api_device = ExperimentalDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        assert!(device.call_method1("switch_layout", ("Test",)).is_err());
        assert!(device_mut.call_method1("switch_layout", ("Test",)).is_err());

        assert!(device
            .call_method0("available_layouts")
            .unwrap()
            .contains("Default")
            .unwrap());
        assert!(device_mut
            .call_method0("available_layouts")
            .unwrap()
            .contains("Default")
            .unwrap());

        let current_layout: String = device
            .call_method0("current_layout")
            .unwrap()
            .extract()
            .unwrap();
        let current_layout_mut: String = device_mut
            .call_method0("current_layout")
            .unwrap()
            .extract()
            .unwrap();

        assert_eq!(current_layout, "Default");
        assert_eq!(current_layout_mut, "Default");

        assert!(device_mut
            .call_method1("add_layout", ("OtherLayout",))
            .is_ok());

        assert!(device
            .call_method1("switch_layout", ("OtherLayout",))
            .is_ok());
        assert!(device_mut
            .call_method1("switch_layout", ("OtherLayout",))
            .is_ok());

        assert!(device
            .call_method0("available_layouts")
            .unwrap()
            .contains("Default")
            .unwrap());
        assert!(device_mut
            .call_method0("available_layouts")
            .unwrap()
            .contains("Default")
            .unwrap());
        assert!(device
            .call_method0("available_layouts")
            .unwrap()
            .contains("OtherLayout")
            .unwrap());
        assert!(device_mut
            .call_method0("available_layouts")
            .unwrap()
            .contains("OtherLayout")
            .unwrap());
    })
}

/// Test add_qubit_tweezer_mapping function of ExperimentalDeviceWrapper and ExperimentalMutableDeviceWrapper
#[test]
fn test_qubit_tweezer_mapping() {
    // Setup fake preconfigured device
    let mut exp = ExperimentalDevice::new();
    exp.set_tweezer_single_qubit_gate_time("PauliX", 1, 0.23, None);
    let fake_api_device = ExperimentalDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("PauliX", 1, 0.23))
            .unwrap();

        assert!(device
            .call_method1("add_qubit_tweezer_mapping", (0, 1))
            .is_ok());
        assert!(device_mut
            .call_method1("add_qubit_tweezer_mapping", (0, 1))
            .is_ok());
        assert!(device
            .call_method1("add_qubit_tweezer_mapping", (1, 0))
            .is_err());
        assert!(device_mut
            .call_method1("add_qubit_tweezer_mapping", (1, 0))
            .is_err());
    })
}

#[test]
fn test_deactivate_qubit() {
    // Setup fake preconfigured device
    let mut exp = ExperimentalDevice::new();
    exp.set_tweezer_single_qubit_gate_time("PauliX", 1, 0.23, None);
    let fake_api_device = ExperimentalDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        assert!(device.call_method1("deactivate_qubit", (0,)).is_err());
        assert!(device_mut.call_method1("deactivate_qubit", (0,)).is_err());

        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("PauliX", 1, 0.23))
            .unwrap();

        device
            .call_method1("add_qubit_tweezer_mapping", (0, 1))
            .unwrap();
        device_mut
            .call_method1("add_qubit_tweezer_mapping", (0, 1))
            .unwrap();

        let res_device = device.call_method1("deactivate_qubit", (0,));
        let res_device_mut = device_mut.call_method1("deactivate_qubit", (0,));
        assert!(res_device.is_ok());
        assert!(res_device_mut.is_ok());
        assert_eq!(res_device.unwrap().extract::<&PyDict>().unwrap().len(), 0);
        assert_eq!(
            res_device_mut.unwrap().extract::<&PyDict>().unwrap().len(),
            0
        );
        assert!(device.call_method1("deactivate_qubit", (0,)).is_err());
        assert!(device_mut.call_method1("deactivate_qubit", (0,)).is_err());
    })
}

/// Test _qubit_time functions of ExperimentalDeviceWrapper and ExperimentalMutableDeviceWrapper
#[test]
fn test_qubit_times() {
    // Setup fake preconfigured device
    let mut exp = ExperimentalDevice::new();
    exp.add_layout("OtherLayout").unwrap();
    exp.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.23, Some("OtherLayout".to_string()));
    exp.set_tweezer_two_qubit_gate_time("CNOT", 0, 1, 0.13, Some("OtherLayout".to_string()));
    exp.set_tweezer_three_qubit_gate_time(
        "Toffoli",
        0,
        1,
        2,
        0.45,
        Some("OtherLayout".to_string()),
    );
    exp.set_tweezer_multi_qubit_gate_time(
        "MultiQubitZZ",
        &[0, 1, 2, 3],
        0.65,
        Some("OtherLayout".to_string()),
    );
    exp.switch_layout("OtherLayout").unwrap();
    exp.add_qubit_tweezer_mapping(0, 1).unwrap();
    exp.add_qubit_tweezer_mapping(1, 2).unwrap();
    exp.add_qubit_tweezer_mapping(2, 3).unwrap();
    exp.add_qubit_tweezer_mapping(3, 0).unwrap();
    let fake_api_device = ExperimentalDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        device_mut
            .call_method1("add_layout", ("OtherLayout",))
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("PauliX", 0, 0.23, "OtherLayout"),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("CNOT", 0, 1, 0.13, "OtherLayout"),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_three_qubit_gate_time",
                ("Toffoli", 0, 1, 2, 0.45, "OtherLayout"),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_multi_qubit_gate_time",
                ("MultiQubitZZ", vec![0, 1, 2, 3], 0.6, "OtherLayout"),
            )
            .unwrap();
        device_mut
            .call_method1("switch_layout", ("OtherLayout",))
            .unwrap();

        device_mut
            .call_method1("add_qubit_tweezer_mapping", (0, 1))
            .unwrap();
        device_mut
            .call_method1("add_qubit_tweezer_mapping", (1, 2))
            .unwrap();
        device_mut
            .call_method1("add_qubit_tweezer_mapping", (2, 3))
            .unwrap();
        device_mut
            .call_method1("add_qubit_tweezer_mapping", (3, 0))
            .unwrap();

        assert!(device
            .call_method1("switch_layout", ("OtherLayout",))
            .is_ok());
        assert!(device_mut
            .call_method1("switch_layout", ("OtherLayout",))
            .is_ok());

        assert!(device
            .call_method1("single_qubit_gate_time", ("PauliX", 3))
            .is_ok());
        assert!(device_mut
            .call_method1("single_qubit_gate_time", ("PauliX", 3))
            .is_ok());
        assert!(device
            .call_method1("two_qubit_gate_time", ("CNOT", 3, 0))
            .is_ok());
        assert!(device_mut
            .call_method1("two_qubit_gate_time", ("CNOT", 3, 0))
            .is_ok());
        assert!(device
            .call_method1("three_qubit_gate_time", ("Toffoli", 3, 0, 1))
            .is_ok());
        assert!(device_mut
            .call_method1("three_qubit_gate_time", ("Toffoli", 3, 0, 1))
            .is_ok());
        assert!(device
            .call_method1("multi_qubit_gate_time", ("MultiQubitZZ", vec![3, 0, 1, 2]))
            .is_ok());
        assert!(device_mut
            .call_method1("multi_qubit_gate_time", ("MultiQubitZZ", vec![3, 0, 1, 2]))
            .is_ok());
    })
}

/// Test number_qubits function of ExperimentalDeviceWrapper and ExperimentalMutableDeviceWrapper
#[test]
fn test_number_qubits() {
    // Setup fake preconfigured device
    let mut exp = ExperimentalDevice::new();
    exp.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.23, None);
    exp.set_tweezer_single_qubit_gate_time("PauliX", 1, 0.23, None);
    let fake_api_device = ExperimentalDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("PauliX", 0, 0.23))
            .unwrap();
        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("PauliX", 1, 0.23))
            .unwrap();

        assert_eq!(
            device
                .call_method0("number_qubits")
                .unwrap()
                .extract::<usize>()
                .unwrap(),
            0
        );
        assert_eq!(
            device_mut
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
        device_mut
            .call_method1("add_qubit_tweezer_mapping", (0, 0))
            .unwrap();
        device_mut
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
        assert_eq!(
            device_mut
                .call_method0("number_qubits")
                .unwrap()
                .extract::<usize>()
                .unwrap(),
            2
        );
    })
}

/// Test to_generic_device functions of ExperimentalDeviceWrapper and ExperimentalMutableDeviceWrapper
#[test]
fn test_generic_device() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<ExperimentalDeviceWrapper>();
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let device = device_type.call0().unwrap();
        let device_mut = device_type_mut.call0().unwrap();

        let gen_dev = device.call_method0("generic_device");
        let gen_dev_mut = device_mut.call_method0("generic_device");

        assert!(gen_dev.is_ok());
        assert!(gen_dev_mut.is_ok());

        let num_gen = gen_dev
            .unwrap()
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        let num_gen_mut = gen_dev_mut
            .unwrap()
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();

        assert_eq!(num_gen, num_gen_mut);
    })
}

/// Test copy and deepcopy functions of ExperimentalDeviceWrapper and ExperimentalMutableDeviceWrapper
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<ExperimentalDeviceWrapper>();
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let device = device_type.call0().unwrap();
        let device_mut = device_type_mut.call0().unwrap();

        let copy_op = device.call_method0("__copy__").unwrap();
        let copy_op_mut = device_mut.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_op.extract::<ExperimentalDeviceWrapper>().unwrap();
        let copy_wrapper_mut = copy_op_mut
            .extract::<ExperimentalMutableDeviceWrapper>()
            .unwrap();
        let deepcopy_op = device.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_op_mut = device_mut.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_op.extract::<ExperimentalDeviceWrapper>().unwrap();
        let deepcopy_wrapper_mut = deepcopy_op_mut
            .extract::<ExperimentalMutableDeviceWrapper>()
            .unwrap();

        let device_wrapper = device.extract::<ExperimentalDeviceWrapper>().unwrap();
        let device_wrapper_mut = device_mut
            .extract::<ExperimentalMutableDeviceWrapper>()
            .unwrap();
        assert_eq!(device_wrapper, copy_wrapper);
        assert_eq!(device_wrapper, deepcopy_wrapper);
        assert_eq!(device_wrapper_mut, copy_wrapper_mut);
        assert_eq!(device_wrapper_mut, deepcopy_wrapper_mut);
    });
}

/// Test to_ and from_json functions of ExperimentalDeviceWrapper and ExperimentalMutableDeviceWrapper
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<ExperimentalDeviceWrapper>();
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let device = device_type.call0().unwrap();
        let device_mut = device_type_mut.call0().unwrap();

        let serialised = device.call_method0("to_json").unwrap();
        let serialised_mut = device_mut.call_method0("to_json").unwrap();
        let deserialised = device.call_method1("from_json", (serialised,)).unwrap();
        let deserialised_mut = device_mut
            .call_method1("from_json", (serialised_mut,))
            .unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = device.call_method1("from_json", (vec.clone(),));
        assert!(deserialised_error.is_err());
        let deserialised_mut_error = device_mut.call_method1("from_json", (vec,));
        assert!(deserialised_mut_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());
        let deserialised_error_mut = deserialised_mut.call_method0("from_json");
        assert!(deserialised_error_mut.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());
        let serialised_error_mut = serialised_mut.call_method0("to_json");
        assert!(serialised_error_mut.is_err());

        let serde_wrapper = deserialised.extract::<ExperimentalDeviceWrapper>().unwrap();
        let serde_wrapper_mut = deserialised_mut
            .extract::<ExperimentalMutableDeviceWrapper>()
            .unwrap();
        let device_wrapper = device.extract::<ExperimentalDeviceWrapper>().unwrap();
        let device_wrapper_mut = device_mut
            .extract::<ExperimentalMutableDeviceWrapper>()
            .unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
        assert_eq!(device_wrapper_mut, serde_wrapper_mut);
    });
}

/// Test to_ and from_bincode functions of ExperimentalDeviceWrapper and ExperimentalMutableDeviceWrapper
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<ExperimentalDeviceWrapper>();
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let device = device_type.call0().unwrap();
        let device_mut = device_type_mut.call0().unwrap();

        let serialised = device.call_method0("to_bincode").unwrap();
        let serialised_mut = device_mut.call_method0("to_bincode").unwrap();
        let deserialised = device.call_method1("from_bincode", (serialised,)).unwrap();
        let deserialised_mut = device_mut
            .call_method1("from_bincode", (serialised_mut,))
            .unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = device.call_method1("from_bincode", (vec.clone(),));
        assert!(deserialised_error.is_err());
        let deserialised_mut_error = device_mut.call_method1("from_bincode", (vec,));
        assert!(deserialised_mut_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());
        let deserialised_error_mut = deserialised_mut.call_method0("from_bincode");
        assert!(deserialised_error_mut.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());
        let serialised_error_mut = serialised_mut.call_method0("to_bincode");
        assert!(serialised_error_mut.is_err());

        let serde_wrapper = deserialised.extract::<ExperimentalDeviceWrapper>().unwrap();
        let serde_wrapper_mut = deserialised_mut
            .extract::<ExperimentalMutableDeviceWrapper>()
            .unwrap();
        let device_wrapper = device.extract::<ExperimentalDeviceWrapper>().unwrap();
        let device_wrapper_mut = device_mut
            .extract::<ExperimentalMutableDeviceWrapper>()
            .unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
        assert_eq!(device_wrapper_mut, serde_wrapper_mut);
    });
}

/// Test two_qubit_edges function of ExperimentalDeviceWrapper and ExperimentalMutableDeviceWrapper
#[test]
fn test_two_qubit_edges() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<ExperimentalDeviceWrapper>();
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let device = device_type.call0().unwrap();
        let device_mut = device_type_mut.call0().unwrap();

        let edges = device.call_method0("two_qubit_edges").unwrap();
        let edges_mut = device_mut.call_method0("two_qubit_edges").unwrap();
        let edges_wrapper = edges.extract::<Vec<usize>>().unwrap();
        let edges_wrapper_mut = edges_mut.extract::<Vec<usize>>().unwrap();
        assert_eq!(edges_wrapper.len(), 0);
        assert_eq!(edges_wrapper_mut.len(), 0);
    });
}
