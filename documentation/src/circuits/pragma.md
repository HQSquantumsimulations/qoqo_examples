# Pragma Operations

Pragma operations in qoqo/roqoqo are operations that are not part of the set of operations that can run on all universal quantum computers.

Pragma operations can be used to:

* Annotate a quantum circuit with additional information that is not necessary for execution (e.g `PragmaGlobalPhase`, `PragmaStartDecompositionBlock`)
* Place operations that lead to a repeated execution of a circuit (`PragmaRepeatedMeasurement`, `PragmaSetNumberOfMeasurements`)
* Place operations that are only available on specific hardware (e.g. `PragmaChangeDevice`, `PragmaSleep`)
* Place operations that are only available on simulator (e.g. `PragmaSetStateVector`, `PragmaGetStateVector`)

For a full list of available Pragma operations see the API documentation of[roqoqo](https://docs.rs/roqoqo/latest/roqoqo/operations/index.html)
 and [qoqo](https://qoqo.readthedocs.io/en/latest/generated/generated/qoqo.operations.html#module-qoqo.operations).