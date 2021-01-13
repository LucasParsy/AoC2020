use std::collections::HashMap;

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let mut nums: Vec<i64> = input[0]
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let mut prev_num = *nums.last().unwrap();
    for i in nums.len() + 1..=2020 {
        let mut it = nums.iter().rev().skip(1);
        let pos = it.position(|n| *n == prev_num);
        let mut res: i64 = 0;
        if let Some(n) = pos {
            res = (i - 1 - (i - 2 - n)) as i64;
        }
        nums.push(res);
        prev_num = res
    }
    prev_num
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let mut nums: HashMap<i64, usize> = HashMap::new();

    let mut prev_num = 0;
    for (index, elem) in input[0].split(",").enumerate() {
        prev_num = (*elem).parse().unwrap();
        nums.insert(prev_num, index+1);
    }
    nums.remove(&prev_num);

    for i in nums.len() + 1..30000000 {
        let mut res = 0;
        if let Some(pos) = nums.get(&prev_num) {
            res = i - pos;
        }
        nums.insert(prev_num, i);
        prev_num = res as i64;
        //print!("{} ", prev_num);
    }
    prev_num
}

use crate::myTest;
myTest!();
