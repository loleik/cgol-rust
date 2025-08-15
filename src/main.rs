struct World {
    width: usize,
    height: usize,
    map: Vec<bool>
}

impl World {
    fn new(width: usize, height: usize, map: Vec<bool>) -> World {
        return World { width: width, height: height, map: map}
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
            if world.map[i] {
                print!(" ■ ");
            } else {
                print!(" □ ");
            }
        }
        println!();
    }
}

fn main() {
    println!("Right now this just demonstrates on a 10x10 grid with a terminal visualization.");

    let world: World = init();

    view(&world);
}
