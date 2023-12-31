use std::{ops::{Index, AddAssign}, collections::HashMap};

use crate::terrain::Terrain;

#[derive(Default)]
pub struct Land {
    pub width: u32,
    pub height: u32,
    terrains: HashMap<&'static str, &'static Terrain>,
    data: HashMap<u64, &'static Terrain>
}

impl Land {
    pub fn new(width: u32, height: u32, terrains: &'static [Terrain]) -> Land {
        Land {
            width,
            height,
            terrains: terrains.into_iter().map(|t| (t.name, t)).collect(),
            data: HashMap::with_capacity(width as usize * height as usize)
        }
    }
}

impl AddAssign<(u32, u32, &'static Terrain)> for Land {
    fn add_assign(&mut self, rhs: (u32, u32, &'static Terrain)) {
        let (x, y, terrain) = rhs;
        let index = (x as u64) << 32 | (y as u64);
        self.data.insert(index, terrain);
    }
}

impl Index<(u32, u32)> for Land {
    type Output = &'static Terrain;

    fn index(
        &self, 
        index: (u32, u32)
    ) -> &Self::Output {
        let (x, y) = index;
        let index = (x as u64) << 32 | (y as u64);
        &self.data[&index]
    }
}

impl Index<&'static str> for Land {
    type Output = &'static Terrain;

    fn index(
        &self, 
        index: &'static str
    ) -> &Self::Output {
        &self.terrains[index]
    }
}