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

//! Integration test for public API of QRyd PragmaOperations

use pyo3::prelude::*;
use pyo3::Python;
use std::collections::{HashMap, HashSet};
use std::usize;

use qoqo::operations::PragmaChangeDeviceWrapper;
use qoqo_qryd::pragma_operations::{
    PragmaChangeQRydLayoutWrapper, PragmaDeactivateQRydQubitWrapper, PragmaShiftQRydQubitWrapper,
    PragmaShiftQubitsTweezersWrapper, PragmaSwitchDeviceLayoutWrapper,
};

fn new_pragma_layout(py: Python, layout: usize) -> &PyCell<PragmaChangeQRydLayoutWrapper> {
    let operation_type = py.get_type::<PragmaChangeQRydLayoutWrapper>();
    operation_type
        .call1((layout,))
        .unwrap()
        .downcast::<PyCell<PragmaChangeQRydLayoutWrapper>>()
        .unwrap()
}

fn new_pragma_shift(py: Python, qubit: usize) -> &PyCell<PragmaShiftQRydQubitWrapper> {
    let mut positions: HashMap<usize, (usize, usize)> = HashMap::new();
    positions.insert(qubit, (0, 1));
    let operation_type = py.get_type::<PragmaShiftQRydQubitWrapper>();
    operation_type
        .call1((positions,))
        .unwrap()
        .downcast::<PyCell<PragmaShiftQRydQubitWrapper>>()
        .unwrap()
}

fn new_pragma_deactivate(py: Python, qubit: usize) -> &PyCell<PragmaDeactivateQRydQubitWrapper> {
    let operation_type = py.get_type::<PragmaDeactivateQRydQubitWrapper>();
    operation_type
        .call1((qubit,))
        .unwrap()
        .downcast::<PyCell<PragmaDeactivateQRydQubitWrapper>>()
        .unwrap()
}

fn new_pragma_shift_tweezers(
    py: Python,
    shifts: Vec<(usize, usize)>,
) -> &PyCell<PragmaShiftQubitsTweezersWrapper> {
    let operation_type = py.get_type::<PragmaShiftQubitsTweezersWrapper>();
    operation_type
        .call1((shifts,))
        .unwrap()
        .downcast::<PyCell<PragmaShiftQubitsTweezersWrapper>>()
        .unwrap()
}

fn new_pragma_switch_layout(
    py: Python,
    new_layout: String,
) -> &PyCell<PragmaSwitchDeviceLayoutWrapper> {
    let operation_type = py.get_type::<PragmaSwitchDeviceLayoutWrapper>();
    operation_type
        .call1((new_layout,))
        .unwrap()
        .downcast::<PyCell<PragmaSwitchDeviceLayoutWrapper>>()
        .unwrap()
}

#[test]
fn test_pyo3_new_change_layout() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaChangeQRydLayoutWrapper>();
        let new_op = operation
            .call1((0,))
            .unwrap()
            .downcast::<PyCell<PragmaChangeQRydLayoutWrapper>>()
            .unwrap();

        let comparison_copy = bool::extract(
            new_op
                .call_method1("__eq__", (new_pragma_layout(py, 0),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op.extract::<PragmaChangeQRydLayoutWrapper>().unwrap();
        let new_op_diff = operation
            .call1((1,))
            .unwrap()
            .downcast::<PyCell<PragmaChangeQRydLayoutWrapper>>()
            .unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaChangeQRydLayoutWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaChangeQRydLayoutWrapper { internal: PragmaChangeQRydLayout { new_layout: 0 } }"
        );
    })
}

#[test]
fn test_pyo3_new_shift_positions() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let mut positions: HashMap<usize, (usize, usize)> = HashMap::new();
        positions.insert(0, (0, 1));
        let operation = py.get_type::<PragmaShiftQRydQubitWrapper>();
        let new_op = operation
            .call1((positions,))
            .unwrap()
            .downcast::<PyCell<PragmaShiftQRydQubitWrapper>>()
            .unwrap();

        let comparison_copy = bool::extract(
            new_op
                .call_method1("__eq__", (new_pragma_shift(py, 0),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);

        let mut positions: HashMap<usize, (usize, usize)> = HashMap::new();
        positions.insert(1, (0, 1));
        let pragma_wrapper = new_op.extract::<PragmaShiftQRydQubitWrapper>().unwrap();
        let new_op_diff = operation
            .call1((positions,))
            .unwrap()
            .downcast::<PyCell<PragmaShiftQRydQubitWrapper>>()
            .unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaShiftQRydQubitWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
        format!("{:?}", pragma_wrapper),
        "PragmaShiftQRydQubitWrapper { internal: PragmaShiftQRydQubit { new_positions: {0: (0, 1)} } }"
    );
    })
}

