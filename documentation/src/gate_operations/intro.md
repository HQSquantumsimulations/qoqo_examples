# List of Gate Operations

Operations are the atomic instructions in any quantum program that can be represented by roqoqo. Gate operations are single-, two- or multi-qubit gate operations that act on a set of qubits and can be executed on a quantum computing device. Mathematically speaking, a gate can be represented by a matrix.

A list of the gate operations available in qoqo and roqoqo with their mathematical description is provided in this section.

### Nomenclature

* A rotation angle is usually annotated with \\( \theta \\) and the according argument being `theta`.
* For the phase angle the symbol \\( \phi \\) is used, with the argument name `phi`.
* \\( \sigma_x \\), \\( \sigma_y \\), \\( \sigma_z \\) are the Pauli matrices X, Y, Z
\\[
    \sigma_x = \begin{pmatrix} 0 & 1 \\\\ 1 & 0 \end{pmatrix} := X, \quad \sigma_y = \begin{pmatrix} 0 & -i \\\\ i & 0 \end{pmatrix} := Y,  \quad \sigma_z = \begin{pmatrix} 1 & 0 \\\\ 0 & -1 \end{pmatrix} := Z
\\]

## Single-qubit gates

| Gate | Short Description |
|---------|---------|
| Hadamard     | The Hadamard gate, to create a superposition of states, and so to change the basis.  |
| InvSqrtPauliX     | The inverse square root of the PauliX gate \\( e^{i \frac{\theta}{4} \sigma_x} \\).  |
| PauliX     | The Pauli X gate, rotation with a fixed angle \\( \frac{\pi}{2} \\) corresponds to a "flip" on x-axis.  |
| PauliY     | The Pauli Y gate, rotation with a fixed angle \\( \frac{\pi}{2} \\) corresponds to a "flip" on y-axis.  |
| PauliZ     | The Pauli Z gate, rotation with a fixed angle \\( \frac{\pi}{2} \\) corresponds to a "flip" on z-axis.  |
| PhaseShiftState0     | Rotation around z-axis by angle \\(\theta\\) applied on state \\( \|0> \\) results in a phase shift compared to RotateZ gate. |
| PhaseShiftState1     | Rotation around z-axis by angle \\(\theta\\) applied on state \\( \|1> \\) results in phase shift compared to RotateZ gate. |
| RotateAroundSphericalAxis     | Implements a rotation around an axis in spherical coordinates.  |
| RotateX     | The rotation gate around x-axis \\( e^{-i \frac{\theta}{2} \sigma_x} \\).  |
| RotateXY     | Implements a rotation around an x- and y-axis in spherical coordinates.  |
| RotateY     | The rotation gate around y-axis \\( e^{-i \frac{\theta}{2} \sigma_y} \\).  |
| RotateZ     | The rotation gate around z-axis \\( e^{-i \frac{\theta}{2} \sigma_z} \\).  |
| SGate     | The S gate.  |
| SqrtPauliX     | The square root of the PauliX gate \\( e^{-i \frac{\theta}{4} \sigma_x} \\).  |
| TGate     | The T gate.  |


## Two-qubit gates
