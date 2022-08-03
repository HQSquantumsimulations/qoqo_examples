# Unitary Operations

Unitary operations are gate operations that can be executed on all universal quantum computers. Gate operations are the atomic instructions in any quantum program that can be represented by qoqo/roqoqo. Please refer to section [gate operations](../gate_operations/intro.md) for further details.

## Unitary Matrix

To help determine the type of the gate, the unitary qoqo/roqoqo operations support the function `unitary_matrix()` that returns the definition of the gate in matrix form. This definition ignores the qubits of the gate to fit in the smallest possible matrix dimension.
* For single qubit gates the created matrix always corresponds to `qubit=0` and has a 2x2-dimension.
* For two qubit gates the created matrix always corresponds to `control=1`, `target=0` and is a 4x4-dimensional matrix. This convention corresponds to the little endian encoding described above.
* For multi qubit gates it always corresponds to `qubits=[0..N]` where `N` is the number of qubits in the qubit vector of the multi qubit gate.