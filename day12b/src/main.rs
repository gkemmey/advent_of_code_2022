use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();

  let mut end: (usize, usize) = (0, 0);
  let mut map: Vec<Vec<i32>> = vec![];

  for line in input.lines() {
    map.push(vec![]);

    for c in line.chars() {
      match c {
        'S' => {
          map.last_mut().unwrap().push(0);
        },
        'E' => {
          map.last_mut().unwrap().push(25);
          end = (map.len() - 1, map.last().unwrap().len() - 1);
        },
        'a'..='z' => {
          map.last_mut().unwrap().push(((c as u8) - ('a' as u8)).into());
        },
        _ => {}
      }
    }
  }

  let mut search: VecDeque<((usize, usize), usize)> = VecDeque::from([(end, 0)]);
  let mut visited: HashSet<(usize, usize)> = HashSet::new();

  'bfs: while let Some((cell, length)) = search.pop_front() {
    // println!("checking {:?}", cell);
    // println!("    search {:?}", search);
    visited.insert(cell);

    for adjacent in adjacent_cells(cell, &map) {
      if map[adjacent.0][adjacent.1] == 0 {
        search.push_back((adjacent, length + 1));
        break 'bfs;
      }
      else {
        if !visited.contains(&adjacent) {
          visited.insert(adjacent);
          search.push_back((adjacent, length + 1));
        }
      }
    }
  }

  println!("{:?}", search.back().unwrap().1);
}

fn adjacent_cells(cell: (usize, usize), map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
  let i = cell.0;
  let j = cell.1;
  let mut adjacent_cells: Vec<(usize, usize)> = vec![];

  // up
  if i > 0 && map[i][j] - map[i - 1][j] <= 1 {
    adjacent_cells.push((i - 1, j));
  }

  // down
  if i < map.len() - 1 && map[i][j] - map[i + 1][j] <= 1 {
    adjacent_cells.push((i + 1, j));
  }

  // left
  if j > 0 && map[i][j] - map[i][j - 1] <= 1 {
    adjacent_cells.push((i, j - 1));
  }

  // right
  if j < map[i].len() - 1 && map[i][j] - map[i][j + 1] <= 1 {
    adjacent_cells.push((i, j + 1));
  }

  // println!("    adjacents {:?}", adjacent_cells);
  return adjacent_cells;
}
