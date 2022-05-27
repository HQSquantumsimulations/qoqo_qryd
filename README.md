# qoqo-qryd

This software package is designed to support the [QRydDemo](https://thequantumlaend.de/qryddemo/) project on Quantum computing with Rydberg atoms. It provides components to support QRydDemo quantum computers based on the [qoqo](https://github.com/HQSquantumsimulations/qoqo) quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de) used to represent quantum circuits.

This repository contains two components:

* roqoqo-qryd: the core rust library that builds on the roqoqo rust library.
* qoqo-qryd: the python interface for roqoqo-qryd that uses the qoqo python interface.

The qoqo-qryd/roqoqo-qryd packages provide three modules:

* Backends that execute a compiled qoqo QuantumProgram on QRydDemo hardware or simulators,
* A set of specific operations only available on QRydDemo hardware,
* A collection of devices, representations of the Hardware devices available in the QrydDemo project.


## Accessing QRydDemo WebAPI

To use the API Backend, a QRydDemo API token is required. The token can be obtained via our [online registration form](https://thequantumlaend.de/get-access/).


## Installation

The installation instructions are provided within the individual repositories. The `/qoqo-qryd` folder is there to provide a python interface for the implemented functionalities in `/roqoqo-qryd` (in rust).

## Documentation

A user documentation is provided in the folder `/userdoc`. Although the code snippets in the documentation are provided for the python users, the general information on the design principles and the structure of the project also applies to `roqoqo-qryd`, e.g. "QRydDemo devices and operations" that the user might refer to.

Additionally, API documentation generated from the source code can be build for the rust part. The instructions are given in `/roqoqo-qryd`.

## Examples

A small collection of example python programs for the QRydDemo project can be found [here](https://github.com/HQSquantumsimulations/qoqo_qryd/tree/main/qoqo-qryd/examples).

The examples for the usage of roqoqo_qryd written in rust will follow.

## OpenSSL

Acknowledgments related to using OpenSSL for http requests

"This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit. (http://www.openssl.org/)"

This product includes cryptographic software written by Eric Young
(eay@cryptsoft.com).  This product includes software written by Tim
Hudson (tjh@cryptsoft.com).