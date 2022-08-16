# Installation

## qoqo

qoqo is available on PyPi, both as a pre-built Python wheel for common architectures (windows/linux/macos on x86) and as a source distribution.

For pre-built wheels you can install qoqo with a simple pip command

```bash
pip install qoqo
```

Alternatively, installing from the source distribution is possible. For this, a rust toolchain and the [maturin](https://maturin.rs/) Python package need to be already installed. A Rust toolchain can be installed using [rustup](https://rustup.rs/).
With this Rust toolchain installed, qoqo can be installed using a pip command:

```bash
# After installing the Rust toolchain, execute the following:
pip install maturin
pip install qoqo
```

## roqoqo

To use roqoqo in a Rust project simply add

```toml
roqoqo = "1.0"
```

to the `[dependencies]` section in your Cargo.toml.
