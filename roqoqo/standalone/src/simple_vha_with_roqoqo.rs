// Copyright © 2021 HQS Quantum Simulations GmbH.

extern crate eigenvalues;
extern crate nalgebra;
extern crate ndarray;
extern crate num_complex;
extern crate roqoqo;
extern crate roqoqo_quest;

use roqoqo::{operations::*, Circuit, QuantumProgram};
// for measuring observables
use roqoqo::measurements::{PauliZProduct, PauliZProductInput};
// simulation and measurement of the circuit is handled by the QuEST interface
use qoqo_calculator::CalculatorFloat;
use roqoqo_quest::Backend;
use std::collections::HashMap;
use std::vec::*;

use na::ComplexField;
use nalgebra as na;
use ndarray::{array, Array1, Array2};
use num_complex::Complex64;

use cobyla::{fmin_cobyla, CstrFn};

/// Example how to run a simple variational algorithm with roqoqo
///
/// This example shows how to run a very simple variational algorithm with the rust core of qoqo - roqoqo.
/// The variational algorithm will be a very simple example of a Variational Hamiltonian Ansatz (VHA),
/// the code does not aim to get the best result possible but to show a very simple example.
///
/// For detailed discussions of variational algorithms, VHA and different variants of these algorithms
/// see the literature (e.g. http://arxiv.org/abs/1304.3061, http://arxiv.org/abs/1509.04279).
///
pub fn run_simple_vha() {
    // PART I: Unitary time evolution
    //
    // We construct circuits that apply time evolution under the even and odd hopping Hamiltonians
    // and under the magnetic field using variables t (hopping_parameter) and B (magnetic_field).
    // For each iteration of the evolution we get free symbolic parameters theta_even_i, theta_odd_i and theta_z_i.

    // Helper function to return a circuit for the time-like evolution under even-to-odd hopping.
    //
    // # Arguments
    // * thetasymb: symbolic parameter 'theta' of the even-to-odd time-like evolution.
    // * number_qubits: number of qubits in the system.
    // * hopping_parameter: parameter 't' for the hopping term of the Hamiltonian.
    fn create_even_hopping_circuit(
        thetasymb: CalculatorFloat,
        number_qubits: usize,
        hopping_parameter: f64,
    ) -> Circuit {
        let mut circuit = Circuit::new();

        // Representation of the \sigma^x\sigma^x interaction between two spins by the CNOT and Rotation gates
        for k in (0..number_qubits - 1).step_by(2) {
            // Basis rotation into z-basis
            circuit.add_operation(Hadamard::new(k));
            circuit.add_operation(Hadamard::new(k + 1));
            // Interaction
            circuit.add_operation(CNOT::new(k + 1, k));
            circuit.add_operation(RotateZ::new(
                k,
                (thetasymb.clone() * hopping_parameter).to_string().into(),
            ));
            circuit.add_operation(CNOT::new(k + 1, k));
            // Basis rotation back into x-basis
            circuit.add_operation(Hadamard::new(k));
            circuit.add_operation(Hadamard::new(k + 1))
        }

        circuit
    }

    // Helper function to return a circuit for the time-like evolution under odd-to-even hopping.
    //
    // # Arguments
    // * thetasymb: symbolic parameter 'theta' of the odd-to-even time-like evolution.
    // * number_qubits: number of qubits in the system.
    // * hopping_parameter: parameter 't' for the hopping term of the Hamiltonian.
    fn create_odd_hopping_circuit(
        thetasymb: CalculatorFloat,
        number_qubits: usize,
        hopping_parameter: f64,
    ) -> Circuit {
        let mut circuit = Circuit::new();

        // Representation of the \sigma^x\sigma^x interaction between two spins by the CNOT and Rotation gates
        for k in (1..number_qubits - 1).step_by(2) {
            // Basis rotation into z-basis
            circuit.add_operation(Hadamard::new(k));
            circuit.add_operation(Hadamard::new(k + 1));
            // Interaction
            circuit.add_operation(CNOT::new(k + 1, k));
            circuit.add_operation(RotateZ::new(
                k,
                (thetasymb.clone() * hopping_parameter).to_string().into(),
            ));
            circuit.add_operation(CNOT::new(k + 1, k));
            // Basis rotation back into x-basis
            circuit.add_operation(Hadamard::new(k));
            circuit.add_operation(Hadamard::new(k + 1))
        }

        // Periodic boundary conditions
        circuit.add_operation(Hadamard::new(number_qubits - 1));
        circuit.add_operation(Hadamard::new(0));
        circuit.add_operation(CNOT::new(0, number_qubits - 1));
        circuit.add_operation(RotateZ::new(
            number_qubits - 1,
            (thetasymb * hopping_parameter).to_string().into(),
        ));
        circuit.add_operation(CNOT::new(0, number_qubits - 1));
        circuit.add_operation(Hadamard::new(number_qubits - 1));
        circuit.add_operation(Hadamard::new(0));

        circuit
    }

    // Helper function to return a circuit for the time-like evolution under magnetic field.
    //
    // # Arguments
    // * thetasymb: symbolic parameter 'theta' of the z-rotation.
    // * number_qubits: number of qubits in the system
    // * magnetic_field: parameter 'B' for the magnetic term of the Hamiltonian.
    fn create_magnetic_field_circuit(
        thetasymb: CalculatorFloat,
        number_qubits: usize,
        magnetic_field: f64,
    ) -> Circuit {
        let mut circuit = Circuit::new();

        // Representation of the \sigma^x\sigma^x interaction between two spins by the CNOT and Rotation gates
        for k in 0..number_qubits {
            circuit.add_operation(RotateZ::new(
                k,
                (thetasymb.clone() * magnetic_field).to_string().into(),
            ));
        }

        circuit
    }

    /// Function to construct a Circuit for the unitary evolution.
    ///
    /// Arguments
    /// * iter_evolution: number of iterations of evolution, minimum 1.
    /// * number_qubits: number of qubits in the system
    /// * hopping_parameter: parameter 't' for the hopping term of the Hamiltonian.
    /// * magnetic_field: parameter 'B' for the magnetic term of the Hamiltonian.
    ///
    /// # Returns
    /// Circuit for the unitary evolution
    pub fn create_evolution_circuit(
        iter_evolution: usize,
        number_qubits: usize,
        hopping_parameter: f64,
        magnetic_field: f64,
    ) -> Circuit {
        let mut circuit = Circuit::new();

        // here: theta_even_i, theta_odd_i and theta_z_i are symbolic parameters (free variational parameters)
        for i in 0..iter_evolution {
            circuit += create_even_hopping_circuit(
                ("theta_even_".to_string() + &i.to_string()).into(),
                number_qubits,
                hopping_parameter,
            );
            circuit += create_odd_hopping_circuit(
                ("theta_odd_".to_string() + &i.to_string()).into(),
                number_qubits,
                hopping_parameter,
            );
            circuit += create_magnetic_field_circuit(
                ("theta_z_".to_string() + &i.to_string()).into(),
                number_qubits,
                magnetic_field,
            );
        }

        circuit
    }

    // Combining the parts to QuantumProgram
    //
    // To execute the optimization we combine all the constructed circuits
    // and put them in one qoqo quantum programm.
    // With QuantumProgram we only need to provide the free parameters
    // and get back the measured expectation values.

    // Finally, all the previous steps are collected in a wrapper_function
    // which takes free parameters as argument and returns a single value as result.
    // The wrapper_function can be used to run the optimization routine.
    fn wrapper_function(theta: &[f64], _data: &mut ()) -> f64 {
        // Seting up initial state vector
        let initial_statevector: Array1<Complex64> = array![
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0 / 6_f64.sqrt(), 0.0),
            Complex64::new(1.0 / 6_f64.sqrt(), 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0 / 6_f64.sqrt(), 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0 / 2_f64.sqrt(), 0.0),
        ];

        let mut circuit_init = Circuit::new();

        circuit_init += PragmaSetStateVector::new(initial_statevector);

        // println!("Step 1: Initialization circuit constructed.");

        // variables and parameters
        let number_qubits: usize = 3;
        let magnetic_field: f64 = 1.0;
        let hopping_parameter: f64 = 3.0;
        // In order to achieve better minimization results we default to several iterations of (pseudo) time-evolution
        let iter_evolution: usize = 4;
        // Construction the evolution circuit
        let circuit_evolution: Circuit = create_evolution_circuit(
            iter_evolution,
            number_qubits,
            hopping_parameter,
            magnetic_field,
        );

        // println!("Step 2: Constructed evolution circuit.");

        // Setting up two basis rotation measurement circuits since we need to measure in two different bases.
        let mut circuit_x_basis_measurement = Circuit::new();
        let mut circuit_z_basis_measurement = Circuit::new();

        // Adding 'DefinitionBit' to the circuit defines the classical bit registers used to store the qubit readout.
        circuit_x_basis_measurement += DefinitionBit::new("ro_x".to_string(), number_qubits, true);
        circuit_z_basis_measurement += DefinitionBit::new("ro_z".to_string(), number_qubits, true);

        // Basis rotation with the Hadamard gate: Bring all qubits into z-basis.
        for i in 0..number_qubits {
            circuit_x_basis_measurement.add_operation(Hadamard::new(i));
        }

        // Add measurement operation to all circuits to write the measured values into the classical registers.
        let number_measurements: usize = 10000;
        circuit_z_basis_measurement.add_operation(PragmaRepeatedMeasurement::new(
            "ro_z".to_string(),
            number_measurements,
            None,
        ));
        circuit_x_basis_measurement.add_operation(PragmaRepeatedMeasurement::new(
            "ro_x".to_string(),
            number_measurements,
            None,
        ));

        // Setting up measurement input determining which expectation values of PauliProducts are measured from which circuit
        // and how they are combined linearly
        let mut measurement_input = PauliZProductInput::new(number_qubits, false);
        measurement_input
            .add_pauliz_product("ro_z".to_string(), vec![0])
            .unwrap();
        measurement_input
            .add_pauliz_product("ro_z".to_string(), vec![1])
            .unwrap();
        measurement_input
            .add_pauliz_product("ro_z".to_string(), vec![2])
            .unwrap();
        measurement_input
            .add_pauliz_product("ro_x".to_string(), vec![0, 1])
            .unwrap();
        measurement_input
            .add_pauliz_product("ro_x".to_string(), vec![1, 2])
            .unwrap();
        measurement_input
            .add_pauliz_product("ro_x".to_string(), vec![0, 2])
            .unwrap();
        // Defining and adding the linear combinations of Pauli products that give the expectation values
        let linear_exp_val = HashMap::from([
            (0, magnetic_field),
            (1, magnetic_field),
            (2, magnetic_field),
            (3, hopping_parameter),
            (4, hopping_parameter),
            (5, hopping_parameter),
        ]);
        measurement_input
            .add_linear_exp_val("energy".to_string(), linear_exp_val)
            .unwrap();

        // println!("Step 3: Measurement circuits constructed. Measurement input prepared.");
        // println!("In z-basis: {:?}", circuit_z_basis_measurement);
        // println!("In x-basis: {:?}", circuit_x_basis_measurement);

        // Construct basis rotation measurement to get the expectation values.
        let measurement = PauliZProduct {
            input: measurement_input,
            circuits: vec![
                circuit_init.clone()
                    + circuit_evolution.clone()
                    + circuit_z_basis_measurement.clone(),
                circuit_init.clone() + circuit_evolution + circuit_x_basis_measurement.clone(),
            ],
            constant_circuit: None,
        };

        // One needs to define a backend where the program is executed.
        let backend = Backend::new(number_qubits);
        // QuantumProgram takes the prepared list of circuits and the list of free parameter names (the symbolic values in the circuit).
        let program = QuantumProgram::PauliZProduct {
            measurement,
            input_parameter_names: vec![
                "theta_even_0".to_string(),
                "theta_odd_0".to_string(),
                "theta_z_0".to_string(),
                "theta_even_1".to_string(),
                "theta_odd_1".to_string(),
                "theta_z_1".to_string(),
                "theta_even_2".to_string(),
                "theta_odd_2".to_string(),
                "theta_z_2".to_string(),
                "theta_even_3".to_string(),
                "theta_odd_3".to_string(),
                "theta_z_3".to_string(),
            ],
        };

        // println!("Step 4: QuantumProgram constructed.");

        program.run(backend, theta).unwrap().unwrap()["energy"]
    }

    // Optimization of free parameters
    // Minimiization routine to optimize the free parameters.
    // The wrapper_function consisting of the quantum program has 12 free parameters:
    // theta_even, theta_odd and theta_z for each of the 4 iterations of evolution.

    let mut x = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let cons: Vec<&dyn CstrFn> = vec![];
    let (_status, x_opt) = fmin_cobyla(wrapper_function, &mut x, &cons, (), 0.5, 1e-4, 200, 0);
    let energy_variational = wrapper_function(x_opt, &mut ());

    // println!(">> Optimized parameters theta:  {:?}", x_opt);
    println!(
        ">> Calculated approximate Energy value:  {:?}",
        energy_variational
    );

    //PART II: Compare the calculated (approximate) result to the exact classical solution
    //
    // Here we present an exact solution of the sample Hamiltonian to compare the exact results
    // to the calculated solution of the VHA method.

    // First, construct Hamiltonian
    // Pauli matrices
    let sigmax_array: Array2<Complex64> = array![
        [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
    ];
    let sigmay_array: Array2<Complex64> = array![
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 1.0)],
        [Complex64::new(0.0, -1.0), Complex64::new(0.0, 0.0)],
    ];
    let sigmaz_array: Array2<Complex64> = array![
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)],
    ];
    // Identity
    let identity_array: Array2<Complex64> = array![
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
    ];

    // convert 2d array to nalgebra::DMatrix to use the kronecker product function
    /// Helper function to convert a two-dimensional ndarray to a matrix
    fn convert_matrix(customarray: Array2<Complex64>) -> na::DMatrix<Complex64> {
        let dim = customarray.dim();
        na::DMatrix::<Complex64>::from_iterator(dim.0, dim.1, customarray.t().iter().cloned())
    }

    let sigmax = convert_matrix(sigmax_array);
    let _sigmay = convert_matrix(sigmay_array);
    let sigmaz = convert_matrix(sigmaz_array);
    let identity = convert_matrix(identity_array);

    // hopping term for 3 qubits
    let hopping_factor = Complex64::new(3.0, 0.0);
    let h_hopping = ((sigmax.kronecker(&sigmax)).kronecker(&identity)
        + (identity.clone().kronecker(&sigmax)).kronecker(&sigmax)
        + (sigmax.clone().kronecker(&identity)).kronecker(&sigmax))
        * hopping_factor;

    // magnetic term for 3 qubits
    let magnetic_factor = Complex64::new(1.0, 0.0);
    let h_magnetic = ((sigmaz.kronecker(&identity)).kronecker(&identity)
        + (identity.clone().kronecker(&sigmaz)).kronecker(&identity)
        + (identity.clone().kronecker(&identity)).kronecker(&sigmaz))
        * magnetic_factor;

    // total Hamiltonian
    let h_matrix: na::DMatrix<Complex64> = h_hopping + h_magnetic;

    // get the exact eigenvalues
    let mut eigenvalues = vec![];
    for k in &h_matrix.eigenvalues().unwrap() {
        eigenvalues.push(k.real());
    }
    eigenvalues.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!(">> Calculated exact eigenvalues:  {:?}", eigenvalues);

    // final print-out
    let delta = energy_variational - eigenvalues[0];
    println!(
        ">> Difference between VHA result and exact result: {}",
        delta
    );
}
