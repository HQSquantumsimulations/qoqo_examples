# Cheated Measurement

The `Cheated` measurement in qoqo/roqoqo can be used only on a simulator backend to obtain a quantum state. Cheated measurements are only possible with simulator backends that can return the state vector or the density matrix of the quantum state. The expectation values are defined by a matrix representation of the measured observables. The advantage here is that one can obtain the pure expectation values directly, independent of the post-processing routines of the certain measurement methods (e.g. compared to [CheatedPauliZProduct](pauliz_cheated.md)).
