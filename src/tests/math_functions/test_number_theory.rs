#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::number_theory::{
        fast_exponentiation, miller_rabin, modulo_inverse,
    };
    use std::str::FromStr;
    use bigdecimal::num_bigint::{BigInt, BigUint};

    #[test]
    fn fast_exponentiation_happy_flow() {
        let base = &BigUint::from(561563u32);
        let exponent = &BigUint::from(1300u32);
        let modul = &BigUint::from(564u32);
        assert_eq!(fast_exponentiation(base, exponent, modul), BigUint::from(205u32));

        let base = &BigUint::from(56156334590832345u64);
        let exponent = &BigUint::from(109458390583094852904812340u128);
        let modul = &BigUint::from(564234859u32);
        assert_eq!(fast_exponentiation(base, exponent, modul), BigUint::from(558376785u32));
    }

    #[test]
    fn fast_exponentiation_exponent_one() {
        let base = &BigUint::from(561563u32);
        let exponent = &BigUint::from(1u32);
        let modul = &BigUint::from(564u32);
        assert_eq!(fast_exponentiation(base, exponent, modul), BigUint::from(383u32));
    }

    #[test]
    fn fast_exponentiation_exponent_zero() {
        let base = &BigUint::from(561563u32);
        let exponent = &BigUint::from(0u32);
        let modul = &BigUint::from(564u32);
        assert_eq!(fast_exponentiation(base, exponent, modul), BigUint::from(1u32));
    }

    #[test]
    fn fast_exponentiation_base_zero() {
        let base = &BigUint::from(0u32);
        let exponent = &BigUint::from(561563u32);
        let modul = &BigUint::from(564u32);
        assert_eq!(fast_exponentiation(base, exponent, modul), BigUint::from(0u32));
    }

    #[test]
    fn fast_exponentiation_modul_one() {
        let base = &BigUint::from(3459860u32);
        let exponent = &BigUint::from(561563u32);
        let modul = &BigUint::from(1u32);
        assert_eq!(fast_exponentiation(base, exponent, modul), BigUint::from(0u32));
    }

    #[test]
    fn fast_exponentiation_big_numbers() {
        let base = &BigUint::from(3459860u32).pow(50);
        let exponent = &BigUint::from(561563u32).pow(50);
        let modul = &BigUint::from(345902u32).pow(50);
        assert_eq!(
            fast_exponentiation(base, exponent, modul),
            BigUint::from_str("8408769600151667634624424658533698267981486479206207837400880482584240253227309957477768628293816726387378101618036878012339908404810884160600885622896014991205830046127587869666780551862829518403374323613697514544975024376847169484921172903282824411000834600056510593848311808").unwrap()
        );

        let base = &BigUint::from(5345890u32).pow(50);
        let exponent = &BigUint::from(561563u32).pow(50);
        let modul = &BigUint::from(402u32).pow(453);
        assert_eq!(
            fast_exponentiation(base, exponent, modul),
            BigUint::from_str("3865812286670140244135659583582784422868607182053532234892119221318009519049840848928181595205903918350994052685502954280587622821049262028969781448838683538880584872202936321233747460151097175519490133290457481138503925299942681667894197628591512134856836351617429125260375231408269314105167853219462996964862842118589984098650391290176408417243048103776854044821474824889320850667507890395769983356345061882029969478502315374312501247186250351754934295084246986742329795390786311078628530440754856479205141409578653316171690967468840184450509388030981031737577810850620779725958417192948561835194773695941361612225803217565675522961705627046360944634654621872586535332182764197222888226270388954006295237642124544864715066176310557964933132099186352875757363791307327674799516376108738026266118794617758595269081482227829599835454778768141217737588053896979935711190022064516393010329090283156674631737792250605934457811116131051558041088439411741976788375428610551855415643131350851177642819664715803674680663207355557044121133492467414028969595889934382173724212977872023208326609954389561973252556941619801122899324611797790397387249251896640501121328429738301895459647520768").unwrap()
        );

        assert_eq!(
            fast_exponentiation(&BigUint::from(37u32), &BigUint::from(2u32), &BigUint::from(89u32)),
            BigUint::from(34u32)
        )
    }

    #[test]
    fn modulo_inverse_test() {
        //assert_eq!(modulo_inverse(BigInt::from(1), BigInt::from(3)).unwrap(), BigInt::from(1));
        assert_eq!(modulo_inverse(BigInt::from(5), BigInt::from(11)).unwrap(), BigInt::from(9));
        assert_eq!(
            modulo_inverse(BigInt::from(315), BigInt::from(661643)).unwrap(),
            BigInt::from(342374)
        );
        assert_eq!(
            modulo_inverse(
                BigInt::from_str("485398853520739824211578869461").unwrap(),
                BigInt::from_str("79617341660363802320192939486040130094939703771377").unwrap()
            )
            .unwrap(),
            BigInt::from_str("7173228757438794445922076835963679049602847038123").unwrap()
        );
        assert!(modulo_inverse(BigInt::from(78), BigInt::from(99)).is_err());
    }

    #[test]
    fn miller_rabin_test() {
        assert_eq!(miller_rabin(&BigUint::from(11u32), 40), true);
        assert_eq!(miller_rabin(&BigUint::from(8727030382015287123761u128), 40), false);
        assert_eq!(miller_rabin(&BigUint::from(2459872438590349034582u128), 40), false);
        assert_eq!(miller_rabin(&BigUint::from(2211u32), 40), false);
        assert_eq!(
            miller_rabin(
                &BigUint::from_str("79617341660363802320192939486040130094939703771377").unwrap(),
                40
            ),
            true
        );
    }
}
