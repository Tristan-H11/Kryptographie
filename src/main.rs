mod math;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();


    math::fast_exponentiation::hello();
    math::expanded_euclidean_algorithm::hello();
}
