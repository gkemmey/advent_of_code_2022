// use std::fs::File;
// use std::io::{BufRead, BufReader};

// fn main() {
//   let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
//   let mut elves: Vec<Vec<i32>> = vec![vec![]];

//   for l in lines {
//     let l = l.unwrap();

//     if l == "" {
//       elves.push(vec![])
//     }
//     else {
//       let elf = elves.last_mut().unwrap();
//       elf.push(l.trim().parse().unwrap());
//     }
//   }

//   println!("{:?}", elves.iter().map(|e| e.iter().sum::<i32>()).max().unwrap());
// }

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
// Read the input file and parse the Calories for each Elf.
let input_file = File::open("input.txt").expect("Failed to open input file.");
let mut elves = HashMap::new();
let mut current_elf = 0;
let mut current_total = 0;
for line in BufReader::new(input_file).lines() {
    let line = line.expect("Failed to read line from input file.");
    if line.is_empty() {
        // A blank line indicates the end of an Elf's inventory.
        // Store the Elf's total Calories and start a new Elf.
        elves.insert(current_elf, current_total);
        current_elf += 1;
        current_total = 0;
    } else {
        // Parse the Calories for this food item and add it to
        // the Elf's total Calories.
        let calories = line.parse::<u32>().expect("Failed to parse Calories.");
        current_total += calories;
    }
}

// Store the final Elf's total Calories.
elves.insert(current_elf, current_total);

// Identify the Elf carrying the most Calories.
let mut max_elf = 0;
let mut max_calories = 0;
for (elf, calories) in &elves {
    if calories > &max_calories {
        max_elf = *elf;
        max_calories = *calories;
    }
}

// Print the name of the Elf carrying the most Calories.
println!("The Elf carrying the most Calories is Elf {} with {} Calories.", max_elf, max_calories);
}
