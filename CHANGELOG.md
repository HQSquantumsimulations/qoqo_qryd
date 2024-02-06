# Qoqo qryd changelog

Tracks qoqo-qryd changes after 0.5

# 0.13.0

* Added `TweezerDevice.get_qubit_to_tweezer_mapping()`
* Added `with_trivial_map` parameter to `TweezerDevice.switch_layout()`
* Added `layout_name` parameter to `TweezerDevice.number_tweezer_positions()`
* Added `TweezerDevice` support for `SimulatorBackend`
* Added `number_qubits` parameter to `SimulatorBackend`
* Modified `TweezerDevice.number_qubits()` back to index-based implementation
* Dropped `FirstDevice` support for `SimulatorBackend`
* Updated to `mockito` 1.2
* Updated the MSRV to 1.68

# 0.12.2

* Added `TweezerDevice.number_tweezer_positions()`

# 0.12.1

* Updated to Qoqo 1.9
* Fixed `TweezerDevice.number_qubits()` incorrect implementation

# 0.12.0

* Updated to Qoqo 1.8
* Updated to Pyo3 0.20

# 0.11.7

* Modified `TweezerDevice.two_qubit_edges()` to consider any two-qubit gate as valid for an edge

# 0.11.6

* Modified `TweezerDevice`'s current layout to be optional

# 0.11.5

* Modified `TweezerDevice.from_api()` endpoint, default device name for better `APIBackend` support
* Added support for `PragmaActiveReset` for `APIBackend`, `TweezerDevice`

# 0.11.4

* Added `api_version`, `seed`, `dev` parameters to `TweezerDevice.from_api()`
* Fixed `TweezerDevice` support for `APIBackend`
* Modified `TweezerDevice` seed parameter to default to None, instead of 0
* Added `api_version` parameter to `APIBackend`

# 0.11.3

* Modified `TweezerDevice.from_json()` and `TweezerMutableDevice.set_default_layout()` to automatically switch the layout of the device if a default one was set
* Modified `TweezerDevice` and `TweezerMutableDevice.to_json()` such that it returns an error in case no QRyd-valid gates are executable
* Added `TweezerDevice.from_mutable()` static method
* Added `dev` parameter in `APIBackend` constructor
* Added `TweezerDevice` support for `APIBackend`

# 0.11.2

* Modified `APIBackend.post_job()` to substitute `PragmaRepeatedMeasurement` into `MeasureQubit` and `PragmaSetNumberOfMeasurements` instances

# 0.11.1

* Corrected the check of the validity of a `PragmaShiftQubitsTweezers` operation for `TweezerDevice`
* Added `TweezerDevice.set_allowed_tweezer_shifts_from_rows()`
* Added `TweezerDevice.two_tweezer_edges()`
* Added `TweezerDevice.set_default_layout()`
* Added `APIBackend.set_dev()`
* Corrected docs after mdbook port

# 0.11.0

* Substituted `httpmock` with `mockito` in mock testing
* Reduced the number of usecases that need openssl
* ExperimentalDevice renamed to TweezerDevice
* `TweezerDevice.add_qubit_tweezer_mapping()` returns the new mapping
* Added `TweezerDevice.two_tweezer_edges()`
* Added `TweezerDevice.set_allowed_tweezer_shifts()`
* Added `PragmaShiftQubitsTweezers`
* Added `PragmaSwitchDeviceLayout`
* Added ControlledControlledPauliZ, ControlledControlledPhaseShift and PragmaControlledCircuit to `APIBackend`'s allowed operations

# 0.10.0

* Added experimental device support
* Added `PragmaDeactivateQRydQubit`
* Updated to qoqo 1.6

# 0.9.1

* Updated to qoqo 1.5.1
* Updated to pyo3 0.19

# 0.9.0

* Added optional parameters for 3-qubit operations in FirstDevice and QrydEmuTriangularDevice

# 0.8.7

* Updated to qoqo 1.5

## 0.8.6

* Updated to qoqo 1.4

## 0.8.5

* Updated to qoqo 1.3
* Updated qoqo_qryd devices initialization signature

## 0.8.4

* Updated dependencies addressing open-ssl security advisory

## 0.8.3

* Updated dependencies

## 0.8.2

* Updated supported gates

## 0.8.1

* Fixes in the phi-theta relation computation

## 0.8.0

* Support for PhaseShiftedControlledPhase operation in devices
* Switch from given phase shift for two-qubit gates to selecting phase realtions for PhaseShiftedControlledPhase and PhaseShiftedControlledZ

## 0.7.1

* Update to new version of qoqo (1.2.0)

## 0.7.0

* Switches to new API version v3_0 of QRyd Web-API for the Web-API backend

## 0.6.0

* Fix for ValidationError format: detail containing a list of Detail maps
* Fixed QuantumProgram send to v2_0 WebAPI to use roqoqo 1.0 instead of 1.1 for compatability
* Updated to qoqo 1.1.0-beta.2 dependency and corresponding updated Device trait
* Made the Web-API backend with the reqwest dependency an optional feature
* Updated to qoqo 1.1.0-beta.1 dependency and corresponding updated Device trait
