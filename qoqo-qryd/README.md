# qoqo-

[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qryd/workflows/ci_tests_main/badge.svg)](https://github.com/HQSquantumsimulations/qoqo-qryd/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo-qryd)](https://pypi.org/project/qoqo-qryd/)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo-qryd)](https://pypi.org/project/qoqo-qryd/)
[![Crates.io](https://img.shields.io/crates/v/qoqo-qryd)](https://crates.io/crates/qoqo-qryd)
![Crates.io](https://img.shields.io/crates/l/qoqo-qryd)

The `/qoqo-qryd` folder is there to provide a python interface for the implemented functionalities in `/roqoqo-qryd` (in rust) for the QRydDemo project.

## Installation

The `qoqo_qryd` package is a standard python package and can be installed with the pip command.

```bash
pip install qoqo-qryd
```

## Documentation

A user documentation is provided in the folder `/userdoc`.
The API-documentation for qoqo-qryd is provided can be found as an appendix of the user documentation or built separately with sphinx.

## Examples

A small collection of example python programs for the QRydDemo project is located in `/examples`. The folder includes

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