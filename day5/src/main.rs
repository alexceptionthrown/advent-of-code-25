use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut counter_part1: i32 = 0;
    let instructions = fs::read_to_string("input.txt")?;
    let instructions_with_linux_endings = instructions.replace("\r\n", "\n");
    let (ranges_text, ids_text) = instructions_with_linux_endings.split_once("\n\n").unwrap();
    let ids: Vec<u64> = ids_text.lines().map(|value| value.parse::<u64>().unwrap()).collect();
    let ranges: Vec<(u64, u64)> = ranges_text
        .lines()
        .map(|range_text| range_text.split_once('-').unwrap())
        .map(|(value1, value2)| (value1.parse::<u64>().unwrap(), value2.parse::<u64>().unwrap()))
        .collect();
    for id in ids {
        for range in &ranges {
            let (start, end) = *range;
            if start <= id && id <= end {
                counter_part1 += 1;
                break;
            }
        }
    }
    println!("Solution part 1: {counter_part1}");
    Ok(())
}