Welcome to the qoqo-qryd/roqoqo-qryd user documentation!
========================================================

This software is designed to support the [QRydDemo](<https://thequantumlaend.de/qryddemo/>) project on quantum computing with Rydberg atoms. It provides components to support QRydDemo quantum computers based on the [qoqo](<https://github.com/HQSquantumsimulations/qoqo>) quantum toolkit by [HQS Quantum Simulations](<https://quantumsimulations.de>) used to represent quantum circuits.

To learn more about the general approach to create quantum programs and executing them in qoqo see [Introduction](src/introduction.md) and [Execution](src/execution.md).

This software is split into two packages:

* roqoqo-qryd: Implementing the core functionality and a Rust interface.
* qoqo-qryd: The Python interface.

The packages are based on the open source [qoqo](<https://github.com/HQSquantumsimulations/qoqo>) quantum computing toolkit.


Interface to the current QRydDemo WebAPI
----------------------------------------

At the moment QRydDemo WebAPI allows access to Quantum Hardware Emulators of different device topologies. 
qoqo-qryd/roqoqo-qryd support interfacing with the corresponding REST-API with low level calls, i.e. using `Circuit`,  as well as with high-level backend based functionalities, i.e. by using `QuantumPrograms` in qoqo. For more details see [WebAPI](webapi.md).


QRydDemo specific hardware operations (prototype)
-------------------------------------------------

Rydberg atom based quantum devices support, in principle, operations not commonly found in other quantum hardware.
Changes in device topology are one of these operations. 
roqoqo-qryd/qoqo-qryd adds support for changes in device topology to qoqo. For more details see [QRyd Specifics](qrydspecifics.md).
Note that this is a preview prototype and does not represent a finalized set of operations on the QRydDemo hardware.


Local simulator supporting specific hardware operations
-------------------------------------------------------

qoqo-qryd/roqoqo-qryd include a local [QuEST](https://github.com/QuEST-Kit/QuEST>) based simulator for quantum devices supporting the Rydberg specific quantum operations. The simulator is intended to let users test the capabilities of quantum hardware with the additional operations. For more details see [QRyd Specifics](qrydspecifics.md). Note that the devices for the simulator do not represent a finalized design for QRydDemo.


Examples
--------

A collection of example python programs can be found in [Examples](examples.md).


OpenSSL
-----------------

Acknowledgments related to using OpenSSL for http requests:

This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit (http://www.openssl.org/).

This product includes cryptographic software written by Eric Young
(eay@cryptsoft.com).  This product includes software written by Tim
Hudson (tjh@cryptsoft.com).