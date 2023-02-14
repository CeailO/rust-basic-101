use std::io;

/*
 * Ownership in rust is unique from the rest of languages:
 * Rules:
 * 1. Each value in rust is owned by a variable
 * 2. When the owner goes out of scope, the value will be deallocated.
 * 3. There can be only be ONE owner at a time
 */

fn main() {
    // String live on heap as its size is not known at compile time
    let mut input = String::new();
    /* Ownership of the String::new() is moved to the variable s. Cause variable input no longer valid.
     * String type is complex, thus can't be copied
     * Example can be copied:
     * let a = 5; (size known at compiled time, no allocation on heap)
     * let b = a; (variable b will copy value a)
     * Two different 5, and not different pointers.
     */
    // let mut s = input; // Invalid
    // some_fn(input); // Invalid
    io::stdin().read_line(&mut input);
    let mut mars_weight = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 100.0;
    print!("Weight on Mars: {}g", mars_weight)
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn _some_fn(_s: String) {}
