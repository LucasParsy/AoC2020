use crate::myTest;

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let num_vec: Vec<i64> = input.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    for (i, a) in num_vec.iter().enumerate() {
        for b in num_vec[i + 1..].iter() {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    -1
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let num_vec: Vec<i64> = input.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    for (i, a) in num_vec.iter().enumerate() {
        for (u, b) in num_vec[i + 1..].iter().enumerate() {
            for c in num_vec[u + 1..].iter() {
                if a + b +c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    -1
}

myTest!();