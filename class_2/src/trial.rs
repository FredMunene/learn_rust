fn main(){

let s1 :String = String::from("hello");
 hello(&s1);

 println!("{s1}");


}
fn hello(par1 : String) -> String{
    println!("Param 1 {}", par1);
    par1
}