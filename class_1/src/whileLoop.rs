fn main(){
    let mut num = 0;
    while num < 5 {
        println!("{num}");
        num += 1;
    }

    let a = [1,2,3,4,5];

    for element in a {
        println!("the value of element is {element}");
    }

    for element in 5..10 {
        println!("the value of element is {element}");
    }
}