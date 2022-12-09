fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let mut trees: Vec<Vec<i32>> = vec![];

  for line in input.lines() {
    trees.push(vec![]);
    for c in line.chars() {
      trees.last_mut().unwrap().push(String::from(c).parse().unwrap());
    }
  }

  let mut visible = 0;

  for (i, row) in trees.iter().enumerate() {
    for (j, height) in row.iter().enumerate() {
      if i == 0 || i == trees.len() - 1 { // top or bottom edge
        visible += 1;
      }
      else if j == 0 || j == row.len() { // left or right edge
        visible += 1;
      }
      else {
        if (0..i).map(|ii| trees[ii][j]).all(|h| h < *height) { // check all above
          visible += 1;
          continue;
        }

        if ((i + 1)..trees.len()).map(|ii| trees[ii][j]).all(|h| h < *height) { // check all below
          visible += 1;
          continue;
        }

        if (0..j).map(|jj| trees[i][jj]).all(|h| h < *height) { // check all left
          visible += 1;
          continue;
        }

        if ((j + 1)..row.len()).map(|jj| trees[i][jj]).all(|h| h < *height) { // check all right
          visible += 1;
          continue;
        }
      }
    }
  }

  println!("{}", visible);
}
