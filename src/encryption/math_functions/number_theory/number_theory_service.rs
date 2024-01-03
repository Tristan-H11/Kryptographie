use crate::encryption::math_functions::number_theory::extended_euclid_result::ExtendedEuclidResult;
use num::BigInt;

use crate::encryption::math_functions::number_theory::fast_number_theory_service::FastNumberTheoryService;
use crate::encryption::math_functions::number_theory::slow_number_theory_service::SlowNumberTheoryService;
use crate::encryption::math_functions::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::shared::errors::ArithmeticError;

/// Gibt an, ob die schnelle oder die langsame Implementierung des `NumberTheoryService` verwendet werden soll.
pub enum NumberTheoryServiceSpeed {
    Fast,
    Slow,
}

/// Diese Struktur stellt Implementierungen zur Verfügung, um die Zahlentheorie zu berechnen.
#[derive(Clone, Copy)]
pub enum NumberTheoryService {
    FastService(FastNumberTheoryService),
    SlowService(SlowNumberTheoryService),
}

impl NumberTheoryService {
    pub fn new(speed: NumberTheoryServiceSpeed) -> NumberTheoryService {
        match speed {
            NumberTheoryServiceSpeed::Fast => {
                NumberTheoryService::FastService(FastNumberTheoryService::new())
            }
            NumberTheoryServiceSpeed::Slow => {
                NumberTheoryService::SlowService(SlowNumberTheoryService::new())
            }
        }
    }
}

impl NumberTheoryServiceTrait for NumberTheoryService {
    fn extended_euclid(&self, a: &BigInt, b: &BigInt) -> ExtendedEuclidResult {
        match self {
            NumberTheoryService::FastService(service) => service.extended_euclid(a, b),
            NumberTheoryService::SlowService(service) => service.extended_euclid(a, b),
        }
    }

    fn fast_exponentiation(&self, base: &BigInt, exponent: &BigInt, modul: &BigInt) -> BigInt {
        match self {
            NumberTheoryService::FastService(service) => {
                service.fast_exponentiation(base, exponent, modul)
            }
            NumberTheoryService::SlowService(service) => {
                service.fast_exponentiation(base, exponent, modul)
            }
        }
    }

    fn modulo_inverse(&self, n: &BigInt, modul: &BigInt) -> Result<BigInt, ArithmeticError> {
        match self {
            NumberTheoryService::FastService(service) => service.modulo_inverse(n, modul),
            NumberTheoryService::SlowService(service) => service.modulo_inverse(n, modul),
        }
    }

    fn is_probably_prime(
        &self,
        p: &BigInt,
        repeats: u32,
        random_generator: &PseudoRandomNumberGenerator,
    ) -> bool {
        match self {
            NumberTheoryService::FastService(service) => {
                service.is_probably_prime(p, repeats, random_generator)
            }
            NumberTheoryService::SlowService(service) => {
                service.is_probably_prime(p, repeats, random_generator)
            }
        }
    }
}

/// Hält die notwendigen Methoden der Zahlentheorie bereit.
pub trait NumberTheoryServiceTrait {
    /// Berechnet den erweiterten euklidischen Algorithmus für zwei Zahlen.
    ///
    /// # Argumente
    ///
    /// * `a` - Die erste Zahl.
    /// * `b` - Die zweite Zahl.
    ///
    /// # Rückgabewert
    ///
    /// * Ein ExtendedEuclidResult, das den ggT, x und y enthält.
    ///
    /// # Beispiel
    ///
    /// ```rust
    /// let result = NumberTheoryService.extended_euclid(&BigInt::from(12), &BigInt::from(30));
    ///
    /// assert_eq!(result.ggT, BigInt::from(6));
    /// assert_eq!(result.x, BigInt::from(2));
    /// assert_eq!(result.y, BigInt::from(1));
    /// ```
    fn extended_euclid(&self, a: &BigInt, b: &BigInt) -> ExtendedEuclidResult;

    /// Berechnet die Schnellexponentiation für eine Basis `base`, einen Exponent `exponent`
    /// und einen Modulus `modul`.
    ///
    /// # Argumente
    ///
    /// * `base` - Die Basis.
    /// * `exponent` - Der Exponent.
    /// * `modul` - Der Modulus.
    ///
    /// # Rückgabewert
    ///
    /// * Das Ergebnis der Schnellexponentiation.
    ///
    /// # Beispiel
    ///
    /// ```rust
    /// let base = BigInt::from(2);
    /// let exponent = BigInt::from(3);
    /// let modulus = BigInt::from(5);
    ///
    /// let result = NumberTheoryService.fast_exponentiation(&base, &exponent, &modulus, true);
    ///
    /// assert_eq!(result, BigInt::from(3));
    /// ```
    fn fast_exponentiation(&self, base: &BigInt, exponent: &BigInt, modul: &BigInt) -> BigInt;

