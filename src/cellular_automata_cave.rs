use crate::{generator::{Generator, neighbors}, land::Land, terrain::Terrain};

pub struct CellularAutomataCave<'a> {
    land: &'a mut Land,
    round: u8,
}

impl<'a> Generator<'a> for CellularAutomataCave<'a> {
    fn terrains() -> &'static [Terrain] {
        &[
            Terrain {
                name: "Passage",
                r: 0xFF,
                g: 0,
                b: 0,
                a: 0xFF,
            },
            Terrain {
                name: "Wall",
                r: 0,
                g: 0,
                b: 0xFF,
                a: 0xFF,
            },
        ]
    }

    fn new(land: &'a mut Land) -> Self {
        Self { land, round: 0 }
    }

    fn iteration(&mut self) -> bool {
        self.round = self.round + 1;

        for x in 0..self.land.width {
            for y in 0..self.land.height {
                let neighbors = neighbors(x, y, 1, &self.land);
                let passages = neighbors.iter().filter(|n| n.name == "Passage").count();

                let terrain = self.land[(x, y)];

                if terrain.name == "Passage" && passages >= 4 {
                    continue;
                };

                if terrain.name == "Wall" && passages >= 5 {
                    let new = self.land["Passage"];
                    *self.land += (x, y, new);
                } else {
                    let new = self.land["Wall"];
                    *self.land += (x, y, new);
                }
            }
        }

        self.round < 3
    }
}
