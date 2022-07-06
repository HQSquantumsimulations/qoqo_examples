use roqoqo::backends::{EvaluatingBackend, RegisterResult};
use roqoqo::measurements::{PauliZProduct, PauliZProductInput};
use roqoqo::{operations::*, registers::*, Circuit, QuantumProgram};
use roqoqo_quest::Backend;
use std::collections::{HashMap, HashSet};

// Introduction examples for simple circuits and measurements.
// For extended examples on "Fine control over decoherence", usage of "Symbolic parameters",
// or "Testing performance with qoqo_mock" backend,
// please refer to Intro examples in Jupyter notebooks.

/// Example to entangle a circuit snippet.
///
/// Similar to many other toolkits the unitary entangling circuit can be constructed by adding operations to a circuit.
///
pub fn entangling_circuit_snippet() {
    println!(">> Introduction example start.");
    // Create a new modifiable circuit
    let mut circuit = Circuit::new();
    // Prepare qubits 0 and 1 in a superposition state by adding the Hadamard gate
    circuit += Hadamard::new(0);
    circuit += Hadamard::new(1);
    // Establish entanglement between qubits 0 and 1
    circuit += CNOT::new(0, 1);

    // Print
    println!("{:?}", circuit);
    println!("Circuit length: {:?}", circuit.len());
    println!("Operation types: {:?}", circuit.get_operation_types(),);

    // For demonstrative purposes only:
    // Compare obtained operation types to those expected for this example
    let mut types: HashSet<&str> = HashSet::new();
    types.insert("CNOT");
    types.insert("Hadamard");
    assert_eq!(circuit.get_operation_types(), types);

    // Compare the derived circuit length to the expected one for this example
    assert_eq!(circuit.len(), 3);
}

/// Example for measuring qubits.
///
/// Qoqo uses classical registers for the readout. We need to add a classical register definition to the circuit and a measurement statement.
/// The number of projective measurements can be directly set in the circuit.  
/// The simulation and measurement of the circuit is handled by the roqoqo_quest interface (in this example).
///
pub fn measuring_qubits() {
    // Create new modifiable circuit
    let mut circuit = Circuit::new();
    // Define classical bit register for the readout of the measurement
    let register_name: String = "ro".to_string();
    circuit += DefinitionBit::new(register_name.clone(), 2, true);
    // Add operations to the circuit
    circuit += Hadamard::new(0);
    circuit += CNOT::new(0, 1);
    // Add operation to the circuit to perform repeated measurements in a quantum computing simulation.
    circuit += PragmaRepeatedMeasurement::new(register_name, 10, None);
    println!(
        ">> Circuit prepared for a simulated measurement: {:?}",
        circuit,
    );

    let backend = Backend::new(2);
    let result: RegisterResult = backend.run_circuit(&circuit);
    let result_registers: (
        HashMap<String, BitOutputRegister>,
        HashMap<String, FloatOutputRegister>,
        HashMap<String, ComplexOutputRegister>,
    ) = result.unwrap();

    println!(">> Bit output register 'ro' contains the following single projective measurements:");
    for single_projective_measurements in &result_registers.0["ro"] {
        println!("{:?}", single_projective_measurements);
    }
}

/// Example for measuring observables.
///
/// Qoqo includes the direct evaluation of projective measurements to an observable measurement e.g. 3 * < Z0 > + < Z0 Z1 >.
/// The measurement is defined by a set of expectation values of a product of pauli operators and a matrix that combines the expectation values.
///
pub fn measuring_observables() {
    let mut circuit = Circuit::new();
    circuit.add_operation(DefinitionBit::new("ro".to_string(), 2, true));
    circuit.add_operation(Hadamard::new(0));
    circuit.add_operation(CNOT::new(0, 1));
    circuit.add_operation(PragmaRepeatedMeasurement::new("ro".to_string(), 10, None));

    let mut measurement_input = PauliZProductInput::new(2, false);
    // From readout 'ro' measure two pauli products 0: < Z0 > and 1: < Z0 Z1 >
    measurement_input
        .add_pauliz_product("ro".to_string(), vec![0])
        .unwrap();
    measurement_input
        .add_pauliz_product("ro".to_string(), vec![0, 1])
        .unwrap();
    // One expectation value: 3 * pauli_product0 + 1 * pauli_product1
    measurement_input
        .add_linear_exp_val("example".to_string(), HashMap::from([(0, 3.0), (1, 1.0)]))
        .unwrap();
    println!(">> Measurement input defined: {:?}", measurement_input,);

    let measurement = PauliZProduct {
        input: measurement_input,
        circuits: vec![circuit.clone()],
        constant_circuit: None,
    };
    let backend = Backend::new(2);
    let program = QuantumProgram::PauliZProduct {
        measurement,
        input_parameter_names: vec![],
    };

    // Measurement result
    let result = program.run(backend, &[]).unwrap().unwrap()["example"];
    println!(">> Result of Quantum Program: {:?}", result);

    // Validation check
    assert!(result > -4.0 * 10.0);
    assert!(result < 4.0 * 10.0);
}

/// De/Serializing the quantum program
///
/// Same procedure as introduced in the example 1.3 "Measurement observables", but now the measurement, and afterwards the quantum program, are serialized to and de-serialized from json.
/// The measurement result is compared before and after the de/-serialization.
///
pub fn serialization_quantum_program() {
    let mut circuit = Circuit::new();
    circuit.add_operation(DefinitionBit::new("ro".to_string(), 2, true));
    circuit.add_operation(PauliX::new(0));
    circuit.add_operation(CNOT::new(0, 1));
    circuit.add_operation(PragmaRepeatedMeasurement::new("ro".to_string(), 10, None));

    let mut measurement_input = PauliZProductInput::new(2, false);
    // From readout 'ro' measure two pauli products 0: < Z0 > and 1: < Z0 Z1 >
    measurement_input
        .add_pauliz_product("ro".to_string(), vec![0])
        .unwrap();
    measurement_input
        .add_pauliz_product("ro".to_string(), vec![0, 1])
        .unwrap();
    // One expectation value: 3 * pauli_product0 + 1 * pauli_product1
    measurement_input
        .add_linear_exp_val("example".to_string(), HashMap::from([(0, 3.0), (1, 1.0)]))
        .unwrap();
    println!(">> Measurement input defined: {:?}", measurement_input,);

    let measurement = PauliZProduct {
        input: measurement_input,
        circuits: vec![circuit.clone()],
        constant_circuit: None,
    };

    let program = QuantumProgram::PauliZProduct {
        measurement: measurement.clone(),
        input_parameter_names: vec![],
    };

    // First, let's de-/serialize the PauliZProduct measurement and test the outcome
    let measurement_json = serde_json::to_string(&measurement).unwrap();
    let measurement_new: PauliZProduct = serde_json::from_str(&measurement_json).unwrap();
    assert!(measurement_new == measurement);
    println!(">> De/Serialization of PauliZProduct performed successfully.");

    // Next, de-/serialize the Quantum Program
    let program_json = serde_json::to_string(&program).unwrap();
    let program_new: QuantumProgram = serde_json::from_str(&program_json).unwrap();
    assert!(program == program_new);
    println!(">> De/Serialization of QuantumProgram performed successfully.");
    println!(">> Introduction example end.")
}
