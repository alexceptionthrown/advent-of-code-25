use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = fs::read_to_string("input.txt")?;
    let mut count: u64 = 0;
    for range  in instructions.trim().rsplit(','){
        let (start, end) = range.split_once('-').unwrap();
        for num in start.parse::<u64>().unwrap()..end.parse::<u64>().unwrap()+1{
            let num_str = num.to_string();
            if num_str.len() % 2 == 0 {
                let (start_split, end_split) = num_str.split_at(num_str.len()/2);
                if start_split == end_split {
                    count += num
                }
            }
        }
    }
    println!("{count}");
    Ok(())
}
