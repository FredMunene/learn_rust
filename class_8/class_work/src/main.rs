mod triangle_area;
use triangle_area::Triangle;


// struct User{
//     name : String,
//     age : i32,
//     id_number : i32,
// }

// struct Cars{
//     name: String,
//     number: i32,
//     model: String
// }
// implementation block
// impl User {

    // pub fn new(name: String, age: i32, id_number: String) -> Self {
    //     User{name, age, id_number}
    // }

//     pub fn walk(&self) {
//         println!("User {} is walking",&self.name)
//     }

//     pub fn eat(&self){
//         println!("User {} is eating",&self.name)
//     }

//     pub fn is_adult(&self) -> bool {
//         if self.age > 18{
//             return true;
//         }
//         return false;
//     }
// }


fn main(){
    // let kenn = User {
    //     name :String::from("kenn"),
    //     age : 300,
    //     id_number : 9876543
    // };

    // let kiki = User{
    //     name: String::from("kiki"),
    //     age: 40,
    //    id_number: 2345678
    // };

    // let _car = Cars{
    //     name :String::from("toyota"),
    //     number: 40,
    //     model: String::from("benz")
    // };

    // println!("Hello {}", kenn.name);
    // println!("Hello {}", kiki.name);

    // walk(kenn);

    // let result:bool = kenn.is_adult();
    // println!("Is ken an adult?, {result}")

    let triangle = Triangle::new(10,10);
    let results : i32 = triangle.area();
    println!("The area is {}", results);
}
// walk only takes arg of struct User
// fn walk(user:User){
//     println!("User {} is walking", user.name)
// }






// use math_rust::{sqrt,gcd};

// fn main() {

//     let result: f32 = sqrt(4.8);
//     let result1 :i32 = gcd(10,55);
//     println!("result {result}");
//     println!("result {result1}");

// }
