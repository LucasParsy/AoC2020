use std::collections::HashMap;

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

type Passport<'a> = HashMap<&'a str, &'a str>;

fn parse_valid_first_pass_passports(input: &[String]) -> Vec<Passport> {
    let mut res: Vec<Passport> = Vec::new();
    let fields = &vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    let optional_field = fields[7];

    let mut passport: Passport = HashMap::new();
    for line in input.iter() {
        if line.is_empty() {
            if fields
                .iter()
                .all(|f| passport.contains_key(*f) || *f == optional_field)
            {
                res.push(passport.clone());
            }
            passport.clear();
        } else {
            for couple in line.split(' ') {
                let infos: Vec<&str> = couple.split(':').collect();
                passport.insert(infos[0], infos[1]);
            }
        }
    }
    res
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    parse_valid_first_pass_passports(input).len() as i64
}

fn parse_date(passport: &Passport, field: &str, min: i64, max: i64) -> Result<(), ()> {
    if let Ok(n) = passport.get(field).unwrap().parse::<i64>() {
        if n >= min && n <= max {
            return Ok(());
        }
    }
    Err(())
}

fn parse_height(passport: &Passport) -> Result<(), ()> {
    let hgt = passport.get("hgt").unwrap();
    let unit = &hgt[hgt.len() - 2..hgt.len()];
    let (min, max) = match unit {
        "cm" => (150, 193),
        "in" => (59, 76),
        _ => return Err(()),
    };
    if let Ok(n) = hgt[..hgt.len() - 2].parse::<i64>() {
        if n >= min && n <= max {
            return Ok(());
        }
    }
    Err(())
}

fn parse_hair_color(passport: &Passport) -> Result<(), ()> {
    let hcl = *passport.get("hcl").unwrap();
    let mut chars = hcl.chars();
    if hcl.len() != 7 || chars.next() != Some('#') {
        return Err(());
    }
    if !chars.all(|f| f.is_digit(16)) {
        return Err(());
    }
    Ok(())
}

fn parse_passport(pass: &Passport) -> Result<(), ()> {
    parse_date(pass, "byr", 1920, 2002)?;
    parse_date(pass, "iyr", 2010, 2020)?;
    parse_date(pass, "eyr", 2020, 2030)?;
    parse_height(pass)?;
    parse_hair_color(pass)?;

    let eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if !eye_colors.contains(pass.get("ecl").unwrap()) {
        return Err(());
    }
    let pid = pass.get("pid").unwrap();
    if pid.len() != 9 || !pid.chars().all(char::is_numeric) {
        return Err(());
    }

    Ok(())
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let passports: Vec<Passport> = parse_valid_first_pass_passports(input);
    passports
        .iter()
        .map(|f| parse_passport(f))
        .filter(|f| f.is_ok())
        .count() as i64
}

use crate::myTest;
myTest!();