# Noise Operations

Qoqo/roqoqo enables the user to construct finely controlled noise models. Noise acting on the quantum commputer is modeled as a single noise operation affecting all gates applied on the given qubit on a quantum computer.

The noise operations can be directly added to a quantum circuit and can be simulated by compatible backends. Since noise operations can _not_ run on all universal quantum computers, they are defined as [Pragma](pragma.md) operations in qoqo/roqoqo. The strength of the noise is determined by defining a `gate_time` and a `rate`. The noise pragma operation affects the system like Lindblad type noise acting on the system with the rate `rate` for the time `gate_time`.

For example to add dephasing noise to qubit 0, damping noise to qubit 1 and depolarising noise to qubit 2 acting on the system after a `CNOT` gate has been applied we implement

in python: 

```python

from qoqo import Circuit
from qoqo import operations

circuit = Circuit()
circuit += operations.CNOT(0,1)
#Adding dephasing noise acting on gate 0 with gate_time 1.0 and rate 1e-3
circuit += operations.PragmaDephasing(qubit=0, gate_time=1.0, rate=1e-3)
circuit += operations.PragmaDamping(1, 1.0, 2e-3)
circuit += operations.PragmaDepolarising(3, 1.0, 5e-3)

```

in Rust:

```rust

:dep roqoqo = "1.0.0-alpha.4"
extern crate roqoqo;

use roqoqo::Circuit;
use roqoqo::operations;

let mut circuit = Circuit::new();
circuit += operations::CNOT::new(0,1);
// Adding dephasing noise acting on gate 0 with gate_time 1.0 and rate 1e-3
circuit += operations::PragmaDephasing::new(0, 1.0, 1e-3);
circuit += operations::PragmaDamping::new(1, 1.0, 2e-3);
circuit += operations::PragmaDepolarising::new(3, 1.0, 5e-3);

```


## PragmaGeneralNoise

The most general noise can be modelled in qoqo by the PragmaGeneralNoise operation. This Pragma operation applies a noise term according to the given rates. The rates are represented by a 3x3 matrix:

\\[
 M = \begin{pmatrix}
 a & b & c \\\\
 d & e & f \\\\
 g & h & j \\\\
 \end{pmatrix}
\\]

where the coefficients correspond to the following summands expanded from the first term of the non-coherent part of the Lindblad equation:
\\[
 \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i, \rho \}
\\]

with \\( L_0 = \sigma^{+} \\), \\( L_1 = \sigma^{-} \\) and \\( L_3 = \sigma_{z} \\).

<!-- yielding result{sigma_z, sigma_minus} = sigma_z (x) sigma_minus.T - 1/2 * (sigma_minus.T * sigma_z) (x) 1 - 1/2 * 1 (x) (sigma_minus.T * sigma_z).T -->

Applying the Pragma with a given `gate_time` corresponds to applying the full time-evolution under the Lindblad equation for `gate_time` time.
