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

//! Integration test for Tweezer Devices

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{IntoPyDict, PyDict},
};
#[cfg(feature = "web-api")]
use serde_json::Value;

use qoqo_qryd::{
    tweezer_devices::convert_into_device, TweezerDeviceWrapper, TweezerMutableDeviceWrapper,
};
use roqoqo_qryd::{phi_theta_relation, TweezerDevice};

#[cfg(feature = "web-api")]
use wiremock::matchers::method;
#[cfg(feature = "web-api")]
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Test new instantiation of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_new() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<TweezerDeviceWrapper>();
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let res = device_type.call1((2,));
        let res_mut = device_type_mut.call1((2,));

        assert!(res.is_ok());
        assert!(res_mut.is_ok());

        let res = res.unwrap();
        let res_mut = res_mut.unwrap();
        assert_eq!(
            res.call_method0("qrydbackend")
                .unwrap()
                .extract::<String>()
                .unwrap(),
            "qryd_tweezer_device"
        );
        assert_eq!(
            res_mut
                .call_method0("qrydbackend")
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
        assert_eq!(
            res_mut
                .call_method0("seed")
                .unwrap()
                .extract::<Option<usize>>()
                .unwrap(),
            Some(2)
        );

        let res_emp = device_type.call0().unwrap();
        let res_mut_emp = device_type_mut.call0().unwrap();

        assert_eq!(
            res_emp
                .call_method0("seed")
                .unwrap()
                .extract::<Option<usize>>()
                .unwrap(),
            None
        );
        assert_eq!(
            res_mut_emp
                .call_method0("seed")
                .unwrap()
                .extract::<Option<usize>>()
                .unwrap(),
            None
        );
    })
}

/// Test from_mutable() of TweezerDeviceWrapper
#[test]
fn test_from_mutable() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<TweezerDeviceWrapper>();
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device_mut = device_type_mut.call0().unwrap();

        device_mut
            .call_method1("add_layout", ("triangle",))
            .unwrap();

        let dev = device_type.call_method1("from_mutable", (device_mut,));

        assert!(dev.is_ok());
        assert!(dev
            .unwrap()
            .call_method0("available_layouts")
            .unwrap()
            .extract::<Vec<String>>()
            .unwrap()
            .contains(&"triangle".to_string()))
    })
}

