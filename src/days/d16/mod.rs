pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

extern crate nom;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::map_res,
    sequence::tuple,
    IResult,
};

use core::slice::Iter;
use std::str::FromStr;

struct TicketInfo(String, (u64, u64), (u64, u64), Vec<u64>);

fn num_parse(input: &str) -> IResult<&str, u64> {
    map_res(digit1, u64::from_str)(input)
}

fn range_parse(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, n3) = tuple((num_parse, tag("-"), num_parse))(input)?;
    assert!(n3.0 < n3.2);
    Ok((input, (n3.0, n3.2)))
}

fn ticket_info_parser(input: &str) -> IResult<&str, TicketInfo> {
    let (input, name) = take_until(":")(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, r) = tuple((range_parse, tag(" or "), range_parse))(input)?;
    Ok((input, TicketInfo(name.into(), r.0, r.2, Vec::new())))
}

fn parse_ticket_line(line: &str) -> Vec<u64> {
    line.split(',').map(|f| f.parse::<u64>().unwrap()).collect()
}

fn get_ticket_info(input_iter: &mut Iter<String>) -> Vec<TicketInfo> {
    let mut ticket_info = Vec::new();

    loop {
        let l = input_iter.next().unwrap();
        if l.is_empty() {
            break;
        }
        let info_line = ticket_info_parser(l);
        match info_line {
            Ok(i) => ticket_info.push(i.1),
            Err(e) => {
                eprintln!("invalid line '{}', {:?}", l, e);
                panic!();
            }
        }
        //let info = ticket_info.last().unwrap();
        //println!("field name {:?}, {:?} {:?}", info.0, info.1, info.2)
    }
    ticket_info
}

fn get_my_ticket(input_iter: &mut Iter<String>) -> Vec<u64> {
    assert_eq!("your ticket:", input_iter.next().unwrap());
    parse_ticket_line(input_iter.next().unwrap())
}

fn ticket_value_is_valid(info: &TicketInfo, val: u64) -> bool {
    (val >= info.1 .0 && val <= info.1 .1) || (val >= info.2 .0 && val <= info.2 .1)
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let mut input_iter = input.iter();
    let ticket_info = get_ticket_info(&mut input_iter);
    let _my_ticket = get_my_ticket(&mut input_iter);

    input_iter.next();
    assert_eq!("nearby tickets:", input_iter.next().unwrap());

    let mut res = 0;
    for line in input_iter {
        let ticket = parse_ticket_line(line);
        for val in ticket {
            if !ticket_info
                .iter()
                .any(|info| ticket_value_is_valid(info, val))
            {
                res += val;
            }
        }
    }
    res as i64
}

fn _debug_print_ticket_infos(ticket_info: &[TicketInfo]) {
    for info in ticket_info.iter() {
        println!("{}, {:?}", info.0, info.3);
    }
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let mut input_iter = input.iter();
    let mut ticket_info = get_ticket_info(&mut input_iter);
    let my_ticket = get_my_ticket(&mut input_iter);

    input_iter.next();
    assert_eq!("nearby tickets:", input_iter.next().unwrap());

    let mut valid_tickets: Vec<Vec<u64>> = vec![Vec::new(); ticket_info.len()];

    for line in input_iter {
        let ticket = parse_ticket_line(line);
        for (index, val) in ticket.iter().enumerate() {
            if ticket_info
                .iter()
                .any(|info| ticket_value_is_valid(info, *val))
            {
                valid_tickets[index].push(*val);
            }
        }
    }

    for (index, valids) in valid_tickets.iter().enumerate() {
        for info in ticket_info.iter_mut() {
            if valids.iter().all(|v| ticket_value_is_valid(&info, *v)) {
                info.3.push(index as u64);
            }
        }
    }

    ticket_info.sort_by(|a, b| a.3.len().partial_cmp(&b.3.len()).unwrap());

    for ticket_index in 0..ticket_info.len() {
        let ticket = &ticket_info[ticket_index];
        if ticket.3.len() == 1 {
            let to_remove = ticket.3[0];
            for t2 in ticket_info.iter_mut() {
                if t2.3.len() != 1 {
                    if let Some(pos) = t2.3.iter().position(|x| *x == to_remove) {
                        t2.3.remove(pos);
                    }
                }
            }
        }
    }

    let res = ticket_info
        .iter()
        .filter(|t| t.0.starts_with("departure"))
        .fold(1, |res, t| res * my_ticket[t.3[0] as usize]);

    //debug_print_ticket_infos(&ticket_info);
    //println!("{:?}", my_ticket);

    res as i64
}

use crate::myTest;
myTest!();
