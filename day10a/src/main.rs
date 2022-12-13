fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();

  let mut interesting_signals: Vec<isize> = vec![];
  let mut pc = 1;
  let mut register = 1;

  for line in input.lines() {
    match line {
      _ if line.starts_with("noop") => {
        collect_interesting_signals(&mut interesting_signals, pc, register);
        pc += 1;
      },

      l if line.starts_with("addx") => {
        let addend = l.split(" ").nth(1).unwrap().parse::<isize>().unwrap();
        for _ in 0..2 {
          collect_interesting_signals(&mut interesting_signals, pc, register);
          pc += 1;
        }
        println!("    adding {} to {}", addend, register);
        register += addend;
      },

      _ => { panic!("here"); }
    }
  }

  println!("{:?}", interesting_signals.iter().enumerate().map(|(i, s)| (i as isize * 40 + 20) * s).sum::<isize>());
}

fn collect_interesting_signals(memo: &mut Vec<isize>, pc: isize, register: isize) {
  println!("checking {} with register == {}", pc, register);
  if pc == 20 || pc == 60 || pc == 100 || pc == 140 || pc == 180 || pc == 220 {
    memo.push(register);
  }
}
