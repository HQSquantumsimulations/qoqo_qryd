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
pub use qryd_devices::*;

#[cfg(test)]
mod api_devices;
pub use api_devices::*;

#[cfg(test)]
#[cfg(feature="web-api")]
mod api_backend;
#[cfg(feature="web-api")]
pub use api_backend::*;

#[cfg(test)]
mod operations;
pub use operations::*;

#[cfg(test)]
#[cfg(feature = "simulator")]
mod simulator_backend;
#[cfg(test)]
#[cfg(feature = "simulator")]
pub use simulator_backend::*;
