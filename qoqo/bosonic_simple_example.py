# Copyright © 2023 HQS Quantum Simulations GmbH. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
# in compliance with the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed under the License
# is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
# or implied. See the License for the specific language governing permissions and limitations under
# the License.
"""Creating a circuit with bosonic operations and executing it on the appropriate backend."""
from qoqo_strawberry_fields import StrawberryFieldsBackend
from qoqo import operations, Circuit
import numpy as np

# Defining the inputs for the bosonic gates
squeezing = 0.1
squeezing_angle = np.pi / 4
beamsplitter_50_50 = np.pi / 4
beamsplitter_50_50_angle = np.pi / 2

# Creating the circuit defined in Figure 20 of https://arxiv.org/abs/1805.02645 as an example.
circuit = Circuit()
circuit += operations.DefinitionFloat("ro", 4, True)
circuit += operations.Squeezing(0, squeezing, squeezing_angle)
circuit += operations.Squeezing(1, squeezing, squeezing_angle)
circuit += operations.Squeezing(2, squeezing, squeezing_angle)
circuit += operations.Squeezing(3, squeezing, squeezing_angle)
circuit += operations.BeamSplitter(0, 1, beamsplitter_50_50, beamsplitter_50_50_angle)
circuit += operations.BeamSplitter(2, 3, beamsplitter_50_50, beamsplitter_50_50_angle)
circuit += operations.BeamSplitter(1, 2, beamsplitter_50_50, beamsplitter_50_50_angle)
circuit += operations.BeamSplitter(0, 1, beamsplitter_50_50, beamsplitter_50_50_angle)
circuit += operations.BeamSplitter(2, 3, beamsplitter_50_50, beamsplitter_50_50_angle)
circuit += operations.BeamSplitter(1, 2, beamsplitter_50_50, beamsplitter_50_50_angle)
circuit += operations.PhotonDetection(0, "ro", 0)
circuit += operations.PhotonDetection(1, "ro", 1)
circuit += operations.PhotonDetection(2, "ro", 2)
circuit += operations.PhotonDetection(3, "ro", 3)
circuit += operations.PragmaSetNumberOfMeasurements(100, "ro")

# Running the circuit on the StrawberryFieldsBackend, which converts from 
# HQS's qoqo to Xanadu's Strawberry Fields
backend = StrawberryFieldsBackend(number_modes=4, device="gaussian")
result = backend.run_circuit(circuit)
print(result)

# To visualise the results better, we can convert them into averages:
(number_shots, number_modes) = result[0]["ro"].shape
converted_results = np.zeros(number_modes, dtype=float)
for line in result[0]["ro"]:
    for i, mode in np.ndenumerate(line):
        if mode is True:
            converted_results[i] += 1

converted_results /= number_shots
print(converted_results)
