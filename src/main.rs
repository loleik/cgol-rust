use std::{thread, time};
use clap::{arg, Command};
use std::collections::{HashMap, HashSet};
use regex::Regex;

// Offsets used for checking neighbors.
const OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1),  (0, 1),  (1, 1)
];

// Struct for tracking the world. Uses double buffering to keep with the original rules.
#[derive(Clone)]
struct World {
    height: usize, width: usize, // Dimensions of the viewport specifically
    current: HashSet<(isize, isize)>,
    next: HashSet<(isize, isize)>,
    neighbors: HashMap<(isize, isize), usize>
}

impl World {
    fn new(
        height: usize, width: usize,
        map1: HashSet<(isize, isize)>,
        map2: HashSet<(isize, isize)>
    ) -> World {
        return World { 
            height: height, 
            width: width, 
            current: map1, 
            next: map2,
            neighbors: HashMap::new()
        }
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
                .about("Run the simulation with a specified pattern")
                .arg(arg!(<PATTERN> "Starting pattern e.g.[(0, 1),(1, 1),(2, 1)]"))
                .arg_required_else_help(true)
                .arg(arg!(<HEIGHT> "Viewport height"))
                .arg_required_else_help(true)
                .arg(arg!(<WIDTH> "Viewport width"))
                .arg_required_else_help(true)
        )
}

// Initialize a world structure using an initial pattern. Hard coded for now.
fn init(
    height: usize,
    width: usize,
    pattern: &str
) -> World {    
    let extract = Regex::new(r"\((\d+),(\d+)\)").unwrap();

    let coords: HashSet<(isize, isize)> = extract.captures_iter(pattern)
                    .map(|cap| {(
                        cap[1].parse::<isize>().unwrap(),
                        cap[2].parse::<isize>().unwrap(),
                    )}).collect();

    let world: World = World::new(height, width, coords.clone(), coords);

    world
}

// For viewing the world at each tick.
fn view(
    o_x: isize,
    o_y: isize,
    world: &World
) {
    for y in 0..world.height {
        for x in 0..world.width {
            if world.current.contains(
                &(x as isize + o_x, y as isize + o_y)
            ) {
                print!("■ ");
            } else {
                print!("□ ");
            }
        }
        println!();
    }
}

// Uses offset values to check how many neighbors each cell has.
fn check_neighbors(
    world: &World, 
    x: isize, 
    y: isize
) -> usize {
    let mut count: usize = 0;

    for (ox, oy) in OFFSETS {
        let cx: isize = x + ox;
        let cy: isize = y + oy;

        if world.current.contains(&(cx, cy)) {
            count += 1
        }
    }

    count
}

// Uses check_neighbors and acts accordingly depending on the result.
// Doesn't mutate the current world until the end of the tick, this is the double buffering mentioned earlier.
fn tick(
    world: &mut World
) {
    for &(x, y) in &world.current {
        let count: usize = check_neighbors(&world, x, y);

        for (ox, oy) in OFFSETS {
            *world.neighbors.entry((x + ox, y + oy)).or_insert(0) += 1;
        }

        match count {
            0 | 1 => { world.next.remove(&(x, y)); }, // underpopulation
            2 | 3 => {}, // unchanged
            _ => { world.next.remove(&(x, y)); } // overpopulation
        }        
    }

    // Handle reproduction
    for (&(x, y), &c) in &world.neighbors {
        if c == 3 {
            world.next.insert((x, y));
        }
    }

    world.current = world.next.clone();
    world.neighbors = HashMap::new();
}

fn main() -> Result<(), ()> {
    let matches: clap::ArgMatches = cli().get_matches();

    match matches.subcommand() {
        Some(("pattern", sub_matches)) => {
            let pattern: &String = sub_matches.get_one::<String>("PATTERN")
                    .expect("Required");

            let height: usize = sub_matches.get_one::<String>("HEIGHT")
                    .expect("Required")
                    .parse::<usize>()
                    .expect("Not an integer");

            let width: usize = sub_matches.get_one::<String>("WIDTH")
                    .expect("Required")
                    .parse::<usize>()
                    .expect("Not an integer");

            let validate = Regex::new(r"^\[\(\d+,\d+\)(?:,\(\d+,\d+\))*\]$").unwrap();

            if !validate.is_match(pattern) {
                println!("Please enter a coordinate list of the following form:");
                println!("[(x1,y1),(x2,y2),...(xn,yn)]");
                return Ok(())
            }

            let mut world: World = init(height, width, &pattern);

            for i in 0..51 {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

                let (mut min_x, mut min_y) = (isize::MAX, isize::MAX);
                let (mut max_x, mut max_y) = (isize::MIN, isize::MIN);
                
                for &(x, y) in &world.current {
                    if x < min_x { min_x = x }
                    if x > max_x { max_x = x }
                    if y < min_y { min_y = y }
                    if y > max_y { max_y = y }
                }
    
                let center: (isize, isize) = (
                    (min_x + max_x) / 2,
                    (min_y + max_y) / 2
                );
    
                let o_x: isize = center.0 - (width as isize) / 2;
                let o_y: isize = center.1 - (height as isize) / 2;
        
                view(o_x, o_y, &world);

                println!("Running: {pattern} | Tick: {i} | Pattern center: {center:?}");

                tick(&mut world);

                let wait: time::Duration = time::Duration::from_millis(250);
                thread::sleep(wait);
            }
        },
        _ => println!("Invalid subcommand: {}", matches.subcommand().unwrap().0)
    }

    Ok(())
}
