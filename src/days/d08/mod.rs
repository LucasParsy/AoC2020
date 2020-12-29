pub mod console;
use console::Console;

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let mut console = match Console::new(input) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("{}", err);
            return 0;
        }
    };
    console.solve_step1().unwrap_err()
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let mut console = match Console::new(input) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("{}", err);
            return 0;
        }
    };
    console.solve_step_2()
}

use crate::myTest;
myTest!();