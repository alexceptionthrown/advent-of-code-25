use std::fs;
use std::cmp::max;
use std::error::Error;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Tile {
    x: u32,
    y: u32
}

impl Tile {
    fn area_with(&self, other: &Tile) -> u64 {
        ((self.x as i64 - other.x as i64).abs() as u64 + 1) * ((self.y as i64 - other.y as i64).abs() as u64 + 1) 
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = fs::read_to_string("input.txt")?;
    let tiles: Vec<Tile> = instructions.lines().map(self::parse_tile).collect();
    let mut max_area: u64 = 0;
    for i in 0..tiles.len() {
        for j in i+1..tiles.len() {
            let tile1 = &tiles[i];
            let tile2 = &tiles[j];
            max_area = max(max_area, tile1.area_with(tile2))
        }
    }
    println!("Solution part 1: {max_area}");
    Ok(())
}

fn parse_tile(line: &str) -> Tile {
    let mut coords = line.split(',');
    Tile {
        x: coords.next().unwrap().parse::<u32>().unwrap(),
        y: coords.next().unwrap().parse::<u32>().unwrap(),
    }
}