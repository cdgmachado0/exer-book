fn main() {
    let mut list = vec![3, 5, 7, 9, 10, 14, 32];
    get_median(&mut list);
    get_median2(&mut list);

}


fn get_median(list: &mut Vec<i32>) {
    list.sort();

    let mut median = list.len() / 2;
    median -= 1;
    let median = list[median];

    println!("The median value is: {}", median);
}

lazy_static! {
    static ref LENGTH: i32 = 
}

fn get_median2(list: &mut Vec<i32>) {
    let mut sum = 0;
    struct Number {
        num: i32,
        remainder: i32,
    }

    let numbers: [Number; list.len()];

    for num in list.iter() {
        sum += num;
    }

    let half = sum / 2;

    for num in list.iter() {
        let remainder = half % num;

        // let x = Number {
        //     num: *num,
        //     remainder: half % num,
        // };
    }
}





