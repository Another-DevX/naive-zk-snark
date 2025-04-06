mod crs;
mod prover;
mod verifier;

use crs::trusted_setup;
use prover::prove;
use verifier::verify;

use ark_bls12_381::Fr;
use ark_poly::univariate::DensePolynomial;
use ark_poly::DenseUVPolynomial;

fn main() {
    let t_coeffs = [Fr::from(1), Fr::from(2), Fr::from(3)];
    let p_coeffs = [Fr::from(1), Fr::from(2), Fr::from(3)];

    run_zk_snark(&t_coeffs, &p_coeffs);
}

fn run_zk_snark(t_coeffs: &[Fr], p_coeffs: &[Fr]) {
    let t_poly = DensePolynomial::from_coefficients_slice(t_coeffs);
    let (g2_alpha, g2_t_s, g2, g_s, g_s_alpha) = trusted_setup(3, &t_poly);

    let (g_p, g_h, g_p_alpha) = prove(p_coeffs, &t_poly, &g_s, &g_s_alpha);

    verify(&g_p, &g_h, &g_p_alpha, &g2_alpha, &g2_t_s, &g2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_proof() {
        // t(x) = x + 1
        let t_coeffs = vec![Fr::from(1), Fr::from(1)];

        // p(x) = (x + 1)(x + 2) = x^2 + 3x + 2
        let p_coeffs = vec![Fr::from(2), Fr::from(3), Fr::from(1)];

        let t_poly = DensePolynomial::from_coefficients_slice(&t_coeffs);
        let (g2_alpha, g2_t_s, g2, g_s, g_s_alpha) = trusted_setup(3, &t_poly);

        let (g_p, g_h, g_p_alpha) = prove(&p_coeffs, &t_poly, &g_s, &g_s_alpha);

        let verified = verify(&g_p, &g_h, &g_p_alpha, &g2_alpha, &g2_t_s, &g2);
        assert!(verified, "The verification failed");
    }

    #[test]
    #[should_panic(expected = "Failed to check polynomial cofactors")]
    fn test_invalid_proof() {
        // t(x) = x + 1
        let t_coeffs = vec![Fr::from(1), Fr::from(1)];

        // p(x) = x^2 + 3x + 5 → no divisible por (x + 1)
        let p_coeffs = vec![Fr::from(5), Fr::from(3), Fr::from(1)];

        let t_poly = DensePolynomial::from_coefficients_slice(&t_coeffs);
        let (g2_alpha, g2_t_s, g2, g_s, g_s_alpha) = trusted_setup(3, &t_poly);

        let (g_p, g_h, g_p_alpha) = prove(&p_coeffs, &t_poly, &g_s, &g_s_alpha);

        verify(&g_p, &g_h, &g_p_alpha, &g2_alpha, &g2_t_s, &g2);
    }
}
