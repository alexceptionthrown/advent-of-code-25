use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut counter_part1: i32 = 0;
    let instructions = fs::read_to_string("input.txt")?;
    let lines = instructions.lines();
    let mut grid: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let symbol: char = grid[y][x];
            if symbol == 'S' {
                safe_grid_modify(y as i32 + 1, x as i32, '|', &mut grid);
            }
            if symbol == '|' {
                if safe_grid_get(y as i32 + 1, x as i32, &grid) == '^' {
                    safe_grid_modify(y as i32 + 1, x as i32 - 1, '|', &mut grid);
                    safe_grid_modify(y as i32 + 1, x as i32 + 1, '|', &mut grid);
                    counter_part1 += 1;
                } else {
                    safe_grid_modify(y as i32 + 1, x as i32, '|', &mut grid);
                }
            }
        }
    }
    println!("Solution part 1: {counter_part1}");
    Ok(())
}

fn print_grid(grid: &Vec<Vec<char>>) -> () {
    for line in grid {
        for char in line {
            print!("{char}")
        }
        print!("\n")
    }
    print!("\n")
}

fn safe_grid_modify(y: i32, x: i32, value: char, grid: &mut Vec<Vec<char>>) -> () {
    if 0 <= y && y < grid.len() as i32 && 0 <= x && y < grid.len() as i32 {
        if (*grid)[y as usize][x as usize] == '.' {
            (*grid)[y as usize][x as usize] = value
        }
    }
}

fn safe_grid_get(y: i32, x: i32, grid: &Vec<Vec<char>>) -> char {
    if 0 <= y && y < grid.len() as i32 && 0 <= x && y < grid.len() as i32 {
        (*grid)[y as usize][x as usize]
    } else {
        '#'
    }
}