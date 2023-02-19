/*
 * Examples:
 * println!("");"
 * Can receive variable number of arguments. Also can called
 * string contain {} (curly braces) that substitute with value.
 *
 * ! is macro definition. Used for meta programming purposes.
 * More complex than function.
 *
 * Terminal:
 * https://stackoverflow.com/questions/67094650/set-a-project-to-use-nightly-by-default
 * `rustup override set nightly` - enabling nightly mode
 * `cargo install cargo-expand`
 * `cargo expand`
 */

 fn main() {
    println!("Hello, world!");
    println!("Number: {}, string {}", 100, "abcd");
    print!("Weight on Mars: {}kg", calculate_weight_on_mars(100.0))
}

/*
 * Formula for calculating weight on Mars:
 * weight on Mars = (Weight on Earth / 9.81ms-2) * 3.711ms-2
 */
fn calculate_weight_on_mars(_weight: f32) -> f32 {
    (_weight / 9.81) * 3.711
}
