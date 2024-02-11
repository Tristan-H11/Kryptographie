use bigdecimal::num_bigint::BigInt;

///
/// Repräsentiert einen Punkt auf einer elliptischen Kurve.
///
pub struct Point {
    pub x: BigInt,
    pub y: BigInt,
}

impl Point {
    pub fn new(x: BigInt, y: BigInt) -> Self {
        Self { x, y }
    }
}