fn main() {
    /*
     * No need to infer type for variables name
     * let mars_weight = calculate_weight_on_mars(100.0);
     */
    let mut mars_weight = calculate_weight_on_mars(100.0);
    // Can't assign to immutable variables thus mars_weight need to become mutable
    mars_weight = mars_weight * 100.0;
    print!("Weight on Mars: {}g", mars_weight)
}

/*
 * Formula for calculating weight on Mars:
 * weight on Mars = (Weight on Earth / 9.81ms-2) * 3.711ms-2
 */
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
