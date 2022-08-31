# qoqo-qryd

This software package is designed to support the [QRydDemo](https://thequantumlaend.de/qryddemo/) project on quantum computing with Rydberg atoms. It provides components to support QRydDemo quantum computers based on the [qoqo](https://github.com/HQSquantumsimulations/qoqo) quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de) used to represent quantum circuits.

This repository contains two components:

* roqoqo-qryd: the core rust library that builds on the roqoqo rust library.
* qoqo-qryd: the python interface for roqoqo-qryd that uses the qoqo python interface.

The qoqo-qryd/roqoqo-qryd packages provide the following functionality:

### Interface to the current QRydDemo WebAPI

At the moment QRydDemo WebAPI allows access to Quantum Hardware Emulators of different device topology. qoqo-qryd/roqoqo-qryd support interfacing with the corresponding [REST-API](https://api.qryddemo.itp3.uni-stuttgart.de/docs) with low level calls as well as a high-level backend to qoqo quantum programs.

### QRydDemo specific hardware operations (prototype)

Rydberg atom based quantum devices support, in principle, operations not commonly found in other quantum hardware. Changes in device topology are one of these operations. roqoqo-qryd/qoqo-qryd adds support for changes in device topology to qoqo.
Note that this is a preview prototype and does not represent a finalized set of operations on the QRydDemo hardware.

### Local simulator supporting specific hardware operations

qoqo-qryd/roqoqo-qryd include a local [QuEST](https://github.com/QuEST-Kit/QuEST) based simulator for quantum devices supporting the Rydberg specific quantum operations. The simulator is intended to let users test the capabilities of quantum hardware with the additional operations. Note that the devices for the simulator do not represent a finalized design for QRydDemo.

## Accessing QRydDemo WebAPI

To use the WebAPI, a QRydDemo account is required. Users can register via the [online registration form](https://thequantumlaend.de/get-access/).

## Installation

Installation instructions are provided by the corresponding READMEs of [qoqo-qryd](https://github.com/HQSquantumsimulations/qoqo_qryd/blob/main/qoqo-qryd/README.md) and [roqoqo-qryd](https://github.com/HQSquantumsimulations/qoqo_qryd/blob/main/roqoqo-qryd/README.md).

## Documentation

We recommend getting started with the [user documentation](https://hqsquantumsimulations.github.io/qoqo_qryd/).
 The example code snippets in the documentation are provided for the python users of `qoqo-qryd`, the general information on the design principles and the structure of the project also applies to `roqoqo-qryd`, e.g. "QRydDemo devices and operations" that the user might refer to.

The `roqoqo-qryd` API documentation can be found [here](https://docs.rs/roqoqo-qryd/).

## Examples

A small collection of example python programs for the QRydDemo project can be found [here](https://github.com/HQSquantumsimulations/qoqo_qryd/tree/main/qoqo-qryd/examples).

The examples for the usage of roqoqo-qryd written in rust will follow.

## OpenSSL

Acknowledgments related to using OpenSSL for http requests:

"This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit (http://www.openssl.org/)."

This product includes cryptographic software written by Eric Young
(eay@cryptsoft.com).  This product includes software written by Tim
Hudson (tjh@cryptsoft.com).