# Noise Operations

qoqo/roqoqo enables finely controlled noise models. Noise acting on the quantum commputer is modeled as a single noise operation affecting on a quantum computer.
The noise operations can be directly added to a quantum circuit and be simulated by compatible backends.
The strength of noise is determined by defining a `gate_time` and a `rate`. The noise pragma affects the system like Lindblad type noise acting on the system with the rate `rate` for the time `gate_time`

For example to add dephasing noise to qubit 0, damping noise to qubit 1 and depolarising noise to qubit 2 acting on the system after a `CNOT` has been applied we implement:

```rust

use roqoqo::Circuit;
use roqoqo::operations;

let mut circuit = Circuit::new();
circuit += operations::CNOT::new(0,1);
// Adding dephasing noise acting on gate 0 with gate_time 1.0 and rate 1e-3
circuit += operations::PragmaDephasing::new(0, 1.0, 1e-3);
circuit += operations::PragmaDamping::new(1, 1.0, 2e-3);
circuit += operations::PragmaDepolarising::new(3, 1.0, 5e-3);

```

```python

from qoqo import Circuit
from qoqo import operations

circuit = Circuit()
circuit += operations.CNOT(0,1)
#Adding dephasing noise acting on gate 0 with gate_time 1.0 and rate 1e-3
circuit += operations.PragmaDephasing(qubit=0, gate_time=1.0, rate=1e-3)
circuit += operations.PragmaDamping(1, 1.0, 2e-3)
circuit += operations.PragmaDepolarising(3, 1.0, 5e-3)

```