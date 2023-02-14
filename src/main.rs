use std::io;

/*
 * Borrowing: Allow to refer to a value without taking ownership of the value.
 * Reference are immutable by default like others variables.
 *
 * a) Immutable borrow can be more than one or
 * b) there's only a single mutable borrow
 * at a time
 */

fn main() {
    let mut input = String::new();
    // Taking reference value from input

    // Either a) or b)

    // Example of immutable borrow. a)
    let s1 = &input;
    let s2 = &input;
    println!("{} {}", s1, s2); // immutable reference ended here and worked

    // Example of mutable borrow. b)
    // let mut s3 = &mut input; // invalid as mutable references already exist
    // println!("{}", s3);

    some_fn(&mut input);
    // println!("{} {}", s1, s2); // Does not work after mutated input by some_fn()
    io::stdin().read_line(&mut input);
    println!("input {}:", input);
    let mut mars_weight = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 100.0;
    print!("Weight on Mars: {}g", mars_weight)
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

// Taking reference (Ampersand for reference)
fn some_fn(s: &mut String) {
    // Trying to mutate s thus &mut is needed for s in the parameter
    s.push_str("a")
}
