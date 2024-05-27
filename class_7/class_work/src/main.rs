
// creating a struct implement a game character
// charcter ; name, score, level

mod game_chess;

use game_chess::game::Game;

fn main(){
    // let game1 = Game{
    //     name: String::from("Chess"),
    //     score: 12,
    //     level: 1,
    // };

    // let _game2 = Game{
    //     name: String::from("ludo"),
    //     score: 30,
    //     level: 2,
    // };
    // // print the level of character chess
    // let result = game1.level;
    // let result1 = _game2.level;
    // println!("The game is at level {}", result);
    // println!("The game is at level {}", result1);


    let mut result = Game::new(String::from("chess"),12,1);
    println!("The game chess is at level {}, the score is {}", result.return_level(),  result.return_score());
    result.increase_level();
    println!("The game chess is at level {}", result.return_level());
}


















// Recursion function
// mod one_to_ten; // name of folder

// use one_to_ten::recursive::one_to_ten; //folder(module);fileName;FunctionName
// use one_to_ten::fibonacci::fibonnaci;
// fn main(){

//   //  recursion_func(10)
//   one_to_ten();
//   fibonnaci(8);

// }

// fn recursion_func(mut num: i32){
//     println!("{}",num);
//     if num <= 10 {
//         num += 1;
//         recursion_func(num);
//     }   
// }









// // a func that returns a slice

// fn main(){
//     let _sub1 = "Fred";
//     let _sub2 = String::from("Munene");
//     let (one,_two) = slice_maker();

//     println!("{}",one);
// }

// fn slice_maker() -> (i32,String){
//     let text = "Gitonga";

//    // println("{}, {}", sliceA,sliceB);
//     return (2,text.to_string());
// }














// fn main() {
//     // Array of seven characters

//     let array: [char; 7] = ['a','b','c','d','e','f','g'];
//        // loops & conditions
//     for elem in array {
//         println!("{}", elem);
//     }

//     let mut indx :usize = 0;

//     println!("LOOP");

//     loop {
//         if indx == array.len(){
//             break;
//         }
//         println!("{}", array[indx]);
//         indx += 1;
//     }
// }
