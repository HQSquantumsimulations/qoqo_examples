extern crate roqoqo;
extern crate eigenvalues;
extern crate nalgebra;
extern crate ndarray;
extern crate num_complex;

use nalgebra as na;
use na::ComplexField;
use ndarray::{array, Array2};
use num_complex::Complex64;

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
    let h_hopping = (
        (sigmax.kronecker(&sigmax)).kronecker(&identity)
        + (identity.clone().kronecker(&sigmax)).kronecker(&sigmax)
        + (sigmax.clone().kronecker(&identity)).kronecker(&sigmax)
    ) * hopping_factor;

    // magnetic term for 3 qubits
    let magnetic_factor = Complex64::new(1.0, 0.0);
    let h_magnetic = (
        (sigmaz.kronecker(&identity)).kronecker(&identity)
        + (identity.clone().kronecker(&sigmaz)).kronecker(&identity)
        + (identity.clone().kronecker(&identity)).kronecker(&sigmaz)
    ) * magnetic_factor;

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
    // let delta = final_result.fun - eigenvalues[0];
    // println!("{}{}", ">> Difference between VHA result and exact result: ", delta);
}
