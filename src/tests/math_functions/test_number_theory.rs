use crate::rsa::math_functions::number_theory::fast_exponentiation;

#[test]
fn fast_exponentiation_happy_flow() {
    let result = fast_exponentiation(561563, 1300, 564);
    assert_eq!(result, 205);
}

#[test]
fn fast_exponentiation_exponent_one() {
    let result = fast_exponentiation(561563, 1, 564);
    assert_eq!(result, 383);
}
