# CheatedPauliZProduct Measurement

The `CheatedPauliZProduct` measurement in qoqo/roqoqo can be used only on a simulator backend. Apart from that limitation the required input `CheatedPauliZProductInput` follows the same structure as [PauliZProduct](pauliz.md) measurement, that can run on real quantum computing hardware.

Since this measurement operation is a cheated, and not a real, measurement its result does _not_ depend on the number of measurements, but the returned expectation value is always an exact solution. Thus, the `CheatedPauliZProduct` operation can be used to benchmark the implemented algorithm. Besides that, the `ChatedPauliZProduct` is faster than the `PauliZProduct` when used on a simulator.

## Example

The following code snippet represents the equivalent example as used for [PauliZProduct](pauliz.md) measurement, but with the code syntax adapted for the `CheatedPauliZProduct` operation.

Example in python:

```python
from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import CheatedPauliZProduct, CheatedPauliZProductInput
from qoqo import QuantumProgram

# initialize |\psi> = (|0> + |1>)/ sqrt(2)
init_circuit = Circuit()
init_circuit += ops.Hadamard(0)

# Z-basis measurement circuit
z_circuit = Circuit()
z_circuit += ops.DefinitionBit("ro_z", 1, is_output=True)

# X-basis measurement 
x_circuit = Circuit()
x_circuit += ops.DefinitionBit("ro_x", 1, is_output=True)

# The dictionary of the pauli matrix to apply to each qubit in the form {qubit: pauli}.
# Allowed values to be provided for 'pauli' are: 0 = identity, 1 = PauliX, 2 = PauliY, 3 = PauliZ.
pauliz_products = {0: 3}
paulix_products = {0: 1}
# PragmaGetPauliProduct returns a Pauli product expectation value after applying
# a Rotate to another basis. It performs all of the operation on a clone of the quantum register,
# so that the actual quantum register remains unchanged.
z_circuit += ops.PragmaGetPauliProduct(qubit_paulis=pauliz_products, readout="ro_z", circuit=init_circuit)
x_circuit += ops.PragmaGetPauliProduct(qubit_paulis=paulix_products, readout="ro_x", circuit=init_circuit)

# Preparing the measurement input for CheatedPauliZProductInput
measurement_input = CheatedPauliZProductInput()
# Next, pauli products are added to the CheatedPauliZProductInput
x_basis_index = measurement_input.add_pauliz_product("ro_x")
z_basis_index = measurement_input.add_pauliz_product("ro_z")

# Add a result (the expectation value of H) that is a combination of
# the PauliProduct expectation values.
measurement_input.add_linear_exp_val(
    "<H>", {x_basis_index: 0.1, z_basis_index: 0.2},
)

measurement = CheatedPauliZProduct(
   constant_circuit=init_circuit,
   circuits=[x_circuit, z_circuit],
   input=measurement_input,
)
```

The same example in Rust:

```Rust
:dep roqoqo = "1.0.0-alpha.5"

extern crate roqoqo;

use roqoqo::{Circuit, operations::*, QuantumProgram};
use roqoqo::measurements::{CheatedPauliZProduct, CheatedPauliZProductInput};
use roqoqo::backends::{EvaluatingBackend, RegisterResult};
use std::collections::HashMap;

// initialize |psi>
let mut init_circuit = Circuit::new();
init_circuit.add_operation(Hadamard::new(0));

// Z-basis measurement circuit with 1000 shots
let mut z_circuit = Circuit::new();
z_circuit.add_operation(DefinitionBit::new("ro_z".to_string(), 1, true));
z_circuit.add_operation(
    PragmaGetPauliProduct::new(HashMap::from([(0, 3)]), "ro_z".to_string(), init_circuit.clone()),
);

// X-basis measurement circuit with 1000 shots
let mut x_circuit = Circuit::new();
x_circuit.add_operation(DefinitionBit::new("ro_x".to_string(), 1, true));
x_circuit.add_operation(
    PragmaGetPauliProduct::new(HashMap::from([(0, 1)]), "ro_x".to_string(), init_circuit.clone()),
);

// Preparing the measurement input for one qubit
// The PauliZProductInput starts with just the number of qubtis
// and if to use a flipped measurements set.
let mut measurement_input = CheatedPauliZProductInput::new();
// Next, pauli products are added to the PauliZProductInput
// Read out product of Z on site 0 for register ro_z (no basis change)
measurement_input
    .add_pauliz_product("ro_z".to_string());
// Read out product of Z on site 0 for register ro_x
// (after basis change effectively a <X> measurement)
measurement_input
    .add_pauliz_product("ro_x".to_string());

// Last, instructions how to combine the single expectation values
// into the total result are provided.
// Add a result (the expectation value of H) that is a combination
// of the PauliProduct expectation values.
measurement_input
    .add_linear_exp_val(
        "<H>".to_string(), HashMap::from([(0, 0.1), (1, 0.2)]),
    )
    .unwrap();

let measurement = CheatedPauliZProduct {
    input: measurement_input,
    circuits: vec![z_circuit.clone(), x_circuit.clone()],
    constant_circuit: Some(init_circuit.clone()),
};

// Now, the `PauliZProduct` measurement is prepared to be used
// in a QuantumProgram just like:
let program = QuantumProgram::CheatedPauliZProduct {
    measurement,
    input_parameter_names: vec![],
};
```
