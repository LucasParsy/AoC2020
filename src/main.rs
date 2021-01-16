extern crate indexmap;
use indexmap::IndexMap;

extern crate rayon;
use std::sync::mpsc;

#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};

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

struct DayResult(u8, i64, i64);

type ModuleMap = IndexMap<u8, DayInfo>;

fn map_modules() -> ModuleMap {
    let mut res: ModuleMap = IndexMap::new();
    aoc_2020::gen_module_map!();
    res
}

fn call_day(t: (&u8, &DayInfo)) -> Option<DayResult> {
    //println!("day {:02} part 1: {:?}", t.0, &t.1.filename);
    if let Ok(lines) = file_to_lines(&t.1.filename) {
        let r1 = (t.1.p1)(&lines, false);
        let r2 = (t.1.p2)(&lines, false);
        return Some(DayResult(*t.0, r1, r2));
    }
    None
}

fn call_day_threaded(t: (&u8, &DayInfo), tx: &mpsc::Sender<DayResult>) {
    if let Some(res) = call_day(t) {
        tx.send(res).unwrap();
    }
}

fn print_day(rec: &DayResult) {
    println!("day {:02} part 1: {}", rec.0, rec.1);
    println!("day {:02} part 2: {}", rec.0, rec.2);
}

fn single_thread_call_all(m: &IndexMap<u8, DayInfo>) {
    for val in m.iter() {
        if let Some(res) = call_day(val) {
            print_day(&res);
        }
    }
}

// When running on single thread, it appears the pool executes the days in reverse order.
// so reversed the iterator when thread_num == 1
fn call_all(m: &mut IndexMap<u8, DayInfo>, thread_num: usize) {
    if thread_num == 1 {
        return single_thread_call_all(m);
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_num)
        .build()
        .unwrap();

    pool.scope(|s| {
        let (tx, rx) = mpsc::channel();

        let mut received_map: IndexMap<u8, DayResult> = IndexMap::new();
        let mut exercices_order_it = m.keys().peekable();
        for t in m.iter() {
            let n_tx = tx.clone();
            s.spawn(move |_| call_day_threaded(t, &n_tx));
        }

        //order print of days results
        for rec in rx {
            match exercices_order_it.peek() {
                Some(x) if **x == rec.0 => {
                    print_day(&rec);
                    exercices_order_it.next();
                    while let Some(n_ex) = exercices_order_it.peek() {
                        match received_map.get(*n_ex) {
                            Some(v) => {
                                print_day(v);
                                exercices_order_it.next();
                            }
                            None => break,
                        }
                    }
                }
                Some(_) => {
                    received_map.insert(rec.0, rec);
                }
                None => {}
            }
            if exercices_order_it.peek() == None {
                break;
            }
        }
    });
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

fn get_num_treads(matches: &ArgMatches) -> usize {
    match matches.value_of("threads") {
        Some(threads_str) => match threads_str.parse::<usize>() {
            Ok(res) => res,
            Err(_) => {
                eprintln!(
                    "invalid thread number '{}', using 1 thread instead",
                    threads_str
                );
                1
            }
        },
        None => 1,
    }
}

fn main() {
    let mut modules = map_modules();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let threads = get_num_treads(&matches);

    match matches.subcommand_name() {
        Some("all") => call_all(&mut modules, threads),
        Some("list") => list_exercises(&modules),
        Some("interactives") => list_interactive(&modules),
        Some("day") => {
            let sub = matches.subcommand_matches("day").unwrap();
            let day = sub.value_of("number").unwrap();
            let day: u8 = day.parse().unwrap_or(0);
            let part = sub.value_of("part").unwrap().parse().unwrap();
            call_precise_exercise(&modules, day, part);
        }
        _ => call_all(&mut modules, threads),
    }
}
