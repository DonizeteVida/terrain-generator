use std::cmp::{Eq, PartialEq};

#[derive(Debug, PartialEq, Eq)]
pub struct Terrain {
    pub name: &'static str,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}