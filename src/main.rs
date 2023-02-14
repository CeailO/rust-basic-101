use std::io;
fn main() {
    println!("Enter your weight (kg):");
    let mut input = String::new();
    /*
     * 1. A method in results from read_line return type, which called unwrap().
     * if results return error, the program terminated.
     * if results return success, it will yield the contents of the result.
     */
    io::stdin().read_line(&mut input).unwrap();

    /*
     * 2. Trimming whitespace
     */
    // let weight = input.trim();
    // println!("{}", weight);

    /*
     * 3. Trimming whitespace and pass into the weight variable type
     * Error caused by as the compiler  doesn't know what is weight type
     * Because during the input processing, there's might be impure elements
     * such as characters or ASCII characters. Therefore, parsing is failed.
     */
    // let weight = input.trim().parse();
    // println!("{}", weight);

    // Debug output
    // dbg!(weight);
    // println!("Input: {}", input);

    /*
     * 4. Trimming whitespace and pass into the weight variable type
     * The weight type is f64. The unwrap function is used as its satisfies
     * its return type, Yield the result or failed cause error. Explaination
     * on number 1.
     */
    let weight = input.trim().parse::<f64>().unwrap();
    // println!("{}", weight);

    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f64) -> f64 {
    (weight / 9.81) * 3.711
}
