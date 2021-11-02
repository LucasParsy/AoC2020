pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

use std::str::FromStr;
use std::{
    num::ParseIntError,
    ops::{Index, IndexMut},
};

mod parser;
use crate::days::d18::parser::parser_methods::expression_parse;
use crate::myTest;

pub enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn from_char(s: char) -> Result<Self, usize> {
        match s {
            '+' => Ok(Operator::Add),
            '*' => Ok(Operator::Mul),
            _ => Err(1),
        }
    }
}

pub enum Term {
    Num(usize),
    Expr(Expression),
}

impl FromStr for Term {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<usize>() {
            Ok(v) => Ok(Term::Num(v)),
            Err(v) => Err(v),
        }
    }
}

pub struct Expression {
    terms: Vec<Term>,
    operators: Vec<Operator>,
}

impl Expression {
    pub fn calculate(&mut self) -> usize {
        let mut res = 0;
        //self.terms.push(Term::Num(1));
        for (index, term) in self.terms.iter_mut().enumerate() {
            let val = match term {
                Term::Num(n) => *n,
                Term::Expr(e) => e.calculate(),
            };
            match self.operators.index(index) {
                Operator::Add => res += val,
                Operator::Mul => res *= val,
            }
        }
        res
    }

    pub fn calculate_p2(&mut self) -> usize {
        //self.terms.push(Term::Num(1));
        for index in (0..self.terms.len()).rev() {
            let v_ind = match self.terms.index_mut(index) {
                Term::Num(n) => *n,
                Term::Expr(e) => e.calculate_p2(),
            };
            *self.terms.index_mut(index) = Term::Num(v_ind);
            if index == (self.terms.len() - 1) {
                continue;
            }
            if let Operator::Add = self.operators.index(index + 1) {
                if let Term::Num(v_prev) = self.terms.index(index + 1) {
                    *self.terms.index_mut(index) = Term::Num(v_ind + v_prev);
                    self.terms.remove(index + 1);
                    self.operators.remove(index + 1);
                }
            }
        }
        self.calculate()
    }
}

fn execute_part(input: &[String], is_p1: bool) -> i64 {
    let mut res = 0;
    for my_line in input.iter() {
        match expression_parse(my_line) {
            Ok((_, mut ex)) => {
                let val = match is_p1 {
                    true => ex.calculate(),
                    false => ex.calculate_p2(),
                };
                res += val;
            }
            Err(e) => {
                eprintln!("error parsing line {} : {}", my_line, e);
                return 0;
            }
        }
    }
    res as i64
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    execute_part(input, true)
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    execute_part(input, false)
}

myTest!();