    /// Berechnet das modulare Inverse von `n` modulo `modul`.
    ///
    /// # Argumente
    ///
    /// * `n` - Die Zahl, für die das Inverse berechnet werden soll.
    /// * `modul` - Der Modulus.
    ///
    /// # Rückgabewert
    ///
    /// * Das modulare Inverse von `n` modulo `modul`.
    ///
    /// # Fehler
    ///
    /// * `ArithmeticError::NoInverseError` - Wenn `n` und `modul` nicht teilerfremd sind, dann existiert kein Inverses.
    ///
    /// # Beispiel
    ///
    /// ```rust
    /// let n = BigInt::from(2);
    /// let modul = BigInt::from(5);
    ///
    /// let result = NumberTheoryService.modulo_inverse(&n, &modul, true);
    ///
    /// assert_eq!(result, Ok(BigInt::from(3)));
    /// ```
    fn modulo_inverse(&self, n: &BigInt, modul: &BigInt) -> Result<BigInt, ArithmeticError>;

    /// Diese Methode führt einen probabilistischen Primzahltest für den angegebenen Integer durch.
    ///
    /// # Argumente
    /// * `p`: Der Integer, für den der Primzahltest durchgeführt werden soll.
    /// * `repeats`: Die Anzahl der Wiederholungen des Tests.
    /// * `random_generator`: Ein Pseudozufallszahlengenerator, der für die Erzeugung
    ///   der Zufallszahlen verwendet wird.
    ///
    /// # Rückgabe
    /// * `true`, wenn der Integer eine vermutlich Primzahl ist, `false`, wenn nicht.
    fn is_probably_prime(
        &self,
        p: &BigInt,
        repeats: u32,
        random_generator: &PseudoRandomNumberGenerator,
    ) -> bool;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::{Fast, Slow};

    use super::*;

    fn run_test_for_all_services(test: impl Fn(NumberTheoryService)) {
        test(NumberTheoryService::new(Slow)); // Langsame, eigene Implementierung
        test(NumberTheoryService::new(Fast)); // Schnelle, externe Implementierung
    }

    #[test]
    fn extended_euclid_test() {
        run_test_for_all_services(|service| {
            assert_eq!(
                service.extended_euclid(&6.into(), &11.into()),
                ExtendedEuclidResult::new(1.into(), 2.into(), BigInt::from(-1))
            );
            assert_eq!(
                service.extended_euclid(&78.into(), &99.into()),
                ExtendedEuclidResult::new(3.into(), 14.into(), BigInt::from(-11))
            );
            assert_eq!(
                service.extended_euclid(&315.into(), &661643.into()),
                ExtendedEuclidResult::new(1.into(), BigInt::from(-319269), 152.into())
            );
            assert_eq!(
                service.extended_euclid(
                    &BigInt::from_str("485398853520739824211578869461").unwrap(),
                    &BigInt::from_str("79617341660363802320192939486040130094939703771377")
                        .unwrap(),
                ),
                ExtendedEuclidResult::new(
                    1.into(),
                    BigInt::from_str("7173228757438794445922076835963679049602847038123").unwrap(),
                    BigInt::from(-43732645957409398462249346726i128)
                )
            );
        });
    }

    #[test]
    fn modulo_inverse_test() {
        run_test_for_all_services(|service| {
            assert_eq!(
                service.modulo_inverse(&1.into(), &3.into()).unwrap(),
                1.into()
            );
            assert_eq!(
                service.modulo_inverse(&5.into(), &11.into()).unwrap(),
                9.into()
            );
            assert_eq!(
                service.modulo_inverse(&315.into(), &661643.into()).unwrap(),
                342374.into()
            );
            assert_eq!(
                service
                    .modulo_inverse(
                        &BigInt::from_str("485398853520739824211578869461").unwrap(),
                        &BigInt::from_str("79617341660363802320192939486040130094939703771377")
                            .unwrap(),
                    )
                    .unwrap(),
                BigInt::from_str("7173228757438794445922076835963679049602847038123").unwrap()
            );
            assert!(service.modulo_inverse(&78.into(), &99.into()).is_err());
        });
    }

