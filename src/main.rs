use std::io;


fn main() {

    let mut weight_input = String::new();

    println!("What's your current Weight?");

    let result = io::stdin().read_line(&mut weight_input);
    match result {
        Ok(n) => {
            let mut weight_on_mars = calculate_weight_on_mars(n as f32);
            weight_on_mars = weight_on_mars * 1000.0;
            println!("Weight on Mars: {}kg", weight_on_mars);
        }
        Err(error) => {
            println!("error: {error}")
        },
    };

}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81)  * 3.711
}
