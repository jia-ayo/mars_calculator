use std::io;
fn main() {
    println!("Enter your weight (kg):  ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    
    let mars_weight = calc_for_weight_on_mars(weight );

    println!("weight on mars: {}kg", mars_weight)
}
fn calc_for_weight_on_mars(weight: f32) -> f32{
    (weight / 9.81) * 3.711
}
