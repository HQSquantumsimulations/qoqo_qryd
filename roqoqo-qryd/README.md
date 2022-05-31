# roqoqo-qryd
[![Crates.io](https://img.shields.io/crates/v/roqoqo-qryd)](https://crates.io/crates/roqoqo-qryd)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qryd/workflows/ci_tests_main/badge.svg)](https://github.com/HQSquantumsimulations/qoqo/actions)
[![docs.rs](https://img.shields.io/docsrs/roqoqo-qryd)](https://docs.rs/roqoqo-qryd/)
![Crates.io](https://img.shields.io/crates/l/roqoqo-qryd)

The `roqoqo-qryd` rust crate implements [qoqo](https://github.com/HQSquantumsimulations/qoqo) support for quantum computers and quantum computer emulators of the [QRydDemo](https://thequantumlaend.de/qryddemo/) project.

The QRydDemo project builds on Quantum computers using Rydberg atoms.
qoqo is quantum computing toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

The roqoqo-qryd package contains the following functionality:

### Interface to the current QRydDemo WebAPI

At the moment QRydDemo WebAPI allows access to Quantum Hardware Emulators of different device topology. roqoqo-qryd supports interfacing with the corresponding [REST-API](https://api.qryddemo.itp3.uni-stuttgart.de/docs)with low level calls as well as a high-level backend to qoqo quantum programs. For this it provides the backend `APIBackend` to evaluate roqoqo quantum programs and the `api_devices` module to represent devices available on the emulators.

### QRydDemo specific hardware operations (prototype)

Rydberg atom based quantum devices support, in principle, operations not commonly found in other quantum hardware. Changes in device topology are one of these operations. roqoqo-qryd adds support for changes in device topology to roqoqo via the operations in its `pragma_operations` module.
Note that this is a preview prototype and does not represent a finalized set of operations on the QRydDemo hardware.

### Local simulator supporting specific hardware operations

roqoqo-qryd includes a local [QuEST](https://github.com/QuEST-Kit/QuEST) based simulator for quantum devices supporting the Rydberg specific quantum operations. The simulator is intended to let users test the capabilities of quantum hardware with the additional operations.
roqoqo-qryd provides the simulator via the `SimulatorBackend` backend the implements the roqoqo `Backend` trait.The backend uses the device prototypes in roqoqo-qryd's `qryd_devices` module.
Note that the devices for the simulator do not represent a finalised design for QRydDemo.

## Accessing QRydDemo WebAPI

To use the WebAPI, a QRydDemo account is required. Users can register via the [online registration form](https://thequantumlaend.de/get-access/).
## Installation

For using roqoqo-qryd in rust code including the optional simulator simply add

```toml
roqoqo-qryd = {version="0.5", features=["simulator"]}
```

to the `[dependencies]` section of your Cargo.toml.

## Documentation

Although the code snippets in the user documentation are provided for the python users, the rust user might refer to the [user documentation](https://github.com/HQSquantumsimulations/qoqo_qryd/tree/main/userdoc) to find some general information, e.g. on "QRydDemo devices and operations.

The API documentation for the roqoqo-qryd rust package can be found here: [API-documentation](https://docs.rs/roqoqo-qryd/)

## Examples

Examples are to follow soon.

## OpenSSL

Acknowledgments related to using OpenSSL for http requests

"This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit. (http://www.openssl.org/)"

This product includes cryptographic software written by Eric Young
(eay@cryptsoft.com). This product includes software written by Tim
Hudson (tjh@cryptsoft.com).