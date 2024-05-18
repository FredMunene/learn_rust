
// use rust_math::num::{sqrt,gcd};
// fn main() {

//     // importing external --on terminal, run 'cargo add package_name i.e rust_math'
    
//    let result :f32 = sqrt(4.0);
//    let gcdd :i32 = gcd(10,24);

//    println!("result is {result}");
//    println!("THe gcd is {gcdd}");
// }

// struct

struct User{
    name: String,
    age: i32,
    id_number: i32,
}

fn main(){
    let kenn = User{
        name: String::from{s: "kenn"},
        age: 300,
        id_number: 987654321,
    };

    println!("hello {}", kenn.name)
}