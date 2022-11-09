pub mod map_parser;
pub mod piece;

use grid::Grid;
use piece::Piece;

use crate::myTest;

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);
pub static ISP2: bool = true;

fn find_corners_piece<'a>(puzzle: &'a Vec<Piece>, piece: &Piece) -> Vec<(usize, &'a Piece)> {
    let mut corners: Vec<(usize, &'a Piece)> = Vec::new();

    for other_piece in puzzle.iter() {
        if other_piece.num == piece.num {
            continue;
        }
        for num in piece.ids.iter() {
            if other_piece.ids.contains(num) {
                corners.push((*num, other_piece));
                break;
            }
        }
        if corners.len() > 2 {
            break;
        }
    }
    corners
}

fn first_corner_rotation(piece: &Piece, corners: Vec<(usize, &Piece)>) -> (u8, bool) {
    let mut rotation: u8;
    let mut pos = Vec::new();
    pos.push(position_in_arr(&piece.ids, corners[0].0));
    pos.push(position_in_arr(&piece.ids, corners[1].0));
    let flipped = pos[0] > 3;
    pos = pos.iter().map(|f| f % 4).collect();
    pos.sort();
    if pos[0] == 0 && pos[1] == 3 {
        rotation = 0;
    } else {
        rotation = pos[1] as u8;
    }
    rotation = (rotation + 2) % 4;

    //println!("fc, rotation {}, corners {:?}", rotation, pos);
    //println!("{:?} {:?} {:?}", piece, corners[0].1, corners[1].1);

    (rotation, flipped)
}

fn _get_opposite_side(rotation: u8, index: usize, flipped: bool) -> usize {
    let flip_num = if flipped { 4 } else { 0 };
    ((index + usize::from(rotation) + 2) % 4) + flip_num
}

fn rotate_grid<T: Copy>(grid: &Grid<T>, rotation: u8, flipped: bool) -> Grid<T> {
    let mut res: Grid<T> = grid::grid![];
    let gsize = grid.cols();
    for index in 0..gsize {
        let mut x = index;
        if rotation == 3 || rotation == 2 {
            x = gsize - index - 1;
        }
        let it = match rotation {
            1 | 3 => grid.iter_col(x),
            _ => grid.iter_row(x).step_by(1),
        };
        let should_flip = flipped ^ (rotation == 1 || rotation == 2);
        let it: Box<dyn Iterator<Item = &T>> = match should_flip {
            true => Box::new(it.rev()),
            false => Box::new(it),
        };
        res.push_row(it.copied().collect());
    }
    res
}

fn _test_rotation_grid(piece: &mut Piece) {
    println!("grid rotation test");
    println!("{:?} id: {}, {:?}", piece, 0, piece.ids);

    let initial_grid = piece.grid.clone();

    println!("{:?}", piece);
    for i in 1..=8 {
        println!("rotation {}", i);
        piece.grid = initial_grid.clone();
        piece.grid = rotate_grid(&piece.grid, i % 4, i >= 4);
        println!("{:?} {}", piece, piece.ids[usize::from(i % 8)]);
    }
}

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let puzzle = map_parser::parse_map(input);
    //find corners fast and easy way
    let mut res: i64 = 1;
    let puzzle_it = puzzle.iter();
    for piece in puzzle_it {
        if find_corners_piece(&puzzle, &piece).len() == 2 {
            res *= piece.num;
        }
    }
    res
}

fn position_in_arr<T: std::cmp::PartialEq>(arr: &Vec<T>, val: T) -> usize {
    arr.iter().position(|x| val == *x).unwrap()
}

fn get_first_piece(puzzle: &mut Vec<Piece>) -> Option<Piece> {
    let puzzle_it = puzzle.iter();

    let mut first_piece: Piece;
    
    let mut test_num_corner = 0;

    for (index, piece) in puzzle_it.enumerate() {
        let corners = find_corners_piece(&puzzle, &piece);
        if corners.len() == 2 {
            if test_num_corner != 0 {
                test_num_corner += 1;
                continue;
            }
            //println!("corners first piece {:?}", corners);
            let (rotation, flipped) = first_corner_rotation(piece, corners);
            first_piece = puzzle.remove(index);
            //test_rotation_grid(& mut first_piece);
            first_piece.grid = rotate_grid(&first_piece.grid, rotation, !flipped);
            first_piece.orientation = rotation;
            first_piece.north_pos = rotation  as usize;
            first_piece.flipped = flipped;
            return Some(first_piece);
        }
    }
    None
}

