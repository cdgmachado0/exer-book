use std::collections::HashMap;

pub fn get_mode(list: &mut Vec<i32>) {
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