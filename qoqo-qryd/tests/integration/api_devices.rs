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

//! Integration test for public API of QRyd WebAPI devices

use pyo3::prelude::*;
use pyo3::Python;
use qoqo_qryd::api_devices::{QrydEmuSquareDeviceWrapper, QrydEmuTriangularDeviceWrapper};
use std::collections::HashSet;
use std::usize;

// Helper function to create a python object of square device
fn create_square_device(py: Python) -> &PyCell<QrydEmuSquareDeviceWrapper> {
    let seed: Option<usize> = Some(11);
    let device_type = py.get_type::<QrydEmuSquareDeviceWrapper>();
    let device: &PyCell<QrydEmuSquareDeviceWrapper> = device_type
        .call1((seed,))
        .unwrap()
        .cast_as::<PyCell<QrydEmuSquareDeviceWrapper>>()
        .unwrap();
    device
}

// Test to create a new device
#[test]
fn test_new_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<QrydEmuSquareDeviceWrapper>();
        let result = device_type.call1((Some(10),));
        assert!(result.is_ok());
        let device = result
            .unwrap()
            .cast_as::<PyCell<QrydEmuSquareDeviceWrapper>>();
        assert!(device.is_ok());
    });
}

// Test number_qubits function of the square device
#[test]
fn test_numberqubits_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_square_device(py);

        let number_qubits_get = device
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_qubits_get, 30_usize);
    });
}

/// Test copy and deepcopy for square device
#[test]
fn test_copy_deepcopy_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_square_device(py);

        let copy_op = device.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_op.extract::<QrydEmuSquareDeviceWrapper>().unwrap();
        let deepcopy_op = device.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_op.extract::<QrydEmuSquareDeviceWrapper>().unwrap();

        let device_wrapper = device.extract::<QrydEmuSquareDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, copy_wrapper);
        assert_eq!(device_wrapper, deepcopy_wrapper);
    });
}

/// Test to_ and from_bincode functions of square device
#[test]
fn test_to_from_bincode_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_square_device(py);

        let serialised = device.call_method0("to_bincode").unwrap();
        let deserialised = device.call_method1("from_bincode", (serialised,)).unwrap();

        let not_correct: HashSet<usize> = HashSet::new();
        let extract_error = device.call_method1("from_bincode", (not_correct,));
        assert!(extract_error.is_err());

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = device.call_method1("from_bincode", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised
            .extract::<QrydEmuSquareDeviceWrapper>()
            .unwrap();
        let device_wrapper = device.extract::<QrydEmuSquareDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

/// Test to_ and from_json functions of square device
#[test]
fn test_to_from_json_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_square_device(py);

        let serialised = device.call_method0("to_json").unwrap();
        let deserialised = device.call_method1("from_json", (serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = device.call_method1("from_json", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised
            .extract::<QrydEmuSquareDeviceWrapper>()
            .unwrap();
        let device_wrapper = device.extract::<QrydEmuSquareDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

/// Test to_ and from_bincode functions of square device
#[test]
fn test_enum_to_bincode_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_square_device(py);

        let serialised = device.call_method0("_enum_to_bincode");
        assert!(serialised.is_ok());
    });
}

// Test fields of the square device
#[test]
fn test_fields_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_square_device(py);

        let seed = device
            .call_method0("seed")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(seed, 11);

        let qrydbackend = device
            .call_method0("qrydbackend")
            .unwrap()
            .extract::<String>()
            .unwrap();
        assert_eq!(qrydbackend, "qryd_emu_cloudcomp_square");
    });
}

// Test two_qubit_edges of the square device
#[test]
fn test_twoqubitedges_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_square_device(py);

        let twoqubitedges_get = device
            .call_method0("two_qubit_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();
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
        assert_eq!(twoqubitedges_get, two_qubit_edges);
    });
}

// Helper function to create a python object of triangular device
fn create_triangular_device(py: Python) -> &PyCell<QrydEmuTriangularDeviceWrapper> {
    let seed: Option<usize> = Some(11);
    let device_type = py.get_type::<QrydEmuTriangularDeviceWrapper>();
    let device: &PyCell<QrydEmuTriangularDeviceWrapper> = device_type
        .call1((seed,))
        .unwrap()
        .cast_as::<PyCell<QrydEmuTriangularDeviceWrapper>>()
        .unwrap();
    device
}

// Test to create a new device
#[test]
fn test_new_triangular() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device_type = py.get_type::<QrydEmuTriangularDeviceWrapper>();
        let result = device_type.call1((Some(10),));
        assert!(result.is_ok());
        let device = result
            .unwrap()
            .cast_as::<PyCell<QrydEmuTriangularDeviceWrapper>>();
        assert!(device.is_ok());
    });
}

// Test number_qubits function of the triangular device
#[test]
fn test_numberqubits_triangular() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_triangular_device(py);

        let number_qubits_get = device
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(number_qubits_get, 30_usize);
    });
}

/// Test copy and deepcopy for triangular device
#[test]
fn test_copy_deepcopy_triangular() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_triangular_device(py);

        let copy_op = device.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_op.extract::<QrydEmuTriangularDeviceWrapper>().unwrap();
        let deepcopy_op = device.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_op
            .extract::<QrydEmuTriangularDeviceWrapper>()
            .unwrap();

        let device_wrapper = device.extract::<QrydEmuTriangularDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, copy_wrapper);
        assert_eq!(device_wrapper, deepcopy_wrapper);
    });
}

