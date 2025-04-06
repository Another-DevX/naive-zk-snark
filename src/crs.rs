use ark_bls12_381::{Fr, G1Projective, G2Projective};
use ark_ec::PrimeGroup;
use ark_ff::{Field, PrimeField, UniformRand};
use ark_poly::univariate::DensePolynomial;
use ark_poly::Polynomial;
use rand::thread_rng;

pub fn trusted_setup(
    degree: usize,
    t_poly: &DensePolynomial<Fr>,
) -> (
    G2Projective,
    G2Projective,
    G2Projective,
    Vec<G1Projective>,
    Vec<G1Projective>,
) {
    let mut rng = thread_rng();

    let s = Fr::rand(&mut rng);
    let alpha = Fr::rand(&mut rng);
    let t_s = t_poly.evaluate(&s);

    let g1 = G1Projective::generator();
    let g2 = G2Projective::generator();

    let g2_alpha = g2.mul_bigint(alpha.into_bigint());
    let g2_t_s = g2.mul_bigint(t_s.into_bigint());

    let mut g_s_powers = Vec::with_capacity(degree + 1);
    let mut g_s_alpha_powers = Vec::with_capacity(degree + 1);

    let mut current_power = Fr::ONE;

    for _ in 0..=degree {
        let g_si = g1.mul_bigint(current_power.into_bigint());
        g_s_powers.push(g_si);

        let alpha_si = alpha * current_power;
        let g_alpha_si = g1.mul_bigint(alpha_si.into_bigint());

        g_s_alpha_powers.push(g_alpha_si);

        current_power *= s;
    }

    (g2_alpha, g2_t_s, g2, g_s_powers, g_s_alpha_powers)
}
