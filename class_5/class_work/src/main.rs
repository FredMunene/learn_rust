fn main(){
    let date_time_string :&str = "2024-05-23T12:34:56";

    // Split the date and time string
    let parts: Vec<&str> = date_time_string.split('T').collect();
    
    // Check if there are two parts (date and time)
    if parts.len() == 2 {
        // Extract and print the time part
        println!("Time: {}", parts[1]);
    } else {
        println!("Invalid date time string format.");
    }
}















// fn main (){
//     let mut x:Vec<&str> = Vec::new();
//     x.push("Hello");
//     x.push("World");
//     x.push("....");
//     println!("before {:?}",x);



//     let finals: usize = x.len().saturating_sub(1);
//     x.truncate(finals);


//     println!("after {:?}",x);

//      let mut y: Vec<i32> = Vec::new();
//      y.push(1);
//      y.push(2);
//      y.push(3);

//      // use a loop to mutate each item in Vec
//     println!("Before {:?}", y);
//      for mut m in &mut y {
//         *m += 10;
//      }
//      println!("After {:?}", y);
// }






















// fn main() {
//     let x:Vec<i32> = vec![1,2,3,4,5];
//     println!("{:?}", x);

//     // for item in x {
//     //     println!("Looping though {}", item);
//     // }
//         let mut j = 0;
//     loop {
//         if j < x.len(){
//             println!("Looping through {}", x[j]);
//             j += 1;
//         } else {
//             break
//         }
//     }

//     // let mut k = 0;
//     // while k < x.len(){
//     //     println!("Looping through {}", x[k]);
//     //     k = k+1;
//     // }
// }
