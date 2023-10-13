use crate::rsa::math_functions::number_theory::modulo_inverse;

mod rsa;
mod tests;

fn main() {
    println!("{}", modulo_inverse(315, 661643));
}
