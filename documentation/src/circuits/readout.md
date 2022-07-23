# Readout

In order to extract information from a quantum computer a measurement operation needs to be applied. For measurements in qoqo classical registers are defined in the quantum circuit. The measurement of a qubit on quantum computing hardware is always a projective measurement in the `z`-basis yielding `0` or `1`. The measurement results are stored in the classical registers. To define all information necessary for a readout in qoqo, the user needs to add a `Definition` and a `Measurement` operation to the circuit.

A qoqo/roqoqo circuit can define three types of registers: Bit (`DefinitionBit`), Float (`DefinitionFloat`) and Complex (`DefinitionComplex`).
Each register is declared in a Circuit with its special operation tag, the given register name and a lenght. Whether or not a register is returned as an output is controlled by a boolean paramater, as shown in the example below.

A code snippet example in Rust:

```rust
:dep roqoqo = "1.0.0-alpha.4"
extern crate roqoqo;

use roqoqo::Circuit;
use roqoqo::operations;

let mut circuit = Circuit::new();
// A bit register of length 2 that is returned as an output of the circuit
circuit += operations::DefinitionBit::new("bit_register".to_string(), 2, true);
// A complex register of length 3 that is not returned as an output
circuit += operations::DefinitionComplex::new("complex_register".to_string(), 3, false);
```

A code snippet example in python:

```python
from qoqo import Circuit
from qoqo import operations as ops

circuit = Circuit()
circuit += ops.DefinitionBit("bit_register", 2, is_output= True)
circuit += ops.DefinitionComplex("complex_register", 3, is_output = False)
```

The content of one register is a list/vector of the corresponding type. Since quantum circuits are typically executed many times on quantum computing hardware, to account for the probabilistic nature of quantum mechanics, registers that are marked as output are returned as a list of lists from a qoqo/roqoqo [backend](src/backend).

Registers are filled in the circuits by applying measurement operations with the correct register set as output in a quantum circuit.
On quantum computing hardware only projective measurements into a bit register are available. On simulators one can also read out the full state vector into a complex register.

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

A code snippet example in python:

```python
from qoqo import Circuit
from qoqo import operations as ops

circuit = Circuit()
# Add a Bit register to the circuit for the qubit readout
circuit += ops.DefinitionBit("bit_register", 2, is_output = True)
# Add measurement instructions for each qubit, when using hardware
circuit += ops.MeasureQubit(0,"bit_register",0)
circuit += ops.MeasureQubit(1,"bit_register",1)

# Alternatively, define a Complex register to readout the state vector
circuit += ops.DefinitionComplex("complex_register", 3, is_output = False)
# Measure the state vector when running the circuit on a simulator
circuit += ops.PragmaGetStateVector("complex_register")
```
