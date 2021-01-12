use itertools::Itertools;
use std::collections::HashMap;

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

fn set_bit(num: i64, bit: usize) -> i64 {
    num | (1 << bit)
}

fn unset_bit(num: i64, bit: usize) -> i64 {
    num & !(1 << bit)
}

enum Tribool {
    Set,
    Unset,
    Floating,
}

type Mask = Vec<(usize, Tribool)>;

fn parse_mask(mask: &mut Mask, line: &str) {
    mask.clear();
    let mut eq_split = line.split("= ");
    eq_split.next();
    for (index, c) in eq_split.next().unwrap().chars().rev().enumerate() {
        match c {
            '1' => mask.push((index, Tribool::Set)),
            '0' => mask.push((index, Tribool::Unset)),
            'X' => mask.push((index, Tribool::Floating)),
            _ => {
                eprintln!("invalid character '{}' in line '{}'", c, line);
            }
        }
    }
}

fn parse_operation(line: &str) -> (usize, i64) {
    let mem_num_str = &line[(line.find('[').unwrap()) + 1..line.find(']').unwrap()];
    let index: usize = mem_num_str.parse().unwrap();
    let mut eq_split = line.split("= ");
    eq_split.next();
    let num: i64 = eq_split.next().unwrap().parse().unwrap();
    (index, num)
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let mut mem: HashMap<usize, i64> = HashMap::new();
    let mut mask: Mask = Vec::new();

    for line in input {
        if line.starts_with("mask") {
            parse_mask(&mut mask, line);
        } else {
            let (index, mut num) = parse_operation(line);
            for (pos, val) in mask.iter() {
                num = match val {
                    Tribool::Set => set_bit(num, *pos),
                    Tribool::Unset => unset_bit(num, *pos),
                    Tribool::Floating => {
                        continue;
                    }
                }
            }
            //eprintln!("{} {}", index, num);
            mem.insert(index, num);
        }
    }
    mem.values().sum()
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let mut mem: HashMap<usize, i64> = HashMap::new();
    let mut mask: Mask = Vec::new();

    for line in input {
        if line.starts_with("mask") {
            parse_mask(&mut mask, line);
        } else {
            let (mut index, num) = parse_operation(line);
            let mut floating = Vec::new();
            for (pos, val) in mask.iter() {
                match val {
                    Tribool::Set => index = set_bit(index as i64, *pos) as usize,
                    Tribool::Floating => {
                        floating.push(2i64.pow(*pos as u32));
                        index = unset_bit(index as i64, *pos) as usize
                    }
                    Tribool::Unset => {
                        continue;
                    }
                };
            }

            //eprintln!("val {} original index {}", num, index);
            mem.insert(index as usize, num);
            for i in 1..=floating.len() {
                for elem in floating.iter().combinations(i) {
                    let npos = index + (elem.iter().copied().sum::<i64>()) as usize;
                    //eprintln!("val {} index {}, {:?}", num, npos, elem);
                    mem.insert(npos as usize, num);
                }
            }
            mem.insert(index, num);
        }
    }

    mem.values().sum()
}

use crate::myTest;
myTest!();
