use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
  println!("{}", lines.map(|l| ord(duped_item(&l.unwrap()))).sum::<i32>())
}

fn duped_item(rucksack: &str) -> Option<char> {
  let length = rucksack.chars().count();
  let midpoint = length / 2;

  for (i, c) in rucksack.chars().enumerate() {
    if i == midpoint {
      break;
    }

    for j in midpoint..length {
      if c == rucksack.chars().nth(j).unwrap() {
        return Some(c)
      }
    }
  }

  return None
}

fn ord(c: Option<char>) -> i32 {
  match c {
    Some('a') =>  1,
    Some('b') =>  2,
    Some('c') =>  3,
    Some('d') =>  4,
    Some('e') =>  5,
    Some('f') =>  6,
    Some('g') =>  7,
    Some('h') =>  8,
    Some('i') =>  9,
    Some('j') => 10,
    Some('k') => 11,
    Some('l') => 12,
    Some('m') => 13,
    Some('n') => 14,
    Some('o') => 15,
    Some('p') => 16,
    Some('q') => 17,
    Some('r') => 18,
    Some('s') => 19,
    Some('t') => 20,
    Some('u') => 21,
    Some('v') => 22,
    Some('w') => 23,
    Some('x') => 24,
    Some('y') => 25,
    Some('z') => 26,
    Some('A') => 27,
    Some('B') => 28,
    Some('C') => 29,
    Some('D') => 30,
    Some('E') => 31,
    Some('F') => 32,
    Some('G') => 33,
    Some('H') => 34,
    Some('I') => 35,
    Some('J') => 36,
    Some('K') => 37,
    Some('L') => 38,
    Some('M') => 39,
    Some('N') => 40,
    Some('O') => 41,
    Some('P') => 42,
    Some('Q') => 43,
    Some('R') => 44,
    Some('S') => 45,
    Some('T') => 46,
    Some('U') => 47,
    Some('V') => 48,
    Some('W') => 49,
    Some('X') => 50,
    Some('Y') => 51,
    Some('Z') => 52,
    _ => 0
  }
}
