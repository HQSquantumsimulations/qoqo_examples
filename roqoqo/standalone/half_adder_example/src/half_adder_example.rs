use core::f64::consts::PI as Pi;
use qoqo_calculator::CalculatorFloat;
use roqoqo::backends::EvaluatingBackend;
use roqoqo::{operations as ops, Circuit};
use roqoqo_quest::Backend;

// In this example we write a quantum algorithm to perform an add operation between two qubits and store the result in two qubits
// that act as output registers.

pub fn half_adder_main() {
    println!(">> Half adder example start.");

    // We define a circuit that generates the main block of the algorithm

    let mut half_adder_main_block = Circuit::new();
    //  Least relevant bit
    half_adder_main_block += ops::CNOT::new(0, 2);
    half_adder_main_block += ops::CNOT::new(1, 2);
    //  Most relevant bit
    //  -Controlled H
    half_adder_main_block += ops::RotateY::new(3, CalculatorFloat::Float(Pi) / 4.0);
    half_adder_main_block += ops::CNOT::new(0, 3);
    half_adder_main_block += ops::RotateY::new(3, -CalculatorFloat::Float(Pi) / 4.0);
    //  -Controlled Z
    half_adder_main_block += ops::Hadamard::new(3);
    half_adder_main_block += ops::CNOT::new(1, 3);
    half_adder_main_block += ops::Hadamard::new(3);
    //  -Controlled H
    half_adder_main_block += ops::RotateY::new(3, CalculatorFloat::Float(Pi) / 4.0);
    half_adder_main_block += ops::CNOT::new(0, 3);
    half_adder_main_block += ops::RotateY::new(3, -CalculatorFloat::Float(Pi) / 4.0);

    // Let's add everything together. We add a complex classical register, called 'DefinitionComplex', to store the state vector
    // of the qubits after our calculation. Other types of registers available in qoqo are `DefinitionBit` for bit registers 
    // used to store actual measurement results of a quantum computer and `DefinitionFloat` to store real valued results.

    // We use the `GetStateVector` pragma operation. A Pragma operation is information for the compiler / qoqo only and will not
    // be sent to the actual quantum computer. We will encounter other important Pragmas later on. The `GetStateVector`
    // pragma operation obtains the state vector of the qubits and stores it in the defined output register ("ro").
    // It accepts an additional measurement circuit that would be added to the circuit before measuring.
    // The state vector can only be obtained in simulations on conventional computers, never from the real device.

    let mut half_adder = Circuit::new();
    half_adder += ops::DefinitionComplex::new("ro".to_string(), 2_usize.pow(4), true);
    //  Initialization
    half_adder += ops::PauliX::new(0);
    half_adder += ops::PauliY::new(1);
    //  Addition of the main block
    half_adder += half_adder_main_block.clone();
    //  Measurement
    half_adder += ops::PragmaGetStateVector::new("ro".to_string(), Some(Circuit::new()));

    println!("Prepared circuit: {:?}", half_adder);

    // We simulate the half adder using `qoqo_quest`. Running the circuit in the backend returns a tuple with entries
    // for all registers of the three different types.

    // We print the complex coefficients (amplitudes) of the quantum state. The input qubits are the two bits on the right side
    // (as they are qubits 0 and 1) while the output is stored in the most relevant bits (the two right bits).
    // As expected, the state `|1011>` is populated while all other states are empty.

    let backend = Backend::new(4);
    let result_of_run = backend.run_circuit(&half_adder);
    let (_result_bit_registers, _result_float_registers, result_complex_registers) =
        result_of_run.unwrap();

    println!(
        "Result complex registers :{:?}",
        result_complex_registers["ro"]
    );

    // Simulating an experiment

    // How would the result that we would get from a real quantum computer look?

    // We define a number of measurements, i.e., repetitions and measurements of the circuit and add a bit register to store
    // the measured values for each run.
    // We add `MeasureQubit` operations for the two output qubits.
    // To make this more interesting we initialize the input qubits in a superposition of all possible states using Hadamard gates `H`.

    let number_of_measurements: usize = 1000;
    half_adder = Circuit::new();
    half_adder += ops::DefinitionBit::new("ro".to_string(), 2, true);
    //  Input
    half_adder += ops::Hadamard::new(0);
    half_adder += ops::Hadamard::new(1);
    //  Main
    half_adder += half_adder_main_block;
    //  Measurement
    half_adder += ops::MeasureQubit::new(2, "ro".to_string(), 0);
    half_adder += ops::MeasureQubit::new(3, "ro".to_string(), 1);
    half_adder += ops::PragmaSetNumberOfMeasurements::new(number_of_measurements, "ro".to_string());

    let backend = Backend::new(4);
    let result_of_run = backend.run_circuit(&half_adder);
    let (result_bit_registers, _result_float_registers, _result_complex_registers) =
        result_of_run.unwrap();

    println!("Result bit registers :{:?}", result_bit_registers["ro"]);

    println!(">> Half adder example end.");
}
