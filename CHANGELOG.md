# Qoqo qryd changelog

Tracks qoqo-qryd changes after 0.5

# 0.11.0

* Substituted `httpmock` with `mockito` in mock testing
* Reduced the number of usecases that need openssl
* ExperimentalDevice renamed to TweezerDevice
* `TweezerDevice.add_qubit_tweezer_mapping()` returns the new mapping
* Added `TweezerDevice.set_allowed_tweezer_shifts()`
* Added `PragmaShiftQubitsTweezers`
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
