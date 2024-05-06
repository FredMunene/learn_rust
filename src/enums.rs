fn main(){

    const age : i32 = 13;

    // Attributes
    #[derive(Debug)]

    // create data type
    enum STATUS {
        ADULT, 
        CHILD,
    }

    let user_status : STATUS = if age > 18 {
        STATUS::ADULT
    } else{
        STATUS::CHILD
    };

    println!("The status is {:?}", user_status);
}

