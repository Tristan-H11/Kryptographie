///
/// Erstellt einen BigUint aus einem unsigned Integer.
///
#[macro_export]
macro_rules! big_u {
    ($x:expr) => {
        BigUint::from($x as u128)
    };
}

///
/// Erstellt einen BigInt aus einem signed Integer.
///
#[macro_export]
macro_rules! big_i {
    ($x:expr) => {
        BigInt::from($x as i128)
    };
}

///
/// Erstellt ein BigDecimal aus einem Float.
///
#[macro_export]
macro_rules! big_d {
    ($x:expr) => {
        BigDecimal::from($x)
    };
}
