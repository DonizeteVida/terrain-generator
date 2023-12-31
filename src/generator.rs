use crate::{land::Land, terrain::Terrain};

pub fn neighbors(
    x: u32,
    y: u32,
    length: i32,
    land: &Land,
) -> Vec<&'static Terrain> {
    use std::cmp::{max, min};

    let start_x = max(x as i32 - length, 0) as u32;
    let start_y = max(y as i32 - length, 0) as u32;
    let end_x = min(x as i32 + length, land.width as i32 - 1) as u32;
    let end_y = min(y as i32 + length, land.height as i32 - 1) as u32;

    let mut neighbors = vec![];

    for i in start_y..end_y + 1 {
        for j in start_x..end_x + 1 {
            if i == y && j == x {
                continue;
            }
            neighbors.push(land[(j, i)]);
        }
    }

    neighbors
}

pub trait Generator<'a> {
    fn terrains() -> &'static [Terrain];

    fn new(land: &'a mut Land) -> Self;

    fn iteration(
        &mut self
    ) -> bool;
}
