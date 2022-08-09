// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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

mod measurement_example;
use measurement_example::measurement_main;

mod teleportation_example;
use teleportation_example::teleportation_main;

mod intro_to_roqoqo;
use intro_to_roqoqo::*;

fn main() {
    entangling_circuit_snippet();
    measuring_qubits();
    measuring_observables();
    serialization_quantum_program();
    measurement_main();
    teleportation_main();
}
