use std::{thread, time};

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1),  (0, 1),  (1, 1)
];

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

// For now everything assumes a 10x10 grid. Obviously that will change later to be variable I hope.
fn init() -> World{
    // Crude definition of an initial pattern for testing.
    let initial: Vec<bool> = vec![
        false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true, false, false, false, false, false,
        false, false, false, true, true, true, false, false, false, false,
        false, false, false, false, true, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true, false, false, false, false, false,
        false, false, false, true, true, true, false, false, false, false,
        false, false, false, false, true, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
    ];

    let world = World::new(10, 10, initial);

    world
}

fn view(world: &World) {
    for y in 0..world.height {
        for x in 0..world.width {
            let i = y * world.width + x;
            print!("{}", if world.current[i] { " ■ " } else { " □ " })
        }
        println!();
    }
}

fn check_neighbors(world: &World, x: usize, y: usize) -> usize {
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

fn tick(world: &mut World) {
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
    let mut world: World = init();

    for _ in 0..12 {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        view(&world);

        tick(&mut world);

        let wait: time::Duration = time::Duration::from_secs(1);
        thread::sleep(wait);
    }
}
