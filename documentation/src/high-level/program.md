# QuantumProgram and Variable-Replacement

The main use of QuantumProgram is to contain a Measurement that treats expectation values or output registers of quantum circuits that contain symbolic parameters. Circuit with symbolic parameters can not be simulated or executed on real hardware. The symbolic parameters need to be replaced with real floating point numbers first. Thus, a QuantumProgram contains a list of the free parameters (`input_parameter_names`) and can automatically replace the parameters when it is executed. It replaces the parameters by its `run` methods and returns the result.

To demonstrate the replacement of variables the example discussed in [PauliZProduct](pauliz.md) is modified to use a state \\( |\psi> \\) with a free angle between `|0>` and `|1>`. Such a state can be prepared by the application of `RotateX` gate (instead of `Hadamard` gate).

In python:

```python
from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import PauliZProduct, PauliZProductInput
from qoqo import QuantumProgram
from qoqo_quest import Backend

# initialize |psi>
init_circuit = Circuit()
# Apply a RotateY gate with a symbolic angle
# To execute the circuit this symbolic parameter needs to be replaced
# by a real number with the help of a QuantumProgram
init_circuit += ops.RotateX(0, "angle")

# Z-basis measurement circuit with 1000 shots
z_circuit = Circuit()
z_circuit += ops.DefinitionBit("ro_z", 1, is_output=True)
z_circuit += ops.PragmaRepeatedMeasurement("ro_z", 1000, None)

# X-basis measurement circuit with 1000 shots
x_circuit = Circuit()
x_circuit += ops.DefinitionBit("ro_x", 1, is_output=True)
# Changing to the X basis with a Hadamard gate
x_circuit += ops.Hadamard(0)
x_circuit += ops.PragmaRepeatedMeasurement("ro_x", 1000, None)

# Preparing the measurement input for one qubit
measurement_input = PauliZProductInput(1, False)
# Read out product of Z on site 0 for register ro_z (no basis change)
z_basis_index = measurement_input.add_pauliz_product("ro_z", [0,])
# Read out product of Z on site 0 for register ro_x
# (after basis change effectively a <X> measurement)
x_basis_index = measurement_input.add_pauliz_product("ro_x", [0,])

# Add a result (the expectation value of H) that is a combination of
# the PauliProduct expectation values.
measurement_input.add_linear_exp_val(
    "<H>", {x_basis_index: 0.1, z_basis_index: 0.2},
)

measurement = PauliZProduct(
   constant_circuit=init_circuit,
   circuits=[z_circuit, x_circuit],
   input=measurement_input,
)

# A quantum program is created from the measurement
# and "angle" is registered as a free input parameter.
# The QuantumProgram now has one free parameter
# that needs to set when executing it.
# The symbolic value angle in the circuits will be replaced
# by that free parameter during execution.
program = QuantumProgram(
   measurement=measurement,
   input_parameter_names=["angle"],
)

# To execute a QuantumProgram a backend needs to be defined.
# Create a backend simulating one qubit.
backend = Backend(1)

# Run QuantumProgram on the backend by setting the parameter value.
expectation_values = program.run(backend, [0.785])
```

The same example in Rust:

```Rust
:dep roqoqo = "1.0.0-alpha.5"
:dep roqoqo-quest = "0.7.0"

extern crate roqoqo;
extern crate roqoqo_quest;

use roqoqo::{Circuit, operations::*, QuantumProgram};
use roqoqo::measurements::{PauliZProduct, PauliZProductInput};
use roqoqo::backends::{EvaluatingBackend, RegisterResult};
use roqoqo_quest::Backend;
use std::collections::HashMap;

// initialize |psi>
let mut init_circuit = Circuit::new();
init_circuit.add_operation(RotateX::new(0, "angle".into()));

// Z-basis measurement circuit with 1000 shots
let mut z_circuit = Circuit::new();
z_circuit.add_operation(DefinitionBit::new("ro_z".to_string(), 1, true));
z_circuit.add_operation(
    PragmaRepeatedMeasurement::new("ro_z".to_string(), 1000, None),
);

// X-basis measurement circuit with 1000 shots
let mut x_circuit = Circuit::new();
x_circuit.add_operation(DefinitionBit::new("ro_x".to_string(), 1, true));
// Changing to the X-basis with a Hadamard gate
x_circuit.add_operation(Hadamard::new(0));
x_circuit.add_operation(
    PragmaRepeatedMeasurement::new("ro_x".to_string(), 1000, None),
);

// Preparing the measurement input for one qubit
let mut measurement_input = PauliZProductInput::new(1, false);
// Read out product of Z on site 0 for register ro_z (no basis change)
measurement_input
    .add_pauliz_product("ro_z".to_string(), vec![0])
    .unwrap();
// Read out product of Z on site 0 for register ro_x
// (after basis change effectively a <X> measurement)
measurement_input
    .add_pauliz_product("ro_x".to_string(), vec![0])
    .unwrap();

// Add a result (the expectation value of H) that is a combination
// of the PauliProduct expectation values.
measurement_input
    .add_linear_exp_val(
        "<H>".to_string(), HashMap::from([(0, 0.1), (1, 0.2)]),
    )
    .unwrap();

let measurement = PauliZProduct {
    input: measurement_input,
    circuits: vec![z_circuit.clone(), x_circuit.clone()],
    constant_circuit: Some(init_circuit.clone()),
};

// A quantum program is created from the measurement
// and "angle" is registered as a free input parameter.
// The QuantumProgram now has one free parameter
// that needs to set when executing it.
// The symbolic value angle in the circuits will be replaced
// by that free parameter during execution.
let program = QuantumProgram::PauliZProduct {
    measurement,
    input_parameter_names: vec!["angle".to_string()],
};

// To execute a QuantumProgram a backend needs to be defined.
// Create a backend simulating one qubit.
let backend = Backend::new(1);

// Run QuantumProgram on the backend by setting the parameter value.
let expectation_values = program.run(backend, &[0.785]).unwrap();
```
