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

use pyo3::prelude::*;

use qoqo_qryd::{ExperimentalDeviceWrapper, ExperimentalMutableDeviceWrapper};
use roqoqo_qryd::ExperimentalDevice;

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

#[test]
fn test_layouts() {
    // Setup fake preconfigured device
    let mut exp = ExperimentalDevice::new();
    exp.add_qubit_tweezer_mapping(0, 1);
    exp.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.23, None);
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

        assert!(device_mut.call_method1("add_layout", ("OtherLayout",)).is_ok());

        assert!(device.call_method1("switch_layout", ("OtherLayout",)).is_ok());
        assert!(device_mut.call_method1("switch_layout", ("OtherLayout",)).is_ok());
    })
}

#[test]
fn test_qubit_tweezer_mapping() {}

#[test]
fn test_qubit_times() {
    // Setup fake preconfigured device
    let mut exp = ExperimentalDevice::new();
    exp.add_qubit_tweezer_mapping(0, 1);
    exp.add_qubit_tweezer_mapping(1, 2);
    exp.add_qubit_tweezer_mapping(2, 3);
    exp.add_qubit_tweezer_mapping(3, 0);
    exp.add_layout("OtherLayout").unwrap();
    exp.set_tweezer_single_qubit_gate_time("PauliX", 0, 0.23, Some("OtherLayout".to_string()));
    exp.set_tweezer_two_qubit_gate_time("CNOT", 0, 1, 0.13, Some("OtherLayout".to_string()));
    exp.set_tweezer_three_qubit_gate_time("Toffoli", 0, 1, 2, 0.45, Some("OtherLayout".to_string()));
    exp.set_tweezer_multi_qubit_gate_time("MultiQubitZZ", &[0, 1, 2, 3], 0.65, Some("OtherLayout".to_string()));
    let fake_api_device = ExperimentalDeviceWrapper { internal: exp };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fake_api_pypyany = fake_api_device.into_py(py);
        let device_type_mut = py.get_type::<ExperimentalMutableDeviceWrapper>();
        let device = fake_api_pypyany.as_ref(py);
        let device_mut = device_type_mut.call0().unwrap();

        device_mut.call_method1("add_qubit_tweezer_mapping", (0, 1)).unwrap();
        device_mut.call_method1("add_qubit_tweezer_mapping", (1, 2)).unwrap();
        device_mut.call_method1("add_qubit_tweezer_mapping", (2, 3)).unwrap();
        device_mut.call_method1("add_qubit_tweezer_mapping", (3, 0)).unwrap();

        device_mut.call_method1("add_layout", ("OtherLayout",)).unwrap();
        device_mut.call_method1("set_tweezer_single_qubit_gate_time", ("PauliX", 0, 0.23, "OtherLayout",)).unwrap();
        device_mut.call_method1("set_tweezer_two_qubit_gate_time", ("CNOT", 0, 1, 0.13, "OtherLayout",)).unwrap();
        device_mut.call_method1("set_tweezer_three_qubit_gate_time", ("Toffoli", 0, 1, 2, 0.45, "OtherLayout",)).unwrap();
        device_mut.call_method1("set_tweezer_multi_qubit_gate_time", ("MultiQubitZZ", vec![0, 1, 2, 3], 0.6, "OtherLayout",)).unwrap();

        assert!(device.call_method1("switch_layout", ("OtherLayout",)).is_ok());
        assert!(device_mut.call_method1("switch_layout", ("OtherLayout",)).is_ok());
        
        assert!(device.call_method1("single_qubit_gate_time", ("PauliX", 3)).is_ok());
        assert!(device_mut.call_method1("single_qubit_gate_time", ("PauliX", 3)).is_ok());
        assert!(device.call_method1("two_qubit_gate_time", ("CNOT", 3, 0)).is_ok());
        assert!(device_mut.call_method1("two_qubit_gate_time", ("CNOT", 3, 0)).is_ok());
        assert!(device.call_method1("three_qubit_gate_time", ("Toffoli", 3, 0, 1)).is_ok());
        assert!(device_mut.call_method1("three_qubit_gate_time", ("Toffoli", 3, 0, 1)).is_ok());
        assert!(device.call_method1("multi_qubit_gate_time", ("MultiQubitZZ", vec![3, 0, 1, 2])).is_ok());
        assert!(device_mut.call_method1("multi_qubit_gate_time", ("MultiQubitZZ", vec![3, 0, 1, 2])).is_ok());
    })
}
