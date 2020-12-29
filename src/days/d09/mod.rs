pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

extern crate itertools;
use itertools::Itertools;
use std::cmp::{Ordering, min, max};

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let nums: Vec<u64> = input.iter().map(|f| f.parse::<u64>().unwrap()).collect();
    let preamble = if nums.len() < 25 { 5 } else { 25 }; // unit test, for once, are peculiar
    let mut index = preamble;
    loop {
        if !nums[index - preamble..index]
            .iter()
            .combinations(2)
            .any(|t| t[0] + t[1] == nums[index])
        {
            break;
        }
        index += 1;
    }
    nums[index] as i64
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let to_find = p1(input, false) as u64;
    let nums: Vec<u64> = input.iter().map(|f| f.parse::<u64>().unwrap()).collect();
    for i in 0..nums.len()
    {
        let mut sum = 0;
        let mut n_min = 10000000000; 
        let mut n_max = 0; 
        for c  in nums.iter().skip(i)
        {
            n_min = min(n_min, *c);
            n_max = max(n_max, *c);
            sum += c;
            match sum.cmp(&to_find) {
                Ordering::Less => (),
                Ordering::Equal => return (n_min + n_max) as i64,
                Ordering::Greater => continue
            };
            
        }
    }
    0
}

use crate::myTest;
myTest!();