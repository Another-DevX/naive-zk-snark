use ark_bls12_381::{Fr, G1Projective};
use ark_ec::PrimeGroup;
use ark_ff::{AdditiveGroup, PrimeField, UniformRand};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial};

use rand::thread_rng;

pub fn prove(
    coeffs: &[Fr],
    t_poly: &DensePolynomial<Fr>,
    g_s: &Vec<G1Projective>,
    g_s_alpha: &Vec<G1Projective>,
) -> (G1Projective, G1Projective, G1Projective) {
    let p_poly = DensePolynomial::from_coefficients_slice(coeffs);

    let h_poly = divide_poly(&p_poly, t_poly);

    assert!(
        coeffs.len() <= g_s.len(),
        "Not enough coeffs for the choosen CRS"
    );

    let mut rng = thread_rng();

    let delta = Fr::rand(&mut rng);

    let result = evaluate_encrypted_poly(coeffs, g_s, &delta);
    let result_h = evaluate_encrypted_poly(&h_poly.coeffs, g_s, &delta);
    let result_prime = evaluate_encrypted_poly(coeffs, g_s_alpha, &delta);

    (result, result_h, result_prime)
}

fn evaluate_encrypted_poly(coeffs: &[Fr], g_s: &[G1Projective], delta: &Fr) -> G1Projective {
    let mut result = G1Projective::ZERO;

    for (i, &coeff) in coeffs.iter().enumerate() {
        let term = g_s[i].mul_bigint(coeff.into_bigint());

        result += term;
    }

    result.mul_bigint(delta.into_bigint())
}

fn divide_poly(p_poly: &DensePolynomial<Fr>, t_poly: &DensePolynomial<Fr>) -> DensePolynomial<Fr> {
    p_poly.clone() / t_poly.clone()
}
