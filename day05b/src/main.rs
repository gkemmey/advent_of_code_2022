use regex::Regex;

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();

  let mut dock = Dock::parse(&input);
  let instructions: Vec<Instruction> = Instruction::parse_all(&input);

  for instruction in instructions {
    dock.apply(instruction);
  }

  println!("{:?}", dock.tops());
}

#[derive(Debug)]
struct Dock {
  stacks: Vec<Vec<char>>
}
impl Dock {
  fn parse(input: &str) -> Dock {
    let raw: Vec<&str> = input.lines().take_while(|&l| l != "").
                                       collect::<Vec<&str>>().
                                       into_iter().
                                       rev().
                                       collect();
    let n_stacks: usize = raw[0].split_whitespace().last().unwrap().parse().unwrap();

    let mut dock = Dock { stacks: vec![vec![] ; n_stacks] };

    for l in &raw[1..] {
      for n in 0..n_stacks {
        match l.chars().nth(n * 4 + 1) {
          Some(' ') => {},
          Some(id) => dock.push_container(n, id),
          _ => {}
        }
      }
    }

    return dock;
  }

  fn push_container(&mut self, stack: usize, id: char) {
    self.stacks[stack].push(id);
  }

  fn pop_container(&mut self, stack: usize) -> Option<char> {
    return self.stacks[stack].pop();
  }

  fn apply(&mut self, instruction: Instruction) {
    let mut ids: Vec<char> = vec![];

    for _i in 0..instruction.n_containers {
      ids.push(self.pop_container(instruction.from).unwrap());
    }

    for id in ids.into_iter().rev() {
      self.push_container(instruction.to, id);
    }
  }

  fn tops(&self) -> String {
    // return self.stacks.iter().map(|s| s.last().unwrap_or(&' ')).collect();
    return self.stacks.iter().map(|s| s.last()).
                              filter(|s| s.is_some()).
                              map(|s| s.unwrap()).
                              collect();
  }
}

#[derive(Debug)]
struct Instruction {
  n_containers: usize,
  from: usize,
  to: usize
}

impl Instruction {
  fn parse_all(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    let instruction_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for l in input.lines() {
      match instruction_regex.captures(l) {
        Some(captures) => {
          match (captures.get(1), captures.get(2), captures.get(3)) {
            (Some(n_containers), Some(from), Some(to)) => {
              let n_containers: usize = n_containers.as_str().parse().unwrap();
              let from: usize         =         from.as_str().parse().unwrap();
              let to: usize           =           to.as_str().parse().unwrap();

              instructions.push(Instruction { n_containers, from: from - 1, to: to - 1 })
            },
            _ => {}
          }
        },
        _ => {}
      }
    }

    return instructions;
  }
}
