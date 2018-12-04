use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::HashMap;


fn main() {
    let file = File::open("assets/input.txt").expect("Unable to read file!");
    let buffer = BufReader::new(file);

    let mut count_2 = 0;
    let mut count_3 = 0;

    for line in buffer.lines() {
        let line = line.expect("Unable to read line");
        let (has_2, has_3) = count_letters(&line);
        if has_2 {
            count_2 += 1;
        }
        if has_3 {
            count_3 += 1;
        }
    }
    println!("{}*{}={}", count_2, count_3, count_2 * count_3);

}

fn count_letters(word: &str) -> (bool, bool) {
    let mut counter = HashMap::new();
    for letter in word.chars() {
        let count = counter.entry(letter).or_insert(0);
        *count +=1;
    }
    let mut has_two = false;
    let mut has_three = false;
    for (_, value) in counter.iter() {
        if *value == 2 {
            has_two = true
        }
        if *value == 3 {
            has_three = true
        }
    }
    (has_two, has_three)
}