fn get_adjacent_neighbourgh(
    prev_neigh: &Piece,
    puzzle: &mut Vec<Piece>,
    angle: u8,
) -> Option<Piece> {
    let mut prev_id_index = (angle + prev_neigh.north_pos as u8) % 4;
    if prev_neigh.flipped {
        prev_id_index += 4;
    }
    let id = prev_neigh.ids[usize::from(prev_id_index)];
    //println!(
    //    "piece, {:?}, id to find: {},  ids {:?}",
    //    prev_neigh, id, prev_neigh.ids
    //);

    let puzzle_it = puzzle.iter();
    for (index, piece) in puzzle_it.enumerate() {
        if piece.ids.contains(&id) {
            //println!("found piece with id {}", piece.num);
            let mut piece = puzzle.remove(index);
            let id_index = piece.ids.iter().position(|&r| r == id).unwrap();
            let flipped = id_index > 3;
            let orientation = (id_index as u8 + angle) % 4;
            println!("{}, flipped: {:?}, id_index {}, angle {}, {:?}", piece.num, flipped, id_index, angle, piece);
            let rotation = match flipped
            {
                false => (id_index + angle as usize) % 4,
                true => 4 + (id_index - angle as usize) % 4,
            };
            piece.grid = rotate_grid(&piece.grid, rotation as u8, !flipped);
            piece.orientation = orientation;
            piece.north_pos = (angle as usize + id_index) % 4;
            piece.flipped = flipped;
            return Some(piece);
        }
    }
    println!("didnt found any piec with id {}", id);
    // I have lost all credibility as a programmer with this line
    return get_adjacent_neighbourgh(prev_neigh, puzzle, angle +2);
}

fn fill_temp_puzzle_grid_lines(lines: &mut Vec<Vec<bool>>, piece: &Piece) {
    let lines_size = lines.len();
    for l in 0..lines_size {
        let row: &mut Vec<bool> = &mut piece.grid.iter_row(lines_size - l - 1).cloned().collect();
        //println!("new row: {:?}", row);
        lines[l].append(row);
    }
}

fn find_monster(grid: grid::Grid<bool>) -> i64
{
    let p = Piece {
        num: 0,
        ids: vec![],
        orientation: 0,
        flipped: false,
        north_pos: 0,
        pos: (0,0),
        grid,
    };
    println!("{:?}", p);
    2
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let mut puzzle = map_parser::parse_map(input);
    let square_size = f64::from(puzzle.len() as u16).sqrt() as usize;
    let first_piece = get_first_piece(&mut puzzle).unwrap();
    let piece_size = first_piece.grid.rows();

    //println!("size square =  {} {}", square_size, puzzle.len());

    let mut current_row = 0;
    let mut pgrid: grid::Grid<bool> = grid::grid![];
    //println!("gs: {}", square_size);
    let mut top_left: Piece = first_piece.clone();
    for y in 0..square_size {
        let mut previous_neigh: Piece = top_left.clone();

        let mut lines: Vec<Vec<bool>> = vec![vec![]; piece_size];
        fill_temp_puzzle_grid_lines(&mut lines, &top_left);
        print!("{} ", top_left.num);
        let mut angle  = 1;
        if previous_neigh.flipped {
            angle = 3;
        }
        for _ in 1..square_size {
            println!("flipped: {:?} {:?}, north {}", previous_neigh.flipped, previous_neigh.ids, previous_neigh.north_pos);
            let neigh = get_adjacent_neighbourgh(&previous_neigh, &mut puzzle,angle).unwrap();
            fill_temp_puzzle_grid_lines(&mut lines, &neigh);
            previous_neigh = neigh;
            print!("{} ", previous_neigh.num);
        }
        println!("flipped: {:?} {:?}, north {}", previous_neigh.flipped, previous_neigh.ids, previous_neigh.north_pos);

        while let Some(row) = lines.pop() {
            pgrid.push_row(row);
        }
        if y != square_size - 1 {
            //println!("flipped: {:?} {:?}", previous_neigh.flipped, previous_flipped);
            top_left = get_adjacent_neighbourgh(&top_left, &mut puzzle, 2).unwrap();
            top_left.north_pos += 2;

            //if top_left.flipped && (top_left.rotation_id == 1 || top_left.rotation_id == 3)
            //{
            //    top_left.rotation_id += 2;
            //} 
            println!();
        }
    }
    find_monster(pgrid)
}

myTest!();
