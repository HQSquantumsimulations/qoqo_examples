# Quantum circuit

Qoqo is a quantum computing toolkit based on the so-called circuit model. A `Circuit` in qoqo is a an element that includes a sequence of gate operations applied on the involved qubits. A Circuit can be interpreted as an iterable helper object without a physical manifistation. It can be used as a schematic representation of the system.

In order to create a useful Circuit in qoqo, the following items need to be added after the Circuit initialization:
* Operation: to represent the actual instructions, for example `RotateZ` or `CNOT` gate operations.
* Measurement: to specify the required measurement details that are necessary to extract the information from a quantum computer.
* [Definition](readout.md): to define the classical register where the measurement results are stored.

All these three types of items are treated the same way, i.e. as `operations`. For example, they can be added to a Circuit by using the operand `+=`. 

A sample code snippet with qoqo in python is provided here:

```python
from qoqo import Circuit
from qoqo import operations as ops
# create a new circuit
circuit = Circuit()
# Rotation around Z axis by pi/2 on qubit 0
circuit += ops.RotateZ(qubit=0, theta=1.57)
# Entangling qubits 0 and 1 with CNOT gate
circuit += ops.CNOT(control=0, target=1)
```

The same sample code snippet with roqoqo in Rust is given here:

```rust
:dep roqoqo = "1.0.0-alpha.5"
extern crate roqoqo;

use roqoqo::{Circuit, operations::*};

// Create a new _modifiable_ circuit
let mut circuit = Circuit::new();
// Apply rotation around Z axis by pi/2 on qubit 0
circuit += RotateZ::new(0, 1.57.into());
// Establish entanglement between qubits 0 and 1
circuit += CNOT::new(0, 1);
```


Besides the simple example qoqo provides a variety of operations, i.e.
* [Unitary operations](unitary.md): the basic gate operations that represent the atomic instructions.
* [Noise operations](noise.md): to be used in simulations to model the noise of a system.
* [Pragma operations](pragma.md): that are specific operations that can _not_ run on all universal quantum computers.


For details on the **available functions** that can be applied on a Circuit please refer to the **API documentation** of [roqoqo](https://docs.rs/roqoqo/latest/roqoqo/struct.Circuit.html) and [qoqo](https://qoqo.readthedocs.io/en/latest/generated/generated/qoqo.Circuit.html).
