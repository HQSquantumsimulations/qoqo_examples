# Backends

To obtain results based on a Circuit, Measurement or a QuantumProgram defined in qoqo/roqoqo, the calculation needs to be performed on real quantum computing hardware or on a simulator. For each individual hardware or simulator a Backend can be created that implements qoqo's 'EvaluatingBackend' functionality and executes QuantumPrograms. A backend in qoqo/roqoqo is meant to represent a quantum computing hardware, a quantum computing simulator or other software packages. 

The following evaluating backends are implemented in qoqo/roqoqo and supported by HQS Quantum Simulations GmbH.
* [qoqo_aqt](https://github.com/HQSquantumsimulations/qoqo_aqt)
* [qoqo_mock](https://github.com/HQSquantumsimulations/qoqo_aqt)
* [qoqo_qryd](https://github.com/HQSquantumsimulations/qoqo_qryd)
* [qoqo_quest](https://github.com/HQSquantumsimulations/qoqo-quest)

The implementation of individual backend is provided not in qoqo itself, but in other packages (as listed above) to make the software modular. The above backends are so-called _evaluating_ backends, that means that one can execute a circuit or a quantum program and process the results returned by the actual backend.

An EvaluatingBackend provides the functionality to run:

* A _single_ circuit. The backend will execute just the circuit and return the measurement results of all registers in a tuple (bit-registers, float-registers, complex-registers). More details on registers can be found in section [readout](circuits/readout.md). All the postprocessing of the bare results needs to be done manually.
* A measurement. _All_ circuits collected in the measurement are executed and the post-processed expectation values are returned.
* A quantum program. A qoqo QuantumProgram also handles replacement of symbolic variables. It provides its own `run()` method and calls the given backend internally.

That is, all evaluating backends provide the same functions: `run_circuit()`, `run_measurement()` or `run_measurement_registers()`, and `run()`.

Other backends implemented in qoqo/roqoqo are
* [qoqo_qasm](https://github.com/HQSquantumsimulations/qoqo_qasm)

Non-evaluating backend, like `qoqo_qasm`, cannot be used to execute a quantum circuit. Instead of that such backends are typically used to translate qoqo/roqoqo circuits into a different nomenclature used by other quantum toolkits.

## Example

In this subsection an example in Rust and in python is provided.  A [QuantumProgram](hight-level/program.md) is created to be executed on the [qoqo_quest](https://github.com/HQSquantumsimulations/qoqo-quest) simulator backend. Here, all three options supported by an `EvaluatingBackend` are presented:
* to run a single circuit,
* to run a measurement, and
* to run a quantum program.

In Rust:

```rust
:dep roqoqo = "1.0.0-alpha.4"
extern crate roqoqo;

// Code to follow
```

In python:

```python
from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import PauliZProduct, PauliZProductInput
from qoqo import QuantumProgram
from qoqo_quest import SimulatorBackend
# initialize |psi>
init_circuit = Circuit()
# Apply a RotateY gate with a symbolic angle
# To execute the circuit this symbolic parameter needs to be replaced
# with a real number with the help of a QuantumProgram
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

# Add a result (the expectation value of H) that is a combination of the PauliProduct
# expectation values
measurement_input.add_linear_exp_val("<H>", {x_basis_index: 0.1, z_basis_index: 0.2})

measurement = PauliZProduct(
   constant_circuit=init_circuit,
   circuits=[z_circuit, x_circuit],
   input=measurement_input,
)

# Here we show three alternative options that can be ran:
# a single circuit, a measurement, and a quantum program.

# Create a backend simulating one qubit
backend = SimulatorBackend(1)

# a) Run a single circuit
(bit_registers, float_registers, complex_registers) = backend.run_circuit(z_circuit)

# b) To run a measurement we need to replace the free parameter by hand
executable_measurement = measurement.substitute_parameters({"angle": 0.2})
expecation_values = backend.run_measurement(executable_measurement)
print(expecation_values)

# c) Run a quantum program
# The QuantumProgram now has one free parameter that needs to bet set when executing it.
# The symbolic value "angle" in the circuits will be replaced by that free parameter
# during execution.
program = QuantumProgram(measurement=measurement, input_parameter_names=["angle"])
# Run the program with  0.1 substituting `angle`
expecation_values = program.run(backend, [0.1])
```

