/*
 * function declaration keyword:
 * fn <concat_name>(<parameter>: <parameter type>) -> <return type> {}
 */

 fn main() {
    //invoke function
    calculate_weight_on_mars(100.0);
}

fn calculate_weight_on_mars(_weight: f32) -> f32 {
    return 50.0;
    // or 50.0 (no semicolonization)
}
