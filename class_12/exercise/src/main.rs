
use std::collections::HashMap;
fn main(){
 // a shop managemnt sys that stores items and their qtts e.g pen=10
// structs && traits create a solution, use trait to stock summary

    let mut stock: HashMap::<String, Box <StockItem>> = HashMap::new();
    stock.insert(
        String::from("pen"),
        Box::new(Product{
            name:String::from("pen"),
            quantity: 10,
        }),
    );

    for (key,item) in stock {
        println!("Item: {} Quantity:{}",key,item.quantity());
    }

    let vec1: Vec<Product> = Vec::new();
    let item: Product=Product{item:"Cakes".to_string(),quantity:4};
    

    summary_data()

}

struct Product {
    name:String,
    quantity:u32,
}
 trait StockItem{
    fn name(&self) -> &str;
    fn quantity(&self) -> u32;
 }
 trait Summary{
    fn summary_data(&self);
 }

impl StockItem for Product {
    fn name(&self) -> &str{
        &self.name
    }

    fn quantity(&self) -> u32 {
        self.quantity
    }
}

impl Summary for Product{
    fn summary_data(&self){
        println!("Stock: {}, Quantity: {}", self.name,self.quantity)
    }
}














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