/// Test to_ and from_bincode functions of triangular device
#[test]
fn test_to_from_bincode_triangular() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_triangular_device(py);

        let serialised = device.call_method0("to_bincode").unwrap();
        let deserialised = device.call_method1("from_bincode", (serialised,)).unwrap();

        let not_correct: HashSet<usize> = HashSet::new();
        let extract_error = device.call_method1("from_bincode", (not_correct,));
        assert!(extract_error.is_err());

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = device.call_method1("from_bincode", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised
            .extract::<QrydEmuTriangularDeviceWrapper>()
            .unwrap();
        let device_wrapper = device.extract::<QrydEmuTriangularDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

/// Test to_ and from_json functions of triangular device
#[test]
fn test_to_from_json_triangular() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_triangular_device(py);

        let serialised = device.call_method0("to_json").unwrap();
        let deserialised = device.call_method1("from_json", (serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = device.call_method1("from_json", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised
            .extract::<QrydEmuTriangularDeviceWrapper>()
            .unwrap();
        let device_wrapper = device.extract::<QrydEmuTriangularDeviceWrapper>().unwrap();
        assert_eq!(device_wrapper, serde_wrapper);
    });
}

/// Test to_ and from_bincode functions of triangular device
#[test]
fn test_enum_to_bincode_triangular() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_triangular_device(py);

        let serialised = device.call_method0("_enum_to_bincode");
        assert!(serialised.is_ok());
    });
}

// Test fields of the triangular device
#[test]
fn test_fields_triangular() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_triangular_device(py);

        let seed = device
            .call_method0("seed")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(seed, 11);

        let qrydbackend = device
            .call_method0("qrydbackend")
            .unwrap()
            .extract::<String>()
            .unwrap();
        assert_eq!(qrydbackend, "qryd_emu_cloudcomp_triangle");
    });
}

// Test two_qubit_edges of the triangular device
#[test]
fn test_twoqubitedges_triangular() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_triangular_device(py);

        let twoqubitedges_get = device
            .call_method0("two_qubit_edges")
            .unwrap()
            .extract::<Vec<(usize, usize)>>()
            .unwrap();
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
        assert_eq!(twoqubitedges_get, two_qubit_edges);
    });
}

// Test generic_device() for square device
#[test]
fn test_generic_device_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_square_device(py);
        let genericdevice = device.call_method0("generic_device").unwrap();

        let num_gen = genericdevice
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        let num_dev = device
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(num_gen, num_dev);
    })
}

// Test generic_device() for triangular device
#[test]
fn test_generic_device_triangular() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let device = create_triangular_device(py);
        let genericdevice = device.call_method0("generic_device").unwrap();

        let num_gen = genericdevice
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        let num_dev = device
            .call_method0("number_qubits")
            .unwrap()
            .extract::<usize>()
            .unwrap();
        assert_eq!(num_gen, num_dev);
    })
}

// Test phi-theta relation methods.
#[test]
fn test_phi_theta_relation() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let triangular = create_triangular_device(py);
        let square = create_square_device(py);

        let pscz_tr = triangular
            .call_method0("phase_shift_controlled_z")
            .unwrap()
            .extract::<f64>()
            .unwrap();
        let pscz_sq = square
            .call_method0("phase_shift_controlled_z")
            .unwrap()
            .extract::<f64>()
            .unwrap();
        assert!(pscz_tr.is_finite());
        assert!(pscz_sq.is_finite());

        let pscp_tr = triangular
            .call_method1("phase_shift_controlled_phase", (1.0,))
            .unwrap()
            .extract::<f64>()
            .unwrap();
        let pscp_sq = square
            .call_method1("phase_shift_controlled_phase", (1.0,))
            .unwrap()
            .extract::<f64>()
            .unwrap();
        assert!(pscp_tr.is_finite());
        assert!(pscp_sq.is_finite());

        let gtcz_tr_err = triangular.call_method1("gate_time_controlled_z", (0, 1, 0.3));
        let gtcz_sq_err = square.call_method1("gate_time_controlled_z", (0, 1, 0.3));
        assert!(gtcz_tr_err.is_err());
        assert!(gtcz_sq_err.is_err());

        let gtcz_tr_ok = triangular
            .call_method1("gate_time_controlled_z", (0, 1, pscz_tr))
            .unwrap()
            .extract::<f64>()
            .unwrap();
        let gtcz_sq_ok = square
            .call_method1("gate_time_controlled_z", (0, 1, pscz_sq))
            .unwrap()
            .extract::<f64>()
            .unwrap();
        assert!(gtcz_tr_ok.is_finite());
        assert!(gtcz_sq_ok.is_finite());

        let gtcp_tr_err = triangular.call_method1("gate_time_controlled_phase", (0, 1, 0.3, 0.7));
        let gtcp_sq_err = square.call_method1("gate_time_controlled_phase", (0, 1, 0.3, 0.7));
        assert!(gtcp_tr_err.is_err());
        assert!(gtcp_sq_err.is_err());

        let gtcp_tr_ok = triangular
            .call_method1("gate_time_controlled_phase", (0, 1, pscp_tr, 1.0))
            .unwrap()
            .extract::<f64>()
            .unwrap();
        let gtcp_sq_ok = square
            .call_method1("gate_time_controlled_phase", (0, 1, pscp_sq, 1.0))
            .unwrap()
            .extract::<f64>()
            .unwrap();
        assert!(gtcp_tr_ok.is_finite());
        assert!(gtcp_sq_ok.is_finite());
    })
}
