use std::time::Instant;

mod rsa;
mod tests;

use crate::rsa::math_functions::number_theory::fast_exponentiation;

fn main() {
    let start = Instant::now();
    println!("{}", fast_exponentiation(561563, 1300, 564));
    println!("{}", start.elapsed().as_nanos());
}
