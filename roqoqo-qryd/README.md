# roqoqo-qryd
[![Crates.io](https://img.shields.io/crates/v/roqoqo-qryd)](https://crates.io/crates/roqoqo-qryd)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qryd/workflows/ci_tests_main/badge.svg)](https://github.com/HQSquantumsimulations/qoqo/actions)
[![docs.rs](https://img.shields.io/docsrs/roqoqo-qryd)](https://docs.rs/roqoqo-qryd/)
![Crates.io](https://img.shields.io/crates/l/roqoqo-qryd)

The `/roqoqo-qryd` folder is there to provide the core rust library that builds on the roqoqo rust library for the QRydDemo project.

## Installation

For using roqoqo-qryd in rust code including the optional simulator simply add

```toml
roqoqo-qryd = {version="0.1", path="...", features=["simulator"]}
```

to the `[dependencies]` section of your Cargo.toml.

## Documentation

Although the code snippets in the user documentation are provided for the python users, the rust user might refer to the [user documentation](https://github.com/HQSquantumsimulations/qoqo_qryd/tree/main/userdoc) to find some general information, e.g. on "QRydDemo devices and operations.

To create the API documentation for the roqoqo-qryd rust package can be found here: [API-documentation](https://docs.rs/roqoqo-qryd/)

## Examples

The examples for the usage of roqoqo_qryd written in rust are to follow.

## OpenSSL

Acknowledgments related to using OpenSSL for http requests

"This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit. (http://www.openssl.org/)"

This product includes cryptographic software written by Eric Young
(eay@cryptsoft.com).  This product includes software written by Tim
Hudson (tjh@cryptsoft.com).