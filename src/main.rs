fn main() {
    
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut median = list.len() / 2;
    median -= 1;
    median = list[median];

    println!("The median value is: {median}");


}
