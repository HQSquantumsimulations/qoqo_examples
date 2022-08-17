// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use roqoqo::backends::EvaluatingBackend;
use roqoqo::{operations as ops, Circuit};
use roqoqo_quest::Backend;

fn main() {
    println!(">> Deutsch-Josza example start.");

    // We have given a function `f: {0, 1}^n->{0, 1}` from input bitstrings with length `n`, *e.g.*, `00110`, to a single bit output.
    // The given function is either balanced (50% of inputs yield 0, 50% yield 1 as output) or the function is constant (always 1 or always 0).
    // The task for our algorithm is to decide if the given function is constant or balanced.

    // On a conventional computer we can query the function using the different input bitstrings.
    // As soon as we have seen two different outputs we know that the function is balanced.
    // However, if we have measured k-times the same value we only know that the function is constant with probability `P_k=1-1/2^(k-1)`.
    // If we want to be 100% certain we need to query 50% of all `2^n` bitstrings, *i.e.*, `2^(n-1)` queries.

    // The Deutsch-Josza algorithm can perform the same task with exactly 1 query using `n+1` qubits.

    //  ORACLES
    // The function that we want to analyze is encoded into the oracle. This sub-circuit takes two takes two input registers `|x>` and `|y>`
    // and returns '|x>|y oplus f(x)>' where 'y oplus f(x)' is understood as addition modulo 2. For example, 0 oplus 1= 1 and 1 oplus 1 = 0.
    // In our example we will use a function defined on 2 (q)bits: f:{0,1}^2 -> {0,1}.

    // We can define a balanced oracle as a sub-circuit composed by a CNOT gate with control qubit 0 and target qubit 2 (the output qubit)
    // followed by another CNOT gate with control qubit 1 and target qubit 2. A constant oracle, on the other hand,
    // can be composed by a single NOT operation on the output qubit, leaving the two input qubits unaltered.

    //  IMPLEMENTATION
    // Required is the first step of the circuit: flip the 3rd qubit from `0 ` to `1` and apply a Hadamard gate to all 3 qubits.
    // We use the qoqo toolkit to represent quantum circuits.

    // A `Circuit` is the main class to represent quantum circuits. The `qoqo.operations` module contains one- and two-qubit operations
    // such as Hadamard, PauliX or CNOT. For the initialization circuit we require two different gates, the `PauliX` and the `Hadamard` operation.

    // The following code writes a circuit that applies the required operations.

    fn deutsch_josza_circuit(number_qubits: usize, oracle: Circuit) -> Circuit {
        let mut circuit = Circuit::new();
        circuit += ops::PauliX::new(number_qubits);
        for q in 0..number_qubits {
            circuit += ops::Hadamard::new(q);
        }
        circuit += ops::Hadamard::new(number_qubits);
        circuit += oracle;
        for q in 0..number_qubits {
            circuit += ops::Hadamard::new(q);
        }
        circuit
    }

    // The following code implements a circuit for the balanced oracle and one for the constant oracle.
    // For the balanced oracle a CNOT operation which takes a control and target qubit is required.

    fn balanced_oracle(number_qubits: usize) -> Circuit {
        let mut oracle = Circuit::new();
        for c in 0..number_qubits {
            oracle += ops::CNOT::new(c, number_qubits);
        }
        oracle
    }

    fn constant_oracle(number_qubits: usize) -> Circuit {
        let mut oracle = Circuit::new();
        oracle += ops::PauliX::new(number_qubits);
        oracle
    }

    // To finalize the circuit, we define a measurement circuit that:
    //  - applies a Hadamard gate to the first two qubits
    //  - defines a bit register
    //  - applies a MeasureQubit operation to the first two qubits

    // For this step we require two additional qoqo operations:
    //  - DefinitionBit : create a classical bit register to store measured bit values
    //  - MeasureQubit : measure a qubit and store the input in the classical bit register

    let number_qubits = 2;

    let mut balanced = Circuit::new();
    balanced += deutsch_josza_circuit(number_qubits, balanced_oracle(number_qubits));
    balanced += ops::DefinitionBit::new("ro".to_string(), number_qubits, true);
    for q in 0..number_qubits {
        balanced += ops::MeasureQubit::new(q, "ro".to_string(), q);
    }
    println!("{:?}", balanced);

    let mut constant = Circuit::new();
    constant += deutsch_josza_circuit(number_qubits, constant_oracle(number_qubits));
    constant += ops::DefinitionBit::new("ro".to_string(), number_qubits, true);
    for q in 0..number_qubits {
        constant += ops::MeasureQubit::new(q, "ro".to_string(), q);
    }
    println!("{:?}", constant);

    //  SIMULATION

    // The algorithm is then tested on a (simulated) quantum computer. We use the `qoqo_quest` library to run the simulation and
    // from this library we need the `Backend`.
    // A circuit can be simulated on the backend using `run_circuit`. The method returns a tuple.
    // The first entry of the tuple is a dictionary of BitRegisters. The result of `run_circuit` is saved into `res`,
    // we then access our registry via `res[0]['ro']`.

    // The following code runs the simulation for the balanced and the constant oracle.

    fn checking_constant(res: &[bool]) -> bool {
        return res.iter().all(|&el| !el);
    }

    let backend = Backend::new(number_qubits + 1);
    let result_of_run = backend.run_circuit(&balanced);
    let (result_bit_registers, _result_float_registers, _result_complex_registers) =
        result_of_run.unwrap();
    println!("Running balanced: \n{:?}", result_bit_registers);
    println!(
        "Is constant? {}",
        checking_constant(&result_bit_registers["ro"][0])
    );

    let backend = Backend::new(number_qubits + 1);
    let result_of_run = backend.run_circuit(&constant);
    let (result_bit_registers, _result_float_registers, _result_complex_registers) =
        result_of_run.unwrap();
    println!("Running constant: \n{:?}", result_bit_registers);
    println!(
        "Is constant? {}",
        checking_constant(&result_bit_registers["ro"][0])
    );

    //  RESULTS INTERPRETRATION
    // The Deutsch-Josza algorithm uses destructive interference to suppress all amplitues but the |00> state if the function is constant.
    // This means that in the constant case you will alway measure `[False, False]` as a result of the circuit.

    // For the balanced oracle the interference suppresses the amplitue of the $|00>$ state.
    // This means that you can measure all bitstrings but `[False, False]`.

    // In conclusion, measuring '[False, False]' means that the function is constant whereas all other results mean that the function is balanced.
}
