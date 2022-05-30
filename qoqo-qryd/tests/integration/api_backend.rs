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

//! Integration test for public API of QRyd WebAPI backend

use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::Python;
use qoqo_qryd::api_backend::APIBackendWrapper;
use qoqo_qryd::api_devices::{QrydEmuSquareDeviceWrapper, QrydEmuTriangularDeviceWrapper};
use std::f64::consts::PI;
use std::usize;

// Helper function to create a python object of square device
fn create_backend_with_square_device(py: Python) -> &PyCell<APIBackendWrapper> {
    let seed: Option<usize> = Some(11);
    let pcz_theta: f64 = PI / 4.0;
    let device_type = py.get_type::<QrydEmuSquareDeviceWrapper>();
    let device: &PyCell<QrydEmuSquareDeviceWrapper> = device_type
        .call1((seed, pcz_theta))
        .unwrap()
        .cast_as::<PyCell<QrydEmuSquareDeviceWrapper>>()
        .unwrap();

    let backend_type: &PyType = py.get_type::<APIBackendWrapper>();
    let backend: &PyCell<APIBackendWrapper> = backend_type
        .call1((device, ""))
        .unwrap()
        .cast_as::<PyCell<APIBackendWrapper>>()
        .unwrap();
    backend
}

// Test to create a new backend
#[test]
fn test_new_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let seed: Option<usize> = Some(11);
        let pcz_theta: f64 = PI / 4.0;
        let device_type = py.get_type::<QrydEmuSquareDeviceWrapper>();
        let device: &PyCell<QrydEmuSquareDeviceWrapper> = device_type
            .call1((seed, pcz_theta))
            .unwrap()
            .cast_as::<PyCell<QrydEmuSquareDeviceWrapper>>()
            .unwrap();

        let backend_type: &PyType = py.get_type::<APIBackendWrapper>();
        let backend = backend_type
            .call1((device, ""))
            .unwrap()
            .cast_as::<PyCell<APIBackendWrapper>>();
        assert!(backend.is_ok());
    });
}

// Test to create a new backend
#[test]
fn test_new_triangle() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let seed: Option<usize> = Some(11);
        let pcz_theta: f64 = PI / 4.0;
        let device_type = py.get_type::<QrydEmuTriangularDeviceWrapper>();
        let device: &PyCell<QrydEmuTriangularDeviceWrapper> = device_type
            .call1((seed, pcz_theta))
            .unwrap()
            .cast_as::<PyCell<QrydEmuTriangularDeviceWrapper>>()
            .unwrap();

        let backend_type: &PyType = py.get_type::<APIBackendWrapper>();
        let backend = backend_type
            .call1((device, ""))
            .unwrap()
            .cast_as::<PyCell<APIBackendWrapper>>();
        assert!(backend.is_ok());
    });
}

/// Test copy and deepcopy for api backend with square device
#[test]
fn test_copy_deepcopy_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = create_backend_with_square_device(py);

        let copy_op = backend.call_method0("__copy__").unwrap();
        let copy_wrapper = copy_op.extract::<APIBackendWrapper>().unwrap();
        let deepcopy_op = backend.call_method1("__deepcopy__", ("",)).unwrap();
        let deepcopy_wrapper = deepcopy_op.extract::<APIBackendWrapper>().unwrap();

        let backend_wrapper = backend.extract::<APIBackendWrapper>().unwrap();
        assert_eq!(backend_wrapper, copy_wrapper);
        assert_eq!(backend_wrapper, deepcopy_wrapper);
    });
}

/// Test to and from json for api backend with square device
#[test]
fn test_json_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = create_backend_with_square_device(py);

        let serialised = backend.call_method0("to_json").unwrap();
        let deserialised = backend.call_method1("from_json", (serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = backend.call_method1("from_json", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_json");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_json");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<APIBackendWrapper>().unwrap();
        let backend_wrapper = backend.extract::<APIBackendWrapper>().unwrap();
        assert_eq!(backend_wrapper, serde_wrapper);
    });
}

/// Test to and from bincode for api backend with square device
#[test]
fn test_bincode_square() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = create_backend_with_square_device(py);

        let serialised = backend.call_method0("to_bincode").unwrap();
        let deserialised = backend.call_method1("from_bincode", (serialised,)).unwrap();

        let vec: Vec<u8> = Vec::new();
        let deserialised_error = backend.call_method1("from_bincode", (vec,));
        assert!(deserialised_error.is_err());

        let deserialised_error = deserialised.call_method0("from_bincode");
        assert!(deserialised_error.is_err());

        let serialised_error = serialised.call_method0("to_bincode");
        assert!(serialised_error.is_err());

        let serde_wrapper = deserialised.extract::<APIBackendWrapper>().unwrap();
        let backend_wrapper = backend.extract::<APIBackendWrapper>().unwrap();
        assert_eq!(backend_wrapper, serde_wrapper);
    });
}
