use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut accessible_counter: i32 = 0;
    let instructions = fs::read_to_string("input.txt")?;
    let lines = instructions.lines();
    let grid: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut grid = compute_next_grid(&grid, &mut accessible_counter);
    println!("Solution part 1: {accessible_counter}");

    let mut counter_current_iter = accessible_counter;
    while counter_current_iter != 0 {
        counter_current_iter = 0;
        grid = compute_next_grid(&grid, &mut counter_current_iter);
        accessible_counter += counter_current_iter;
    }
    println!("Solution part 2: {accessible_counter}");
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

fn is_accessible(y: usize, x: usize, grid: &Vec<Vec<char>>) -> bool {
    let height = grid.len();
    let width = grid[0].len();
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
            return true;
        }
    }
    false
}

fn compute_next_grid(current_grid: &Vec<Vec<char>>, additional_accessible: &mut i32) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    for y in 0..current_grid.len() {
        let mut line: Vec<char> = Vec::new();
        for x in 0..current_grid[0].len() {
            if (*current_grid)[y][x] == '.' {
                line.push('.');
            } else {
                if is_accessible(y, x, current_grid) {
                    line.push('.');
                    *additional_accessible += 1;
                } else {
                    line.push('@');
                }
            }
        }
        new_grid.push(line);
    }
    new_grid
}
