mod product;

use product::Book;

fn main(){
    // let happybook = Book{
    //     name : String::from("Happy Moments with her"),
    //     status : true,
    //    category : String::from("fiction")
    // };

    let sadbook = Book::new(String::from("Chicken"),
                        true, 
                         String::from("storybook")
            );

    //let result :bool = happybook.avail();
    let result :bool = sadbook.avail();
    // let category : String = sadbook.categ();

    println!("The book is available. {}, the category is {}", result,sadbook.categ());
}