#[test]
fn test_pyo3_new_deactivate_qubit() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaDeactivateQRydQubitWrapper>();
        let new_op = operation
            .call1((0,))
            .unwrap()
            .downcast::<PyCell<PragmaDeactivateQRydQubitWrapper>>()
            .unwrap();

        let comparison_copy = bool::extract(
            new_op
                .call_method1("__eq__", (new_pragma_deactivate(py, 0),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op
            .extract::<PragmaDeactivateQRydQubitWrapper>()
            .unwrap();
        let new_op_diff = operation
            .call1((1,))
            .unwrap()
            .downcast::<PyCell<PragmaDeactivateQRydQubitWrapper>>()
            .unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaDeactivateQRydQubitWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaDeactivateQRydQubitWrapper { internal: PragmaDeactivateQRydQubit { qubit: 0 } }"
        );
    })
}

#[test]
fn test_pyo3_new_shift_tweezers() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaShiftQubitsTweezersWrapper>();
        let new_op = operation
            .call1((vec![(0, 1)],))
            .unwrap()
            .downcast::<PyCell<PragmaShiftQubitsTweezersWrapper>>()
            .unwrap();

        let comparison_copy = bool::extract(
            new_op
                .call_method1("__eq__", (new_pragma_shift_tweezers(py, vec![(0, 1)]),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op
            .extract::<PragmaShiftQubitsTweezersWrapper>()
            .unwrap();
        let new_op_diff = operation
            .call1((vec![(1, 2)],))
            .unwrap()
            .downcast::<PyCell<PragmaShiftQubitsTweezersWrapper>>()
            .unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaShiftQubitsTweezersWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaShiftQubitsTweezersWrapper { internal: PragmaShiftQubitsTweezers { shifts: [(0, 1)] } }"
        );
    })
}

#[test]
fn test_pyo3_new_switch_layout() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = py.get_type::<PragmaSwitchDeviceLayoutWrapper>();
        let new_op = operation
            .call1(("Square",))
            .unwrap()
            .downcast::<PyCell<PragmaSwitchDeviceLayoutWrapper>>()
            .unwrap();

        let comparison_copy = bool::extract(
            new_op
                .call_method1(
                    "__eq__",
                    (new_pragma_switch_layout(py, "Square".to_string()),),
                )
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);

        let pragma_wrapper = new_op.extract::<PragmaSwitchDeviceLayoutWrapper>().unwrap();
        let new_op_diff = operation
            .call1(("Triangle",))
            .unwrap()
            .downcast::<PyCell<PragmaSwitchDeviceLayoutWrapper>>()
            .unwrap();
        let pragma_wrapper_diff = new_op_diff
            .extract::<PragmaSwitchDeviceLayoutWrapper>()
            .unwrap();
        let helper_ne: bool = pragma_wrapper_diff != pragma_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = pragma_wrapper == pragma_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{:?}", pragma_wrapper),
            "PragmaSwitchDeviceLayoutWrapper { internal: PragmaSwitchDeviceLayout { new_layout: \"Square\" } }"
        );
    })
}

#[test]
fn test_change_layout_new() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_layout(py, 0);
        let new_layout: usize = operation
            .call_method0("new_layout")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(new_layout, 0);
    });
}

#[test]
fn test_change_layout_to_change_device() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_layout(py, 0);
        let pragma_change_device = operation
            .call_method0("to_pragma_change_device")
            .unwrap()
            .downcast::<PyCell<PragmaChangeDeviceWrapper>>();
        assert!(pragma_change_device.is_ok())
    });
}

#[test]
fn test_shift_positions_new() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_shift(py, 0);
        let new_layout: HashMap<usize, (usize, usize)> = operation
            .call_method0("new_positions")
            .unwrap()
            .extract()
            .unwrap();
        let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
        map.insert(0, (0, 1));
        assert_eq!(new_layout, map);
    });
}

#[test]
fn test_shift_positions_to_change_device() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_shift(py, 0);
        let pragma_change_device = operation
            .call_method0("to_pragma_change_device")
            .unwrap()
            .downcast::<PyCell<PragmaChangeDeviceWrapper>>();
        assert!(pragma_change_device.is_ok())
    });
}

