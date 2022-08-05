# Readout

In general, to extract information from a quantum computer a measurement operation needs to be applied. However, the readout in qoqo/roqoqo is more universal than what is possible on a general quantum computer also encompassing readout from simulators like the complex state vector. In particular:

* qoqo/roqoqo uses register based readouts where all classical information is returned from the quantum circuit using classical registers declared at the start of the circuit.
* Classical registers can contain three types of classical data: Bit (or bool), Float (f64/double) and Complex.
* After being declared at the start of the circuit, information is written to the registers in the `Circuit` by `Measurement` or `Pragma` operations.
* If the register is declared as an output register it is returned after the execution of the circuit.

## Registers

As introduced above, qoqo/roqoqo circuit can define three types of registers: Bit (`DefinitionBit`), Float (`DefinitionFloat`) and Complex (`DefinitionComplex`).
Each register is declared in a `Circuit` with its special operation tag, the given register name and a lenght. Whether or not a register is returned as an output is controlled by a boolean paramater, as shown in the example below.

The content of one register is a list/vector of the corresponding type. Since quantum circuits are typically executed many times on quantum computing hardware, to account for the probabilistic nature of quantum mechanics, registers that are marked as output (`is_output=True`) are returned as a list of lists from a qoqo/roqoqo [backend](src/backend). In some use cases, e.g. for conditional processing within a circuit, the readout does not need to be returned. In that case the user needs to set the parameter to `is_output=False`.


A code snippet example in python:

```python
from qoqo import Circuit
from qoqo import operations as ops

circuit = Circuit()
circuit += ops.DefinitionBit("bit_register", 2, is_output=True)
circuit += ops.DefinitionComplex("complex_register", 3, is_output=False)
```


A code snippet example in Rust:

```rust
:dep roqoqo = "1.0.0-alpha.5"
extern crate roqoqo;

use roqoqo::Circuit;
use roqoqo::operations;

let mut circuit = Circuit::new();
// A bit register of length 2 that is returned as an output of the circuit
circuit += operations::DefinitionBit::new("bit_register".to_string(), 2, true);
// A complex register of length 3 that is not returned as an output
circuit += operations::DefinitionComplex::new("complex_register".to_string(), 3, false);
```


## Measurements

Registers are filled in the circuits by applying measurement operations with the correct register set as output in a quantum circuit.
* On quantum computing _hardware_ only _projective_ measurements into a bit register are available, that is a measurement in the `Z`-basis yielding `0` or `1`.
* On _simulators_ one can also read out the full state vector into a complex register.

As shown in the example below, the operation **MeasureQubit** can be used to provide measurement instructions for each individual qubit when using real quantum computing hardware. The input parameter `qubit` specifies the qubit to be measured, whereas the parameter `readout_index` defines the position in the classical register `readout` where the measurement value of the `qubit` is stored. The explicit assignment of a qubit measurement to a readout register index is necessary to handle any remapping, that might be introduced by a quantum algorithms, properly.

If the measurement is performed on a simulator or if the quantum computer hardware device supports the operation **PragmaRepeatedMeasurement**, it can be used instead of `MeasureQubit` command to provide the measurement instruction for all qubits in `qubit_mapping` that needs to be repeated N times (`number_measurements`).  For further available pragma measurement instructions please refer to the section [pragma operations](pragma.md).

A code snippet example in python:

```python
from qoqo import Circuit
from qoqo import operations as ops

circuit = Circuit()
# Add a Bit register to the circuit for the qubit readout
circuit += ops.DefinitionBit("bit_register", 2, is_output = True)
# Add measurement instructions for each qubit, when using hardware
circuit += ops.MeasureQubit(qubit=0, readout="bit_register", readout_index=0)
circuit += ops.MeasureQubit(qubit=1, readout="bit_register", readout_index=1)

# Alternatively, define a Complex register to readout the state vector
circuit += ops.DefinitionComplex("complex_register", 3, is_output = False)
# Measure the state vector when running the circuit on a simulator
circuit += ops.PragmaGetStateVector("complex_register")
```

A code snippet example in Rust:

```rust
use roqoqo::Circuit;
use roqoqo::operations;

let mut circuit = Circuit::new();
// Add a Bit register to the circuit for the qubit readout
circuit += operations::DefinitionBit::new("bit_register".to_string(), 2, true);
// Add measurement instructions for each qubit, when using hardware
circuit += operations::MeasureQubit::new(0, "bit_register".to_string(), 0);
circuit += operations::MeasureQubit::new(1, "bit_register".to_string(), 1);

// Alternatively, define a Complex register to readout the state vector
circuit += operations::DefinitionComplex::new(
    "complex_register".to_string(), 3, false,
);
// Measure the state vector when running the circuit on a simulator
circuit += operations::PragmaGetStateVector::new(
    "complex_register".to_string(), None,
);
```
