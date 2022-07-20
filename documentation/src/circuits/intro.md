# Quantum circuit

Qoqo is a quantum computing toolkit based on the so-called circuit model. A `Circuit` in qoqo is a an element that includes a sequence of gate operations applied on the involved qubits. A Circuit can be interpreted as an iterable helper object without a physical manifistation. It can be used as a schematic representation of the system.

In order to create a useful Circuit in qoqo, the following items need to be added after the Circuit initialization:
* Operation: to represent the actual instructions, for example `RotateZ` or `CNOT` gate operations.
* Measurement: to specify the required measurement details that are necessary to extract the information from a quantum computer.
* [Definition](readout.md): to define the classical register where the measurement results are stored.

All these three types of items are treated the same way, i.e. as `operations`. For example, they can be added to a Circuit by using the operand `+=`. 

A sample code snippet with qoqo in python is provided here:

    from qoqo import Circuit
    from qoqo import operations as ops
    # create a new circuit
    circuit = Circuit()
    # Rotation around Z axis by pi/2 on qubit 0
    circuit += ops.RotateZ(qubit=0, theta=1.57)
    # Entangling qubits 0 and 1 with CNOT gate
    circuit += ops.CNOT(control=0, target=1)

The same sample code snippet with roqoqo in Rust is given here:

    :dep roqoqo = "1.0.0-alpha.3"
    extern crate roqoqo;
    
    use roqoqo::{Circuit, operations::*};

    // Create a new _modifiable_ circuit
    let mut circuit = Circuit::new();
    // Apply rotation around Z axis by pi/2 on qubit 0
    circuit += RotateZ::new(0, 1.57.into());
    // Establish entanglement between qubits 0 and 1
    circuit += CNOT::new(0, 1);


Besides the simple example qoqo provides a variety of operations, i.e.
* [Unitary operations](unitary.md): the basic gate operations that represent the atomic instructions.
* [Noise operations](noise.md): to be used in simulations to model the noise of a system.
* [Pragma operations](pragma.md): that are specific operations that can _not_ run on all universal quantum computers.


## Available functions

An overview of available functions that can be applied on a Circuit is provided in the table below. For further details on input and output parameter please see the API documentation of [roqoqo](https://docs.rs/roqoqo/latest/roqoqo/struct.Circuit.html) and [qoqo](https://qoqo.readthedocs.io/en/latest/generated/generated/qoqo.Circuit.html).

For Circuit the following functions are defined in qoqo:

| Function | Description | Only in |
|---------|---------|---------| 
| `+` and `+=` | Add two circuits or an operation to the Circuit. | |
| `add(operation)` | Adds an Operation to the Circuit. | Python|
| `add_operation(operation)` | Adds the specified operation to the Circuit. | Rust|
| `count_occurences("operation")` | Returns the number of operations in the Circuit with the specified operation name. | |
| `default()` | Creates an empty Circuit. | Rust|
| `definitions()` | Returns the definitions in the Circuit. | |
| `extend(iterator)` | Adds the operations in the specified iterator to the Circuit. | Rust|
| `filter_by_tag` | Returns a list of operations with given name. | Python|
| `from_bincode` | Converts the bincode representation of the Circuit to a qoqo Circuit. | Python|
| `from_iter(iterator)` | Creates a Circuit from the items in the specified iterator. |  Rust|
| `from_json` | Converts the json representation of the Circuit to a qoqo Circuit. | Python|
| `get(index)` | Returns the operation at the specified index in the Circuit. | |
| `get_mut(index)` | Returns mutable reference to the operation at the specified index in the Circuit. | Rust|
| `get_operation_types()` | Returns a list of all of the operations (names) in the Circuit. | |
| `get_slice` | Returns copy of a slice of the Circuit. | Python|
| `[...]` | Gets a slice of the Circuit (returned as a vector). | Rust |
| `iter()` | Creates an iterator of the Circuit. | Rust|
| `len()` | Returns the length of the Circuit. | |
| `is_empty()` | Returns a boolean of whether the Circuit contains any definitions and operations or not. | |
| `involved_qubits()` | Returns the qubits invovlved in the whole Circuit. | |
| `new()` | Creates an empty Circuit. | Rust|
| `operations()` | Returns the operations in the Circuit. | |
| `overrotate()` | Returns clone of the Circuit with all overrotation Pragmas applied. | |
| `remap_qubits(mapping)` | Remaps the qubits in (a copy of) the Circuit according to the specified mapping. | |
| `substitute_parameters(calculator)` | Substitutes any symbolic parameters in (a copy of) the Circuit according to the specified input parameter (type: Calculator of [qoqo_calculator](https://github.com/HQSquantumsimulations/qoqo_calculator)). | |
| `to_bincode` | Returns the bincode representation of the Circuit. | Python|
| `to_json` | Returns the json representation of the Circuit. | Python|