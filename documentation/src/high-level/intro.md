# High level interface: quantum programs

A quantum program in the definition of qoqo/roqoqo accepts input from a user/calling function, runs operations on a quantum computer and returns an output in the form of expectation values instead of raw output from the quantum computer.
To represent quantum programs qoqo/roqoqo provides a `QuantumProgram`.
It is intended as a high level interface between a complex program and the quantum computer, that can be called (almost) as easily as a normal function and handles variable replacement and the post-processing of the raw output of the quantum computer/simulator.

## Measurements

Post-processing the raw output is handled by `measurements`.
For many applications the measurement results of several circuits need to be combined to extract the required information.
`Measurement` objects group several quantum circuits and `measurement input` that determines how the raw output is post-processed.
A qoqo `measurement` combines one optional `constant_circuit` that is always executed first and a list of `circuits` that are executed after the constant circuit.
The type of `measurement` and `measurement input` depends on the type of readout used in the circuits.

The following `measurements` are implemented in qoqo/roqoqo:

* [PauliZProduct](pauliz.md),
* [ClassicalRegister](classical.md),
* [CheatedPauliZProduct](pauliz_cheated.md),
* [Cheated](cheated.md).

The `PauliZProduct` measurement is based on measuring the product of PauliZ operators for given qubits. Combined with a basis rotation of the measured qubits it can be used to measureme arbitrary expectaiont values. It can be used with projective qubit readouts like `MeasureQubit` or `PragmaRepeatedMeasurement`. It can be run on on real quantum computer hardware and simulators. For further details please refer to section  [PauliZProduct](pauliz.md).

The `ClassicalRegister` measurement returns the raw output of the  classical registers written during circuit execution.

The `CheatedPauliZProduct` measurement uses the the expectation values of products of Pauli matrices directly to calculate expectation values. It uses the `PragmaGetPauliProduct` readout and can only be used on simulator backends.

The `Cheated` measurement calculates expectation values by directly extracting the state vector `PragmaGetStateVector` or the density matrix `PragmaGetDensityMatrix` from a simulator and applying matrix multiplication.

For examples how to construct  measurement input and measurements see the pages for the separate measurement types.

## Variable replacement

Qoqo/roqoqo supports symbolic parameters in Operations, for example the rotation angles \\( \theta \\) in a `RotateX` gate operation. However,  a Circuit with symbolic parameters can not be simulated or executed on real hardware. The symbolic parameters need to be replaced with real floating point numbers first. A QuantumProgram contains a list of the free parameters (`input_parameter_names`).
Whe calling replace the parameters with its `run` method, it replaces the free parameters with the given input, executes the measurement on the given backend and returns the result.

For an example how to `run` a `QuantumProgram` [see here](program.md).
