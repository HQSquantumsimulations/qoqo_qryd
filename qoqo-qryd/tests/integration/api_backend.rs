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
use std::{env, usize};

// Helper function to create a python object of square device
fn _create_backend_with_square_device(py: Python) -> &PyCell<APIBackendWrapper> {
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
        .call1((device,))
        .unwrap()
        .cast_as::<PyCell<APIBackendWrapper>>()
        .unwrap();
    backend
}

// Test to create a new backend
#[test]
fn test_new_square() {
    if env::var("QRYD_API_TOKEN").is_ok() {
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
                .call1((device,))
                .unwrap()
                .cast_as::<PyCell<APIBackendWrapper>>();
            assert!(backend.is_ok());
        });
    }
}

// Test to create a new backend
#[test]
fn test_new_triangle() {
    if env::var("QRYD_API_TOKEN").is_ok() {
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
                .call1((device,))
                .unwrap()
                .cast_as::<PyCell<APIBackendWrapper>>();
            assert!(backend.is_ok());
        });
    }
}
