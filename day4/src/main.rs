use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut counter_part1: i32 = 0;
    let instructions = fs::read_to_string("input.txt")?;
    let lines = instructions.lines();
    let grid: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let height=  grid.len();
    let width = grid[0].len();

    for y in 0..height {
        for x in 0..width {
            counter_part1 += is_accessible(y, x, width, height, &grid);
        }
    }
    println!("Solution part 1: {counter_part1}");
    Ok(())
}

fn print_grid(grid: Vec<Vec<char>>) -> () {
    for line in grid {
        for char in line {
            print!("{char}")
        }
        print!("\n")
    }
}

fn is_accessible(y: usize, x: usize, width: usize, height: usize, grid: &Vec<Vec<char>>) -> i32 {
    if (*grid)[y][x] == '@' {
        let mut adj_count:i32 = 0;
        for j in -1i32..2{
            for i in -1i32..2{
                let new_y = y as i32 + j;
                let new_x = x as i32 + i;
                if !(i == 0 && j == 0) && (0 <= new_y) && (new_y < height as i32) && (0 <= new_x) && (new_x < width as i32) {
                    let value = (*grid)[new_y as usize][new_x as usize];
                    if value == '@' {
                        adj_count += 1;
                    }
                }
            }
        }
        if adj_count < 4 {
            return 1;
        }
    }
    0
}
