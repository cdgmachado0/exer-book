
use crate::mode::get_mode;
use crate::median::get_median;

mod mode;
mod median;

fn main() {
    // let mut list = vec![3, 5, 7, 9, 10, 14, 32, 99];
    let mut list = vec![6, 5, 5, 5, 6, 9, 6, 6, 8, 5];
    get_median(&mut list);
    get_mode(&mut list);

}





