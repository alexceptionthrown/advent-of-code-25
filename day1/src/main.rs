use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = fs::read_to_string("input1.txt")?;
    let lines = instructions.lines();
    let mut pos: i32 = 50;
    let mut counter: i32 = 0;
    for line in lines{
        let (dir, amount)= line.split_at(1);
        let amount: i32 = amount.parse().unwrap();
        if dir == "R" {
            pos = (pos + amount) % 100 
        } else {
            pos = (100 - (((100 - pos) + amount) % 100)) % 100
        }
        if pos == 0 {
            counter+=1;
        }
        }
    println!("{counter}");
    Ok(())
}
