# Installation

## Installing qoqo-qryd

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

Once rust, maturin and cmake are set up, the local wheel can be built and installed by

```shell
cd qoqo-qryd
maturin build --release

# the name of the built wheel is composed as 
# 'qoqo_qryd-<version>-<platform-details>.whl'
pip install ../target/wheels/<wheel-name>.whl
```

### Using roqoqo-qryd in rust code

For using roqoqo-qryd in rust code including the optional simulator simply add

```toml
roqoqo-qryd = {version="0.1", path="...", features=["simulator"]}
```

to the `[dependencies]` section of your Cargo.toml.
