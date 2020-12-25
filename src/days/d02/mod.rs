pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

struct PassRequirements<'a> {
    min: usize,
    max: usize,
    char: char,
    password: &'a str,
}

impl PassRequirements<'_> {
    fn new<'a>(line: &'a String) -> PassRequirements {
        let parts: Vec<&str> = line.split(' ').collect();
        assert_eq!(3, parts.len());
        let mut nums = parts[0].split('-');
        let min: usize = nums.next().unwrap().parse().unwrap();
        let max: usize = nums.next().unwrap().parse().unwrap();

        let char = parts[1].char_indices().next().unwrap().1;
        let password = parts[2];
        PassRequirements {
            min,
            max,
            char,
            password,
        }
    }
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let mut res = 0;
    for line in input.iter() {
        let pass = PassRequirements::new(line);
        let p_count = pass.password.matches(pass.char).count();
        if p_count >= pass.min && p_count <= pass.max {
            res += 1;
        }
    }
    res
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let mut res = 0;
    for line in input.iter() {
        let req = PassRequirements::new(line);
        let pass = req.password;
        let mut count = 0;
        //eprintln!("{:?}  {:?}", pass.get(req.min-1..req.min), pass.get(req.max-1..req.max));
        if pass.get(req.min-1..req.min) == Some(&req.char.to_string()) {count += 1};
        if pass.get(req.max-1..req.max) == Some(&req.char.to_string()) {count += 1};
        if count == 1
        {
            res += 1;
        }
    }
    res
}

use crate::myTest;
myTest!();