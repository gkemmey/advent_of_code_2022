use regex::Regex;

#[derive(Debug, PartialEq)]
enum Operand {
  Int(i64),
  Old
}

#[derive(Debug)]
enum Operation {
  Addition,
  Multiplication
}

#[derive(Debug)]
struct WorryFunction {
  lhs: Operand,
  rhs: Operand,
  operation: Operation
}
impl WorryFunction {
  fn apply(&self, level: i64) -> i64 {
    let lhs = if let Operand::Int(i) = self.lhs { i } else { level };
    let rhs = if let Operand::Int(i) = self.rhs { i } else { level };

    match self.operation {
      Operation::Addition => {
        return lhs + rhs;
      },
      Operation::Multiplication => {
        return lhs * rhs;
      }
    }
  }
}

#[derive(Debug)]
struct Monkey {
  items: Vec<i64>,
  worry_function: WorryFunction,
  test_divisor: i64,
  throw_to_on_pass: usize,
  throw_to_on_fail: usize,
  items_inspected: i64,
}

fn main() {
  let mut monkeys: Vec<Monkey> = vec![];

  let input = std::fs::read_to_string("input.txt").unwrap();
  let regex = Regex::new(r"(?x)
    Monkey\s\d+:\n                                    # Monkey 0:
    \s\sStarting\sitems:\s(.*)\n                      #   Starting items: 79, 98
    \s\sOperation:\snew\s=\sold\s([+\*])\s(old|\d+)\n #   Operation: new = old * 19
    \s\sTest:\sdivisible\sby\s(\d+)\n                 #   Test: divisible by 23
    \s\s\s\sIf\strue:\sthrow\sto\smonkey\s(\d+)\n     #     If true: throw to monkey 2
    \s\s\s\sIf\sfalse:\sthrow\sto\smonkey\s(\d+)\n    #     If false: throw to monkey 3
  ").unwrap();

  for cap in regex.captures_iter(&input) {
    monkeys.push(
      Monkey {
        items: cap[1].split(", ").map(|x| x.parse::<i64>().unwrap()).collect(),
        worry_function: WorryFunction {
                          lhs: Operand::Old,
                          rhs: if &cap[3] == "old" {
                                 Operand::Old
                               } else {
                                 Operand::Int(cap[3].parse::<i64>().unwrap())
                               },
                          operation: match &cap[2] {
                                       "+" => Operation::Addition,
                                       "*" => Operation::Multiplication,
                                       _ => { panic!("") }
                                     }
                        },
        test_divisor: cap[4].parse::<i64>().unwrap(),
        throw_to_on_pass: cap[5].parse::<usize>().unwrap(),
        throw_to_on_fail: cap[6].parse::<usize>().unwrap(),
        items_inspected: 0
      }
    )
  }

  for _ in 0..20 {
    for m in 0..monkeys.len() {
      let monkey = &mut monkeys[m];
      let mut items_to_throw: Vec<(usize, i64)> = vec![];

      while monkey.items.len() > 0 {
        let mut item = monkey.items.remove(0);
        monkey.items_inspected += 1;

        item = monkey.worry_function.apply(item);
        item = item / 3;

        items_to_throw.push((
          match item % monkey.test_divisor {
            0 => monkey.throw_to_on_pass,
            _ => monkey.throw_to_on_fail
          },
          item
        ))
      }

      for (throw_to, item) in &items_to_throw {
        monkeys[*throw_to].items.push(*item);
      }
    }
  }

  let mut items_inspected = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<i64>>();
  items_inspected.sort();
  items_inspected.reverse();

  // println!("{:?}", items_inspected);
  println!("{:?}", items_inspected[0] * items_inspected[1]);
}