/// Test available_layout() switch_layout(), add_layout() and set_tweezers_per_row() methods of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_layouts() {
    // Setup fake preconfigured device
    let mut exp = TweezerDevice::new(None, None, None);
    exp.add_layout("default").unwrap();
    exp.current_layout = Some("default".to_string());
    exp.set_tweezer_single_qubit_gate_time("RotateX", 0, 0.23, None)
        .unwrap();
    exp.add_qubit_tweezer_mapping(0, 0).unwrap();
    exp.add_layout("OtherLayout").unwrap();
    exp.set_tweezer_single_qubit_gate_time("RotateX", 0, 0.23, Some("OtherLayout".to_string()))
        .unwrap();
    let fake_api_device = TweezerDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();
        device_mut.call_method1("add_layout", ("default",)).unwrap();
        device_mut
            .call_method1("switch_layout", ("default",))
            .unwrap();

        assert!(device.call_method1("switch_layout", ("Test",)).is_err());
        assert!(device_mut.call_method1("switch_layout", ("Test",)).is_err());

        assert!(device
            .call_method0("available_layouts")
            .unwrap()
            .contains("default")
            .unwrap());
        assert!(device_mut
            .call_method0("available_layouts")
            .unwrap()
            .contains("default")
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

        assert_eq!(current_layout, "default");
        assert_eq!(current_layout_mut, "default");

        assert!(device_mut
            .call_method1("set_tweezers_per_row", ("error",))
            .is_err());

        assert!(device_mut
            .call_method1("set_tweezers_per_row", (vec![0],))
            .is_ok());

        assert!(device_mut
            .call_method1("add_layout", ("OtherLayout",))
            .is_ok());

        assert!(device_mut
            .call_method1("set_tweezers_per_row", (vec![1], "OtherLayout",))
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
            .contains("default")
            .unwrap());
        assert!(device_mut
            .call_method0("available_layouts")
            .unwrap()
            .contains("default")
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

/// Test add_qubit_tweezer_mapping function of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_qubit_tweezer_mapping() {
    // Setup fake preconfigured device
    let mut exp = TweezerDevice::new(None, None, None);
    exp.add_layout("default").unwrap();
    exp.current_layout = Some("default".to_string());
    exp.set_tweezer_single_qubit_gate_time("RotateX", 1, 0.23, None)
        .unwrap();
    exp.set_tweezer_single_qubit_gate_time("RotateX", 2, 0.23, None)
        .unwrap();
    let fake_api_device = TweezerDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        assert!(device_mut
            .call_method0("get_qubit_to_tweezer_mapping")
            .unwrap()
            .is_none());

        device_mut.call_method1("add_layout", ("default",)).unwrap();

        assert!(device_mut
            .call_method1("switch_layout", ("default", true))
            .is_ok());

        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("RotateX", 1, 0.23))
            .unwrap();
        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("RotateX", 2, 0.23))
            .unwrap();

        assert!(device_mut
            .call_method0("get_qubit_to_tweezer_mapping")
            .unwrap()
            .is_none());

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

        let ex_dict: &PyDict = [(0, 1)].into_py_dict(py);
        assert!(device_mut
            .call_method0("get_qubit_to_tweezer_mapping")
            .unwrap()
            .eq(ex_dict)
            .unwrap());
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
            2
        );

        ex_dict.set_item(3, 2).unwrap();
        assert!(device
            .call_method0("get_qubit_to_tweezer_mapping")
            .unwrap()
            .eq(ex_dict)
            .unwrap());

        device_mut
            .call_method1("add_layout", ("test_trivial_population",))
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 1, 0.23, "test_trivial_population"),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 2, 0.23, "test_trivial_population"),
            )
            .unwrap();
        device_mut
            .call_method1("switch_layout", ("test_trivial_population", false))
            .unwrap();

        assert!(device_mut
            .call_method0("get_qubit_to_tweezer_mapping")
            .unwrap()
            .is_none());

        device_mut
            .call_method1("switch_layout", ("test_trivial_population", true))
            .unwrap();

        let ex_dict: &PyDict = [(0, 0), (1, 1), (2, 2)].into_py_dict(py);
        assert!(device_mut
            .call_method0("get_qubit_to_tweezer_mapping")
            .unwrap()
            .eq(ex_dict)
            .unwrap());
    })
}

/// Test set_allowed_tweezer_shifts of TweeerDeviceMutableWrapper
#[test]
fn test_allowed_tweezer_shifts() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device_mut = device_type_mut.call0().unwrap();

        device_mut.call_method1("add_layout", ("default",)).unwrap();
        device_mut
            .call_method1("switch_layout", ("default",))
            .unwrap();
        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("RotateX", 0, 0.23))
            .unwrap();
        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("RotateX", 1, 0.23))
            .unwrap();
        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("RotateX", 2, 0.23))
            .unwrap();

        assert!(device_mut
            .call_method1("set_allowed_tweezer_shifts", (0, vec![vec![1], vec![2]]))
            .is_ok());
        assert!(device_mut
            .call_method1("set_allowed_tweezer_shifts", (3, vec![vec![1], vec![2]]))
            .is_err());
        assert!(device_mut
            .call_method1("set_allowed_tweezer_shifts", (0, vec![vec![0], vec![2]]))
            .is_err());
        assert!(device_mut
            .call_method1("set_allowed_tweezer_shifts", (1, vec![vec![5], vec![2]]))
            .is_err());
    })
}

/// Test set_allowed_tweezer_shifts_from_rows of TweeerDeviceMutableWrapper
#[test]
fn test_allowed_tweezer_shifts_from_rows() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device_mut = device_type_mut.call0().unwrap();

        device_mut.call_method1("add_layout", ("default",)).unwrap();
        device_mut
            .call_method1("switch_layout", ("default",))
            .unwrap();
        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("RotateX", 0, 0.23))
            .unwrap();
        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("RotateX", 1, 0.23))
            .unwrap();
        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("RotateX", 2, 0.23))
            .unwrap();

        assert!(device_mut
            .call_method1(
                "set_allowed_tweezer_shifts_from_rows",
                (vec![vec![0, 1, 2]],)
            )
            .is_ok());
        assert!(device_mut
            .call_method1("set_allowed_tweezer_shifts_from_rows", (vec![vec![1, 5]],))
            .is_err());
        assert!(device_mut
            .call_method1("set_allowed_tweezer_shifts_from_rows", (vec![vec![0, 0]],))
            .is_err());
    })
}

