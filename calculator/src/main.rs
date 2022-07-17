use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let num1 = first.parse::<f32>().unwrap();
    let num2 = second.parse::<f32>().unwrap();
    let result = operate(operator, num1, num2);

    println!("{:?}", output(num1, operator, num2, result));
}

fn operate(operator: char, num1: f32, num2: f32) -> f32 {
    match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '/' => num1 / num2,
        '*' | 'x' | 'X' => num1 * num2,
        _ => panic!("Invalid operator used")
    }
} 


fn output(num1: f32, operator: char, num2: f32, result: f32) -> String {
    format!("{} {} {} = {}", num1, operator, num2, result)
}

