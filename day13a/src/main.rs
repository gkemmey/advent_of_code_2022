use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
enum Packet {
  Integer(i32),
  List(Vec<Packet>)
}

fn main() {
  let input = std::fs::read_to_string("test.txt").unwrap();
  let mut lines = input.lines();

  let mut packets: Vec<(Packet, Packet)> = vec![];

  while let Some(left) = lines.next() {
    if left.is_empty() {
      continue;
    }
    let right = lines.next().unwrap();
    packets.push((serde_json::from_str(left).unwrap(), serde_json::from_str(right).unwrap()));
  }

  println!("{:?}", packets);
}
