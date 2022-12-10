use std::collections::HashSet;

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();

  let mut head_pos = (0, 0);
  let mut tail_pos = (0, 0);
  let mut visits: HashSet<(i32, i32)> = HashSet::new();
  visits.insert(tail_pos);

  for l in input.lines() {
    let instruction = parse_instruction(l);
    println!(" -------- {:?} --------", instruction);

    for _ in 0..instruction.1 {
      move_head(&mut head_pos, instruction.0);
      println!("head {:?}", head_pos);
      move_tail(&mut tail_pos, &head_pos);
      println!("tail {:?}", tail_pos);
      visits.insert(tail_pos);
    }
  }

  println!("{:?}", visits.len());
}

fn parse_instruction(l: &str) -> (&str, i32) {
  let mut parts = l.split(" ");
  (parts.next().unwrap(), parts.next().unwrap().parse::<i32>().unwrap())
}

fn move_head(head_pos: &mut (i32, i32), direction: &str){
  match direction {
    "R" => {
      head_pos.0 += 1;
    }
    "L" => {
      head_pos.0 -= 1;
    }
    "U" => {
      head_pos.1 += 1;
    }
    "D" => {
      head_pos.1 -= 1;
    }
    _ => {}
  }
}

fn move_tail(tail_pos: &mut (i32, i32), head_pos: &(i32, i32)) {
  let distance = (
                   ((head_pos.0 - tail_pos.0) as f64).powf(2.0) +
                   ((head_pos.1 - tail_pos.1) as f64).powf(2.0)
                 ).sqrt();

  println!("{}", distance);

  if distance >= 2.0 {
    if head_pos.0 == tail_pos.0 {
      tail_pos.1 += if head_pos.1 > tail_pos.1 { 1 } else { -1 };
    }
    else if head_pos.1 == tail_pos.1 {
      tail_pos.0 += if head_pos.0 > tail_pos.0 { 1 } else { -1 };
    }
    else {
      match (head_pos.0 > tail_pos.0, head_pos.1 > tail_pos.1) {
        (true, true) => {
          tail_pos.0 += 1;
          tail_pos.1 += 1;
        },
        (true, false) => {
          tail_pos.0 += 1;
          tail_pos.1 -= 1;
        },
        (false, true) => {
          tail_pos.0 -= 1;
          tail_pos.1 += 1;
        },
        (false, false) => {
          tail_pos.0 -= 1;
          tail_pos.1 -= 1;
        }
      }
    }
  }
}
