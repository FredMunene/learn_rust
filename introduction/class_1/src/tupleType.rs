// struct in go

fn main(){

    let tup: (i32, f64, u8) = (-400, 6.4, 1);

    //destructuring

    let (_x, _y, _z) = tup;

    println!("the value of x is {_x}");
    println!("the value of y is {_y}");
    println!("the value of z is {_z}");
    

}
