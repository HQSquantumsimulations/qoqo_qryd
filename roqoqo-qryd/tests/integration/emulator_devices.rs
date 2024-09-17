// Copyright © 2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use roqoqo_qryd::EmulatorDevice;

/// Test EmulatorDevice new()
#[test]
fn test_new() {
    let device = EmulatorDevice::new(Some(2), None, None);

    assert!(device.internal.current_layout.is_none());
    assert!(device.internal.qubit_to_tweezer.is_none());
    assert!(device.internal.layout_register.is_none());
    assert_eq!(device.internal.seed(), Some(2));
    assert_eq!(device.internal.qrydbackend(), "qryd_tweezer_device");

    let device_emp = EmulatorDevice::new(None, None, None);

    assert_eq!(device_emp.seed(), None);
}