    #[test]
    fn is_probably_prime_test() {
        let slow_service = NumberTheoryService::new(Slow);
        let random_generator: &PseudoRandomNumberGenerator = &PseudoRandomNumberGenerator::new(11);
        assert_eq!(
            slow_service.is_probably_prime(&11.into(), 100, random_generator),
            true
        );

        run_test_for_all_services(|service| {
            assert_eq!(
                service.is_probably_prime(
                    &BigInt::from_str("3884010174220797539108782582068795892283779").unwrap(),
                    40,
                    random_generator,
                ),
                false
            );

            assert_eq!(
                service.is_probably_prime(
                    &BigInt::from_str("3061046931436983206004510256116356531107241").unwrap(),
                    40,
                    random_generator,
                ),
                false
            );

            assert_eq!(
                service.is_probably_prime(
                    &BigInt::from_str("3348205994756289303286119224981125339947473").unwrap(),
                    40,
                    random_generator,
                ),
                false
            );
            assert_eq!(
                service.is_probably_prime(&2211.into(), 40, random_generator),
                false
            );
            assert_eq!(
                service.is_probably_prime(
                    &BigInt::from_str("79617341660363802320192939486040130094939703771377")
                        .unwrap(),
                    400,
                    random_generator,
                ),
                true
            );
        });
    }

    #[test]
    fn fast_exponentiation_happy_flow() {
        run_test_for_all_services(|service| {
            let base = &561563.into();
            let exponent = &1300.into();
            let modul = &564.into();
            assert_eq!(
                service.fast_exponentiation(base, exponent, modul),
                205.into()
            );

            let base = &BigInt::from(56156334590832345u64);
            let exponent = &BigInt::from(109458390583094852904812340u128);
            let modul = &564234859.into();
            assert_eq!(
                service.fast_exponentiation(base, exponent, modul),
                558376785.into()
            );
        });
    }

    #[test]
    fn fast_exponentiation_exponent_one() {
        run_test_for_all_services(|service| {
            let base = &561563.into();
            let exponent = &1.into();
            let modul = &564.into();
            assert_eq!(
                service.fast_exponentiation(base, exponent, modul),
                383.into()
            );
        });
    }

    #[test]
    fn fast_exponentiation_exponent_zero() {
        run_test_for_all_services(|service| {
            let base = &561563.into();
            let exponent = &0.into();
            let modul = &564.into();
            assert_eq!(service.fast_exponentiation(base, exponent, modul), 1.into());
        });
    }

    #[test]
    fn fast_exponentiation_base_zero() {
        run_test_for_all_services(|service| {
            let base = &0.into();
            let exponent = &561563.into();
            let modul = &564.into();
            assert_eq!(service.fast_exponentiation(base, exponent, modul), 0.into());
        });
    }

    #[test]
    fn fast_exponentiation_modul_one() {
        run_test_for_all_services(|service| {
            let base = &3459860.into();
            let exponent = &561563.into();
            let modul = &1.into();
            assert_eq!(service.fast_exponentiation(base, exponent, modul), 0.into());
        });
    }

    #[test]
    fn fast_exponentiation_big_numbers() {
        run_test_for_all_services(|service| {
            let base: &BigInt = &BigInt::from(3459860).pow(50);
            let exponent = &BigInt::from(561563).pow(50);
            let modul = &BigInt::from(345902).pow(50);
            assert_eq!(
                service.fast_exponentiation(base, exponent, modul),
                BigInt::from_str("8408769600151667634624424658533698267981486479206207837400880482584240253227309957477768628293816726387378101618036878012339908404810884160600885622896014991205830046127587869666780551862829518403374323613697514544975024376847169484921172903282824411000834600056510593848311808").unwrap()
            );

            let base: &BigInt = &BigInt::from(5345890).pow(50);
            let exponent = &BigInt::from(561563).pow(50);
            let modul = &BigInt::from(402).pow(453);
            assert_eq!(
                service.fast_exponentiation(base, exponent, modul),
                BigInt::from_str("3865812286670140244135659583582784422868607182053532234892119221318009519049840848928181595205903918350994052685502954280587622821049262028969781448838683538880584872202936321233747460151097175519490133290457481138503925299942681667894197628591512134856836351617429125260375231408269314105167853219462996964862842118589984098650391290176408417243048103776854044821474824889320850667507890395769983356345061882029969478502315374312501247186250351754934295084246986742329795390786311078628530440754856479205141409578653316171690967468840184450509388030981031737577810850620779725958417192948561835194773695941361612225803217565675522961705627046360944634654621872586535332182764197222888226270388954006295237642124544864715066176310557964933132099186352875757363791307327674799516376108738026266118794617758595269081482227829599835454778768141217737588053896979935711190022064516393010329090283156674631737792250605934457811116131051558041088439411741976788375428610551855415643131350851177642819664715803674680663207355557044121133492467414028969595889934382173724212977872023208326609954389561973252556941619801122899324611797790397387249251896640501121328429738301895459647520768").unwrap()
            );

            assert_eq!(
                service.fast_exponentiation(&37.into(), &2.into(), &89.into()),
                34.into()
            )
        });
    }
}
