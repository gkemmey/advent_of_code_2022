use std::fs::File;
use std::io::{BufRead, BufReader};

struct Round {
  me: char,
  them: char
}
impl Round {
  fn score(&self) -> i32 {
    match (self.them, self.me) {
      ('A', 'X') => 1 + 3, // (rock, rock)
      ('A', 'Y') => 2 + 6, // (rock, paper)
      ('A', 'Z') => 3 + 0, // (rock, scissors)

      ('B', 'X') => 1 + 0, // (paper, rock)
      ('B', 'Y') => 2 + 3, // (paper, paper)
      ('B', 'Z') => 3 + 6, // (paper, scissors)

      ('C', 'X') => 1 + 6, // (scissors, rock)
      ('C', 'Y') => 2 + 0, // (scissors, paper)
      ('C', 'Z') => 3 + 3, // (scissors, scissors)

      _ => 0
    }
  }
}

fn main() {
  let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
  let mut rounds: Vec<Round> = vec![];

  for l in lines {
    let l = l.unwrap();
    let (them, me) = l.split_once(" ").unwrap();
    let them = them.chars().nth(0).unwrap().clone();
    let me = me.chars().nth(0).unwrap().clone();

    rounds.push(Round { them, me })
  }

  println!("{}", rounds.iter().map(|r| r.score()).sum::<i32>());
}
