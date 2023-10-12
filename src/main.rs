use ibig::ubig;
use std::time::Instant;

mod rsa;
mod tests;

use crate::rsa::math_functions::number_theory::fast_exponentiation;

fn main() {
    let base = ubig!(561563);
    let exponent = ubig!(1300);
    let modul = ubig!(564);
    let start = Instant::now();
    println!("{}", fast_exponentiation(&base, &exponent, &modul));
    println!("{}", start.elapsed().as_nanos());
}
