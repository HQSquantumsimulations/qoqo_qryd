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

//! Integration test for public API of QRyd devices

use ndarray::{array, Array2};
use numpy::ToPyArray;
use std::collections::HashMap;
use std::usize;

use pyo3::prelude::*;
use pyo3::Python;

use qoqo_qryd::qryd_devices::{convert_into_device, FirstDeviceWrapper};
use roqoqo_qryd::qryd_devices::{FirstDevice, QRydDevice};

use test_case::test_case;

#[test_case(3,2,vec![2,2,2], 1.0, array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]]; "2rows_3columns")]
#[test_case(2,2,vec![1,1], 3.0,  array![[0.0, 1.0], [0.0, 1.0]]; "2rows_2columns")]
fn test_creating_device(
    number_rows: usize,
    number_columns: usize,
    qubits_per_row: Vec<i32>,
    row_distance: f64,
    initial_layout: Array2<f64>,
) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((
                number_rows,
                number_columns,
                qubits_per_row.clone(),
                row_distance,
                initial_layout.to_pyarray(py),
            ))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let number_qubits: i32 = qubits_per_row.iter().sum();
        let number_qubits_get = device
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_qubits_get, number_qubits as usize);

        let number_rows_get = device
            .call_method0("number_rows")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_rows_get, number_rows);

        let number_columns_get = device
            .call_method0("number_columns")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_columns_get, number_columns);
    });
}

/// Test copy and deepcopy functions of FirstDeviceWrapper
#[test]
fn test_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((
                3,
                2,
                vec![2, 2, 2],
                1.0,
                array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]].to_pyarray(py),
            ))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let copy_op = device.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_op.extract::<FirstDeviceWrapper>().unwrap();
        let deepcopy_op = device.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_op.extract::<FirstDeviceWrapper>().unwrap();

        let device_wrapper = device.extract::<FirstDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, copy_wrapper);
        assert_eq!(device_wrapper, deepcopy_wrapper);
    });
}

/// Test to_ and from_bincode functions of FirstDeviceWrapper
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((
                3,
                2,
                vec![2, 2, 2],
                1.0,
                array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]].to_pyarray(py),
            ))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let serialised = device.call_method0("to_bincode").unwrap();
        let deserialised = device.call_method1("from_bincode", (serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = device.call_method1("from_bincode", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<FirstDeviceWrapper>().unwrap();
        let device_wrapper = device.extract::<FirstDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

/// Test _enum_to_bincode function of FirstDeviceWrapper
#[test]
fn test_enum_to_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((
                3,
                2,
                vec![2, 2, 2],
                1.0,
                array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]].to_pyarray(py),
            ))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let serialised = device.call_method0("_enum_to_bincode");
        assert!(serialised.is_ok());
    });
}

/// Test to_ and from_json functions of FirstDeviceWrapper
#[test]
fn test_to_from_json() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((
                3,
                2,
                vec![2, 2, 2],
                1.0,
                array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]].to_pyarray(py),
            ))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let serialised = device.call_method0("to_json").unwrap();
        let deserialised = device.call_method1("from_json", (serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = device.call_method1("from_json", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<FirstDeviceWrapper>().unwrap();
        let device_wrapper = device.extract::<FirstDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

/// Test add_ and switch_layout functions of FirstDeviceWrapper
#[test]
fn test_switch_layout() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let original_layout = array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]];
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((3, 2, vec![2, 2, 2], 1.0, original_layout.to_pyarray(py)))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let new_layout: Array2<f64> = array![[0.5, 0.5], [0.5, 0.4], [0.4, 0.3]];
        let updated_device = device
            .call_method1("add_layout", (1, new_layout.to_pyarray(py)))
            .unwrap();
        updated_device.call_method1("switch_layout", (1,)).unwrap();

        let comp_device = device_type
            .call1((3, 2, vec![2, 2, 2], 1.0, new_layout.to_pyarray(py)))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let number_qubits_get = updated_device
            .call_method0("number_columns")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        let number_qubits_comp = comp_device
            .call_method0("number_columns")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_qubits_get, number_qubits_comp);

        let number_qubits_get = updated_device
            .call_method0("number_rows")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        let number_qubits_comp = comp_device
            .call_method0("number_rows")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_qubits_get, number_qubits_comp);

        let number_qubits_get = updated_device
            .call_method0("qubit_positions")
            .unwrap()
            .extract::<HashMap<usize, (usize, usize)>>()
            .unwrap();
        let number_qubits_comp = comp_device
            .call_method0("qubit_positions")
            .unwrap()
            .extract::<HashMap<usize, (usize, usize)>>()
            .unwrap();
        assert_eq!(number_qubits_get, number_qubits_comp);

        let number_qubits_get = updated_device
            .call_method0("two_qubit_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();
        let number_qubits_comp = comp_device
            .call_method0("two_qubit_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();
        assert_eq!(number_qubits_get, number_qubits_comp);
    });
}

