use std::collections::{HashMap, HashSet};
use std::fs;
use std::error::Error;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32
}

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> f64 {
        ((
            (self.x as i64 - other.x as i64).pow(2) +
            (self.y as i64 - other.y as i64).pow(2) + 
            (self.z as i64 - other.z as i64).pow(2)
        ) as f64).sqrt()
    }
}

struct JunctionBoxPair<'a> {
    box1: &'a JunctionBox, 
    box2: &'a JunctionBox,
    distance: f64
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Circuit {
    boxes: Vec<JunctionBox>
}

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = fs::read_to_string("input.txt")?;
    let junction_boxes: Vec<JunctionBox> = instructions.lines().map(|line| parse_junction_box(line)).collect();
    let mut sorted_pairs = build_sorted_junction_pairs(&junction_boxes);
    let mut circuits: HashSet<Circuit>= junction_boxes.iter().map(|&junction_box| Circuit{boxes: vec![junction_box]}).collect();
    let mut box_to_circuit: HashMap<JunctionBox, Circuit> = circuits
        .iter()
        .map(|circuit| (*circuit.boxes.first().unwrap(), circuit.clone()))
        .collect();
    const MAX_CABLES: u32 = 1000;
    let mut i: u32 = 0;
    while i < MAX_CABLES  {
        let pair = sorted_pairs.pop().unwrap();
        if box_to_circuit[pair.box1] != box_to_circuit[pair.box2] {
            merge_circuits(&pair, &mut box_to_circuit, &mut circuits);
        }
        i += 1;
    }
    let mut circuit_lengths: Vec<usize> = circuits.iter().map(|circuit| circuit.boxes.len()).collect();
    circuit_lengths.sort();
    let counter_part1 = circuit_lengths.iter().rev().take(3).cloned().reduce(|acc, e| acc * e).unwrap();
    println!("Solution part 1: {counter_part1}");
    let mut last_pair_opt: Option<JunctionBoxPair> = None;
    while circuits.len() > 1  {
        let pair = sorted_pairs.pop().unwrap();
        if box_to_circuit[pair.box1] != box_to_circuit[pair.box2] {
            merge_circuits(&pair, &mut box_to_circuit, &mut circuits);
        }
        if circuits.len() == 1 {
            last_pair_opt = Some(pair)
        }
    }
    let last_pair = last_pair_opt.unwrap();
    let counter_part2 = last_pair.box1.x * last_pair.box2.x;
    println!("Solution part 2: {counter_part2}");
    Ok(())
}

fn parse_junction_box(line: &str) -> JunctionBox {
    let mut coords = line.split(',');
    JunctionBox {
        x: coords.next().unwrap().parse::<u32>().unwrap(),
        y: coords.next().unwrap().parse::<u32>().unwrap(),
        z: coords.next().unwrap().parse::<u32>().unwrap()
    }
}

fn build_sorted_junction_pairs<'a>(junction_boxes: &'a Vec<JunctionBox>) -> Vec<JunctionBoxPair<'a>> {
    let mut pairs: Vec<JunctionBoxPair> = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in i+1..junction_boxes.len() {
            let box1: &JunctionBox = &junction_boxes[i];
            let box2: &JunctionBox = &junction_boxes[j];
            pairs.push(
                JunctionBoxPair { box1, box2, distance: box1.distance(box2) }
            );
        }
    }
    pairs.sort_by(|a, b| b.distance.total_cmp(&a.distance));
    pairs
}

fn merge_circuits(pair: &JunctionBoxPair, box_to_circuit: &mut HashMap<JunctionBox, Circuit>, circuits: &mut HashSet<Circuit>) {
    let circuit1 = &box_to_circuit[pair.box1].clone();
    let circuit2 = &box_to_circuit[pair.box2].clone();
    let mut new_boxes = circuit1.boxes.clone();
    new_boxes.extend(circuit2.boxes.clone());
    let new_circuit = Circuit {
        boxes: new_boxes
    };
    for junction_box in &new_circuit.boxes {
        box_to_circuit.insert(*junction_box, new_circuit.clone());
    }
    circuits.remove(circuit1);
    circuits.remove(circuit2);
    circuits.insert(new_circuit);
}