/// Test allow_reset for TweezerMutableDeviceWrapper
#[test]
fn test_allow_reset() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device_mut = device_type_mut.call0().unwrap();

        assert!(device_mut.call_method1("set_allow_reset", (true,)).is_ok());
    })
}

/// Test deactivate_qubit function of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_deactivate_qubit() {
    // Setup fake preconfigured device
    let mut exp = TweezerDevice::new(None, None, None);
    exp.add_layout("default").unwrap();
    exp.current_layout = Some("default".to_string());
    exp.set_tweezer_single_qubit_gate_time("RotateX", 1, 0.23, None)
        .unwrap();
    exp.set_tweezer_single_qubit_gate_time("RotateZ", 0, 0.23, None)
        .unwrap();
    let fake_api_device = TweezerDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        assert!(device.call_method1("deactivate_qubit", (0,)).is_err());
        assert!(device_mut.call_method1("deactivate_qubit", (0,)).is_err());

        device_mut.call_method1("add_layout", ("default",)).unwrap();
        device_mut
            .call_method1("switch_layout", ("default",))
            .unwrap();
        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("RotateX", 1, 0.23))
            .unwrap();
        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("RotateZ", 0, 0.23))
            .unwrap();
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

        let res_device = device.call_method1("deactivate_qubit", (0,));
        let res_device_mut = device_mut.call_method1("deactivate_qubit", (0,));
        assert!(res_device.is_ok());
        assert!(res_device_mut.is_ok());
        assert_eq!(res_device.unwrap().extract::<&PyDict>().unwrap().len(), 1);
        assert_eq!(
            res_device_mut.unwrap().extract::<&PyDict>().unwrap().len(),
            1
        );
        assert!(device.call_method1("deactivate_qubit", (0,)).is_err());
        assert!(device_mut.call_method1("deactivate_qubit", (0,)).is_err());
    })
}

/// Test _qubit_time functions of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_qubit_times() {
    // Setup fake preconfigured device
    let mut exp = TweezerDevice::new(None, None, None);
    exp.add_layout("OtherLayout").unwrap();
    exp.set_tweezer_single_qubit_gate_time("RotateX", 0, 0.23, Some("OtherLayout".to_string()))
        .unwrap();
    exp.set_tweezer_two_qubit_gate_time(
        "PhaseShiftedControlledZ",
        0,
        1,
        0.13,
        Some("OtherLayout".to_string()),
    )
    .unwrap();
    exp.set_tweezer_three_qubit_gate_time(
        "ControlledControlledPauliZ",
        0,
        1,
        2,
        0.45,
        Some("OtherLayout".to_string()),
    )
    .unwrap();
    // TODO: Add this back in when the backend supports multi-qubit gates
    // exp.set_tweezer_multi_qubit_gate_time(
    //     "MultiQubitZZ",
    //     &[0, 1, 2, 3],
    //     0.65,
    //     Some("OtherLayout".to_string()),
    // )
    // .unwrap();
    exp.switch_layout("OtherLayout", None).unwrap();
    exp.add_qubit_tweezer_mapping(0, 1).unwrap();
    exp.add_qubit_tweezer_mapping(1, 2).unwrap();
    // exp.add_qubit_tweezer_mapping(2, 3).unwrap();
    exp.add_qubit_tweezer_mapping(3, 0).unwrap();
    let fake_api_device = TweezerDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        device_mut
            .call_method1("add_layout", ("OtherLayout",))
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 0.23, "OtherLayout"),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 0.13, "OtherLayout"),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_three_qubit_gate_time",
                ("ControlledControlledPauliZ", 0, 1, 2, 0.45, "OtherLayout"),
            )
            .unwrap();
        // TODO: Add this back in when the backend supports multi-qubit gates
        // device_mut
        //     .call_method1(
        //         "set_tweezer_multi_qubit_gate_time",
        //         ("MultiQubitZZ", vec![0, 1, 2, 3], 0.6, "OtherLayout"),
        //     )
        //     .unwrap();
        device_mut
            .call_method1("switch_layout", ("OtherLayout",))
            .unwrap();

        device_mut
            .call_method1("add_qubit_tweezer_mapping", (0, 1))
            .unwrap();
        device_mut
            .call_method1("add_qubit_tweezer_mapping", (1, 2))
            .unwrap();
        // TODO: Add this back in when the backend supports multi-qubit gates
        // device_mut
        //     .call_method1("add_qubit_tweezer_mapping", (2, 3))
        //     .unwrap();
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
            .call_method1("single_qubit_gate_time", ("RotateX", 3))
            .is_ok());
        assert!(device_mut
            .call_method1("single_qubit_gate_time", ("RotateX", 3))
            .is_ok());
        assert!(device
            .call_method1("two_qubit_gate_time", ("PhaseShiftedControlledZ", 3, 0))
            .is_ok());
        assert!(device_mut
            .call_method1("two_qubit_gate_time", ("PhaseShiftedControlledZ", 3, 0))
            .is_ok());
        assert!(device
            .call_method1(
                "three_qubit_gate_time",
                ("ControlledControlledPauliZ", 3, 0, 1)
            )
            .is_ok());
        assert!(device_mut
            .call_method1(
                "three_qubit_gate_time",
                ("ControlledControlledPauliZ", 3, 0, 1)
            )
            .is_ok());
        // TODO: Add this back in when the backend supports multi-qubit gates
        // assert!(device
        //     .call_method1("multi_qubit_gate_time", ("MultiQubitZZ", vec![3, 0, 1, 2]))
        //     .is_ok());
        // assert!(device_mut
        //     .call_method1("multi_qubit_gate_time", ("MultiQubitZZ", vec![3, 0, 1, 2]))
        //     .is_ok());
    })
}

