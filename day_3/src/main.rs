use std::fs;
mod suit_measure_parser;



// puzzle 1
fn main() {
    let data = fs::read_to_string("assets/input.txt").expect("Unable to read file!");
    let data = data.split("\n");

    for raw_measure in data {
        let suit_measure = suit_measure_parser::SuitMeasure::from_raw_measure(raw_measure);
        println!("{}", suit_measure);
    }
}