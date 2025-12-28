use std::fs;
use std::error::Error;
use std::cmp;

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

    let exclusive_ranges = compute_exclusive_ranges(ranges);
    let counter_part2 = exclusive_ranges.iter().fold(0, |acc, &e| acc + e.1 - e.0 + 1);
    println!("Solution part 2: {counter_part2}");
    Ok(())
}

fn has_overlap(range1: (u64, u64), range2: (u64, u64)) -> bool {
    (range1.0 <= range2.0 && range2.0 <= range1.1) || (range2.0 <= range1.0 && range1.0 <= range2.1)
}

fn compute_exclusive_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut exclusive_ranges: Vec<(u64, u64)> = Vec::new();
    for range in ranges {
        let mut future_exclusive_ranges: Vec<(u64, u64)> = Vec::new();
        let mut overlapping_ranges: Vec<(u64, u64)> = Vec::new();
        for exclusive_range in exclusive_ranges {
            if has_overlap(range, exclusive_range) {
                overlapping_ranges.push(exclusive_range);
            } else {
                future_exclusive_ranges.push(exclusive_range);
            }
        }
        let mut start = range.0;
        let mut end = range.1;
        for to_merge in overlapping_ranges {
            start = cmp::min(start, to_merge.0);
            end = cmp::max(end, to_merge.1);
        }
        future_exclusive_ranges.push((start, end));
        exclusive_ranges = future_exclusive_ranges
    }
    exclusive_ranges
}