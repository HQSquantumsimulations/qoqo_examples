# Conventions

This section gives a quick overview of some of the conventions used in qoqo/roqoqo.

## Definitions

qoqo syntax is independent of the underlying hardware of the quantum computer. However, it is modelled for the so-called gate-based quantum computation.

* `gate`: an `operation` that can be applied on a qubit to carry out a specific computational task, for example an operation given by a rotation gate (e.g. `RotateZ`). Mathematically speaking, a gate can be represented by a matrix. Sometimes we might use gate, operation and gate operation interchangeably throughout the documentation.
* `qubit`: the smallest unit of data that a quantum computer can process and store. If referring to a mathematical representation, a qubit can be written in form of a vector.
* `Circuit`: is a an element that includes a sequence of gate operations applied on the involved qubits. A Circuit can be interpreted as an iterable helper object without a physical manifistation. It can be used as a schematic representation of the system.


## Qubit states
For the two basis states of a single qubit we define

\\[
 \left\| 0 \right>  =  \left|\textrm{false} \right> =  \left| \downarrow \right> = \begin{pmatrix}
 1 \\\\
 0
 \end{pmatrix} \\\\
 \\]

 \\[
 \left \|1 \right>  =  \left|\textrm{true} \right> =  \left| \uparrow \right> = \begin{pmatrix}
 0 \\\\
 1
 \end{pmatrix} \\\\
 \\]

Whenever a new Circuit is created all qubits are initialized in the state |0> which can be changed by the application of single qubits gates. 
For example the user can apply the PauliX gate to flip the qubit from state |0> into state |1> and vice versa.

Python code snippet example:

```python
from qoqo import operations, Circuit

circuit = Circuit()
# qubit 0 is initialized in state |0> and flipped into state |1> by PauliX gate.
circuit += operations.PauliX(qubit=0)
```

Rust code snippet example:

```rust
:dep roqoqo = "1.0.0-alpha.4"
extern crate roqoqo;

use roqoqo::{Circuit, operations::PauliX};

let mut circuit = Circuit::new();
// qubit 0 is initialized in state |0> and flipped into state |1> by PauliX gate.
circuit += PauliX::new(0);
```


## Endianness

To define the convention referring to the sequence of qubits, qoqo and roqoqo use the so-called little endian encoding, that is the least significant qubit is at the smallest index. This information might be especially relevant if it comes to two-qubit gates, for example.

For a two-qubit state space we have the following ordering of qubit states:

 \\[
 \left|00 \right>  =  \textrm{state} 0 \\\\
 \left|01 \right>  =  \textrm{state} 1 \\\\
 \left|10 \right>  =  \textrm{state} 2 \\\\
 \left|11 \right>  =  \textrm{state} 3 \\\\
 \\]

Therefore if we combine two single-qubit gates in Matrix form it follows:

\\[
 \textrm{PauliX}(0) \  \textrm{PauliZ}(1)  = \begin{pmatrix}
 1 & 0 \\\\
 0 & -1
 \end{pmatrix} \otimes \begin{pmatrix}
 0 & 1 \\\\
 1 & 0
 \end{pmatrix}
 \\]


## Operation order

 When adding qoqo operations to circuits and reading them written out. The first operation added to the circuit, the first operation from the left will be executed first. This leads to an inversion of the order when transcribing to matrix form where the matrix to the right acts first.

 \\[
 \textrm{PauliX}(0) \cdot  \textrm{PauliZ}(0)  =  \textrm{PauliZ(0).unitary_matrix()} \cdot  \textrm{PauliX(0).unitary_matrix()} = \begin{pmatrix}
 1 & 0 \\\\
 0 & -1
 \end{pmatrix}  \begin{pmatrix}
 0 & 1 \\\\
 1 & 0
 \end{pmatrix}
 \\]

## Qubit names

Qoqo distinguishes between single-qubit gates, tw- qubit gates and multi-qubit gates.
* In single-qubit gates the qubit is always referred to as `qubit`, 
* in two-qubit gates the two qubits are referred to as `control` and `target`,
* and in multi-qubit gates the `qubits` are given as a vector of all involved qubits.

When initializing two-qubit gates, the `control` is always the first and `target` the second argument.

## Unitary Matrix

To help determine the type of the gate, the unitary qoqo/roqoqo operations support the function `unitary_matrix()` that returns the definition of the gate in matrix form. This definition ignores the qubits of the gate to fit in the smallest possible matrix dimension.
* For single-qubit gates the created matrix always corresponds to `qubit=0` and has a 2x2-dimension.
* For two-qubit gates the created matrix always corresponds to `control=1`, `target=0` and is a 4x4-dimensional matrix. This convention corresponds to the little endian encoding described above.
* For multi-qubit gates it always corresponds to `qubits=[0..N]` where `N` is the number of qubits in the qubit vector of the multi-qubit gate.
