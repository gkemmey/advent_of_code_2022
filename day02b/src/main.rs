use std::fs::File;
use std::io::{BufRead, BufReader};

struct Round {
  me: char,
  them: char
}
impl Round {
  fn score(&self) -> i32 {
    match (self.them, self.me) {
      ('A', 'X') => 3 + 0, // (rock, lose)
      ('A', 'Y') => 1 + 3, // (rock, draw)
      ('A', 'Z') => 2 + 6, // (rock, win)

      ('B', 'X') => 1 + 0, // (paper, lose)
      ('B', 'Y') => 2 + 3, // (paper, draw)
      ('B', 'Z') => 3 + 6, // (paper, win)

      ('C', 'X') => 2 + 0, // (scissors, lose)
      ('C', 'Y') => 3 + 3, // (scissors, draw)
      ('C', 'Z') => 1 + 6, // (scissors, win)

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
