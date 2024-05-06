fn main(){
   // print_hello();
    print_hello();
    get_age(23);
    sum(num1: 122, num2: 556);
    sum2(num1: 453, num2: 654);
    sum3(num1: 657, num2: 101);
    sum3(num1: 657, num2: 101);

}

fn print_hello(){
    println!("Hello World");
}

fn get_age(age: u32){
    println!("Your age is {age}");
}
// sum takes in two parameters
fn sum(num1: i32, num2 :i32){
    let num3 : i32 = num1 + num3;
    println!("1) sum of 1 and 2 is {}",num3)
}

fn sum2(num1: i32, num2 :i32)-> i32{
    let num3 : i32 = num1 + num3;
    println!("s2) um of 1 and 2 is {}",num3);
    return num3;
}
fn sum3(num1: i32, num2 :i32)-> i32{
    let num3 : i32 = num1 + num3;
    println!("3) sum of 1 and 2 is {}",num3);
    num3 // implicit return
}