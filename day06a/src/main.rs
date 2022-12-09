use std::collections::HashSet;

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();

  for (i, _) in input.chars().enumerate() {
    let chunk = input.chars().skip(i).take(4).collect::<String>();

    if is_packet_marker(&chunk) {
      println!("{}", i + 4);
      break;
    }
  }
}

fn is_packet_marker(chunk: &str) -> bool {
  if chunk.len() != 4 { return false; }

  let mut seen = HashSet::new();
  for c in chunk.chars() {
    if seen.contains(&c) { return false; }
    seen.insert(c);
  }

  return true;
}
