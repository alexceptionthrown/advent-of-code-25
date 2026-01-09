use std::fs;
use std::cmp::max;
use std::error::Error;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
    let mut counter_part1: u64 = 0;
    let mut counter_part2: u64 = 0;

    for i in 0..tiles.len() {
        for j in i+1..tiles.len() {
            let tile1 = &tiles[i];
            let tile2 = &tiles[j];
            let area = tile1.area_with(tile2);
            counter_part1 = max(counter_part1, area);
            if path_does_not_go_inside(tile1, tile2, &tiles) {
                counter_part2 = max(counter_part2, area)
            }
        }
    }
    println!("Solution part 1: {counter_part1}\nSolution part 2: {counter_part2}");
    // 87130620 too low
    Ok(())
}

fn parse_tile(line: &str) -> Tile {
    let mut coords = line.split(',');
    Tile {
        x: coords.next().unwrap().parse::<u32>().unwrap(),
        y: coords.next().unwrap().parse::<u32>().unwrap(),
    }
}

fn path_does_not_go_inside(corner1: &Tile, corner2: &Tile, path: &Vec<Tile>) -> bool {
    let left: &Tile;
    let right: &Tile;
    if corner1.x <= corner2.x {
        left = corner1;
        right = corner2;
    } else {
        left = corner2;
        right = corner1;
    }

    let top: &Tile;
    let bottom: &Tile;
    if corner1.y <= corner2.y {
        top = corner1;
        bottom = corner2;
    } else {
        top = corner2;
        bottom = corner1;
    }

    for i in 0..path.len() {
        let j = (i + 1) % path.len();
        let tile1 = &path[i];
        let tile2 = &path[j];

        if tile1.x == tile2.x {
            // vertical segment
            if tile1.x > left.x && tile1.x < right.x {
                let top_of_segment: &Tile;
                let bottom_of_segment: &Tile;
                if tile1.y > tile2.y {
                    top_of_segment = tile2;
                    bottom_of_segment = tile1;
                }
                else {
                    top_of_segment = tile1;
                    bottom_of_segment = tile2;
                }
                if top_of_segment.y < bottom.y && bottom_of_segment.y > top.y {
                    return false;
                }
            }
        } else {
            // horizontal segment
            if tile1.y > top.y && tile1.y < bottom.y {
                let left_of_segment: &Tile;
                let right_of_segment: &Tile;
                if tile1.x > tile2.x {
                    left_of_segment = tile2;
                    right_of_segment = tile1;
                }
                else {
                    left_of_segment = tile1;
                    right_of_segment = tile2;
                }
                if left_of_segment.y < right.x && right_of_segment.x > left.x {
                    return false;
                }
            }
        }
    }

    true
}


fn is_adjacent(tile1: &Tile, tile2: &Tile) -> bool {
    ((tile1.x as i32 - tile2.x as i32).abs() <= 1 && tile1.y == tile2.y) || ((tile1.y as i32 - tile2.y as i32).abs() <= 1 && tile1.x == tile2.x)
}