#[test]
fn test_deactivate_qubit_new() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_deactivate(py, 0);
        let new_layout: usize = operation.call_method0("qubit").unwrap().extract().unwrap();
        assert_eq!(new_layout, 0);
    });
}

#[test]
fn test_deactivate_qubit_to_change_device() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_deactivate(py, 0);
        let pragma_change_device = operation
            .call_method0("to_pragma_change_device")
            .unwrap()
            .downcast::<PyCell<PragmaChangeDeviceWrapper>>();
        assert!(pragma_change_device.is_ok())
    });
}

#[test]
fn test_shift_tweezers_new() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_shift_tweezers(py, vec![(0, 1)]);
        let new_shifts: Vec<(usize, usize)> =
            operation.call_method0("shifts").unwrap().extract().unwrap();
        assert_eq!(new_shifts, vec![(0, 1)]);
    });
}

#[test]
fn test_shift_tweezers_to_change_device() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_shift_tweezers(py, vec![(0, 1)]);
        let pragma_change_device = operation
            .call_method0("to_pragma_change_device")
            .unwrap()
            .downcast::<PyCell<PragmaChangeDeviceWrapper>>();
        assert!(pragma_change_device.is_ok())
    });
}

#[test]
fn test_switch_layout_new() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_switch_layout(py, "Square".to_string());
        let new_layout: String = operation
            .call_method0("new_layout")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(new_layout, "Square".to_string());
    });
}

#[test]
fn test_switch_layout_to_change_device() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_switch_layout(py, "Square".to_string());
        let pragma_change_device = operation
            .call_method0("to_pragma_change_device")
            .unwrap()
            .downcast::<PyCell<PragmaChangeDeviceWrapper>>();
        assert!(pragma_change_device.is_ok())
    });
}

/// Test involved_qubits function for Pragmas
#[test]
fn test_pragmas_involved_qubits() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ops: [&PyAny; 5] = [
            new_pragma_layout(py, 0),
            new_pragma_shift(py, 0),
            new_pragma_deactivate(py, 0),
            new_pragma_shift_tweezers(py, vec![(0, 1)]),
            new_pragma_switch_layout(py, "Square".to_string()),
        ];
        for operation in ops {
            let to_involved = operation.call_method0("involved_qubits").unwrap();
            let involved_op: HashSet<&str> = HashSet::extract(to_involved).unwrap();
            let mut involved_param: HashSet<&str> = HashSet::new();
            involved_param.insert("All");
            assert_eq!(involved_op, involved_param);
        }
    });
}

/// Test to_ and from_bincode functions for Pragmas
#[test]
fn test_to_from_bincode() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ops: [&PyAny; 5] = [
            new_pragma_layout(py, 0),
            new_pragma_shift(py, 0),
            new_pragma_deactivate(py, 0),
            new_pragma_shift_tweezers(py, vec![(0, 1)]),
            new_pragma_switch_layout(py, "Square".to_string()),
        ];
        for operation in ops {
            let serialised = operation.call_method0("to_bincode").unwrap();
            let deserialised = operation
                .call_method1("from_bincode", (serialised,))
                .unwrap();

            let vec: Vec<u8> = Vec::new();
            let deserialised_error = operation.call_method1("from_bincode", (vec,));
            assert!(deserialised_error.is_err());

            let deserialised_error = deserialised.call_method0("from_bincode");
            assert!(deserialised_error.is_err());

            let serialised_error = serialised.call_method0("to_bincode");
            assert!(serialised_error.is_err());

            let comparison =
                bool::extract(deserialised.call_method1("__eq__", (operation,)).unwrap()).unwrap();
            assert!(comparison)
        }
    });
}

/// Test tags function for Pragmas
#[test]
fn test_pragmas_tags() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ops: [&PyAny; 5] = [
            new_pragma_layout(py, 0),
            new_pragma_shift(py, 0),
            new_pragma_deactivate(py, 0),
            new_pragma_shift_tweezers(py, vec![(0, 1)]),
            new_pragma_switch_layout(py, "Square".to_string()),
        ];
        for (operation, name) in ops.iter().zip(
            [
                "PragmaChangeQRydLayout",
                "PragmaShiftQRydQubit",
                "PragmaDeactivateQRydQubit",
                "PragmaShiftQubitsTweezers",
            ]
            .iter(),
        ) {
            let to_tag = operation.call_method0("tags").unwrap();
            let tags_op: &Vec<&str> = &Vec::extract(to_tag).unwrap();
            let tags_param: &[&str] = &["Operation", "PragmaOperation", name];
            assert_eq!(tags_op, tags_param);
        }
    });
}

