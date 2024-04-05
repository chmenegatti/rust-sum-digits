use std::io;

fn convert_to_int(s: & String) -> i32 {
    let x: i32 = s.trim().parse::<i32>().unwrap();
    x
}
fn main() {
    let mut sum = 0;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input value");
    let mut value_i32 = convert_to_int(&input);
    while value_i32 != 0 {
        let r = value_i32 % 10;
        sum = sum + r;
        value_i32 = value_i32 / 10;
    }

    println!("Sum of digits: {}", sum);
}
