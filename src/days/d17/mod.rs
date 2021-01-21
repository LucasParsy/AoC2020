pub mod render_3d;
pub mod mesh_animations;

pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (true, false);

type World = [Vec<Vec<Vec<(bool, bool)>>>];
type WorldVec = Vec<Vec<Vec<Vec<(bool, bool)>>>>;

fn _debug_print_world(world: &World, levels: usize, w_levels: usize) {
    println!("step {}\n", levels);
    let w_levels = w_levels as i8;
    let levels = levels as i8;

    for w in -w_levels..=w_levels {
        for z in -levels..=levels {
            println!("\nz= {} w= {}", z, w);
            let level = &world[w.abs() as usize][z.abs() as usize];
            for (y, line) in level.iter().enumerate() {
                print!("{:02} ", y);
                for elem in line {
                    let c = if elem.0 { '#' } else { '.' };
                    print!("{}", c);
                }
                println!();
            }
        }
    }
    println!("\n");
}

fn count_active_3d(world: &[Vec<Vec<(bool, bool)>>]) -> usize {
    let z_zero = world.first().unwrap();

    let orig = z_zero.iter().flatten().filter(|e| e.0).count();
    orig + world
        .iter()
        .skip(1)
        .flatten()
        .flatten()
        .filter(|e| e.0)
        .count()
        * 2
}

pub fn count_active(world: &World) -> i64 {
    let orig = count_active_3d(&world.first().unwrap());
    (orig
        + world
            .iter()
            .skip(1)
            .map(|e| -> usize { count_active_3d(e) })
            .sum::<usize>()
            * 2) as i64
}

fn gen_depth_range(level: usize, v: usize, is_p1_w: bool) -> Vec<usize> {
    let mut l_range = vec![v];
    if is_p1_w {
        return l_range;
    }
    if v != 0 {
        l_range.push(v - 1);
    }
    if v < level - 1 {
        l_range.push(v + 1);
    }
    l_range
}

fn check_cell(
    x: usize,
    y: usize,
    z: usize,
    w: usize,
    level: usize,
    is_p1: bool,
    world: &mut World,
) {
    let z_range = gen_depth_range(level, z, false);
    let w_range = gen_depth_range(level, w, is_p1);

    let is_active = world[w][z][y][x].0;
    let mut neighbors = 0;

    for dw in w_range {
        for dz in z_range.iter() {
            for dy in y - 1..=y + 1 {
                for dx in x - 1..=x + 1 {
                    if dx == x && dy == y && *dz == z && dw == w {
                        continue;
                    }
                    if world[dw][*dz][dy][dx].0 {
                        let mut res = 1;
                        if z == 0 && *dz == 1 {
                            res *= 2;
                        }
                        if w == 0 && dw == 1 {
                            res *= 2;
                        }

                        neighbors += res;
                        if neighbors >= 4 {
                            break;
                        }
                    }
                }
            }
        }
    }
    if !is_active && neighbors == 3 || is_active && (neighbors != 2 && neighbors != 3) {
        world[w][z][y][x].1 = true;
    }
}

pub fn update_cells(world: &mut World) {
    for elem in world
        .iter_mut()
        .flatten()
        .flatten()
        .flatten()
        .filter(|e| e.1)
    {
        *elem = (!elem.0, false);
    }
}

fn initialize_2d_world(start: usize, input: &[String], m: &mut Vec<Vec<(bool, bool)>>) {
    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                m[start + y][start + x].0 = true;
            }
        }
    }
}

pub fn init_world(
    input: &[String],
    is_p1: bool,
    cycles: usize,
) -> (WorldVec, usize, usize) {
    let sx = input[0].len();
    let sy = input.len();
    assert_eq!(sx, sy);

    let padding = (cycles + 1) * 2 + 1;
    let map_size = sx + padding;
    let flat_slice = vec![vec![(false, false); map_size]; map_size];
    let three_d_world = vec![flat_slice; cycles + 1];
    let world_w_size = if is_p1 { 1 } else { cycles + 1 };
    let mut world = vec![three_d_world; world_w_size];

    let start = cycles + 1;
    initialize_2d_world(start, input, &mut world[0][0]);
    //_debug_print_world(&world, 0, 0);
    (world, start, sx)
}

pub fn step_life(world: &mut World, level: usize, start: usize, map_size: usize, is_p1: bool) {
    let w_range = if is_p1 { 0 } else { level };
    for w in 0..=w_range {
        for z in 0..=level {
            for y in start..=start + map_size + (level * 2) {
                for x in start..=start + map_size + (level * 2) {
                    check_cell(x, y, z, w, level, is_p1, world);
                }
            }
        }
    }
}

fn solve_p1_and_p2(input: &[String], is_p1: bool, interactive: bool) -> i64 {
    if interactive && is_p1 {
        if !cfg!(feature = "render_3d") {
            eprintln!("feature 'render_3d' was disabled, cannot do 3D render");
        }
        #[cfg(feature = "render_3d")]
        return render_3d::render::start(input);
    }
    let cycles = 6;
    let (mut world, mut start, map_size) = init_world(input, is_p1, cycles);
    for level in 1..=cycles {
        start -= 1;
        step_life(&mut world, level, start, map_size, is_p1);
        update_cells(&mut world);
        //_debug_print_world(&world, level, if is_p1 { 0 } else { level });
    }
    count_active(&world)
}

pub fn p1(input: &[String], interactive: bool) -> i64 {
    solve_p1_and_p2(input, true, interactive)
}

pub fn p2(input: &[String], interactive: bool) -> i64 {
    solve_p1_and_p2(input, false, interactive)
}

use crate::myTest;
myTest!();
