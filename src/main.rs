use std::time::Instant;

mod rsa;
mod tests;

fn main() {
    let start = Instant::now();
    println!("{}", rsa::number_theory_functions::fast_exponentiation(561563, 1300, 564));
    println!("{}", start.elapsed().as_nanos());
}
