use ark_bls12_381::Bls12_381;
use ark_bls12_381::{G1Projective, G2Projective};
use ark_ec::pairing::Pairing;

pub fn verify(
    g_p: &G1Projective,
    g_h: &G1Projective,
    g_p_prime: &G1Projective,
    g_alpha: &G2Projective,
    g_t_s: &G2Projective,
    g: &G2Projective,
) -> bool {
    let poly_restriction = check_poly_restriction(g_p_prime, g, g_p, g_alpha);
    assert!(poly_restriction, "Failed to check polynomial restriction");

    let poly_cofactors = check_poly_cofactors(g_p, g, g_t_s, g_h);
    assert!(poly_cofactors, "Failed to check polynomial cofactors");

    println!("Verification succesfully");
    true
}

fn check_poly_restriction(
    g_p_prime: &G1Projective,
    g: &G2Projective,
    g_p: &G1Projective,
    g_alpha: &G2Projective,
) -> bool {
    let left = Bls12_381::pairing(g_p_prime, g);
    let right = Bls12_381::pairing(g_p, g_alpha);
    left == right
}

fn check_poly_cofactors(
    g_p: &G1Projective,
    g: &G2Projective,
    g_t_s: &G2Projective,
    g_h: &G1Projective,
) -> bool {
    let left = Bls12_381::pairing(g_p, g);
    let right = Bls12_381::pairing(g_h, g_t_s);
    left == right
}