/// Test hqslang function for Pragmas
#[test]
fn test_pragmas_hqslang() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ops: [&PyAny; 5] = [
            new_pragma_layout(py, 0),
            new_pragma_shift(py, 0),
            new_pragma_deactivate(py, 0),
            new_pragma_shift_tweezers(py, vec![(0, 1)]),
            new_pragma_switch_layout(py, "Square".to_string()),
        ];
        for (operation, name) in ops.iter().zip(
            [
                "PragmaChangeQRydLayout",
                "PragmaShiftQRydQubit",
                "PragmaDeactivateQRydQubit",
                "PragmaShiftQubitsTweezers",
            ]
            .iter(),
        ) {
            let hqslang_op: String =
                String::extract(operation.call_method0("hqslang").unwrap()).unwrap();
            assert_eq!(hqslang_op, name.to_string());
        }
    });
}

/// Test is_parametrized function for Pragmas
#[test]
fn test_pragmas_is_parametrised() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ops: [&PyAny; 5] = [
            new_pragma_layout(py, 0),
            new_pragma_shift(py, 0),
            new_pragma_deactivate(py, 0),
            new_pragma_shift_tweezers(py, vec![(0, 1)]),
            new_pragma_switch_layout(py, "Square".to_string()),
        ];
        for operation in ops {
            assert!(!bool::extract(operation.call_method0("is_parametrized").unwrap()).unwrap());
        }
    });
}

/// Test substitute_parameters function for Pragmas
#[test]
fn test_pragmas_substitute_parameters() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ops: [&PyAny; 5] = [
            new_pragma_layout(py, 0),
            new_pragma_shift(py, 0),
            new_pragma_deactivate(py, 0),
            new_pragma_shift_tweezers(py, vec![(0, 1)]),
            new_pragma_switch_layout(py, "Square".to_string()),
        ];
        for operation in ops {
            let mut substitution_dict: HashMap<&str, f64> = HashMap::new();
            substitution_dict.insert("test", 1.0);
            let substitute_op = operation
                .call_method1("substitute_parameters", (substitution_dict,))
                .unwrap();

            let comparison =
                bool::extract(substitute_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
            assert!(comparison);
        }
    });
}

/// Test remap_qubits function for PragmaShiftQRydQubit
#[test]
fn test_pragmas_remap_qubits_shift() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_shift(py, 0);
        let op2 = new_pragma_shift(py, 2);
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 2);
        qubit_mapping.insert(2, 0);
        let remapped_op = operation
            .call_method1("remap_qubits", (qubit_mapping,))
            .unwrap();

        let comparison =
            bool::extract(remapped_op.call_method1("__eq__", (op2,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test remap_qubits function for PragmaChangeQRydLayout
#[test]
fn test_pragmas_remap_qubits_change_layout() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_layout(py, 0);
        let qubit_mapping: HashMap<usize, usize> = HashMap::new();

        let remapped_op = operation
            .call_method1("remap_qubits", (qubit_mapping,))
            .unwrap();
        let comparison =
            bool::extract(remapped_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(comparison);
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 2);
        qubit_mapping.insert(2, 0);
        let remapped_op = operation.call_method1("remap_qubits", (qubit_mapping,));
        assert!(remapped_op.is_err());
    });
}

/// Test remap_qubits function for PragmaDeactivateQRydQubit
#[test]
fn test_pragmas_remap_qubits_deactivate_qubit() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_deactivate(py, 0);
        let qubit_mapping: HashMap<usize, usize> = HashMap::new();

        let remapped_op = operation
            .call_method1("remap_qubits", (qubit_mapping,))
            .unwrap();
        let comparison =
            bool::extract(remapped_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(comparison);
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 2);
        qubit_mapping.insert(2, 0);
        let remapped_op = operation.call_method1("remap_qubits", (qubit_mapping,));
        assert!(remapped_op.is_err());
    });
}

/// Test remap_qubits function for PragmaShiftQubitsTweezers
#[test]
fn test_pragmas_remap_qubits_shift_tweezers() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_shift_tweezers(py, vec![(0, 1)]);
        let operation2 = new_pragma_shift_tweezers(py, vec![(2, 1)]);
        let qubit_mapping: HashMap<usize, usize> = HashMap::new();

        let remapped_op = operation
            .call_method1("remap_qubits", (qubit_mapping,))
            .unwrap();
        let comparison =
            bool::extract(remapped_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(comparison);

        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 2);
        qubit_mapping.insert(2, 0);
        let remapped_op = operation
            .call_method1("remap_qubits", (qubit_mapping,))
            .unwrap();

        let comparison =
            bool::extract(remapped_op.call_method1("__eq__", (operation2,)).unwrap()).unwrap();
        assert!(comparison);
    });
}

