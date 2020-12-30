use std::{collections::HashMap, fmt::Display};

extern crate petgraph;
use petgraph::graph::{Graph, NodeIndex};

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
];

fn print_map(is_debug: bool, step: usize, sx: i32, sy: i32, chairs: &ChairMap, graph: &ChairGraph) {
    if !is_debug {
        return;
    }
    println!("===step {} ===", step);
    for y in 0..sy {
        for x in 0..sx {
            match chairs.get(&(y, x)) {
                None => print!("."),
                Some(index) => {
                    match &graph[*index].occupied {
                        true => print!("#"),
                        false => print!("L"),
                    };
                }
            };
        }
        println!();
    }
    println!()
}

struct Seat {
    occupied: bool,
    modified: bool,
    pos: (i32, i32),
}

impl Seat {
    fn new(pos: (i32, i32)) -> Self {
        Self {
            occupied: true,
            modified: false,
            pos,
        }
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = if self.occupied { '✗' } else { '✓' };
        write!(f, "({},{}) {}", self.pos.0, self.pos.1, c)
    }
}

type ChairMap = HashMap<(i32, i32), NodeIndex>;
type ChairGraph = Graph<Seat, (), petgraph::Undirected>;

fn check_neighbor_p1(
    index: &NodeIndex,
    dir: &(i32, i32),
    pos: &(i32, i32),
    _map_size: &(i32, i32),
    table: &mut ChairGraph,
    chairs_map: &ChairMap,
) {
    let npos = (pos.0 + dir.0, pos.1 + dir.1);
    if let Some(neigh) = chairs_map.get(&npos) {
        table.add_edge(*index, *neigh, ());
    }
}

fn check_neighbor_p2(
    index: &NodeIndex,
    dir: &(i32, i32),
    pos: &(i32, i32),
    map_size: &(i32, i32),
    table: &mut ChairGraph,
    chairs_map: &ChairMap,
) {
    let mut npos = (pos.0, pos.1);

    loop {
        npos.0 += dir.0;
        npos.1 += dir.1;
        if npos.0 < 0 || npos.0 > map_size.0 || npos.1 < 0 || npos.1 > map_size.1 {
            return;
        }
        if let Some(neigh) = chairs_map.get(&npos) {
            table.add_edge(*index, *neigh, ());
            return;
        }
    }
}

type NodeChecker =
    dyn Fn(&NodeIndex, &(i32, i32), &(i32, i32), &(i32, i32), &mut ChairGraph, &ChairMap);

fn build_graph(
    input: &[String],
    map_size: &(i32, i32),
    checker_method: &NodeChecker,
) -> (ChairGraph, ChairMap) {
    let mut table: ChairGraph = Graph::new_undirected();

    let mut chairs_map: ChairMap = HashMap::new();
    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.chars().into_iter().enumerate() {
            if c == 'L' {
                let pos = (y as i32, x as i32);
                let index = table.add_node(Seat::new(pos));

                for dir in DIRECTIONS.iter() {
                    checker_method(&index, &dir, &pos, map_size, &mut table, &chairs_map);
                }
                chairs_map.insert(pos, index);
            }
        }
    }
    (table, chairs_map)
}

fn step_life(limit_flip: usize, table: &mut ChairGraph) -> bool {
    for seat_index in table.node_indices() {
        let is_occupied = table[seat_index].occupied;
        let mut adj = 0;
        for neighborg in table.neighbors(seat_index) {
            if table[neighborg].occupied {
                adj += 1;
                if !is_occupied || adj == limit_flip {
                    break;
                }
            }
        }
        if (!is_occupied && adj == 0) || adj == limit_flip {
            let seat = &mut table[seat_index];
            seat.modified = true;
        }
    }

    let mut modified = false;
    for seat in table.node_weights_mut() {
        if seat.modified {
            seat.occupied = !seat.occupied;
            seat.modified = false;
            modified = true;
        }
    }
    modified
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let sx = input[0].len() as i32;
    let sy = input.len() as i32;
    let is_debug = false;

    let (mut table, chairs_map) = build_graph(input, &(sx, sy), &check_neighbor_p1);
    let mut count: usize = 0;
    print_map(is_debug, count, sx, sy, &chairs_map, &table);
    while step_life(4, &mut table) {
        count += 1;
        print_map(is_debug, count, sx, sy, &chairs_map, &table);
    }
    print_map(is_debug, count, sx, sy, &chairs_map, &table);
    table.node_weights_mut().filter(|x| x.occupied).count() as i64
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let map_size = (input[0].len() as i32, input.len() as i32);
    let (mut table, _) = build_graph(input, &map_size, &check_neighbor_p2);
    while step_life(5, &mut table) {}
    table.node_weights_mut().filter(|x| x.occupied).count() as i64
}

use crate::myTest;
myTest!();
