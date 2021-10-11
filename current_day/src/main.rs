pub mod common;
pub mod day;

fn main() {
    if let Ok(input) = common::file_to_lines("src/day/input.txt") {
        let res: i64;
        if day::ISP2 {
            res = day::p2(&input, true)
        } else {
            res = day::p1(&input, true)
        }
        println!("res: {}", res)
    }
}