/// Test number_qubits function of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_number_qubits() {
    // Setup fake preconfigured device
    let mut exp = TweezerDevice::new(None, None, None);
    exp.add_layout("default").unwrap();
    exp.switch_layout("default", None).unwrap();
    exp.set_tweezer_single_qubit_gate_time("RotateX", 0, 0.23, None)
        .unwrap();
    exp.set_tweezer_single_qubit_gate_time("RotateX", 1, 0.23, None)
        .unwrap();
    let fake_api_device = TweezerDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        device_mut.call_method1("add_layout", ("default",)).unwrap();
        device_mut
            .call_method1("switch_layout", ("default",))
            .unwrap();

        assert_eq!(
            device_mut
                .call_method0("number_qubits")
                .unwrap()
                .extract::<usize>()
                .unwrap(),
            0
        );

        device_mut
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 0.23, "default"),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 1, 0.23, "default"),
            )
            .unwrap();

        // Setting tweezer times should not affect the number of qubits
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

/// Test number_tweezer_positions function of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_number_tweezer_positions() {
    // Setup fake preconfigured device
    let mut exp = TweezerDevice::new(None, None, None);
    exp.add_layout("default").unwrap();
    exp.switch_layout("default", None).unwrap();
    exp.set_tweezer_single_qubit_gate_time("RotateX", 0, 0.23, None)
        .unwrap();
    exp.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledZ", 1, 2, 0.23, None)
        .unwrap();
    let fake_api_device = TweezerDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        device_mut.call_method1("add_layout", ("default",)).unwrap();
        device_mut.call_method1("add_layout", ("empty",)).unwrap();

        assert_eq!(
            device_mut
                .call_method1("number_tweezer_positions", ("empty",))
                .unwrap()
                .extract::<usize>()
                .unwrap(),
            0
        );

        assert!(device_mut.call_method0("number_tweezer_positions").is_err());
        assert!(device_mut
            .call_method1("number_tweezer_positions", ("error",))
            .is_err());

        device_mut
            .call_method1("switch_layout", ("default",))
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 0.23, "default"),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 1, 2, 0.23, "default"),
            )
            .unwrap();

        // Setting tweezer times should not affect the number of qubits
        assert_eq!(
            device
                .call_method0("number_tweezer_positions")
                .unwrap()
                .extract::<usize>()
                .unwrap(),
            3
        );
        assert_eq!(
            device_mut
                .call_method0("number_tweezer_positions")
                .unwrap()
                .extract::<usize>()
                .unwrap(),
            3
        );
    })
}