/// Test remap_qubits function for PragmaSwitchDeviceLayout
#[test]
fn test_pragmas_remap_qubits_switch_layout() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = new_pragma_switch_layout(py, "Square".to_string());
        let qubit_mapping: HashMap<usize, usize> = HashMap::new();

        let remapped_op = operation
            .call_method1("remap_qubits", (qubit_mapping,))
            .unwrap();
        let comparison =
            bool::extract(remapped_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
        assert!(comparison);
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 2);
        qubit_mapping.insert(2, 0);
        let remapped_op = operation.call_method1("remap_qubits", (qubit_mapping,));
        assert!(remapped_op.is_err());
    });
}

/// Test __copy__, __deepcopy__ functions for Pragmas
#[test]
fn test_pragmas_copy_deepcopy() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ops: [&PyAny; 5] = [
            new_pragma_layout(py, 0),
            new_pragma_shift(py, 0),
            new_pragma_deactivate(py, 0),
            new_pragma_shift_tweezers(py, vec![(0, 1)]),
            new_pragma_switch_layout(py, "Square".to_string()),
        ];
        for operation in ops {
            let copy_op = operation.call_method0("__copy__").unwrap();
            let deepcopy_op = operation.call_method1("__deepcopy__", ("",)).unwrap();

            let comparison_copy =
                bool::extract(copy_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
            assert!(comparison_copy);
            let comparison_deepcopy =
                bool::extract(deepcopy_op.call_method1("__eq__", (operation,)).unwrap()).unwrap();
            assert!(comparison_deepcopy);
        }
    });
}

/// Test __format__,__repr__ functions for Pragmas
#[test]
fn test_pragmas_format_repr() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ops: [&PyAny; 5] = [
            new_pragma_layout(py, 0),
            new_pragma_shift(py, 0),
            new_pragma_deactivate(py, 0),
            new_pragma_shift_tweezers(py, vec![(0, 1)]),
            new_pragma_switch_layout(py, "Square".to_string()),
        ];
        for (operation, format_repr) in ops.iter().zip(
            [
                "PragmaChangeQRydLayout { new_layout: 0 }",
                "PragmaShiftQRydQubit { new_positions: {0: (0, 1)} }",
                "PragmaDeactivateQRydQubit { qubit: 0 }",
                "PragmaShiftQubitsTweezers { shifts: [(0, 1)] }",
                "PragmaSwitchDeviceLayout { new_layout: \"Square\" }",
            ]
            .iter(),
        ) {
            let to_format = operation.call_method1("__format__", ("",)).unwrap();
            let format_op: &str = <&str>::extract(to_format).unwrap();
            let to_repr = operation.call_method0("__repr__").unwrap();
            let repr_op: &str = <&str>::extract(to_repr).unwrap();
            assert_eq!(format_op, *format_repr);
            assert_eq!(repr_op, *format_repr);
        }
    });
}

/// Test __eq__,__ne__ functions for Pragmas
#[test]
fn test_pragmas_richcmp() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let ops: [(&PyAny, &PyAny); 5] = [
            (new_pragma_layout(py, 0), new_pragma_layout(py, 1)),
            (new_pragma_shift(py, 0), new_pragma_shift(py, 1)),
            (new_pragma_deactivate(py, 0), new_pragma_deactivate(py, 1)),
            (
                new_pragma_shift_tweezers(py, vec![(0, 1)]),
                new_pragma_shift_tweezers(py, vec![(1, 0)]),
            ),
            (
                new_pragma_switch_layout(py, "Square".to_string()),
                new_pragma_switch_layout(py, "Triangle".to_string()),
            ),
        ];
        for (operation_one, operation_two) in ops {
            let comparison = bool::extract(
                operation_one
                    .call_method1("__eq__", (operation_two,))
                    .unwrap(),
            )
            .unwrap();
            assert!(!comparison);

            let comparison = bool::extract(
                operation_one
                    .call_method1("__ne__", (operation_two,))
                    .unwrap(),
            )
            .unwrap();
            assert!(comparison);

            let comparison = operation_one.call_method1("__eq__", (vec!["fails"],));
            assert!(comparison.is_err());

            let comparison = operation_one.call_method1("__ge__", (operation_two,));
            assert!(comparison.is_err());
        }
    });
}
