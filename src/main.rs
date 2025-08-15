use std::{thread, time};
use clap::{arg, Command};
use std::collections::HashMap;

// Offsets used for checking neighbors.
const OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1),  (0, 1),  (1, 1)
];

// Struct for tracking the world. Uses double buffering to keep with the original rules.
#[derive(Clone)]
struct World {
    width: usize,
    height: usize,
    current: Vec<bool>,
    next: Vec<bool>
}

impl World {
    fn new(width: usize, height: usize, map: Vec<bool>) -> World {
        return World { width: width, height: height, current: map.clone(), next: map}
    }
}

fn cli() -> Command {
    Command::new("cgol-rust")
        .about("A rust implementation of Conway's Game of Life.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("pattern")
                .about("Specify a starting pattern from [\"copperhead\"]")
                .arg(arg!(<NAME> "The name of the pattern"))
                .arg_required_else_help(true),
        )
}


fn place_pattern(
    map: &mut Vec<bool>,
    map_width: usize,
    pattern: &Vec<bool>,
    pattern_width: usize,
    pattern_height: usize,
    offset: (usize, usize),
) {
    let (ox, oy) = offset;

    for y in 0..pattern_height {
        for x in 0..pattern_width {
            let board_x = ox + x;
            let board_y = oy + y;

            // make sure we don't go out of bounds
            if board_x < map_width && board_y < map.len() / map_width {
                let board_idx = board_y * map_width + board_x;
                let pattern_idx = y * pattern_width + x;
                map[board_idx] = pattern[pattern_idx];
            }
        }
    }
}

// Initialize a world structure using an initial pattern. Hard coded for now.
fn init(
    height: usize, 
    width: usize,
    name: &str
) -> World{
    // Hardcoded example with the start co-ordinate.
    let copperhead: (Vec<bool>, (usize, usize), usize, usize) = (vec![
        false, false, false, false, false, true, false, true, true, false, false, false,
        false, false, false, false, true, false, false, false, false, false, false, true,
        false, false, false, true, true, false, false, false, true, false, false, true,
        true, true, false, true, false, false, false, false, false, true, true, false,
        true, true, false, true, false, false, false, false, false, true, true, false,
        false, false, false, true, true, false, false, false, true, false, false, true,
        false, false, false, false, true, false, false, false, false, false, false, true,
        false, false, false, false, false, true, false, true, true, false, false, false,
    ], (1, 6), 12, 8);

    let patterns: HashMap<&'static str, (Vec<bool>, (usize, usize), usize, usize)> = HashMap::from([
        ("copperhead", copperhead)
    ]);

    let mut initial: Vec<bool> = vec![false; width * height];

    place_pattern(
        &mut initial, 
        width, 
        &patterns.get(name).unwrap().0, 
        patterns.get(name).unwrap().2, 
        patterns.get(name).unwrap().3, 
        patterns.get(name).unwrap().1
    );

    let world: World = World::new(width, height, initial);

    world
}

// For viewing the world at each tick.
fn view(
    world: &World
) {
    for y in 0..world.height {
        for x in 0..world.width {
            let i = y * world.width + x;
            print!("{}", if world.current[i] { " ■ " } else { " □ " })
        }
        println!();
    }
}

// Uses offset values to check how many neighbors each cell has.
fn check_neighbors(
    world: &World, 
    x: usize, 
    y: usize
) -> usize {
    let mut count: usize = 0;

    for (ox, oy) in OFFSETS {
        let cx: isize = x as isize + ox;
        let cy: isize = y as isize + oy;

        if cx >= 0 && cx < world.width as isize && cy >= 0 && cy < world.height as isize {
            let index: usize = cy as usize * world.width + cx as usize;

            if world.current[index] { count += 1 }
        }
    }

    count
}

// Uses check_neighbors and acts accordingly depending on the result.
// Doesn't mutate the current world until the end of the tick, this is the double buffering mentioned earlier.
fn tick(
    world: &mut World
) {
    for i in 0..world.current.len() {
        let x: usize = i % world.width;
        let y: usize = i / world.width;

        let count: usize = check_neighbors(world, x, y);

        match (world.current[i], count) {
            (true, 0) | (true, 1) => { world.next[i] = false }, // underpopulation
            (true, 2) | (true, 3) => {}, // unchanged
            (true, _) => { world.next[i] = false }, // overpopulation
            (false, 3) => { world.next[i] = true }, // reproduction
            (_, _) => {}
        }
    }

    world.current.copy_from_slice(&world.next);
}

fn main() {
    let matches: clap::ArgMatches = cli().get_matches();

    match matches.subcommand() {
        Some(("pattern", sub_matches)) => {
            let name: &String = sub_matches.get_one::<String>("NAME").expect("Required");

            let mut world: World = init(20, 40, &name);

            for i in 0..200 {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        
                view(&world);

                println!("Running: {name} | Tick: {i}");
        
                tick(&mut world);
        
                let wait: time::Duration = time::Duration::from_millis(250);
                thread::sleep(wait);
            }
        }
        _ => {},
    }
}