/// Test to_generic_device functions of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_generic_device() {
    // Setup fake preconfigured device
    let mut tw = TweezerDevice::new(None, None, None);
    tw.add_layout("default").unwrap();
    tw.current_layout = Some("default".to_string());
    tw.set_tweezer_single_qubit_gate_time("RotateX", 0, 0.23, None)
        .unwrap();
    tw.set_tweezer_single_qubit_gate_time("RotateX", 1, 0.23, None)
        .unwrap();
    tw.switch_layout("default", None).unwrap();
    let fake_api_device = TweezerDeviceWrapper { internal: tw };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        device_mut.call_method1("add_layout", ("default",)).unwrap();
        device_mut
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 0.23, "default"),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 1, 0.23, "default"),
            )
            .unwrap();
        device_mut
            .call_method1("switch_layout", ("default",))
            .unwrap();

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

/// Test copy and deepcopy functions of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<TweezerDeviceWrapper>();
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = device_type.call0().unwrap();
        let device_mut = device_type_mut.call0().unwrap();

        let copy_op = device.call_method0("__copy__").unwrap();
        let copy_op_mut = device_mut.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_op.extract::<TweezerDeviceWrapper>().unwrap();
        let copy_wrapper_mut = copy_op_mut
            .extract::<TweezerMutableDeviceWrapper>()
            .unwrap();
        let deepcopy_op = device.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_op_mut = device_mut.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_op.extract::<TweezerDeviceWrapper>().unwrap();
        let deepcopy_wrapper_mut = deepcopy_op_mut
            .extract::<TweezerMutableDeviceWrapper>()
            .unwrap();

        let device_wrapper = device.extract::<TweezerDeviceWrapper>().unwrap();
        let device_wrapper_mut = device_mut.extract::<TweezerMutableDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, copy_wrapper);
        assert_eq!(device_wrapper, deepcopy_wrapper);
        assert_eq!(device_wrapper_mut, copy_wrapper_mut);
        assert_eq!(device_wrapper_mut, deepcopy_wrapper_mut);
    });
}

/// Test to_ and from_json functions of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_to_from_json() {
    // Setup fake preconfigured device
    let mut ext = TweezerDevice::new(None, None, None);
    ext.add_layout("default").unwrap();
    ext.current_layout = Some("default".to_string());
    ext.set_tweezer_single_qubit_gate_time("PhaseShiftState1", 0, 0.23, None)
        .unwrap();
    ext.set_tweezer_single_qubit_gate_time("PhaseShiftState1", 1, 0.23, None)
        .unwrap();
    let fake_api_device = TweezerDeviceWrapper { internal: ext };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type = py.get_type::<TweezerDeviceWrapper>();
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_empty = device_type.call0().unwrap();
        let device_mut = device_type_mut.call0().unwrap();

        device_mut
            .call_method1("add_layout", ("Triangle",))
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateZ", 0, 0.23, "Triangle"),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledPhase", 0, 1, 0.13, "Triangle"),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_three_qubit_gate_time",
                ("ControlledControlledPhaseShift", 0, 1, 2, 0.13, "Triangle"),
            )
            .unwrap();

        device_mut
            .call_method1("set_allowed_tweezer_shifts", (0, vec![vec![1]], "Triangle"))
            .unwrap();

        device_mut
            .call_method1("set_default_layout", ("Triangle",))
            .unwrap();

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

        let serde_wrapper = deserialised.extract::<TweezerDeviceWrapper>().unwrap();
        let serde_wrapper_mut = deserialised_mut
            .extract::<TweezerMutableDeviceWrapper>()
            .unwrap();
        let device_wrapper = device.extract::<TweezerDeviceWrapper>().unwrap();
        let device_wrapper_mut = device_mut.extract::<TweezerMutableDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
        assert_eq!(device_wrapper_mut, serde_wrapper_mut);

        let device_from_mut = device_type.call0().unwrap();
        let deserialized_dev_from_mut_dev = device_from_mut
            .call_method1("from_json", (serialised_mut,))
            .unwrap();
        let device_from_mut_wrapper = deserialized_dev_from_mut_dev
            .extract::<TweezerDeviceWrapper>()
            .unwrap();
        assert_eq!(device_from_mut_wrapper.current_layout(), "Triangle");

        let serialised_empty = device_empty.call_method0("to_json");
        assert!(serialised_empty.is_err());
        assert_eq!(
            serialised_empty.unwrap_err().to_string(),
            PyValueError::new_err(
                "The device does not have valid QRyd gates available. ".to_owned() +
                "The valid gates are RotateXY, PhaseShiftState1 and PhaseShiftedControlledPhase.",
            ).to_string()
        );
    });
}

