// fibonnaci

pub fn fibonnaci(num: i32) {
    // Initialize the first two Fibonacci numbers
    let mut num1 = 0;
    let mut num2 = 1;

    // Print the first Fibonacci number
    if num >= 0{
        println!("{}", num1);
    }

    // Print the second Fibonacci number
    if num >= 2{
        println!("{}", num2);
    }
    
    // Generate and print rest of the Fibonacco
    for _ in 2..num {
        let next_num = num1 + num2;
        println!("{}", next_num);
        num1 = num2;
        num2 = next_num;
    }
}