# Installation


## Installing qoqo-qryd

For the python package we recommend checking out the latest tagged version from github installing it via pip. The pip installation requires rust and cmake to be installed locally. We recommend using [rustup](https://rustup.rs) to set up a rust toolchain. The pip should also automatically install  [maturin](https://github.com/PyO3/maturin) tool to build a python package locally and install it.
Maturin needs an installed rust toolchain.

For a quick installation you can also use:

```shell
pip install ./qoqo-qryd/qoqo-qryd/
```

Specifically for macOS on Apple Silicon the following build command should be used:

```shell
RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup" pip install ./qoqo-qryd/qoqo-qryd/
```

For a quick installation of the dependencies you can also use:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install maturin
```

The user can also first build a python package with maturin and install it manually. Please note that the package should be built from the top level directory of the workspace selecting the qoqo package with the `-m qoqo/Cargo.toml` option.
A typical build and run command is

```shell
maturin build -m qoqo-qryd/Cargo.toml --release
pip install target/wheels/$NAME_OF_WHEEL
```

### Using roqoqo-qryd in rust code

For using roqoqo-qryd in rust code including the optional simulator simply add

```toml
roqoqo-qryd = {version="0.1", path="...", features=["simulator"]}
```

to the `[dependencies]` section of your Cargo.toml.