/// Test to_ and from_bincode functions of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<TweezerDeviceWrapper>();
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = device_type.call0().unwrap();
        let device_mut = device_type_mut.call0().unwrap();

        device_mut.call_method1("add_layout", ("default",)).unwrap();
        device_mut
            .call_method1("switch_layout", ("default",))
            .unwrap();
        device_mut
            .call_method1("set_tweezer_single_qubit_gate_time", ("RotateZ", 0, 0.23))
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledPhase", 0, 1, 0.13),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_three_qubit_gate_time",
                ("ControlledControlledPhaseShift", 0, 1, 2, 0.13),
            )
            .unwrap();
        device_mut
            .call_method1("set_allowed_tweezer_shifts", (0, vec![vec![1]]))
            .unwrap();

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

        let serde_wrapper = deserialised.extract::<TweezerDeviceWrapper>().unwrap();
        let serde_wrapper_mut = deserialised_mut
            .extract::<TweezerMutableDeviceWrapper>()
            .unwrap();
        let device_wrapper = device.extract::<TweezerDeviceWrapper>().unwrap();
        let device_wrapper_mut = device_mut.extract::<TweezerMutableDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
        assert_eq!(device_wrapper_mut, serde_wrapper_mut);
    });
}

/// Test two_qubit_edges function of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_two_qubit_edges() {
    // Setup fake preconfigured device
    let mut ext = TweezerDevice::new(None, None, None);
    ext.add_layout("default").unwrap();
    ext.current_layout = Some("default".to_string());
    let fake_api_device = TweezerDeviceWrapper { internal: ext };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        device_mut.call_method1("add_layout", ("default",)).unwrap();
        device_mut
            .call_method1("switch_layout", ("default",))
            .unwrap();

        let edges = device.call_method0("two_qubit_edges").unwrap();
        let edges_mut = device_mut.call_method0("two_qubit_edges").unwrap();
        let edges_wrapper = edges.extract::<Vec<(usize, usize)>>().unwrap();
        let edges_wrapper_mut = edges_mut.extract::<Vec<(usize, usize)>>().unwrap();
        assert_eq!(edges_wrapper.len(), 0);
        assert_eq!(edges_wrapper_mut.len(), 0);

        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledPhase", 0, 1, 0.13),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 1, 2, 0.13),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 2, 0.13),
            )
            .unwrap();
        device_mut
            .call_method1("add_qubit_tweezer_mapping", (0, 0))
            .unwrap();
        device_mut
            .call_method1("add_qubit_tweezer_mapping", (1, 1))
            .unwrap();
        device_mut
            .call_method1("add_qubit_tweezer_mapping", (2, 2))
            .unwrap();

        let new_edges_mut = device_mut
            .call_method0("two_qubit_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();
        assert_eq!(new_edges_mut.len(), 3);
        assert!(new_edges_mut.contains(&(0, 1)));
        assert!(new_edges_mut.contains(&(1, 2)));
        assert!(new_edges_mut.contains(&(0, 2)));
    });
}

