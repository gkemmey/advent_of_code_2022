fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let mut trees: Vec<Vec<i32>> = vec![];

  for line in input.lines() {
    trees.push(vec![]);
    for c in line.chars() {
      trees.last_mut().unwrap().push(String::from(c).parse().unwrap());
    }
  }

  let mut scores: Vec<i32> = vec![];

  for (i, row) in trees.iter().enumerate() {
    for (j, height) in row.iter().enumerate() {
      // println!("\n\nsolving [{}][{}] | {}", i, j, height);

      let above = distance((0..i).rev().          map(|ii| trees[ii][j]).collect::<Vec<i32>>(), *height);
      let below = distance(((i + 1)..trees.len()).map(|ii| trees[ii][j]).collect::<Vec<i32>>(), *height);

      let left  = distance((0..j).rev().          map(|jj| trees[i][jj]).collect::<Vec<i32>>(), *height);
      let right = distance(((j + 1)..row.len()).  map(|jj| trees[i][jj]).collect::<Vec<i32>>(), *height);

      scores.push(above * below * left * right)
    }
  }

  println!("{:?}", scores.iter().max().unwrap());
}

fn distance(trees: Vec<i32>, height: i32) -> i32 {
  // print!("{:?}", trees);
  let mut iterator = trees.iter().peekable();
  let mut distance = 0;

  while let Some(_) = iterator.next_if(|&n| n < &height) {
    distance += 1;
  }

  // i've either stopped because the iterator ended, or the next tree is >= to me. in the latter
  // case, it should be added to my score.
  if let Some(_) = iterator.peek() {
    distance += 1;
  }

  // println!(" | {}", distance);
  return distance;
}
