fn main() {

    let array_1: [u32; 5] = [1,9,7,2,10];
    let total_summation: u32 = total_summation(&array_1);

    println!("The total summation is {}", total_summation);
}
// total_summation accepts a reference to an array of 5 'u32' elements: '&[u32; 5]'
fn total_summation(array_1: &[u32;5]) -> u32 { 
    let mut sum: u32 = 0;
    // for loop uses 'array_1.iter()' that returns an iterator over the array.
    // Each element is referenced with '&i'
    for &i in array_1.iter() {
        sum = sum + i;
    }
    sum
}