/// Test from_api of TweezerDeviceWrapper
#[tokio::test]
#[cfg(feature = "web-api")]
async fn test_from_api() {
    let mut sent_device = TweezerDevice::new(None, None, None);
    sent_device.add_layout("triangle").unwrap();
    sent_device
        .set_tweezer_single_qubit_gate_time(
            "PhaseShiftState1",
            0,
            0.23,
            Some("triangle".to_string()),
        )
        .unwrap();
    sent_device.set_default_layout("triangle").unwrap();
    let mut received_device = TweezerDevice::new(None, None, None);
    received_device.add_layout("triangle").unwrap();
    received_device
        .set_tweezer_single_qubit_gate_time(
            "PhaseShiftState1",
            0,
            0.23,
            Some("triangle".to_string()),
        )
        .unwrap();
    received_device.switch_layout("triangle", None).unwrap();
    received_device.set_default_layout("triangle").unwrap();
    let received_device_wrapper = TweezerDeviceWrapper {
        internal: received_device.clone(),
    };
    let wiremock_server = MockServer::start().await;
    let _mock = Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&sent_device))
        .expect(1)
        .mount(&wiremock_server)
        .await;

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        // let sent_device_type = sent_device_wrapper.into_py(py);
        // let sent_device_py = sent_device_type.as_ref(py);
        let received_device_type = received_device_wrapper.into_py(py);
        let received_device_py = received_device_type.as_ref(py);

        let device_type = py.get_type::<TweezerDeviceWrapper>();

        let device = device_type
            .call_method1(
                "from_api",
                (
                    Option::<String>::None,
                    Option::<String>::None,
                    wiremock_server.address().port().to_string(),
                    42,
                ),
            )
            .unwrap();

        assert_eq!(
            device
                .call_method0("current_layout")
                .unwrap()
                .extract::<String>()
                .unwrap(),
            "triangle"
        );
        assert_eq!(
            device
                .call_method0("seed")
                .unwrap()
                .extract::<usize>()
                .unwrap(),
            42
        );

        let returned_device_string = device
            .call_method0("to_json")
            .unwrap()
            .extract::<String>()
            .unwrap();
        let original_device_string = received_device_py
            .call_method0("to_json")
            .unwrap()
            .extract::<String>()
            .unwrap();
        let return_device_json: Value = serde_json::from_str(&returned_device_string).unwrap();
        let original_device_json: Value = serde_json::from_str(&original_device_string).unwrap();

        assert_eq!(
            return_device_json.get("qubit_to_tweezer").unwrap(),
            original_device_json.get("qubit_to_tweezer").unwrap()
        );
        assert!(return_device_json
            .get("layout_register")
            .unwrap()
            .get("triangle")
            .is_some());
        assert!(original_device_json
            .get("layout_register")
            .unwrap()
            .get("triangle")
            .is_some());
        assert_eq!(
            return_device_json.get("current_layout").unwrap(),
            original_device_json.get("current_layout").unwrap()
        );
        assert_eq!(
            return_device_json
                .get("controlled_z_phase_relation")
                .unwrap(),
            original_device_json
                .get("controlled_z_phase_relation")
                .unwrap()
        );
        assert_eq!(
            return_device_json
                .get("controlled_phase_phase_relation")
                .unwrap(),
            original_device_json
                .get("controlled_phase_phase_relation")
                .unwrap()
        );
        assert_eq!(
            return_device_json.get("default_layout").unwrap(),
            original_device_json.get("default_layout").unwrap()
        );
    });
    wiremock_server.verify().await;
}

/// Test convert_into_device function
#[test]
fn test_convert_to_device() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<TweezerDeviceWrapper>();
        let device = device_type.call0().unwrap();

        let converted = convert_into_device(device).unwrap();
        let rust_dev: TweezerDevice = TweezerDevice::new(None, None, None);

        assert_eq!(converted, rust_dev);
    });
}

/// Test phase_shift_controlled_... and gate_time_controlled_...  methods
#[test]
fn test_phi_theta_relations() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device_type = py.get_type::<TweezerDeviceWrapper>();
        let device_f = device_type_mut
            .call1((Option::<usize>::None, "2.15", "2.13"))
            .unwrap();
        let device = device_type.call0().unwrap();
        let device_mut = device_type_mut.call0().unwrap();

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

        device_mut.call_method1("add_layout", ("default",)).unwrap();
        device_mut
            .call_method1("switch_layout", ("default",))
            .unwrap();

        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledZ", 0, 1, 0.13),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledPhase", 0, 1, 0.13),
            )
            .unwrap();
        device_mut
            .call_method1("add_qubit_tweezer_mapping", (0, 0))
            .unwrap();
        device_mut
            .call_method1("add_qubit_tweezer_mapping", (1, 1))
            .unwrap();

        let pscz_phase = device_mut
            .call_method0("phase_shift_controlled_z")
            .unwrap()
            .extract::<f64>()
            .unwrap();
        let pscp_phase = device_mut
            .call_method1("phase_shift_controlled_phase", (1.0,))
            .unwrap()
            .extract::<f64>()
            .unwrap();
        assert!(pscz_phase.is_finite());
        assert!(pscp_phase.is_finite());

        let gtcz_err = device_mut.call_method1("gate_time_controlled_z", (0, 1, 0.3));
        let gtcz_ok = device_mut.call_method1("gate_time_controlled_z", (0, 1, pscz_phase));
        assert!(gtcz_err.is_err());
        assert!(gtcz_ok.is_ok());

        let gtcp_err = device_mut.call_method1("gate_time_controlled_phase", (0, 1, 0.3, 0.7));
        let gtcp_ok =
            device_mut.call_method1("gate_time_controlled_phase", (0, 1, pscp_phase, 1.0));
        assert!(gtcp_err.is_err());
        assert!(gtcp_ok.is_ok());
    })
}