/// Test set_cutoff function of FirstDeviceWrapper
#[test]
fn test_set_cutoff() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let original_layout = array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 5.0]];
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((3, 2, vec![2, 2, 2], 1.0, original_layout.to_pyarray(py)))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();
        let _ = device.call_method1("set_cutoff", (3.0,));
        let number_qubits_get = device
            .call_method0("two_qubit_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();
        for i in 0..5 {
            assert!(!number_qubits_get.contains(&(i, 5)));
            assert!(!number_qubits_get.contains(&(5, i)));
        }
    });
}

/// Test change_ and qubit_position functions of FirstDeviceWrapper
#[test]
fn test_change_qubit_positions() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let original_layout = array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]];
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((3, 2, vec![2, 2, 2], 1.0, original_layout.to_pyarray(py)))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let mut new_qubits: HashMap<usize, (usize, usize)> = HashMap::new();
        new_qubits.insert(0, (0, 1));
        new_qubits.insert(1, (0, 0));
        new_qubits.insert(2, (1, 1));
        new_qubits.insert(3, (1, 0));
        new_qubits.insert(4, (2, 1));
        new_qubits.insert(5, (2, 0));
        device
            .call_method1("change_qubit_positions", (new_qubits.clone(),))
            .unwrap();

        let qubit_positions = device
            .call_method0("qubit_positions")
            .unwrap()
            .extract::<HashMap<usize, (usize, usize)>>()
            .unwrap();
        assert_eq!(qubit_positions, new_qubits);
    });
}

// Test gate time methods of FirstDeviceWrapper
#[test]
fn test_gate_times() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let original_layout = array![[0.0, 0.5,], [0.0, 0.5,], [0.0, 0.5]];
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((
                3,
                2,
                vec![2, 2, 2],
                0.5,
                original_layout.to_pyarray(py),
                Option::<&PyAny>::None,
                Option::<&PyAny>::None,
                true,
                true,
            ))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let sing_ok = device.call_method1("single_qubit_gate_time", ("RotateX", 0));
        let sing_err = device.call_method1("single_qubit_gate_time", ("Hadamard", 0));
        assert!(sing_ok.is_ok());
        assert!(sing_err.is_err());

        let two_ok = device.call_method1("two_qubit_gate_time", ("PhaseShiftedControlledZ", 0, 1));
        let two_err = device.call_method1("two_qubit_gate_time", ("CNOT", 0, 1));
        assert!(two_ok.is_ok());
        assert!(two_err.is_err());

        let three_err = device.call_method1("three_qubit_gate_time", ("Toffoli", 0, 1, 2));
        let three_ok_0 = device.call_method1(
            "three_qubit_gate_time",
            ("ControlledControlledPauliZ", 0, 1, 2),
        );
        let three_ok_1 = device.call_method1(
            "three_qubit_gate_time",
            ("ControlledControlledPhaseShift", 0, 1, 2),
        );
        assert!(three_err.is_err());
        assert!(three_ok_0.is_ok());
        assert!(three_ok_1.is_ok());

        let mult = device.call_method1("multi_qubit_gate_time", ("MultiQubitZZ", (0, 1, 2)));
        assert!(mult.is_err());
    })
}

