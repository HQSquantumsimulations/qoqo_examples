# Devices

When working with quantum circuits it is often necessary to know the topology of a target quantum device. Device properties can also be used by backends, for example to accurately simulate a given quantum device.
At the moment, qoqo/roqoqo does not implement devices but defines an interface for obtaining the device topology and the noise properties. The interface is defined by roqoqo's `Device` trait.

Devices based on the roqoqo `Device` trait can be abstract devices or backend devices.

**Abstract devices** contain abstract information about the device topology, the available gates and the noise model.

**Backend devices** are devices that are implemented by a roqoqo [backend](backends.md). They can specify additional information for accessing the device on the backend and can contain additional information. The devices also contain all the information of the abstract devices.

A typical example for abstract devices are linear chains of square lattices in which two-qubit gate operations are only available between neighboring qubits.

The noise model of the `Device` trait is based on the continuous-time Lindblad equation:
\\[
 \frac{d}{dt}\rho = \sum_{i,j=0}^{2} M_{i,j} L_{i} \rho L_{j}^{\dagger} - \frac{1}{2} \{ L_{j}^{\dagger} L_i \rho \},
\\],

with \\( L_0 = \sigma^{+}\\), \\( L_1 = \sigma^{-}\\) and \\(L_3 = \sigma^{z}\\).

It is defined by the decoherence rates `M` and the (pseudo-)time needed to execute a quantum operation.

The matrix representation of the decoherence rates of the Lindblad equation can be obtained by calling the method `qubit_decoherence_rates()` of a device.

The time required for a gate operation can be obtained from the methods `single_qubit_gate_time()`, `two_qubit_gate_time()`, and `multi_qubit_gate_time()` for a specific type of gate (defined by its name) and the qubits the gate should act on. 
The gate time method can also be used to query the topology and available gate operations on a device. If a specific type of gate operation is not available on the given qubits, the gate time method will return `None`.

For further details of the `Device` trait please refer to the API documentation of [roqoqo::devices](https://docs.rs/roqoqo/latest/roqoqo/devices/index.html) (Rust core)
