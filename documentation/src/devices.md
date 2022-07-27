# Devices

Devices in qoqo can be abstract devices or actual hardware devices.

**Actual hardware devices** that are supported by roqoqo [backends](backends.md) and contain the necessary information for accessing the quantum computing hardware. The devices also encode a connectivity model.

**Abstract devices** contain abstract information for the model of a quantum computer and its parameters.

The abstract devices can be used to determine which gate operations are available on a specific device model A typical example for abstract devices are linear chains of square lattices in which two-qubit gate operations are only available between the neighbouring qubits.

The abstract devices can also encode a noise model. The [noise operations](circuits/noise.md) in qoqo/roqoqo are in general based on a (pseudo) time needed to execute a quantum operation and Lindblad rates for the qubits in the device. Specifically in the noise model each qubit undergoes a continuous Lindblad-type decoherence time evolution of the form

\\[
 \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i \rho \},
\\]

with \\( L_0 = \sigma^{+}\\), \\( L_1 = \sigma^{-}\\) and \\(L_3 = \sigma^{z}\\).

The matrix representation of the decoherence rates of the Lindblad equation can be obtained by calling the function `qubit_decoherence_rates()` on the implemented device.

For further functions implemented for the devices in qoqo/roqoqo please refer to the API documentation of [roqoqo::devices](https://docs.rs/roqoqo/latest/roqoqo/devices/index.html) (Rust core)
 and [qoqo](https://qoqo.readthedocs.io/en/latest/generated/qoqo.html) (python interface).