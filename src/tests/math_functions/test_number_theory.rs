use crate::rsa::math_functions::number_theory::fast_exponentiation;

#[test]
fn fast_exponentiation_happy_flow() {
    let base = 561563;
    let exponent = 1300;
    let modul = 564;
    assert_eq!(fast_exponentiation(base, exponent, modul), 205);

    let base = 56156334590832345;
    let exponent = 109458390583094852904812340;
    let modul = 564234859;
    assert_eq!(fast_exponentiation(base, exponent, modul), 558376785);
}

#[test]
fn fast_exponentiation_exponent_one() {
    let base = 561563;
    let exponent = 1;
    let modul = 564;
    assert_eq!(fast_exponentiation(base, exponent, modul), 383);
}

#[test]
fn fast_exponentiation_exponent_zero() {
    let base = 561563;
    let exponent = 0;
    let modul = 564;
    assert_eq!(fast_exponentiation(base, exponent, modul), 1);
}

#[test]
fn fast_exponentiation_base_zero() {
    let base = 0;
    let exponent = 561563;
    let modul = 564;
    assert_eq!(fast_exponentiation(base, exponent, modul), 0);
}

#[test]
fn fast_exponentiation_modul_one() {
    let base = 3459860;
    let exponent = 561563;
    let modul = 1;
    assert_eq!(fast_exponentiation(base, exponent, modul), 0);
}
