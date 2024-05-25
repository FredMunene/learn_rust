fn main(){

    let mut s :String = String::from( "hello");

    change(&mut s);

    println!("->{s}")
}

fn change (some_string : &mut String){
    some_string.push_str( ", world");
}











// fn main(){

//     // let s1 :String = String::from("hello");
//     //  hello(s1.clone());
    
//     //  println!("{s1}");

//     let text: String = "hello".to_string();

//     let text2: &String = hello(&text);

//    // text.push_str(" world");

//     println!("{}{}", text2,text)
// }
// fn hello(par1 : &String) -> &String{
//        // println!("Param 1 {}", par1);
//         par1
// }

//     // fn main() {
// //     println!("Hello, world!");

// //         // annotate
// //     let a : i32 = 3;

// //     let b : i32 = a;

// //     println!("numbers {a}");

// //     // String

//     // 1. Borrow :- use return value 
//     // save
//     // 2. Move :-variable move to another function when called
//     // creating  a variable c
//    // let c : String = String::from("Hello");
//     // hello(c);
//     // let _b: String = hello(c); 

//   println!("print {}", b);

//     //println!("number {} -- {_}"),c,b;
// }

// // fn hello(par1: String) -> String{

// //     println!("param 1 {}", par1);

// //     return par1

// // }