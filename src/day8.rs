use std::collections::HashSet;
fn rotate_clockwise(vec: &mut Vec<Vec<(usize, i32)>>) -> () {
  let mut new_vec = vec![vec![(0, 0); vec.len()]; vec[0].len()];

  for (i, row) in vec.iter().enumerate() {
    for (j, val) in row.iter().enumerate() {
      new_vec[j][vec.len() - i - 1] = *val;
    }
  }
  *vec = new_vec;
}

fn calculate_scenic_score(map: &Vec<Vec<(usize, i32)>>, x: i32, y: i32) -> i32 {
  let mut scores = vec![];
  let hight = map[x as usize][y as usize].1;
  let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
  for (dx, dy) in directions {
    let mut score = 0;
    let mut x = x;
    let mut y = y;
    x += dx;
    y += dy;
    while x >= 0 && y >= 0 && x < map.len() as i32 && y < map[0].len() as i32 {
      score += 1;
      if map[x as usize][y as usize].1 >= hight {
        break;
      }
      x += dx;
      y += dy;
    }
    scores.push(score);
  }
  scores.into_iter().reduce(|a, b| a * b).unwrap()
}

fn calculate_highest_scenic_score(map: &Vec<Vec<(usize, i32)>>) -> i32 {
  let mut highest = 0;
  for x in 0..map.len() {
    for y in 0..map[0].len() {
      let score = calculate_scenic_score(map, x as i32, y as i32);
      if score > highest {
        highest = score;
      }
    }
  }
  highest
}
pub fn run(input: String) {
  let lines = input.lines();
  let mut map: Vec<Vec<(usize, i32)>> = lines
    .enumerate()
    .map(|(x, line)| {
      line
        .chars()
        .enumerate()
        .map(|(y, c)| (x * 100 + y, c as i32 - '0' as i32))
        .collect()
    })
    .collect();

  // println!("Map size: {}x{}", map.len(), map[0].len());

  let mut visible: HashSet<usize> = HashSet::new();
  for _ in 0..2 {
    // for row in map.iter() {
    //   for tree in row {
    //     print!("{} ", tree.0);
    //   }
    //   println!();
    // }
    // for row in map.iter() {
    //   for tree in row {
    //     print!("{} ", tree.1);
    //   }
    //   println!();
    // }
    for row in map.iter() {
      let mut highest: i32 = -1;
      for tree in row.iter() {
        if tree.1 > highest {
          highest = tree.1;
          visible.insert(tree.0);
        }
      }
      let mut highest = -1;
      for tree in row.iter().rev() {
        if tree.1 > highest {
          highest = tree.1;
          visible.insert(tree.0);
        }
      }
    }
    rotate_clockwise(&mut map);
    // Rotate the 2d array 'map' 90 degrees clockwise
  }
  println!(
    "Day 8: {} {}",
    visible.len(),
    calculate_highest_scenic_score(&map)
  );
}
// Too low 1491
