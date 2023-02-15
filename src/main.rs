/**
 * Rust bundled with std library by default.
 * Have collections of:
 * - Hash Maps
 * - Vectors
 * - Data Structures
 * - IO Primitives
 * More info: https://doc.rust-lang.org/std/#modules
 * IO Modules: https://doc.rust-lang.org/std/io/index.html
 */
use std::io;

fn main() {
    let mut input = String::new();
    // The definition accepts mutable string or mutable string reference
    io::stdin().read_line(&mut input);
    let mut mars_weight = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 100.0;
    print!("Weight on Mars: {}g", mars_weight)
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
