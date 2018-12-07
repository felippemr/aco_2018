use std::fs;
use std::collections::HashMap;
mod suit_measure_parser;


// puzzle 1
fn main() {
    let data = fs::read_to_string("assets/input.txt").expect("Unable to read file!");
    let data = data.split("\n");
    let mut edge_count =  HashMap::new();
    for raw_measure in data {
        let suit_measure = suit_measure_parser::SuitMeasure::from_raw_measure(raw_measure);
        let units = suit_measure.unit_top_edges();
        for unit in units.iter() {
            let count = edge_count.entry(*unit).or_insert(0);
            *count += 1;
        }
    }
    let mut total_count = 0;
    for (_, count) in edge_count.iter() {
        if *count > 1 {
            total_count += 1;
        }
    }
    println!("{}", total_count);
}