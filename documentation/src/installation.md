# Installation

## qoqo

qoqo is available on PyPi, both as a pre-built Python wheel for common architectures (windows/linux/macos on x86) and as a source distribution.

For pre-built wheels you can install qoqo with a simple pip command

```bash
pip install qoqo
```

If your architecture does not support this installation process, please install maturin before installing qoqo using `pip install maturin`.
For other platforms we recommend using the source distribution from PyPi to build a python package for qoqo locally via pip. The install requires the maturin tool (also available via pip) and a working rust toolchain. Specifically for macOS on Apple Silicon the following build command should be used:

```shell
RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup" pip install qoqo
```

When using qoqo in a rust project providing a python interface add

```TOML
qoqo = {version="1.0", default-features=false}
```

to the `[dependencies]` section of the project Cargo.toml.

Alternatively one can check out the latest tagged version from github and use the maturin tool to build a python package for qoqo locally and install it via pip. Please note that the package should be built from the top level directory of the workspace selecting the qoqo package with the -m qoqo/Cargo.toml option. 

```shell
maturin build -m qoqo/Cargo.toml  --release
pip install target/wheels/<NAME_OF_WHEEL>
```

Specifically for macOS on Apple Silicon the following build command should be used:

```shell
RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup" maturin build -m qoqo/Cargo.toml  --release
pip install target/wheels/$NAME_OF_WHEEL
```
## roqoqo

To use roqoqo in a Rust project simply add

```toml
roqoqo = "1.0"
```

to the `[dependencies]` section in your Cargo.toml.
