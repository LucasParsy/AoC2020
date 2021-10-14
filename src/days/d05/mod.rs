pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

pub fn get_boarding_id(boarding: &str) -> i64 {
    let binary_str = boarding
        .replace('B', "1")
        .replace('F', "0")
        .replace('R', "1")
        .replace('L', "0");
    let (row, column) = binary_str.split_at(7);
    // BFFFBBFRRR 70 7
    let row = i64::from_str_radix(row, 2).unwrap();
    let column = i64::from_str_radix(column, 2).unwrap();
    row * 8 + column
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    input.iter().map(|l| get_boarding_id(l)).max().unwrap()
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let ids: Vec<i64> = input.iter().map(|l| get_boarding_id(l)).collect();
    for elem in ids.iter() {
        if ids.contains(&(elem + 2)) && !ids.contains(&(elem + 1)) {
            //println!("{}", elem + 1);
            return elem + 1;
        }
    }
    0
}

use crate::myTest;
myTest!();