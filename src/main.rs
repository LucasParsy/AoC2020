extern crate indexmap;
use indexmap::IndexMap;

#[macro_use]
extern crate clap;
use clap::App;

use days::common::file_to_lines;

mod days {
    pub mod common;
    aoc_2020::gen_pub_mod!(); //pub mod d01...d25
}

type ExoMethod = fn(_input: &[String], _interactive: bool) -> i64;

struct DayInfo {
    p1: ExoMethod,
    p2: ExoMethod,
    interactive: (bool, bool),
    filename: String,
}

type ModuleMap = IndexMap<u8, DayInfo>;

fn map_modules() -> ModuleMap {
    let mut res: ModuleMap = IndexMap::new();
    aoc_2020::gen_module_map!();
    res
}

fn call_all(m: &ModuleMap) {
    for t in m.iter() {
        if let Ok(lines) = file_to_lines(&t.1.filename) {
            let res = (t.1.p1)(&lines, false);
            println!("day {:02} part 1: {}", t.0, res);
            let res = (t.1.p2)(&lines, false);
            println!("day {:02} part 2: {}", t.0, res);
        }
    }
}

fn call_precise_exercise(m: &ModuleMap, day: u8, part: u8) {
    match m.get(&day) {
        Some(d) => {
            if let Ok(lines) = file_to_lines(&d.filename) {
                let method = if part == 1 { d.p1 } else { d.p2 };
                let res = method(&lines, true);
                println!("day {:02} part {}: {}", day, part, res)
            }
        }
        None => {
            eprintln!("this day is invalid or not implemented");
        }
    }
}

fn list_exercises(m: &ModuleMap) {
    for t in m.keys().into_iter() {
        println!("day {:02}", *t);
    }
}

fn list_interactive(m: &ModuleMap) {
    for t in m.into_iter() {
        if t.1.interactive.0 || t.1.interactive.1 {
            println!(
                "day {:02} p1:{} p2:{}",
                *t.0, t.1.interactive.0, t.1.interactive.1
            );
        }
    }
}

fn main() {
    let modules = map_modules();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    match matches.subcommand_name() {
        Some("all") => call_all(&modules),
        Some("list") => list_exercises(&modules),
        Some("interactives") => list_interactive(&modules),
        Some("day") => {
            let sub = matches.subcommand_matches("day").unwrap();
            let day = sub.value_of("number").unwrap();
            let day: u8 = day.parse().unwrap_or(0);
            let part = sub.value_of("part").unwrap().parse().unwrap();
            call_precise_exercise(&modules, day, part);
        }
        _ => call_all(&modules),
    }
}
