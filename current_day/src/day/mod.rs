use crate::myTest;

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);
pub static ISP2: bool = true;

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    for _line in input.iter() {}
    1
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    for _line in input.iter() {}
    2
}

myTest!();
