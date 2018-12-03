use std::fs;
use std::collections::HashSet;

fn main() {
    let data = fs::read_to_string("assets/input.txt").expect("Unable to read file!");
    let mut frequencies = HashSet::new();

    let mut acc: i32 = 0;
    while true {
        let data = data.split("\n");
        for value in data {
            match value.parse::<i32>() {
                Ok(e) => {
                    acc += e;
                    if frequencies.contains(&acc) {
                        println!("{}", acc);
                        return ();
                    } else {
                        frequencies.insert(acc);
                    }
                }
                _ => {}
            }
        }
    }
}
