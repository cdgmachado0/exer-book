pub fn get_median(list: &mut Vec<i32>) { 
    list.sort();

    let i = list.len() / 2;
    let median = list[i];

    println!("The median value is: {}", median);
    println!("vec: {:?}", list);
}