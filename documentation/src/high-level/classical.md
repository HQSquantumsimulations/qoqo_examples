# ClassicalRegister Measurement

`ClassicalRegister` measurement can be used in a QuantumProgram to return unprocessed measurements. There are many use cases where end users want to receive the full measurement output without post-processing. For example when working with external tools that expect full measurement records or when implementing custom post-processing. For these use cases the `ClassicalRegister` measurement can be used to create three dictionaries: one for registers with bit values, one for registers with float values and one for registers with complex values. Compared to other measurement types in qoqo/roqoqo this measurement does _not_ need a separate `measurement_input` since no post-processing takes place.

To distinguish between a command returning expectation values and a program returning register the command `run_registers()` is used in the fulllowing example.

```python
from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import ClassicalRegister
from qoqo import QuantumProgram
from qoqo_quest import SimulatorBackend

# initialize |psi>
init_circuit = Circuit()
init_circuit += ops.Hadamard(0)

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

measurement = ClassicalRegister(
    constant_circuit=init_circuit,
    circuits=[z_circuit, x_circuit],
)

# A quantum program is created from the measurement
program = QuantumProgram(measurement=measurement, input_parameter_names=[])

# Create a backend simulating one qubit.
backend = SimulatorBackend(1)

(bit_registers, float_registers, complex_registers) = program.run_registers(backend, [])
```

The same example in Rust:

```Rust
// TO DO
```
