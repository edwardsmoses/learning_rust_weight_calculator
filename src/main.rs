use std::io;

fn main() {
    let mut weight_input = String::new();

    println!("What's your current Weight?");

    io::stdin().read_line(&mut weight_input).unwrap();

    let mut weight_on_mars = calculate_weight_on_mars(weight_input.trim().parse().unwrap());
    weight_on_mars = weight_on_mars * 1000.0;
    println!("Weight on Mars: {}kg", weight_on_mars);
   
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

