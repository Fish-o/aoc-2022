use console::style;
use std::{
  collections::{HashMap, HashSet},
  ops::Index,
};
struct Map {
  map: Vec<Vec<u8>>,
  width: usize,
  height: usize,
  start: (usize, usize),
  end: (usize, usize),
}
#[derive(Clone, Copy, PartialEq)]
enum Dir {
  Up,
  Down,
  Left,
  Right,
}

impl Map {
  pub fn new(input: &str) -> Self {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = input
      .lines()
      .enumerate()
      .map(|(x, l)| {
        l.chars()
          .enumerate()
          .map(|(y, c)| {
            if c == 'S' {
              start = (x, y);
              0
            } else if c == 'E' {
              end = (x, y);
              25
            } else {
              c as u8 - 'a' as u8
            }
          })
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    let width = map[0].len();
    let height = map.len();
    Self {
      map,
      width,
      height,
      start,
      end,
    }
  }
  pub fn to_string(&self) -> String {
    let mut s = String::new();
    let pos_map = self.possibilities_map();
    let dis_map = self.distances_map();
    let path = self.find_path(&dis_map, &pos_map);
    for (x, row) in self.map.iter().enumerate() {
      for (y, h) in row.iter().enumerate() {
        let c = (*h + 'a' as u8) as char;
        if (x, y) == self.start {
          s += &style('S').bold().to_string();
        } else if (x, y) == self.end {
          s += &style('E').bold().to_string();
        } else if let Some(d) = path.get(&(x, y)) {
          let c = match d {
            Dir::Up => '↑',
            Dir::Down => '↓',
            Dir::Left => '←',
            Dir::Right => '→',
          };
          s += &style(c)
            // .bg(console::Color::Color256(255))
            .bold()
            .to_string();
        } else {
          s += &style(c)
            .bg(console::Color::Color256(232 + h.min(&(255 - 232))))
            .to_string();
        }
      }
      s.push('\n');
    }
    s
  }
  pub fn possibilities_map(&self) -> Vec<Vec<Vec<Dir>>> {
    let mut possibilities = vec![vec![vec![]; self.width]; self.height];
    for (x, row) in self.map.iter().enumerate() {
      for (y, h) in row.iter().enumerate() {
        let h = *h as i16;
        if x > 0 {
          let up = self.map[x - 1][y] as i16;
          if up <= h || up == h + 1 {
            possibilities[x][y].push(Dir::Up);
          }
        }
        if x < self.height - 1 {
          let down = self.map[x + 1][y] as i16;
          if down <= h || down == h + 1 {
            possibilities[x][y].push(Dir::Down);
          }
        }
        if y > 0 {
          let left = self.map[x][y - 1] as i16;
          if left <= h || left == h + 1 {
            possibilities[x][y].push(Dir::Left);
          }
        }
        if y < self.width - 1 {
          let right = self.map[x][y + 1] as i16;
          if right <= h || right == h + 1 {
            possibilities[x][y].push(Dir::Right);
          }
        }
      }
    }
    possibilities
  }
  pub fn distances_map(&self) -> Vec<Vec<u32>> {
    let end = self.end;
    let mut distances = vec![vec![std::u32::MAX; self.width]; self.height];
    distances[end.0][end.1] = 0;
    let mut changed = false;
    let mut to_be_handled = HashSet::new();
    to_be_handled.insert((end.0, end.1));

    let possibilities = self.possibilities_map();
    while let Some((x, y)) = to_be_handled.iter().next().cloned() {
      to_be_handled.remove(&(x, y));
      let d = distances[x][y];
      let new_d = d + 1;
      // Check if the arrow above it can reach this point
      if x > 0 && possibilities[x - 1][y].contains(&Dir::Down) {
        let new_d = d + 1;
        if new_d < distances[x - 1][y] {
          distances[x - 1][y] = new_d;
          to_be_handled.insert((x - 1, y));
          changed = true;
        }
      }
      // Check if the arrow below it can reach this point
      if x < self.height - 1 && possibilities[x + 1][y].contains(&Dir::Up) {
        if new_d < distances[x + 1][y] {
          distances[x + 1][y] = new_d;
          to_be_handled.insert((x + 1, y));
          changed = true;
        }
      }
      // Check if the arrow to the left can reach this point
      if y > 0 && possibilities[x][y - 1].contains(&Dir::Right) {
        if new_d < distances[x][y - 1] {
          distances[x][y - 1] = new_d;
          to_be_handled.insert((x, y - 1));
          changed = true;
        }
      }
      // Check if the arrow to the right can reach this point
      if y < self.width - 1 && possibilities[x][y + 1].contains(&Dir::Left) {
        if new_d < distances[x][y + 1] {
          distances[x][y + 1] = new_d;
          to_be_handled.insert((x, y + 1));
          changed = true;
        }
      }

      if !changed {
        break;
      }
    }
    distances
  }

  pub fn print_pos_map(&self) {
    let possibilities = self.possibilities_map();
    for (x, row) in possibilities.iter().enumerate() {
      for (y, dirs) in row.iter().enumerate() {
        let c = if dirs.is_empty() {
          ' '
        } else if dirs.len() == 1 {
          match dirs[0] {
            Dir::Up => '↓',
            Dir::Down => '↑',
            Dir::Left => '→',
            Dir::Right => '←',
          }
        } else if dirs.len() == 2 {
          match (dirs[0], dirs[1]) {
            (Dir::Up, Dir::Down) => '│',
            (Dir::Left, Dir::Right) => '─',
            (Dir::Up, Dir::Left) => '┘',
            (Dir::Up, Dir::Right) => '└',
            (Dir::Down, Dir::Left) => '┐',
            (Dir::Down, Dir::Right) => '┌',
            _ => unreachable!(),
          }
        } else if dirs.len() == 3 {
          match (dirs[0], dirs[1], dirs[2]) {
            (Dir::Up, Dir::Down, Dir::Left) => '┤',
            (Dir::Up, Dir::Down, Dir::Right) => '├',
            (Dir::Up, Dir::Left, Dir::Right) => '┴',
            (Dir::Down, Dir::Left, Dir::Right) => '┬',
            _ => unreachable!(),
          }
        } else {
          '┼'
        };
        if (x, y) == self.start {
          print!("{}", style('S').bold());
        } else if (x, y) == self.end {
          print!("{}", style('E').bold());
        } else {
          print!(
            "{}",
            style(c).bg(console::Color::Color256(
              232 + self.map[x][y].min(255 - 232)
            ))
          );
        }
      }
      println!();
    }
  }
  fn find_path(
    &self,
    distance_map: &Vec<Vec<u32>>,
    dir_map: &Vec<Vec<Vec<Dir>>>,
  ) -> HashMap<(usize, usize), Dir> {
    let mut path = HashMap::new();
    let mut pos = self.start;
    let mut dirs = dir_map[pos.0][pos.1].clone();
    let mut distance = distance_map[pos.0][pos.1];
    while pos != self.end {
      let mut best_new_post = None;
      let mut best_new_distance = distance;
      let mut best_dir = None;
      for dir in dirs {
        let new_pos = match dir {
          Dir::Up => (pos.0 as i32 - 1, pos.1 as i32),
          Dir::Down => (pos.0 as i32 + 1, pos.1 as i32),
          Dir::Left => (pos.0 as i32, pos.1 as i32 - 1),
          Dir::Right => (pos.0 as i32, pos.1 as i32 + 1),
        };
        if new_pos.0 >= self.height as i32 || new_pos.1 >= self.width as i32 {
          continue;
        } else if new_pos.0 < 0 || new_pos.1 < 0 {
          continue;
        }
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

        let new_distance = distance_map[new_pos.0][new_pos.1];
        if new_distance < best_new_distance {
          best_new_post = Some(new_pos);
          best_new_distance = new_distance;
          best_dir = Some(dir);
          break;
        }
      }
      if let Some(new_pos) = best_new_post {
        path.insert(pos, best_dir.expect("No best dir"));
        pos = new_pos;
        dirs = dir_map[pos.0][pos.1].clone();
        distance = best_new_distance;
      } else {
        break;
      }
    }
    path
  }

  pub fn print_distances_map(&self) {
    let distances = self.distances_map();

    for (x, row) in distances.iter().enumerate() {
      for (y, d) in row.iter().enumerate() {
        if (x, y) == self.start {
          print!(" {} ", style('S').bold());
        } else if (x, y) == self.end {
          print!(" {} ", style('E').bold());
        } else if *d == std::u32::MAX {
          print!(" . ");
        } else {
          print!(
            "{}",
            style(format!("{:<3}", d)).bg(console::Color::Color256(
              232 + self.map[x][y].min(255 - 232)
            ))
          );
        }
      }
      println!();
    }
  }
  pub fn a(&self) -> u32 {
    let distances = self.distances_map();
    let mut all_As = Vec::new();
    for (x, row) in self.map.iter().enumerate() {
      for (y, c) in row.iter().enumerate() {
        if *c == 0 {
          all_As.push((x, y));
        }
      }
    }
    let mut shortest_a = (std::u32::MAX, (0, 0));
    for (x, y) in all_As {
      let distance = distances[x][y];
      if distance < shortest_a.0 {
        shortest_a = (distance, (x, y));
      }
    }

    shortest_a.0
  }
}

pub fn run(input: String) {
  let map = Map::new(&input);
  // println!("{}", map.to_string());
  println!("Day 12: ??? {}", map.a());
}
