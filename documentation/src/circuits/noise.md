# Noise Operations

Qoqo/roqoqo enables the user to construct finely controlled noise models. Noise acting on the quantum computer is modeled as a single noise operation affecting all gates applied on the given qubit on a quantum computer.

The noise operations can be directly added to a quantum circuit and can be simulated by compatible backends. Since noise _cannot_ be actively controlled on a quantum computer normally, the noise operations are defined as [Pragma](pragma.md) operations in qoqo/roqoqo. The strength of the noise is determined by defining a `gate_time` and a `rate`. The noise pragma operation affects the system as a Lindblad type noise acting on the system with the rate `rate` for the time `gate_time`.

_Note_: as long as gate times and rates are scaled inversely any kind of units can be used. However, we recommend using nanoseconds and inverse nanoseconds as units for gate times and decoherence rates.

## Example

For example to add dephasing noise to qubit 0, damping noise to qubit 1 and depolarising noise to qubit 2 acting on the system after a `CNOT` gate has been applied we implement

In python:

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

In Rust:

```rust
use roqoqo::Circuit;
use roqoqo::operations;

let mut circuit = Circuit::new();
circuit += operations::CNOT::new(0,1);
// Adding dephasing noise acting on gate 0 with gate_time 1.0 and rate 1e-3
circuit += operations::PragmaDephasing::new(0, 1.0, 1e-3);
circuit += operations::PragmaDamping::new(1, 1.0, 2e-3);
circuit += operations::PragmaDepolarising::new(3, 1.0, 5e-3);
```

## Common functions

Common to all pragma noise operations are the following functions:

* `superoperator()` can be called to obtain the superoperator representation of the noise operation in form of a (4x4)-matrix.
* `probability()` returns the probability of noise gate affecting the qubit.
* `powercf(power)` returns the gate to the power of `power`. The input parameter `power` needs to be of type CalculatorFloat as provided by the software module [qoqo_calculator](https://github.com/HQSquantumsimulations/qoqo_calculator).

## Noise operations

The single noise operations shown in the example above are:

* **PragmaDamping** that applies a pure damping error corresponding to _zero_ temperature environments.
* **PragmaDepolarising** which applies a depolarising error corresponding to _infinite_ temperature environments.
* **PragmaDephasing** representing a pure dephasing error.

For a stochastically unravelled combination of dephasing and depolarising the user can choose to use the **PragmaRandomNoise**. The error rate of the depolaristion (`depolarising_rate`) and the error rate of the dephasing (`dephasing_rate`) are provided as input parameters for this random noise operation.

Further information on advanced noise operations available in qoqo/roqoqo is provided in the subsections below.

### PragmaGeneralNoise

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

Applying a pragma noise operation with a given `gate_time` corresponds to applying the full time-evolution under the Lindblad equation.

### PragmaOverrotation

This noise operation applies a statistical overrotation to the next rotation gate in the circuit, which matches the name given in the `gate` parameter of `PragmaOverrotation` and the involved qubits provided in `qubits`. The applied overrotation corresponds to adding a random number to the rotation angle.
The random number is drawn from a normal distribution with mean `0` and standard deviation given by the input parameter `variance` that is multiplied by the `amplitude` parameter.

### PragmaBoostNoise

This operation boosts noise and overrotations in the circuit. The input parameter `noise_coefficient` defines the coefficient by which the noise is boosted, i.e. the number by which the `gate_time` is multiplied.

### PragmaSleep

This operation makes the quantum computer hardware, that provides this functionality, wait a given amount of time (`sleep_time`). Waiting for a given time can increase the effect of continuous noise on the result of running a quantum circuit. This is sometimes used in noise-extrapolation error mitigation techniques.
