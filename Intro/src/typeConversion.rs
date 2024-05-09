fn main(){

    let number1: i32 = 101;

    // mut >> making it changeable

    let mut number1ToString: string  = number1.to_string();

    number1ToString.push_str( string: "people in the room");

    println!("Hey {number1ToString}")


    let input1 : &str = "21";

    let input1Number : i32 = input1.parse().expect(msg: "Was expecting an integer");

    println!("Hey {input1Number}");


    let input2: i16 = 200;

    let input2Number :i32 = input2 as i32;

    println!("Hey {input2Number}")


}