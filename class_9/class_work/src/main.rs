fn main(){

    let date_time_string :&str = "2023-05-23T12:34:56"

    
}




















// fn main(){
//     let mut x : Vec<&i32> = Vec::new();
//     let mut y : Vec<&i32> = Vec::new();

//     x.push(&1);
//     x.push(&2);
//     x.push(&3);

//     println!("after {:?}",x);
//     // using a for loop, mutate each element in vector and print out

//     for i in &x {
//         let j = *i + 1;
//         y.push(&j)
//     }

//     println!("after {:?}",y)
// }
































// fn main(){

//     // create a vector
//     let mut x : Vec<&str> = Vec::new();

//     // add - push three items to vector
//     x.push("Hello");
//     x.push("WOrld");
//     x.push("........");

//     // 
//     println!(" before {:?}",x);

//     // removed onr item
//     let finals : usize = x.len().saturating_sub(1);
//     x.truncate(finals);

//     println!("after {:?}",x)
// }























// LOOPS

// fn main() {

//     // Vector needs to store same type of  data
//     let x : Vec <u32> = vec![1,2,3,4,5];
//     println!("{:?}", x);
    

//     // For loop
//     let t : Vec <u32> = vec![1,2,3,4,5];

//     for k in &t {
//         println!("For looping through {}",k);
//     }

//     // While loop
//     let m : Vec <u32> = vec![1,2,3,4,5];
//     let mut j: usize = 0;
//     while j < x.len(){
//         println!("While looping through {}",m[j]);
//         j += 1;
//     }

//     // Loop
//     let mut i :usize = 0;
//     let q : Vec <u32> = vec![1,2,3,4,5];

//     loop {
//         if i >= q.len(){
//             break;
//         }

//         println!("Looop through {}", q[i]);
//         i += 1;
//     }

// }
