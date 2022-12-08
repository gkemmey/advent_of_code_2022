use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Assignment {
  lower: i32,
  upper: i32
}
impl Assignment {
  fn contains(&self, other: &Assignment) -> bool {
    return other.lower >= self.lower &&
             other.lower <= self.upper &&
             other.upper <= self.upper &&
             other.upper >= self.lower
  }
}

fn parse_pairing(s: String) -> (Assignment, Assignment) {
  let split: Vec<&str> = s.split(",").collect();

  if split.len() != 2 {
    panic!("parings gotta come in twos") ;
  }

  (parse_assignment(split[0]), parse_assignment(split[1]))
}

fn parse_assignment(s: &str) -> Assignment {
  let split: Vec<&str> = s.split("-").collect();

  if split.len() != 2 {
    panic!("assignments gotta come in twos");
  }

  Assignment { lower: split[0].parse::<i32>().unwrap(),
               upper: split[1].parse::<i32>().unwrap() }
}

fn main() {
  let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
  let mut pairs: Vec<(Assignment, Assignment)> = vec![];

  for l in lines {
    let l = l.unwrap();
    pairs.push(parse_pairing(l))
  }

  println!("{:?}", pairs.iter().
                         filter(|(a, b)| a.contains(b) || b.contains(a)).
                         count());
}
