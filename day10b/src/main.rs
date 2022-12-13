fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();

  let mut screen: Vec<&str> = vec!["." ; 240];
  let mut pc: usize = 1;
  let mut register = 1;

  for line in input.lines() {
    match line {
      _ if line.starts_with("noop") => {
        paint(&mut screen, pc, register);
        pc += 1;
      },

      l if line.starts_with("addx") => {
        let addend = l.split(" ").nth(1).unwrap().parse::<isize>().unwrap();
        for _ in 0..2 {
          paint(&mut screen, pc, register);
          pc += 1;
        }
        // println!("  -- adding {} to {}", addend, register);
        register += addend;
      },

      _ => { panic!("here"); }
    }
  }

  for i in 0..screen.len() {
    if i > 0 && i % 40 == 0 { print!("\n"); }
    print!("{}", screen[i]);
  }
  println!("");
}

fn paint(screen: &mut Vec<&str>, pc: usize, register: isize) {
  // println!("{} | {}", pc, register);
  let wrapped = (pc as isize - 1) % 40;

  if register - 1 == wrapped || register == wrapped || register + 1 == wrapped {
    screen[pc] = "#";
  }
}
