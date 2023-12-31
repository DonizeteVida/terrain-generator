mod generator;
mod land;
mod terrain;

mod cellular_automata_cave;

use cellular_automata_cave::CellularAutomataCave;
use generator::Generator;
use land::Land;

static WIDTH: u32 = 600;
static HEIGHT: u32 = 100;

fn main() {
    let terrains = CellularAutomataCave::terrains();
    let mut land = Land::new(WIDTH, HEIGHT, terrains);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let random = rand::random::<usize>();
            let terrain = &terrains[random % terrains.len()];
            land += (x, y, terrain)
        }
    }

    let mut generator = CellularAutomataCave::new(&mut land);

    loop {
        if !generator.iteration() {
            break;
        }
    }

    draw(&land)
}

fn draw(land: &Land) {
    use image::{ImageBuffer, Rgba};
    let image = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        let terrain = land[(x, y)];
        Rgba([terrain.r, terrain.g, terrain.b, terrain.a])
    });
    image.save("image.png").unwrap()
}
