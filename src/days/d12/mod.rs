pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

pub mod point;
use point::Point;

//even if in the end, diagonals are not used.
//it will be useful another time.
fn get_directions() -> [Point; 8] {
    [
        Point::new(1, 0),   // 0 ⬆️ north
        Point::new(1, 1),   // 1 ↗️ north east
        Point::new(0, 1),   // 2 ➡️ east
        Point::new(-1, 1),  // 3 ↘️ south east
        Point::new(-1, 0),  // 4 ⬇️ south
        Point::new(-1, -1), // 5 ↙️ south west
        Point::new(0, -1),  // 6 ⬅️ west
        Point::new(1, -1),  // 7 ↖️ north east
    ]
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let directions = get_directions();

    let mut pos = Point { x: 0, y: 0 };
    let mut orientation: i64 = 2;

    for l in input {
        let (inst, num) = l.split_at(1);
        let num_action: i64 = num.parse().unwrap();
        match inst.chars().next() {
            Some('N') => pos += directions[0] * num_action,
            Some('S') => pos += directions[4] * num_action,
            Some('E') => pos += directions[2] * num_action,
            Some('W') => pos += directions[6] * num_action,
            Some('L') => orientation -= num_action / 45,
            Some('R') => orientation += num_action / 45,
            Some('F') => pos += directions[orientation.rem_euclid(8) as usize] * num_action,
            _ => {
                eprintln!("invalid instruction: '{}'", l)
            }
        };
        //println!("isnt: {}, pos: {} {}", l, pos.x, pos.y);
    }
    //println!("pos: {} {}", pos.x, pos.y);
    (pos.x.abs() + pos.y.abs()) as i64
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let directions = get_directions();

    let mut pos = Point { x: 0, y: 0 };
    let mut waypoint = Point { x: 10, y: 1 };
    //let mut orientation: i32 = 2;

    for l in input {
        let (inst, num) = l.split_at(1);
        let num_action: i64 = num.parse().unwrap();
        match inst.chars().next() {
            Some('N') => waypoint += directions[0] * num_action,
            Some('S') => waypoint += directions[4] * num_action,
            Some('E') => waypoint += directions[2] * num_action,
            Some('W') => waypoint += directions[6] * num_action,
            Some('L') => waypoint.rotate(360 - num_action),
            Some('R') => waypoint.rotate(num_action),
            Some('F') => pos += waypoint * num_action,
            _ => {
                eprintln!("invalid instruction: '{}'", l)
            }
        };
        //println!(
        //    "inst: {}, waypoint: {} {}, pos: {} {}",
        //    l, waypoint.x, waypoint.y, pos.x, pos.y
        //);
    }
    //println!("pos: {} {}", pos.x, pos.y);
    (pos.x.abs() + pos.y.abs()) as i64
}

use crate::myTest;
myTest!();
