fn main (){
    let (data :i32, status :bool , reason) :String = do_sum_maths(45,22);

    println!("Sum = {data} was it success {status} reason : {reason}");
}

fn do_sum_maths(par1: i32 , par2 : i32)-> (i32, bool, String){

    // method 1: return (par1+par2, true, "success".to_string())

    return (0, false, "failed for some reason".to to_string())

}