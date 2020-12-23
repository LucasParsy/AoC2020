use crate::myTest;

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);
pub static ISP2: bool = false;

pub fn p1(_input: &[String], _interactive: bool) -> i64 {
    let num: i64 = _input[0].parse().unwrap();
    num * 2
}

pub fn p2(_input: &[String], _interactive: bool) -> i64 {
    let num: i64 = _input[0].parse().unwrap();
    num * 3
}

myTest!();
