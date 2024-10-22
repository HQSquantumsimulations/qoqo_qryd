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

#[cfg(test)]
mod qryd_devices;

#[cfg(test)]
mod tweezer_devices;

#[cfg(test)]
mod api_devices;

#[cfg(test)]
#[cfg(feature = "web-api")]
mod api_backend;

#[cfg(test)]
mod operations;

#[cfg(test)]
mod emulator_devices;

#[cfg(test)]
#[cfg(feature = "simulator")]
mod simulator_backend;

#[cfg(feature = "web-api")]
#[test]
fn test_device_from_api() {
    use qoqo_qryd::device_from_api;

    let response = device_from_api(None, None, None, None, None);
    assert!(response.is_ok());
    // TODO: add more specific testing once the available devices gathered from the API endpoint can be distinguished
}
