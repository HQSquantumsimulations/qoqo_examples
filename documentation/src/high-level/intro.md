# High level interface: quantum programs

A QuantumProgram in qoqo/roqoqo is an object that contains Circuits, Measurements and can run on a Backend. Qoqo supports symbolic parameters inside the Circuits, for example the rotation angles \\( \theta \\) in a variational algorithm. However,  a Circuit with symbolic parameters can not be simulated or executed on real hardware. The symbolic parameters need to be replaced with real floating point numbers first. A QuantumProgram contains a list of the free parameters (`input_parameter_names`) and can automatically replace the parameters with its `run` methods and return the result. Thus, the QuantumProgram could be interpreted as an equivalent to a multi-parameter function in classical computing that can be called with a set of parameters and returns a result.

The QuantumProgram can be used as a convenient high level interface since it can be serialized and transmitted to run on real quantum computer hardware.

A sample code snippet using a QuantumProgram is given here. To see a full executable example, please refer to the individual subsections.

In python:
```python
program = QuantumProgram(
    measurement=measurement,
    input_parameter_names=["angle1", "angle2"],
)
result = program.run(backend, [0.785, 0.785])
```

In Rust:
```Rust
let program = QuantumProgram::PauliZProduct {
    measurement: measurement,
    input_parameter_names: vec!["angle1".to_string(), "angle2".to_string()],
};
let result = program.run(backend.clone(), &[0.785, 0.785]).unwrap()
    .unwrap()["example"];
```

## Measurements

For many applications the measurement results of several circuits need to be combined to extract the required information. The combination of the results of each quantum circuit and the classical post-processing of the measurement are served by a QuantumProgram that contains one of the possible qoqo Measurements. The implemented Measurements in qoqo/roqoqo are:
* [PauliZProduct](pauliz.md),
* [ClassicalRegister](classical.md),
* [CheatedPauliZProduct](pauliz_cheated.md),
* [Cheated](cheated.md).

The `PauliZProduct` measurement can be executed on a simulator and on real quantum computer hardware. It returns the expectation values measured in the `z`-basis. For further details please refer to section  [PauliZProduct](pauliz.md). The `ClassicalRegister` can be used to return the classical registers written during circuit execution. The cheated measurements can only run on a simulator, as suggested by the name.

Typically, a qoqo Measurement combines one `constant_circuit` that is always executed first and a list of `circuits` that are executed after the constant circuit. All measurement types, except `ClassicalRegister`, also contain a `measurement_input` that encodes the post-processing of the obtained results.

A sample code snippet using a 'PauliZProduct' measurement is given here. To see a full executable example, please refer to the individual subsections.

In python:
```python
measurement = PauliZProduct(
   constant_circuit=init_circuit,
   circuits=[z_circuit, x_circuit],
   input=measurement_input,
)
```

In Rust:
```Rust
let measurement = PauliZProduct {
    constant_circuit: init_circuit.clone(),
    circuits: vec![z_circuit.clone(), x_circuit.clone()],
    input: measurement_input,
};
```
