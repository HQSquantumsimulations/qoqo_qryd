# qoqo-qryd

[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qryd/workflows/ci_tests_main/badge.svg)](https://github.com/HQSquantumsimulations/qoqo-qryd/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo-qryd)](https://pypi.org/project/qoqo-qryd/)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo-qryd)](https://pypi.org/project/qoqo-qryd/)
[![Crates.io](https://img.shields.io/crates/v/qoqo-qryd)](https://crates.io/crates/qoqo-qryd)
![Crates.io](https://img.shields.io/crates/l/qoqo-qryd)

The `qoqo-qryd` python package implements modules to support [qoqo](https://github.com/HQSquantumsimulations/qoqo) usage with quantum computers and quantum computer emulators of the [QRydDemo](https://thequantumlaend.de/qryddemo/) project.

The QRydDemo project builds on quantum computers using Rydberg atoms.
qoqo is quantum computing toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

The qoqo-qryd package contains the following functionality:

### Interface to the current QRydDemo WebAPI

At the moment QRydDemo WebAPI allows access to Quantum Hardware Emulators of different device topology. qoqo-qryd supports interfacing with the corresponding [REST-API](https://api.qryddemo.itp3.uni-stuttgart.de/docs) with low level calls as well as a high-level backend to qoqo quantum programs. For this it provides the backend `APIBackend` to evaluate qoqo quantum programs and the `api_devices` module to represent devices available on the emulators.

### QRydDemo specific hardware operations (prototype)

Rydberg atom based quantum devices support, in principle, operations not commonly found in other quantum hardware. Changes in device topology are one of these operations. qoqo-qryd adds support for changes in device topology to qoqo via the operations in its `pragma_operations` module.
Note that this is a preview prototype and does not represent a finalized set of operations on the QRydDemo hardware.

### Local simulator supporting specific hardware operations

qoqo-qryd includes a local [QuEST](https://github.com/QuEST-Kit/QuEST) based simulator for quantum devices supporting the Rydberg specific quantum operations. The simulator is intended to let users test the capabilities of quantum hardware with the additional operations.
qoqo-qryd provides the simulator via the `SimulatorBackend` qoqo-compatible backend that uses the device prototypes in its `qryd_devices` module.
Note that the devices for the simulator do not represent a finalized design for QRydDemo.

## Accessing QRydDemo WebAPI

To use the WebAPI, a QRydDemo account is required. Users can register via the [online registration form](https://thequantumlaend.de/get-access/).

## Installation

The `qoqo-qryd` package is available on PyPi as a source distribution and as precompiled wheels for linux and macOS on the x86 platform. Other platforms need to install from the source distribution.

For both source distribution and pre-built wheels it can be installed via

```shell
pip install qoqo-qryd
```

When building from source `qoqo-qryd` requires rust, [maturin](https://github.com/PyO3/maturin) and cmake to be installed locally. We recommend using [rustup](https://rustup.rs) to set up a rust toolchain. The pip command should also automatically install maturin to build a python package locally and install it.

For a quick installation of the dependencies you can also use:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install maturin
```

## Documentation

The [user documentation](https://hqsquantumsimulations.github.io/qoqo_qryd/) is provided on github pages.
The API-documentation for qoqo-qryd can be found as an appendix of the user documentation.

## Examples

A small collection of [example python scripts](https://github.com/HQSquantumsimulations/qoqo_qryd/tree/main/qoqo-qryd/examples) for the QRydDemo project can be found in the project github repository. The examples include

* `howto_webapi_qoqo.ipynb` provides an example accessing the QRydDemo's emulator with qoqo.
* `switch_layout_example.py` shows how to construct a QRydDemo device and using the PragmaChangeQrydLayout operation to switch between layouts in a quantum Circuit.
* `shift_qubits_example.py` shows how to construct a QRydDemo device and using the PragmaShiftQrydQubit operation to shift qubits between tweezer positions in a quantum Circuit.
* `multi_qubit_example.py` shows how to use multi-qubit-operations.
* `serialisation_example.py` demonstrates how to serialize a QuantumProgram to JSON format.

## OpenSSL

Acknowledgments related to using OpenSSL for http requests:

"This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit (http://www.openssl.org/)."

This product includes cryptographic software written by Eric Young
(eay@cryptsoft.com).  This product includes software written by Tim
Hudson (tjh@cryptsoft.com).