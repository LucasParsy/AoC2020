use std::{collections::HashMap, convert::TryInto, slice::Iter};

use crate::myTest;

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

fn check_char_id_bit(c: char, index: usize) -> usize {
    let power = index.try_into().unwrap();
    match c {
        '#' => 2_usize.pow(power),
        _ => 0,
    }
}

fn get_horizontal_side(line: &str) -> usize {
    let mut id = 0;
    for (index, c) in line.chars().enumerate() {
        id += check_char_id_bit(c, index);
    }
    id
}

fn check_line_ids(line: &str, east: &mut usize, west: &mut usize, index: usize) {
    let ce = line.chars().next().unwrap();
    let cw = line.chars().nth(9).unwrap();
    *east += check_char_id_bit(ce, index);
    *west += check_char_id_bit(cw, index);
}

fn parse_picture(lines: &mut Iter<String>, map: &mut HashMap<i64, Vec<usize>>) {
    let line = lines.next().unwrap();
    let num = line[5..9].to_string().parse::<i64>().unwrap();

    let mut ids = Vec::new();
    let mut east = 0;
    let mut west = 0;

    let line = lines.next().unwrap();
    ids.push(get_horizontal_side(line));
    check_line_ids(line, &mut east, &mut west, 0);
    for i in 1..=8 {
        let line = lines.next().unwrap();
        check_line_ids(line, &mut east, &mut west, i);
    }
    let line = lines.next().unwrap();
    let south = get_horizontal_side(line);
    check_line_ids(line, &mut east, &mut west, 9);
    ids.push(west);
    ids.push(south);
    ids.push(east);
    let mut inverts: Vec<usize> = ids.iter().map(|&x| reverse_bits(x)).collect();
    ids.append(&mut inverts);

    //println!("{} {:?}", num, ids);

    map.insert(num, ids);
}

fn reverse_bits(num: usize) -> usize {
    let mut res = 0;
    for i in 0..=9 {
       let p = 2_usize.pow(i);
        if ( num & p) != 0 {
            res += 2_usize.pow(9-i);
        }
    }
    res
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let mut lines = input.iter();
    let mut map = HashMap::<i64, Vec<usize>>::new();
    loop {
        parse_picture(&mut lines, &mut map);
        if lines.next().is_none() {
            break;
        }
    }
    let mut corners: i64 = 1;
    let iter = map.iter();

    for (id, nums) in iter {
        let mut num_corners = 0;
        for (other_id, other_nums) in map.iter() {
            if other_id == id {
                continue;
            }
            for num in nums.iter() {
                if other_nums.contains(&num) {
                    num_corners += 1;
                    break;
                }
            }
            if num_corners > 2 {
                break;
            }
        }

        if num_corners == 2 {
            //println!("corner {}", id);
            corners *= id;
        }
    }
    //println!("reverse 183: {}", reverse_bits(183));
    //println!("reverse 300: {}", reverse_bits(300));

    corners
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    for _line in input.iter() {}
    2
}

myTest!();
