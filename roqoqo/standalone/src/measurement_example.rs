// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use roqoqo::backends::EvaluatingBackend;
use roqoqo::{operations, Circuit};

pub fn measurement_main() {
    println!(">> Measurement example start.");

    // #Measuring a single qubit
    let mut state_init = Circuit::new();
    state_init += operations::Hadamard::new(0); //prepare |+> state

    // write state before measuring to readout register 'psi_in'
    let mut read_input = Circuit::new();
    read_input += operations::DefinitionComplex::new("psi_in".to_string(), 2, true);
    read_input += operations::PragmaGetStateVector::new("psi_in".to_string(), Some(Circuit::new()));

    // measure qubit in Z basis and write result to classical register 'M1'
    let mut meas_circ = Circuit::new();
    meas_circ += operations::DefinitionBit::new("M1".to_string(), 1, true);
    meas_circ += operations::MeasureQubit::new(0, "M1".to_string(), 0);

    // write state after measuring to readout register 'psi_out'
    let mut read_output = Circuit::new();
    read_output += operations::DefinitionComplex::new("psi_out".to_string(), 2, true);
    read_output +=
        operations::PragmaGetStateVector::new("psi_out".to_string(), Some(Circuit::new()));

    // put each step of the circuit together
    let mut circuit = state_init + read_input + meas_circ + read_output;

    // run the circuit and collect output
    let mut backend = roqoqo_quest::Backend::new(1);
    let mut result_of_run = backend.run_circuit(&circuit);
    let (result_bit_registers, _result_float_registers, result_complex_registers) =
        result_of_run.unwrap();

    println!("> Measurement in Z basis into classical register:");
    println!("Input state:{:?}", result_complex_registers["psi_in"]);
    println!("Measurement result:{:?}", result_bit_registers["M1"]);
    println!(
        "State after measurement:{:?} \n",
        result_complex_registers["psi_out"]
    );

    // #Measuring a single qubit in the X basis
    let number_of_qubits: usize = 3;

    state_init = Circuit::new();
    state_init += operations::PauliX::new(1);
    state_init += operations::Hadamard::new(0);
    state_init += operations::CNOT::new(0, 1);
    state_init += operations::CNOT::new(0, 2);
    state_init += operations::SGate::new(0);

    // write state before measuring to readout register 'psi_in'
    read_input = Circuit::new();
    read_input += operations::DefinitionComplex::new(
        "psi_in".to_string(),
        2usize.pow(number_of_qubits as u32),
        true,
    );
    read_input += operations::PragmaGetStateVector::new("psi_in".to_string(), Some(Circuit::new()));

    // measure qubits in Z basis and write result to classical register 'M1M2M3'
    meas_circ = Circuit::new();
    meas_circ += operations::DefinitionBit::new("M1M2M3".to_string(), 3, true);
    meas_circ += operations::MeasureQubit::new(0, "M1M2M3".to_string(), 0);
    meas_circ += operations::MeasureQubit::new(1, "M1M2M3".to_string(), 1);
    meas_circ += operations::MeasureQubit::new(2, "M1M2M3".to_string(), 2);

    // write state after measuring to readout register 'psi_out'
    read_output = Circuit::new();
    read_output += operations::DefinitionComplex::new(
        "psi_out".to_string(),
        2usize.pow(number_of_qubits as u32),
        true,
    );
    read_output +=
        operations::PragmaGetStateVector::new("psi_out".to_string(), Some(Circuit::new()));

    circuit = state_init + read_input + meas_circ + read_output;

    // run the circuit and collect output
    backend = roqoqo_quest::Backend::new(number_of_qubits);
    result_of_run = backend.run_circuit(&circuit);
    let (result_bit_registers, _result_float_registers, result_complex_registers) =
        result_of_run.unwrap_or_default();

    println!("> Measuring single qubit in X-basis:");
    println!("Input state:{:?}", result_complex_registers["psi_in"]);
    println!("Measurement result:{:?}", result_bit_registers["M1M2M3"]);
    println!(
        "State after measurement:{:?} \n",
        result_complex_registers["psi_out"]
    );

    // Measuring one qubit from a multi-qubit register

    let number_of_qubits: usize = 3;

    state_init = Circuit::new();
    state_init += operations::PauliX::new(1);
    state_init += operations::Hadamard::new(0);
    state_init += operations::CNOT::new(0, 1);
    state_init += operations::CNOT::new(0, 2);
    state_init += operations::SGate::new(0);

    // write state before measuring to readout register 'psi_in'
    read_input = Circuit::new();
    read_input += operations::DefinitionComplex::new(
        "psi_in".to_string(),
        2usize.pow(number_of_qubits as u32),
        true,
    );
    read_input += operations::PragmaGetStateVector::new("psi_in".to_string(), Some(Circuit::new()));

    // measure qubit in Z basis and write result to classical register 'M1'
    meas_circ = Circuit::new();
    meas_circ += operations::DefinitionBit::new("M1".to_string(), 1, true);
    meas_circ += operations::MeasureQubit::new(0, "M1".to_string(), 0);

    // write state after measuring to readout register 'psi_out'
    read_output = Circuit::new();
    read_output += operations::DefinitionComplex::new(
        "psi_out".to_string(),
        2usize.pow(number_of_qubits as u32),
        true,
    );
    read_output +=
        operations::PragmaGetStateVector::new("psi_out".to_string(), Some(Circuit::new()));

    circuit = state_init + read_input + meas_circ + read_output;

    // run the circuit and collect output
    backend = roqoqo_quest::Backend::new(number_of_qubits);
    result_of_run = backend.run_circuit(&circuit);
    let (result_bit_registers, _result_float_registers, result_complex_registers) =
        result_of_run.unwrap_or_default();

    println!("> Measurement of one qubit from a multi-qubit register:");
    println!("Input state:{:?}", result_complex_registers["psi_in"]);
    println!("Measurement result:{:?}", result_bit_registers["M1"]);
    println!(
        "State after measurement:{:?}",
        result_complex_registers["psi_out"]
    );

    println!(">> Measurement example end.");
}
