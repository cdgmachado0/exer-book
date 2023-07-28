
use std::collections::HashMap;

fn main() {
    // let mut list = vec![3, 5, 7, 9, 10, 14, 32, 99];
    let mut list = vec![6, 5, 5, 5, 6, 9, 6, 6, 8, 5];
    // get_median(&mut list);
    get_mode(&mut list);

}


fn get_median(list: &mut Vec<i32>) { 
    list.sort();

    let i = list.len() / 2;
    let median = list[i];

    println!("The median value is: {}", median);
    println!("vec: {:?}", list);
}

fn get_mode(list: &mut Vec<i32>) {
    let mut map = HashMap::new();
    let mut flag = false;
    
    for num in list.iter() {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut highest_val = 0;
    let mut highest_num = 0;

    for (key, value) in map {

      if value > highest_val {
        highest_num = *key;
        highest_val = value;
      } else if value == highest_val {
        flag = true;
        println!("There are two often numbers: {} and {}", highest_num, key);
      }
    }

    if !flag {
        println!("Most often number is: {}", highest_num);
    }
}

