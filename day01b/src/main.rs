use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
  let mut elves: Vec<Vec<i32>> = vec![vec![]];

  for l in lines {
    let l = l.unwrap();

    if l == "" {
      elves.push(vec![])
    }
    else {
      let elf = elves.last_mut().unwrap();
      elf.push(l.trim().parse().unwrap());
    }
  }

  let mut calorie_totals = elves.iter().map(|e| e.iter().sum::<i32>()).collect::<Vec<i32>>();
  calorie_totals.sort_by(|a, b| b.cmp(a));

  println!("{:?}", calorie_totals.iter().take(3).sum::<i32>());
}
