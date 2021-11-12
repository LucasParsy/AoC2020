pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

use std::collections::HashMap;
use std::char;

mod parser;
use crate::days::d19::parser::parser_methods::expression_parse;
use crate::myTest;

pub struct Rule {
    pub pattern: Vec<u16>,
    pub sec_patt: Option<Vec<u16>>,
}

pub enum Expression {
    Chr(char),
    Rule(Rule),
}

struct MessageChecker {
    rules_map: HashMap<u16, Expression>,
}

impl MessageChecker {
    pub fn insert_element(&mut self, index: u16, expr: Expression) {
        self.rules_map.insert(index, expr);
    }

    pub fn check_message(&self, message: &str) -> bool {
        let r = self.rules_map.get(&0).unwrap();
        match self.check_rule(vec![message], r) {
            Ok(v) => v.iter().any(|st| st.is_empty()),
            Err(_) => false,
        }
    }

    fn check_rule<'a>(
        &self,
        message: Vec<&'a str>,
        current_rule: &Expression,
    ) -> Result<Vec<&'a str>, &'static str> {
        match current_rule {
            Expression::Chr(c) => {
                let mut responses = vec![];
                for text in message {
                    if text.get(0..).unwrap().starts_with(*c) {
                        if let Some(t) = text.get(1..) {
                            responses.push(t);
                        }
                    }
                }
                match responses.is_empty() {
                    true => Err("invalid message"),
                    false => Ok(responses),
                }
            }

            Expression::Rule(r) => {
                let mut responses = vec![];
                if let Ok(mut patterns_res) = self.check_pattern(&message, &r.pattern) {
                    responses.append(&mut patterns_res);
                }
                if let Some(sec_pattern) = &r.sec_patt {
                    if let Ok(mut patterns_res) = self.check_pattern(&message, sec_pattern) {
                        responses.append(&mut patterns_res);
                    }
                }
                match responses.is_empty() {
                    true => Err("invalid message"),
                    false => Ok(responses),
                }
            }
        }
    }

    fn check_pattern<'a>(
        &self,
        message: &[&'a str],
        pattern: &[u16],
    ) -> Result<Vec<&'a str>, &'static str> {
        let mut res = message.to_owned();
        for map_index in pattern {
            match self.check_rule(res, self.rules_map.get(map_index).unwrap()) {
                Ok(val) => res = val,
                Err(e) => return Err(e),
            }
        }
        Ok(res)
    }
}

fn solve_step(input: &[String], is_p1: bool) -> i64 {
    let mut checker = MessageChecker {
        rules_map: HashMap::new(),
    };
    let mut is_messages: bool = false;
    let mut res = 0;

    for line in input {
        if line.is_empty() {
            is_messages = true;

            if !is_p1 {
                let (_, ex) = expression_parse("8: 42 | 42 8").unwrap();
                checker.insert_element(ex.0, ex.1);
                let (_, ex) = expression_parse("11: 42 31 | 42 11 31").unwrap();
                checker.insert_element(ex.0, ex.1);
            }
            continue;
        }
        match is_messages {
            true => {
                if checker.check_message(line) {
                    res += 1;
                }
            }
            false => match expression_parse(line) {
                Ok((_, ex)) => {
                    checker.insert_element(ex.0, ex.1);
                }
                Err(e) => {
                    eprintln!("error parsing line '{}' : {}", line, e);
                    return 0;
                }
            },
        }
    }
    res
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    solve_step(input, true)
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    solve_step(input, false)
}

myTest!();
