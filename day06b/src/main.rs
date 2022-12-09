use std::collections::HashSet;

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();

  for (i, _) in input.chars().enumerate() {
    let chunk = input.chars().skip(i).take(14).collect::<String>();

    if is_message_marker(&chunk) {
      println!("{}", i + 14);
      break;
    }
  }
}

fn is_message_marker(chunk: &str) -> bool {
  if chunk.len() != 14 { return false; }

  let mut seen = HashSet::new();
  for c in chunk.chars() {
    if seen.contains(&c) { return false; }
    seen.insert(c);
  }

  return true;
}
