// program that takes an input from terminal
// takes a second input from terminal
// then divides first_input by second_input
//  flag : division by zero
// flag : integer overflow
// flag : invalid characters

use std::io;
use std::str::FromStr;
// use std::ops::Div

fn main(){

    division()

}

fn division(){
    // Ask for first input from user
    println!("Enter the first digit:");

    let mut first_input: String = String::new();
    io::stdin()
        .read_line(&mut first_input)
        .expect("Failed to read input.");

    // Type casting
    // Parse input strings to integers

    let first_input: i32 = match i32::from_str(first_input.trim()){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid character entered for the first digit.");
            return;
        }
    };

    // Ask for second input from user
    println!("Enter the second digit:");

    let mut second_input: String = String::new();
    io::stdin().read_line(&mut second_input).expect("Failed to read input.");

       // Type casting
    // Parse input strings to integers

    let second_input: i32 = match i32::from_str(second_input.trim()){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid character entered for the second digit.");
            return;
        }
    };

    // Perfom division

    let result: Result<i32, &str> = if second_input == 0 {
        Err("Division by zero")
    } else {
        Ok(first_input/second_input)
    };

    // Print the result or error message

    match result {
        Ok(res) => println!("Results {}", res),
        Err(err) => println!("error:{}", err),
    }
}