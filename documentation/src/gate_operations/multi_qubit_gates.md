# Multi-qubit gates

[< Back to overview](intro.md)

Multi-qubit gates in qoqo/roqoqo represent atomic instructions in any quantum computer that act on `N` number of qubits. In multi-qubit gates the `qubits` are given as a vector of all involved qubits. The unitary matrix of a multi-qubit gate corresponds to the notation based on `qubits=[0..N]` where `N` is the number of qubits in the qubit vector of the multi-qubit gate.

## MultiQubitMS

The Molmer-Sorensen gate between multiple qubits. The gate applies the rotation under the product of PauliX operators on multiple qubits. In mathematical terms the gate applies

\\[
    e^{-i * \theta/2 * X_i0 * X_i1 * ... * X_in},
\\]

whereas \\(\theta\\) is the angle parameter of the multi-qubit Molmer-Sorensen gate.

## MultiQubitZZ

The multi qubit PauliZ-product gate. he gate applies the rotation under the product of PauliZ operators on multiple qubits.

\\[
    e^{-i * \theta/2 * Z_i0 * Z_i1 * ... * Z_in},
\\]

whereas \\(\theta\\) is the angle parameter of the multi-qubit PauliZ-product gate.
