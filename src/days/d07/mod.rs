use std::collections::HashMap;

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

type Bags = HashMap<String, Vec<(String, i64)>>;

fn generate_map_bags(input: &[String]) -> Bags {
    let mut bag_types: Bags = HashMap::new();

    for line in input.iter() {
        let splitted: Vec<&str> = line.split(' ').collect();
        let name: String = splitted[0..2].join(" ");
        let num_elems = (splitted.len() - 4) / 4;
        let mut children = Vec::new();
        for i in 1..=num_elems {
            let num_child: i64 = splitted[i * 4].parse().unwrap();
            let name_index = 1 + i * 4;
            let bags_child = splitted[name_index..name_index + 2].join(" ");
            children.push((bags_child, num_child));
        }
        bag_types.insert(name, children);
    }
    //println!("{:#?}", bag_types);
    bag_types
}

fn get_bags_colors<'a>(
    bags: &'a Bags,
    current: &[&String],
    visited: &'a mut Vec<&'a String>,
) -> i64 {
    let mut res = 0;
    let mut next_call = Vec::new();
    //let mut filter = |(k, _v)| {!visited.contains(k)};
    for (key, val) in bags.iter() {
        if visited.contains(&key) {
            continue;
        }
        for elem in val.iter() {
            if current.contains(&&elem.0) {
                visited.push(key);
                next_call.push(key);
                res += 1;
                break;
            }
        }
    }
    if !next_call.is_empty() {
        res += get_bags_colors(bags, &next_call, visited)
    }
    res
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let bags = generate_map_bags(input);
    let base_name: String = "shiny gold".into();
    let base = vec![&base_name];
    let mut visited = base.clone();
    get_bags_colors(&bags, &base, &mut visited)
}

fn get_nested_bags(bags: &Bags, current: &str) -> i64 {
    let mut res = 0;
    if let Some(b) = bags.get(current) {
        for elem in b.iter() {
            res += elem.1 * (get_nested_bags(bags, &elem.0) + 1);
        }
    }
    res
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let bags = generate_map_bags(input);
    get_nested_bags(&bags, "shiny gold")
}

use crate::myTest;
myTest!();