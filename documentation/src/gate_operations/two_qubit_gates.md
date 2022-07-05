# Two-qubit gates

[< Back to overview](intro.md)

Two-qubit gates in qoqo/roqoqo represent atomic instructions in any quantum computer that act on a pair of qubits. In two qubit gates the two qubits are referred to as `control` and `target`. When initializing two qubit gates, the `control` is always the first and `target` the second argument. For two qubit gates the created matrix always corresponds to `control=1`, `target=0` and is a 4x4-dimensional matrix. This convention corresponds to the little endian encoding as described in chapter [conventions](../conventions.md). The full matrix form of the two-qubit gates implemented in qoqo/roqoqo is documented in this chapter.

The given form of the unitary matrix is consistent with  the following ordering of qubit states in a two-qubit state space:

 \\[
 \left|00 \right>  =  \textrm{state} 0 \\\\
 \left|01 \right>  =  \textrm{state} 1 \\\\
 \left|10 \right>  =  \textrm{state} 2 \\\\
 \left|11 \right>  =  \textrm{state} 3 \\\\
 \\]

## CNOT

The controlled not gate can be used to entangle two qubits. The unitary matrix for the CNOT gate is defined as

\\[
 U  = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & 0 & 1 \\\\
 0 & 0 & 1 & 0
 \end{pmatrix}
 \\]

## ControlledPhaseShift

Controlled phase shift gate implements  a phase shift applied on `target` qubit based on the value of the `control` qubit. The unitary matrix is given by

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & 1 & 0 \\\\
 0 & 0 & 0 & e^{i \theta}
 \end{pmatrix}
\\]

## FSwap

The fermionic SWAP gate has the following form of the unitary matrix

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 0 & 1 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & 0 & -1
 \end{pmatrix}
\\]

## InvSqrtISwap

The inverse square root of the ISwap gate has the full matrix form

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & \frac{1}{\sqrt{2}} & \frac{-i}{\sqrt{2}} & 0 \\\\
 0 & \frac{-i}{\sqrt{2}} & \frac{1}{\sqrt{2}} & 0 \\\\
 0 & 0 & 0 & 1
 \end{pmatrix}
\\]

## ISwap

The unitary matrix of the complex ISwap gate reads

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 0 & i & 0 \\\\
 0 & i & 0 & 0 \\\\
 0 & 0 & 0 & 1
 \end{pmatrix}
\\]

## SqrtISwap

The square root of the ISwap gate is represented by the matrix

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & \frac{1}{\sqrt{2}} & \frac{i}{\sqrt{2}} & 0 \\\\
 0 & \frac{i}{\sqrt{2}} & \frac{1}{\sqrt{2}} & 0 \\\\
 0 & 0 & 0 & 1
 \end{pmatrix}
\\]

 ## SWAP

 The SWAP gate is used to change the positions between two qubits. For example the SWAP gate is used by many compilation routines if the given connectivity on the quantum computing device is limited and the qubits need to be remapped in order to run a quantum program successfully on the quantum computing hardware. The full matrix form is given by

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & 0 & 1 & 0 \\\\
 0 & 1 & 0 & 0 \\\\
 0 & 0 & 0 & 1
 \end{pmatrix}
\\]

## XY

The definition of the XY gate in matrix form is given by

\\[
 U = \begin{pmatrix}
 1 & 0 & 0 & 0 \\\\
 0 & \cos(\theta/2) & i \sin(\theta/2) & 0 \\\\
 0 & i \sin(\theta/2) & \cos(\theta/2) & 0 \\\\
 0 & 0 & 0 & 1
 \end{pmatrix}
\\]