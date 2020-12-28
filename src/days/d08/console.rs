use std::fmt;

#[derive(PartialEq, Copy, Clone)]
enum Inst {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone)]
pub struct InstructionError<'a>(usize, &'a str);

impl fmt::Display for InstructionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid instruction '{}' at line {}", self.1, self.0)
    }
}

pub struct Console {
    code: Vec<(Inst, i64)>,
    acc: i64,
    pointer: i64,
}

impl Console {
    pub fn solve_step_2(&mut self) -> i64 {
        for index in 0..self.code.len() {
            let original = self.code[index].0;
            self.code[index].0 = match self.code[index].0 {
                Inst::Jmp => Inst::Nop,
                Inst::Nop => Inst::Jmp,
                Inst::Acc => Inst::Acc,
            };
            if original != Inst::Acc {
                self.acc = 0;
                self.pointer = 0;
                if let Ok(res) = self.solve_step1() {
                    return res;
                }
                self.code[index].0 = original;
            }
        }
        0
    }

    pub fn solve_step1(&mut self) -> Result<i64, i64> {
        let mut pointers = Vec::new();
        let mut previous_acc = self.acc;
        loop {
            if let Some(res) = self.step() {
                return Ok(res);
            }
            if pointers.contains(&self.pointer) {
                break;
            }
            pointers.push(self.pointer);
            previous_acc = self.acc;
        }
        Err(previous_acc)
    }

    pub fn step(&mut self) -> Option<i64> {
        if self.pointer as usize == self.code.len() {
            return Some(self.acc);
        }
        let line = &self.code[self.pointer as usize];
        match line.0 {
            Inst::Nop => (),
            Inst::Acc => {
                self.acc += line.1;
            }
            Inst::Jmp => {
                self.pointer += line.1 - 1;
            }
        }
        self.pointer += 1;
        None
    }

    pub fn new(input: &[String]) -> Result<Console, InstructionError> {
        let mut code = Vec::new();
        for (num_l, line) in input.iter().enumerate() {
            let mut split = line.split(' ');
            let inst = match split.next() {
                Some("nop") => Inst::Nop,
                Some("acc") => Inst::Acc,
                Some("jmp") => Inst::Jmp,
                _ => {
                    return Err(InstructionError(num_l + 1, line));
                }
            };
            match split.next().unwrap_or("err").parse::<i64>() {
                Ok(num) => code.push((inst, num)),
                _ => {
                    return Err(InstructionError(num_l + 1, line));
                }
            };
        }
        Ok(Console {
            code,
            acc: 0,
            pointer: 0,
        })
    }
}
