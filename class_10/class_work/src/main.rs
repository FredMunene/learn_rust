// trait called shape that has an area func
// program that calculates area of diferent shapes
// area of a circle and a rectangle
// use : traits, impl, 


// trait Shape {
//     fn area(&self) -> i32;
// }





// create struct news
// struct has three properties: Name, Headlines, NUmber
// create trait
// traits has a fn; default implementation
// trait second fn does not have a default implementation
// it creates a summmary for news
// create an implementation of the trait for the new struct
// call the two trait funcs from main

fn main(){

    let news = News{
        name : "Fred Munene".to_string(),
        headlines : "Welcome to Earth".to_string(),
        number : 10_000,
    };
    println!("Summary {}", news.summarise());

    earth_news(&news);

    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
}

struct News{
    name : String,
    headlines : String,
    number : i32,
}

trait NewReader{

    fn summarise(&self) -> String {
       "Summary available ?".to_string()
        }

    // fn read_news(&self)-> String;
}

impl  NewReader for News{

    fn summarise(&self)-> String {
        format!("The author is {}, they wrote '{}' {}times.", self.name,self.headlines, self.number)
    }    
}
 fn earth_news( items : &impl NewReader){
    println!("new book is out: {}", items.summarise());
 }





// // create trait multiply
// // have implementation for vec<i32>
// // implementation should multiply the items in the vector
// // return the results of the impl

// trait Multiplier {
//     fn multiply(&self)-> i32;
// }

// impl Multiplier for Vec<i32>{
//     fn multiply(&self)-> i32 {
//         self.iter().product()
//     }
// }

// fn print_product<T : Multiplier>(items :T) {
//     println!("Product is {}", items.multiply())

// }

// fn main(){
//     let numbers :Vec<i32> = vec![1,2,4,6];
//     print_product(numbers);  
// }





// trait Summable {

//     fn sum(&self)->i32;
    
// }

// impl Summable for Vec<i32>{
//     fn sum(&self) ->  i32 {
//         self.iter().sum()
//     }
// }

// fn print_sum<T : Summable>(items :T){
//     println!("Sum is {}", items.sum())
// }

// fn main(){
//     let numbers :Vec<i32> = vec![1,2,4,6];
//     print_sum(numbers);  
// }
























// struct  Person {
//     name: String,
//     age :i32
// }

// fn main() {
//     let p = Person{
//         name : String::from("Fred"),
//         age:40
//     };

//     println!("{}", p.describe_user());
//     println!("{}", p.describe());
// }


// trait  Describe {
//     fn describe_user(&self) -> String;

//     fn describe(&self) -> String;
// }


// impl Describe for Person {
//     fn describe_user(&self) -> String {
//         format!("User name: {} Age {}", self.name, self.age)
//     }
//     fn describe(&self) -> String{
//         format!("Person implementation")

//     }
// }