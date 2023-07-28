


fn main() {
    let mut list = vec![3, 5, 7, 9, 10, 14, 32, 99];
    get_median(&mut list);

}


fn get_median(list: &mut Vec<i32>) { 
    list.sort();

    let i = list.len() / 2;
    let median = list[i];

    println!("The median value is: {}", median);
    println!("vec: {:?}", list);
}

