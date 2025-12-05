use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = fs::read_to_string("input.txt")?;
    let mut counter_part1: u64 = 0;
    let mut counter_part2: u64 = 0;
    for range  in instructions.trim().rsplit(','){
        let (start, end) = range.split_once('-').unwrap();
        for num in start.parse::<u64>().unwrap()..end.parse::<u64>().unwrap()+1{
            let num_str = num.to_string();
            
            // solving part 1
            if num_str.len() % 2 == 0 {
                let (start_split, end_split) = num_str.split_at(num_str.len()/2);
                if start_split == end_split {
                    counter_part1 += num
                }
            }

            // solving part 2
            let mut current_len: u32 = 0;
            let mut candidates: Vec<&str> = vec!();
            for char in num_str.as_bytes() {
                let mut updated_candidates: Vec<&str> = vec!();
                for candidate in candidates{
                    if *char == (candidate.as_bytes()[(current_len % candidate.len() as u32) as usize]) {
                        updated_candidates.push(candidate);
                    }
                }
                candidates = updated_candidates;
                if current_len < (num_str.len() as u32)/2{
                    candidates.push(&num_str[0..((current_len+1) as usize)]);
                }
                current_len+=1;
            }
            if !candidates.is_empty() {
                counter_part2 += num
            }
        }
    }
    println!("Solution part 1: {counter_part1}\nSolution part 2: {counter_part2}");
    Ok(())
}
