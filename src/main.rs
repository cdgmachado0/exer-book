
use crate::mode::get_mode;
use crate::median::get_median;
use crate::latin::convert_word;
use crate::departments::add_department;

mod mode;
mod median;
mod latin;
mod departments;

fn main() {
    // let mut list = vec![6, 5, 5, 5, 6, 9, 6, 6, 8, 5];
    // get_median(&mut list);
    // get_mode(&mut list);
    // convert_word("first");

    add_department();
}









