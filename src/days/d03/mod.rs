pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

pub fn step_slope(input: &[String], right: usize, down: usize) -> i64 {
    let mut res = 0;
    let mut index = 0;
    let mut line_it = input.iter();
    let mut line: Option<&String> = line_it.next();
    while let Some(l) = line {
        let mod_ind = index % l.len();
        if l.as_bytes()[mod_ind] as char == '#' {
            res += 1;
        }

        index += right;
        for _ in 0..down {
            line = line_it.next();
        }
    }
    res
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    step_slope(input, 3, 1)
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let calcs = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let res = calcs.iter().map(|t| step_slope(input, t.0, t.1));
    //println!("{:#?}", res.collect());
    res.product()
}

use crate::myTest;
myTest!();