#[cfg(test)]
mod tests {
    use crate::rsa::math_functions::number_theory::fast_exponentiation;
    use ibig::ubig;

    #[test]
    fn fast_exponentiation_happy_flow() {
        let base = &ubig!(561563);
        let exponent = &ubig!(1300);
        let modul = &ubig!(564);
        assert_eq!(fast_exponentiation(base, exponent, modul), ubig!(205));

        let base = &ubig!(56156334590832345);
        let exponent = &ubig!(109458390583094852904812340);
        let modul = &ubig!(564234859);
        assert_eq!(fast_exponentiation(base, exponent, modul), ubig!(558376785));
    }

    #[test]
    fn fast_exponentiation_exponent_one() {
        let base = &ubig!(561563);
        let exponent = &ubig!(1);
        let modul = &ubig!(564);
        assert_eq!(fast_exponentiation(base, exponent, modul), ubig!(383));
    }

    #[test]
    fn fast_exponentiation_exponent_zero() {
        let base = &ubig!(561563);
        let exponent = &ubig!(0);
        let modul = &ubig!(564);
        assert_eq!(fast_exponentiation(base, exponent, modul), ubig!(1));
    }

    #[test]
    fn fast_exponentiation_base_zero() {
        let base = &ubig!(0);
        let exponent = &ubig!(561563);
        let modul = &ubig!(564);
        assert_eq!(fast_exponentiation(base, exponent, modul), ubig!(0));
    }

    #[test]
    fn fast_exponentiation_modul_one() {
        let base = &ubig!(3459860);
        let exponent = &ubig!(561563);
        let modul = &ubig!(1);
        assert_eq!(fast_exponentiation(base, exponent, modul), ubig!(0));
    }
}
