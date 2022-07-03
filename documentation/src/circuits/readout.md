# Readout

qoqo/roqoqo uses register based readouts.
A qoqo/roqoqo circuit can define three types of Registers: `Bit`, `Float` and `Complex`.

Each register is declared in a Circuit with a special operation and given a name, a lenght.

```rust
use roqoqo::Circuit;
use roqoqo::operations;

let mut circuit = Circuit::new();
// A bit register of length 2 that is returned as an output of the circuit
circuit += operations::DefinitionBit::new("bit_register".to_string(), 2, true);
// A complex register of length 3 that is not returned as an output
circuit += operations::DefinitionComplex::new("complex_register".to_string(), 3, false);
```

```python
from qoqo import Circuit
from qoqo import operations as ops

circuit = Circuit()
circuit += ops.DefinitionBit("bit_register", 2, is_output= True)
circuit += ops.DefinitionComplex("complex_register", 3, is_output = False)
```

The content of one register is a list/vector of the corresponding type.
Since quantum circuits are typically run many times, registers that are marked as output are returned as a list of lists from a qoqo/roqoqo [backend](src/backend).

Registers are filled in the circuits by applying measurement operations with the correct register set as output in a quantum circuit.
On quantum computing hardware only projective measurements into a bit register are available. On simulators one can also for example read-out the full state-vector into a complex register.

```rust
use roqoqo::Circuit;
use roqoqo::operations;

let mut circuit = Circuit::new();
circuit += operations::DefinitionBit::new("bit_register".to_string(), 2, true);
circuit += operations::MeasureQubit::new(0, "bit_register".to_string(), 0);
circuit += operations::MeasureQubit::new(1, "bit_register".to_string(), 1);
circuit += operations::DefinitionComplex::new("complex_register".to_string(), 3, false);
circuit += operations::PragmaGetStateVector::new("complex_register".to_string(), None);
```

```python
from qoqo import Circuit
from qoqo import operations as ops

circuit = Circuit()
circuit += ops.DefinitionBit("bit_register", 2, is_output= True)
circuit += ops.MeasureQubit(0,"bit_register",0)
circuit += ops.MeasureQubit(1,"bit_register",1)
circuit += ops.DefinitionComplex("complex_register", 3, is_output = False)
circuit += ops.PragmaGetStateVector("complex_register")
```