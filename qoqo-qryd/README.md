# qoqo-qryd

[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qryd/workflows/ci_tests_main/badge.svg)](https://github.com/HQSquantumsimulations/qoqo-qryd/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo-qryd)](https://pypi.org/project/qoqo-qryd/)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo-qryd)](https://pypi.org/project/qoqo-qryd/)
[![Crates.io](https://img.shields.io/crates/v/qoqo-qryd)](https://crates.io/crates/qoqo-qryd)
![Crates.io](https://img.shields.io/crates/l/qoqo-qryd)

The `qoqo-qryd` python package implements support [qoqo](https://github.com/HQSquantumsimulations/qoqo) support for quantum computers and quantum computer emulators of the [QRydDemo](https://thequantumlaend.de/qryddemo/) project.

The QRydDemo project builds on Quantum computers using Rydberg atoms.
qoqo is quantum computing toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

The qoqo-qryd package provides three modules:

* Backends that execute a compiled qoqo QuantumProgram on QRydDemo hardware or simulators,
* A set of specific operations only available on QRydDemo hardware,
* A collection of devices that represent hardware devices available in the QrydDemo project.

## Installation

The `qoqo_qryd` package is a standard python package and can be installed with the pip command.

```bash
pip install qoqo-qryd
```

## Documentation

The [user documentation](https://hqsquantumsimulations.github.io/qoqo_qryd/) is provided on github pages.
The API-documentation for qoqo-qryd is provided can be found as an appendix of the user documentation or built separately with sphinx.

## Examples

A small collection of [example python scripts](https://github.com/HQSquantumsimulations/qoqo_qryd/tree/main/qoqo-qryd/examples) for the QRydDemo project is can be found in the project github repository. The examples include

* `howto_webapi_qoqo.ipynb` provides an example accessing the QRydDemo's emulator with Qoqo.
* `switch_layout_example.py` shows how to construct a QRydDemo device and using the PragmaChangeQrydLayout operation to switch between layouts in a quantum Circuit.
* `shift_qubits_example.py` shows how to construct a QRydDemo device and using the PragmaShiftQrydQubit operation to shift qubits between tweezer positions in a quantum Circuit.
* `multi_qubit_example.py` shows how to use multi-qubit-operations.
* `serialisation_example.py` demonstrates how to serialize a QuantumProgram to json.

## OpenSSL

Acknowledgments related to using OpenSSL for http requests

"This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit. (http://www.openssl.org/)"

This product includes cryptographic software written by Eric Young
(eay@cryptsoft.com).  This product includes software written by Tim
Hudson (tjh@cryptsoft.com).