/// Test phase_shift_ functions of FirstDeviceWrapper
#[test]
fn test_phi_theta_relation() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let original_layout = array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]];
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((3, 2, vec![2, 2, 2], 1.0, original_layout.to_pyarray(py)))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();
        let new_layout: Array2<f64> = array![[0.5, 0.5], [0.5, 0.4], [0.4, 0.3]];
        let updated_device = device
            .call_method1("add_layout", (1, new_layout.to_pyarray(py)))
            .unwrap();
        updated_device.call_method1("switch_layout", (1,)).unwrap();

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

        let gtcz_err = device.call_method1("gate_time_controlled_z", (0, 1, 0.3));
        let gtcz_ok = device.call_method1("gate_time_controlled_z", (0, 1, pscz_phase));
        assert!(gtcz_err.is_err());
        assert!(gtcz_ok.is_ok());

        let gtcp_err = device.call_method1("gate_time_controlled_phase", (0, 1, 0.3, 0.7));
        let gtcp_ok = device.call_method1("gate_time_controlled_phase", (0, 1, pscp_phase, 1.0));
        assert!(gtcp_err.is_err());
        assert!(gtcp_ok.is_ok());

        let device_f_0 = device_type
            .call1((
                3,
                2,
                vec![2, 2, 2],
                1.0,
                original_layout.to_pyarray(py),
                Some("2.15".to_string()),
                Option::<String>::None,
            ))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();
        let device_f_1 = device_type
            .call1((
                3,
                2,
                vec![2, 2, 2],
                1.0,
                original_layout.to_pyarray(py),
                2.15,
                1.36,
            ))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();
        let device_f_2 = device_type
            .call1((
                3,
                2,
                vec![2, 2, 2],
                1.0,
                original_layout.to_pyarray(py),
                2.15,
                Some(1.36.to_string()),
            ))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let pscz_phase_f_0 = device_f_0
            .call_method0("phase_shift_controlled_z")
            .unwrap()
            .extract::<f64>()
            .unwrap();
        let pscz_phase_f_1 = device_f_1
            .call_method0("phase_shift_controlled_z")
            .unwrap()
            .extract::<f64>()
            .unwrap();
        let pscp_phase_f_1 = device_f_1
            .call_method1("phase_shift_controlled_phase", (1.0,))
            .unwrap()
            .extract::<f64>()
            .unwrap();
        let pscp_phase_f_2 = device_f_2
            .call_method1("phase_shift_controlled_phase", (1.0,))
            .unwrap()
            .extract::<f64>()
            .unwrap();
        assert_eq!(pscz_phase_f_0, 2.15);
        assert_eq!(pscz_phase_f_1, 2.15);
        assert_eq!(pscp_phase_f_1, 1.36);
        assert_eq!(pscp_phase_f_2, 1.36)
    });
}

/// Test convert_into_device function
#[test]
fn test_convert_to_device() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((
                3,
                2,
                vec![2, 2, 2],
                1.0,
                array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]].to_pyarray(py),
            ))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let converted = convert_into_device(device).unwrap();
        let rust_dev: QRydDevice = FirstDevice::new(
            3,
            2,
            &[2, 2, 2],
            1.0,
            array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]],
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .into();
        assert_eq!(converted, rust_dev);
    });
}

/// Test 
#[test]
fn test_pyo3_new_change_layout() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let layout = array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]];
        let device_type = py.get_type::<FirstDeviceWrapper>();
        let device = device_type
            .call1((3, 2, vec![2, 2, 2], 1.0, layout.to_pyarray(py)))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();

        let pragma_wrapper = device.extract::<FirstDeviceWrapper>().unwrap();
        let new_op_diff = device_type
            .call1((
                3,
                2,
                vec![2, 2, 2],
                2.0,
                array![[0.0, 1.0,], [0.0, 1.0,], [0.0, 1.0]].to_pyarray(py),
            ))
            .unwrap()
            .downcast::<PyCell<FirstDeviceWrapper>>()
            .unwrap();
        let pragma_wrapper_diff = new_op_diff.extract::<FirstDeviceWrapper>().unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        let check_str = format!("{:?}", pragma_wrapper);
        let check_1: &str = check_str.split("qubit_positions").collect::<Vec<&str>>()[0];
        let check_2: &str = check_str.split("qubit_positions").collect::<Vec<&str>>()[1]
            .split(")}")
            .collect::<Vec<&str>>()[1];
        let comp_str = format!("FirstDeviceWrapper {{ internal: FirstDevice {{ number_rows: 3, number_columns: 2, qubit_positions: {{0: (0, 0), 1: (0, 1), 2: (1, 0), 3: (1, 1), 4: (2, 0), 5: (2, 1)}}, row_distance: 1.0, layout_register: {{0: {:?}}}, current_layout: 0, cutoff: 1.0, controlled_z_phase_relation: \"DefaultRelation\", controlled_phase_phase_relation: \"DefaultRelation\", allow_ccz_gate: true, allow_ccp_gate: false }} }}", layout);
        let comp_1: &str = comp_str.split("qubit_positions").collect::<Vec<&str>>()[0];
        let comp_2: &str = comp_str.split("qubit_positions").collect::<Vec<&str>>()[1]
            .split(")}")
            .collect::<Vec<&str>>()[1];

        assert_eq!(comp_1, check_1);
        assert_eq!(comp_2, check_2);
    })
}
