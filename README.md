<img src="qoqo_Logo_vertical_color.png" alt="qoqo logo" width="300" />

# Examples for qoqo

This repository contains the [user documentation](https://hqsquantumsimulations.github.io/qoqo_examples/) and  a set of introductory examples for the [qoqo/roqoqo](https://github.com/HQSquantumsimulations/qoqo) quantum computing toolkit by [HQS Quantum Simulations GmbH](https://quantumsimulations.de).

roqoqo is a Rust library to represent quantum circuits and qoqo is the python interface to roqoqo.

What roqoqo/qoqo is:

* A toolkit to represent quantum programs including circuits and measurement information
* A thin runtime to run quantum measurements
* A way to serialize quantum circuits and measurement information
* A set of optional interfaces to devices, simulators and toolkits (e.g. [qoqo_qest](https://github.com/HQSquantumsimulations/qoqo-quest), [qoqo_mock](https://github.com/HQSquantumsimulations/qoqo_mock), [qoqo_qasm](https://github.com/HQSquantumsimulations/qoqo_qasm))

What roqoqo/qoqo is **not**:

* A decomposer translating circuits to a specific set of gates
* A quantum circuit optimizer
* A collection of quantum algorithms

The examples in this repository are available in different versions. The versions using the qoqo Python interface are available in the [qoqo folder](./qoqo/) as Ipython notebooks.
The versions using the roqoqo Rust library directly can be found in the [roqoqo folder](./roqoqo/). The examples using roqoqo are again available in two versions.
One version in the form of a standalone rust program and one experimental version using Jupyter notebooks with the Rust [evcxr](https://github.com/google/evcxr) Jupyter kernel.

Currently the available examples include:

1. An introduction to qoqo/roqoqo ([Python](./qoqo/Intro_to_qoqo.ipynb) | [Rust Program](./roqoqo/standaloe/1_Intro_to_roqoqo/) | [Rust Jupyter](./roqoqo/notebooks/1_Intro_to_roqoqo.ipynb))

2. An example how to perform expectation value measurements with qoqo/roqoqo ([Python](./qoqo/Measurement_Example.ipynb) | [Rust Program](./roqoqo/standalone/2_Measurement_example/) | [Rust Jupyter](./roqoqo/notebooks/2_Measurement_example.ipynb))
3. A quantum teleportation example ([Python](./qoqo/3_Teleportation_Example.ipynb) | [Rust Program](./roqoqo/standalone/3_Teleportation_example/) | [Rust Jupyter](./roqoqo/notebook/3_Teleportation_example.ipynb))
4. A simple Variational Quantum Eigensolver (VQE) using the Variation Hamiltonian Ansatz (VHA) ([Python](./qoqo/Simple_VHA_with_qoqo.ipynb) | [Rust Program](./roqoqo/) | [Rust Jupyter](./roqoqo/))

## General Notes

This project is partly supported by [PlanQK](https://planqk.de).
