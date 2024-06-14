// fn has_data(time_of_day: u16) -> Option<u16> {

//     if (time_of_day < 1000){
//         return Ok(time_of_day);
//     }
//     else if time_of_day >= 1001 || time_of_day <= (2000){
//         return Ok(50);
//     }
//     else if (time_of_day > 2300){
//         None
//     }
// }

// if time is before  10 am, return 100
// between 10:01 and 8pm, return 50
// if past 11pm, return none
// write tests
// pub fn add(left: usize, right: usize){

// }

// fn main(){

//     let five_am:  u16 = 500;
//     let ten_30_am: u16 = 1030;
//     let ten_pm: u16 = 2200;

//     assert!(has_data(five_am)) == Some((100));
//     assert!(has_data(five_am)) == Some((100));
//     assert!(has_data(five_am)) == Some((100));
// }


//use std::collections::HashMap;
// fn main(){
//  // a shop managemnt sys that stores items and their qtts e.g pen=10
// // structs && traits create a solution, use trait to stock summary

//     let mut stock: HashMap::<String, Box <dyn StockItem>> = HashMap::new();
//     stock.insert(
//         String::from("pen"),
//         Box::new(Product{
//             name:String::from("pen"),
//             quantity: 10,
//         }),
//     );

//     for (key,item) in stock {
//         println!("Item: {} Quantity:{}",key,item.quantity());
//     }

//     let mut vec1: Vec<Product> = Vec::new();
//     let item: Product=Product{name:"Cakes".to_string(),quantity:4};
//     vec1.push(item);
//     item.summary_data();
// }

// struct Product {
//     name:String,
//     quantity:u32,
// }
//  trait StockItem{
//     fn name(&self) -> &str;
//     fn quantity(&self) -> u32;
//  }
//  trait Summary{
//     fn summary_data(&self);
//  }

// impl StockItem for Product {
//     fn name(&self) -> &str{
//         &self.name
//     }

//     fn quantity(&self) -> u32 {
//         self.quantity
//     }
// }

// impl Summary for Product{
//     fn summary_data(&self){
//         println!("Stock: {}, Quantity: {}", self.name,self.quantity)
//     }
// }



// fn main() {
//     // Create two vecs
//     let vec1: Vec<i32> = (1..=50).collect();
//     let vec2: Vec<i32> = (51..=100).rev().collect();

//     // Chash map with keys from the first vector and values from the second vector 
//     let mut map: HashMap<i32, i32> = HashMap::new();

//     for (index, item) in vec1.iter().enumerate() {
//         match vec2.get(index) {
//             Some(&value) => {
//                 map.insert(*item, value);
//             }
//             None => {
//                 //  the index is out of bounds
//                 println!("Index out of bounds: {}", index);
//             }
//         }
//     }

//     // Print the contents of the hash map
//     for (key, value) in &map {
//         println!("{}:{}", key, value);
//     }
// }












// fn main() {
//     // create two vecs
//     // first - 1> 50
//     // secnd - 51 > 100
//     // create hash map ; key -> first vector; values -> second vector in descen
//     // example 1:100

//     let vec1: Vec<i32> = (1..=50).collect();

//     let vec2: Vec<i32> = (51..=100).rev().collect();

//     let mut map: HashMap<i32, i32> = HashMap::new();

//     for (index,item) in vec1.iter().enumerate(){
//         //println!("{}:{}",index, item);
//         let value = vec2.get(index).unwrap(); // use match***(pattern matching)
//         map.insert(*item,*value);
//     }

//     for (index, item) in map {
//         println!("{}:{}",index, item);
//     }

// }
