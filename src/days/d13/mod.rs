pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let mut min_id = 0;
    let mut min_dist = 999999;

    let start: i64 = input[0].parse().unwrap();
    let splitted = input[1].split(',');
    for bus in splitted.map(|e| e.parse::<i64>()).filter(|e| e.is_ok()) {
        let id = bus.unwrap();
        let num = ((start + id - 1) / id) * id - start;
        if num < min_dist {
            min_dist = num;
            min_id = id;
        }
    }
    min_id * min_dist
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let mut equation: Vec<(i64, i64)> = Vec::new();
    let splitted = input[1].split(',');
    for (index, bus) in splitted.map(|e| e.parse::<i64>()).enumerate() {
        if bus.is_err() {
            continue;
        }
        let num = bus.unwrap();
        let pos = index as i64;
        equation.push((num, pos));
    }
    let total_mult = equation.iter().fold(1, |prev, el| prev * el.0);

    let mut res = 0;
    for elem in equation {
        if elem.1 == 0 {
            continue;
        }
        let mut mult = total_mult / elem.0;
        let base_mult = mult;
        while (mult % elem.0) != 1 {
            mult += base_mult;
        }
        res += elem.1 * mult;
    }

    res = total_mult - (res % total_mult);
    res as i64
}

use crate::myTest;
myTest!();