// Test two_tweezer_edges() method of TweezerDeviceWrapper and TweezerMutableDeviceWrapper
#[test]
fn test_two_tweezer_edges() {
    // Setup fake preconfigured device
    let mut exp = TweezerDevice::new(None, None, None);
    exp.add_layout("default").unwrap();
    exp.current_layout = Some("default".to_string());
    exp.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 0, 1, 0.23, None)
        .unwrap();
    exp.set_tweezer_two_qubit_gate_time("PhaseShiftedControlledPhase", 1, 2, 0.23, None)
        .unwrap();
    let fake_api_device = TweezerDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        device_mut.call_method1("add_layout", ("default",)).unwrap();
        device_mut
            .call_method1("switch_layout", ("default",))
            .unwrap();

        let edges_mut = device_mut.call_method0("two_tweezer_edges").unwrap();
        let edges_wrapper_mut = edges_mut.extract::<Vec<(usize, usize)>>().unwrap();
        assert_eq!(edges_wrapper_mut.len(), 0);

        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledPhase", 0, 1, 0.23),
            )
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_two_qubit_gate_time",
                ("PhaseShiftedControlledPhase", 1, 2, 0.23),
            )
            .unwrap();

        let new_edges_mut = device_mut
            .call_method0("two_tweezer_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();
        let edges = device
            .call_method0("two_tweezer_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();
        assert_eq!(new_edges_mut.len(), 2);
        assert_eq!(edges.len(), 2);
    })
}

/// Test set_default_layout method of TweezerMutableDeviceWrapper
#[test]
fn test_default_layout() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device_mut = device_type_mut.call0().unwrap();
        device_mut
            .call_method1("add_layout", ("triangle",))
            .unwrap();
        device_mut
            .call_method1(
                "set_tweezer_single_qubit_gate_time",
                ("RotateX", 0, 0.23, "triangle".to_string()),
            )
            .unwrap();

        assert!(device_mut
            .call_method1("set_default_layout", ("square",))
            .is_err());

        assert!(device_mut
            .call_method1("set_default_layout", ("triangle",))
            .is_ok());

        assert_eq!(
            device_mut
                .call_method0("current_layout")
                .unwrap()
                .extract::<String>()
                .unwrap(),
            "triangle"
        );
    })
}

#[test]
fn test_setters_native_set_error() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type_mut = py.get_type::<TweezerMutableDeviceWrapper>();
        let device_mut = device_type_mut.call0().unwrap();
        device_mut
            .call_method1("add_layout", ("triangle",))
            .unwrap();

        let single_setter = device_mut.call_method1(
            "set_tweezer_single_qubit_gate_time",
            ("wrong", 0, 0.23, "triangle".to_string()),
        );
        assert!(single_setter.is_err());
        assert!(single_setter.unwrap_err().to_string().contains(
            "Error setting the gate time of a single-qubit gate. Gate wrong is not supported."
        ));

        let two_setter = device_mut.call_method1(
            "set_tweezer_two_qubit_gate_time",
            ("wrong", 0, 1, 0.23, "triangle".to_string()),
        );
        assert!(two_setter.is_err());
        assert!(two_setter.unwrap_err().to_string().contains(
            "Error setting the gate time of a two-qubit gate. Gate wrong is not supported."
        ));

        let three_setter = device_mut.call_method1(
            "set_tweezer_three_qubit_gate_time",
            ("wrong", 0, 1, 2, 0.23, "triangle".to_string()),
        );
        assert!(three_setter.is_err());
        assert!(three_setter.unwrap_err().to_string().contains(
            "Error setting the gate time of a three-qubit gate. Gate wrong is not supported."
        ));

        let multi_setter = device_mut.call_method1(
            "set_tweezer_multi_qubit_gate_time",
            ("wrong", vec![0, 1, 2, 3], 0.23, "triangle".to_string()),
        );
        assert!(multi_setter.is_err());
        assert!(multi_setter.unwrap_err().to_string().contains(
            "Error setting the gate time of a multi-qubit gate. Gate wrong is not supported."
        ));
    })
}
