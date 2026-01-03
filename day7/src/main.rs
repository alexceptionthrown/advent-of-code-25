use std::collections::HashMap;
use std::fs;
use std::error::Error;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut counter_part1: i32 = 0;
    let instructions = fs::read_to_string("input.txt")?;
    let lines = instructions.lines();
    let grid: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let s = find_source(&(grid[0]));

    let mut current_line: HashMap<Coordinate, u64> = HashMap::from([(s, 1)]);
    let mut next_line: HashMap<Coordinate, u64> = HashMap::new();

    for _y in 0..grid.len()-1 {
        let mut particle_queue: Vec<&Coordinate> = current_line.keys().collect();
        while !particle_queue.is_empty() {
            let particle = *particle_queue.pop().unwrap();
            let num_beams = *current_line.get(&particle).unwrap();
            if grid[particle.y + 1][particle.x] == '^' {
                counter_part1 += 1;
                split_safely(particle, num_beams, &mut next_line, &grid);
            } else {
                let lower_particle = Coordinate {
                    x: particle.x,
                    y: particle.y + 1
                };
                insert_or_merge_beam(&lower_particle, num_beams, &mut next_line);
            }
        }
        current_line = next_line;
        next_line = HashMap::new();
    }
    let counter_part2 = current_line.into_values().reduce(|acc, e| acc + e).unwrap();
    println!("Solution part 1: {counter_part1}\nSolution part 2: {counter_part2}");
    Ok(())
}

fn find_source(start_line: &Vec<char>) -> Coordinate {
    Coordinate { x: start_line.iter().position(|&char| char == 'S').unwrap(), y: 0 }
}

fn split_safely(particle: Coordinate, num_beams: u64, next_line: &mut HashMap<Coordinate, u64>, grid: &Vec<Vec<char>>) -> () {
    for x_diff in (-1..2).step_by(2) {
        let new_x = particle.x as i32 + x_diff;
        if 0 <= new_x && new_x < grid[0].len() as i32 && grid[particle.y+1][new_x as usize] == '.' {
            let new_coord = Coordinate{x: new_x as usize, y: particle.y + 1};
            insert_or_merge_beam(&new_coord, num_beams, next_line);
        } 
    }
}

fn insert_or_merge_beam(coordinate: &Coordinate, num_beams: u64, next_line: &mut HashMap<Coordinate, u64>) -> () {
    match next_line.get(&coordinate) {
        None => {next_line.insert(*coordinate, num_beams);},
        Some(_) => *next_line.get_mut(&coordinate).unwrap() += num_beams
    }
}