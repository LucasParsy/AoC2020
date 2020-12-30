use std::collections::HashMap;

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 1),
    (0, 1),
    (-1, 1),
    (1, 0),
    (-1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn debug_print_map(step: usize, sx: i32, sy: i32, chairs: &HashMap<(i32, i32), (bool, bool)>) {
    println!("===step {} ===", step);
    for y in 0..sy {
        for x in 0..sx {
            match chairs.get(&(y, x)) {
                None => print!("."),
                Some((false, _)) => print!("L"),
                Some((true, _)) => print!("#"),
            }
        }
        println!();
    }
    println!()
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    //return maybe_optimized_p1(input, false);

    let mut chairs: HashMap<(i32, i32), (bool, bool)> = HashMap::new();
    let mut keys: Vec<(i32, i32)> = Vec::new();

    let sx = input[0].len();
    let sy = input.len();

    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.chars().into_iter().enumerate() {
            if c == 'L' {
                let k = (y as i32, x as i32);
                chairs.insert(k, (true, false));
                keys.push(k);
            }
        }
    }

    let mut count: usize = 0;

    loop {
        //debug_print_map(count, sx as i32, sy as i32, &chairs);
        count += 1;
        for key in &keys {
            let elem = chairs.get(&key).unwrap();
            let mut adj = 0;
            for dir in DIRECTIONS.iter() {
                let npos = (key.0 + dir.0, key.1 + dir.1);
                let nchair = chairs.get(&npos);
                match nchair {
                    Some((true, _)) => {
                        adj += 1;
                        if elem.0 == false || adj == 4 {
                            break;
                        }
                    }
                    _ => (),
                };
            }
            let elem = chairs.get_mut(&key).unwrap();
            if (elem.0 == false && adj == 0) || adj == 4 {
                (*elem).1 = true;
            }
        }
        let mut modified = false;
        for elem in chairs.iter_mut() {
            if elem.1 .1 {
                *elem.1 = (!elem.1 .0, false);
                modified = true;
            }
        }
        if !modified {
            break;
        }
    }
    //debug_print_map(count, sx as i32, sy as i32, &chairs);
    chairs.iter().filter(|x| x.1 .0).count() as i64
}