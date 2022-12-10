use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
struct Knot {
  x: i32,
  y: i32
}
impl Knot {
  fn distance(a: &Knot, b: &Knot) -> f64 {
    ( ((a.x - b.x) as f64).powf(2.0) + ((a.y - b.y) as f64).powf(2.0) ).sqrt()
  }

  fn step(&mut self, direction: &str) {
    match direction {
      "R" => {
        self.x += 1;
      }
      "L" => {
        self.x -= 1;
      }
      "U" => {
        self.y += 1;
      }
      "D" => {
        self.y -= 1;
      }
      _ => {}
    }
  }

  fn follow(&mut self, leader: Knot) {
    if Knot::distance(&self, &leader) >= 2.0 {
      if leader.x == self.x {
        self.y += if leader.y > self.y { 1 } else { -1 };
      }
      else if leader.y == self.y {
        self.x += if leader.x > self.x { 1 } else { -1 };
      }
      else {
        self.x += if leader.x > self.x { 1 } else { -1 };
        self.y += if leader.y > self.y { 1 } else { -1 };
      }
    }
  }

  fn to_tuple(&self) -> (i32, i32) {
    (self.x, self.y)
  }
}

struct Movement {
  direction: String,
  magnitude: i32,
}
impl Movement {
  fn parse(line: &str) -> Movement {
    let mut split = line.split(" ");

    Movement {
      direction: split.next().unwrap().to_string(),
      magnitude: split.next().unwrap().parse::<i32>().unwrap()
    }
  }
}

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();

  let mut knots = [Knot { x: 0, y: 0 } ; 10];

  let mut visits: HashSet<(i32, i32)> = HashSet::new();
  visits.insert(knots[knots.len() - 1].to_tuple());

  for line in input.lines() {
    let movement = Movement::parse(line);

    for _ in 0..movement.magnitude {
      knots[0].step(&movement.direction);

      for i in 1..knots.len() {
        knots[i].follow(knots[i - 1]);
      }

      visits.insert(knots[knots.len() - 1].to_tuple());
    }
  }

  println!("{:?}", visits.len());
}
