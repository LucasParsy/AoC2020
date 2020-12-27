pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

extern crate array_tool;
use array_tool::vec::{Intersect,Uniq};

fn unique_answers(t: &str) -> usize {
    t.to_string().into_bytes().unique().len()
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let mut st: String = String::new();
    let mut res = 0;
    for l in input.iter() {
        if l.is_empty() {
            res += unique_answers(&st);
            st.clear();
        } else {
            st += l;
        }
    }
    res += unique_answers(&st);
    res as i64
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let mut st: Vec<u8> = Vec::new();
    let mut res = 0;
    let mut first_elem = true;
    for l in input.iter() {
        if l.is_empty() {
            res += st.len();
            //println!("{:#?}", st);
            st.clear();
            first_elem = true;
        } else if first_elem {
            first_elem = false;
            st = l.clone().into_bytes();
        } else {
            st = st.intersect(l.clone().into_bytes());
        }
    }
    //println!("{:#?}", st);
    (res + st.len()) as i64
}

use crate::myTest;
myTest!();