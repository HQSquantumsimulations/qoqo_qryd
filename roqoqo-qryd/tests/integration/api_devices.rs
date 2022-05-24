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

use roqoqo::devices::Device;
use roqoqo_qryd::api_devices::{QRydAPIDevice, QrydEmuSquareDevice, QrydEmuTriangularDevice};

use std::f64::consts::PI;
// use std::f64::EPSILON;

// Test the new function of the square device emulator
#[test]
fn test_new_square() {
    let device = QrydEmuSquareDevice::new(None, PI).unwrap();
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(device.seed(), 0);
    assert_eq!(device.seed(), apidevice.seed());
    assert_eq!(device.pcz_theta(), PI);
    assert_eq!(device.pcz_theta(), apidevice.pcz_theta());
    assert_eq!(device.qrydbackend(), "qryd_emu_cloudcomp_square");
    assert_eq!(device.qrydbackend(), apidevice.qrydbackend());
}

// Test the new function of the triangular device emulator
#[test]
fn test_new_triangular() {
    let device = QrydEmuTriangularDevice::new(Some(1), 0.0).unwrap();
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(device.seed(), 1);
    assert_eq!(device.seed(), apidevice.seed());
    assert_eq!(device.pcz_theta(), 0.0);
    assert_eq!(device.pcz_theta(), apidevice.pcz_theta());
    assert_eq!(device.qrydbackend(), "qryd_emu_cloudcomp_triangle");
    assert_eq!(device.qrydbackend(), apidevice.qrydbackend());
}

// Test the functions from device trait of the square device emulator
#[test]
fn test_device_square() {
    let device = QrydEmuSquareDevice::new(None, PI).unwrap();
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(apidevice.number_qubits(), 30);
}

// Test the functions from device trait of the triangular device emulator
#[test]
fn test_device_triangular() {
    let device = QrydEmuTriangularDevice::new(None, PI).unwrap();
    let apidevice = QRydAPIDevice::from(&device);
    assert_eq!(apidevice.number_qubits(), 30);
}
