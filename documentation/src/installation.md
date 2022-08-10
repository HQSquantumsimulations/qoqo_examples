# Installation

## qoqo

qoqo is available on PyPi either as a pre-build Python wheel for common architectures (windows/linux/macos on x86) and as a source distribution.

For pre-build wheels you can install qoqo with a simple pip command

```bash
pip install qoqo
```

For the source distribution a rust toolchain and the [maturin](https://maturin.rs/) Python package need to be already installed to install qoqo. A Rust toolchain can be installed using [rustup](https://rustup.rs/).

With a Rust toolchain installed, qoqo can again be installed using a pip command.

```bash
pip install maturin
pip install qoqo
```

## roqoqo

To use roqoqo in a Rust project simply add

```toml
roqoqo = "1.0"
```

to the `[dependencies]` section in your Cargo.toml.
