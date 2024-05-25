// use std::io;


fn main(){
    let my_vecta: Vec<i32> = vec![2,3,4];
    if let Some(result) = vecta(my_vecta,2){
        println!("Result:{}", result);
    } else {
        println!("None returned");
    }
}

fn vecta(param_1:Vec<i32>, param_2:usize) -> Option<i32>{
    // if condition checks if len(param_1) is < len(param_2)
    // caste data type to another
    if param_1.len() < param_2{
        println!("length of param_1 < param_2");
        Some(param_1[param_2])
    } else {
        return None;
    }

}


































//    // create  a program that takes an input
//     // parse that input to a number
//     // on success print the number
//     // on failure print a custom error message
//     // use match keyword, Option type
//     use std::io;

//     fn main() {
//         println!("Enter a number:");
    
//         let mut input = String::new();
    
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");
    
//         let number: Result<i32, _> = input.trim().parse();
    
//         match number {
//             Ok(n) => println!("Parsed number: {}", n),
//             Err(_) => println!("Failed to parse input as a number"),
//         }
//     }
    





// fn main(){
//     let res:Option<&str> = print_number("Scientist");

//     println!("{}", res.expect("Occupation not found"));

// }

// fn print_occupation(name : &str)->Option<&str>{
//     match name {
//         "programmer" => Some("Hey fellow coder"),
//         "Mathematician" => Some("Maths is great"),
//         _ => None
//     }
// }

// fn print_number(name : &str)->Option<i32>{
//     match name {
//         "kenn" =>Some(200),
//         "phil"=>Some(201),

//         _ => None
//     }
// }



// fn main(){

//     let my_string :String = "Hello World".to_string();
//     let res:String = match my_string.chars().nth(10){
//         Some(c)=> c.to_string(),
//         None => "No characters at index 10".to_string(),
//     };
//     println!("character is {}", res)
// }









// fn main() {

//     let my_vec :Vec<i32> = vec![1,2,3,4,5];

//     let result :Option<&i32> = my_vec.get(index:99);

//     match result {
//         Some(number :&i32)=>{
//             println!("NUmber found {}", number)
//         }
//         None=>{
//             println!("An error...")
//         }
//     }
// }
