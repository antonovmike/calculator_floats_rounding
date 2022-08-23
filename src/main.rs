use std::env::{args, Args};
use round::round_down;

fn main() {
    let mut arguments: Args = args();

    let first_argument  = arguments.nth(1).unwrap();
    let operator        = arguments.nth(0).unwrap().chars().next().unwrap();
    let second_argument = arguments.nth(0).unwrap();
    
    let first_number: f64 = first_argument.parse().unwrap();
	let second_number = second_argument.parse::<f64>().unwrap();

	let result = operate(operator, first_number, second_number);
    println!();
    println!("ANSWER:");
    println!("{} {} {} = {}", first_number, operator, second_number, result.round());
    println!("{} {} {} = {}", first_number, operator, second_number, round_down(result, 9));
}

fn operate(operator: char, first_number: f64, second_number: f64) -> f64{
    match operator {
        '+'		=> first_number + second_number,
        '-'		=> first_number - second_number,
        '/'		=> first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => 0.0,
    }
}

#[test]
fn add() {
    assert_eq!( 3.32, round_down(operate('+', 1.1, 2.22), 9) );
}
#[test]
fn substruct() {
    assert_eq!(3.0, operate('-', 5.0, 2.0) );
}
#[test]
fn multiply() {
    assert_eq!(4.0, operate('*', 2.0, 2.0) );
    assert_eq!(4.0, operate('x', 2.0, 2.0) );
    assert_eq!(4.0, operate('X', 2.0, 2.0) );
}
#[test]
fn divide() {
    assert_eq!(3.0, operate('/', 6.0, 2.0) );
}
#[test]
fn error() {
    assert_ne!(2.0, operate('u', 4.0, 2.0) );
}
