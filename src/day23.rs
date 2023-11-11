const WIDTH: usize = 73;
const HEIGHT: usize = 73;
const PADDING: usize = 60;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
  Empty,
  Elf,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
  North,
  South,
  East,
  West,
}
struct Map {
  map: [[Tile; WIDTH + (PADDING * 2)]; HEIGHT + (PADDING * 2)],
  directions: [Direction; 4],
}
impl Map {
  pub fn from(input: &String) -> Self {
    let mut map = [[Tile::Empty; WIDTH + (PADDING * 2)]; HEIGHT + (PADDING * 2)];
    for (row, line) in input.lines().enumerate() {
      for (col, char) in line.chars().enumerate() {
        map[row + PADDING][col + PADDING] = match char {
          '.' => Tile::Empty,
          '#' => Tile::Elf,
          _ => panic!("Unknown tile"),
        }
      }
    }
    Map {
      map,
      directions: [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
      ],
    }
  }
  pub fn tick(&mut self) -> bool {
    let mut collision_map = [[0; WIDTH + (PADDING * 2)]; HEIGHT + (PADDING * 2)];
    let mut elf_locations = vec![];
    let mut elf_moved = false;
    for (row, line) in self.map.iter().enumerate() {
      for (col, old_tile) in line.iter().enumerate() {
        match old_tile {
          Tile::Empty => (),
          Tile::Elf => {
            let n = self.map[row - 1][col] == Tile::Empty;
            let ne = self.map[row - 1][col + 1] == Tile::Empty;
            let e = self.map[row][col + 1] == Tile::Empty;
            let se = self.map[row + 1][col + 1] == Tile::Empty;
            let s = self.map[row + 1][col] == Tile::Empty;
            let sw = self.map[row + 1][col - 1] == Tile::Empty;
            let w = self.map[row][col - 1] == Tile::Empty;
            let nw = self.map[row - 1][col - 1] == Tile::Empty;
            elf_locations.push((row, col));
            // Check if all are empty
            if n && ne && e && se && s && sw && w && nw {
              collision_map[row][col] += 1;
              continue;
            }
            elf_moved = true;
            for dir in self.directions {
              match dir {
                Direction::North => {
                  if n && ne && nw {
                    collision_map[row - 1][col] += 1;
                    break;
                  }
                }
                Direction::South => {
                  if s && se && sw {
                    collision_map[row + 1][col] += 1;
                    break;
                  }
                }
                Direction::East => {
                  if e && ne && se {
                    collision_map[row][col + 1] += 1;
                    break;
                  }
                }
                Direction::West => {
                  if w && nw && sw {
                    collision_map[row][col - 1] += 1;
                    break;
                  }
                }
              }
            }
          }
        }
      }
    }
    let mut new_map = [[Tile::Empty; WIDTH + (PADDING * 2)]; HEIGHT + (PADDING * 2)];
    for elf in elf_locations {
      let (row, col) = elf;
      let n = self.map[row - 1][col] == Tile::Empty;
      let ne = self.map[row - 1][col + 1] == Tile::Empty;
      let e = self.map[row][col + 1] == Tile::Empty;
      let se = self.map[row + 1][col + 1] == Tile::Empty;
      let s = self.map[row + 1][col] == Tile::Empty;
      let sw = self.map[row + 1][col - 1] == Tile::Empty;
      let w = self.map[row][col - 1] == Tile::Empty;
      let nw = self.map[row - 1][col - 1] == Tile::Empty;
      // Check if all are empty
      if n && ne && e && se && s && sw && w && nw {
        new_map[row][col] = Tile::Elf;
        continue;
      }
      let mut new_pos = (row, col);
      for dir in self.directions {
        match dir {
          Direction::North => {
            if n && ne && nw {
              new_pos = (row - 1, col);
              break;
            }
          }
          Direction::South => {
            if s && se && sw {
              new_pos = (row + 1, col);
              break;
            }
          }
          Direction::East => {
            if e && ne && se {
              new_pos = (row, col + 1);
              break;
            }
          }
          Direction::West => {
            if w && nw && sw {
              new_pos = (row, col - 1);
              break;
            }
          }
        }
      }
      if collision_map[new_pos.0][new_pos.1] == 1 {
        new_map[new_pos.0][new_pos.1] = Tile::Elf;
      } else {
        new_map[row][col] = Tile::Elf;
      }
    }
    self.map = new_map;
    self.directions.rotate_left(1);
    elf_moved
  }
  pub fn get_bounding_box(&self) -> (usize, usize) {
    let mut min_row = usize::MAX;
    let mut max_row = usize::MIN;
    let mut min_col = usize::MAX;
    let mut max_col = usize::MIN;
    for (row, line) in self.map.iter().enumerate() {
      for (col, tile) in line.iter().enumerate() {
        match tile {
          Tile::Empty => (),
          Tile::Elf => {
            if row < min_row {
              min_row = row;
            }
            if row > max_row {
              max_row = row;
            }
            if col < min_col {
              min_col = col;
            }
            if col > max_col {
              max_col = col;
            }
          }
        }
      }
    }
    (max_row - min_row + 1, max_col - min_col + 1)
  }
  pub fn elf_count(&self) -> u32 {
    let mut count = 0;
    for line in &self.map {
      for tile in line {
        match tile {
          Tile::Empty => (),
          Tile::Elf => count += 1,
        }
      }
    }
    count
  }
}

impl std::fmt::Display for Map {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for row in &self.map {
      for tile in row {
        match tile {
          Tile::Empty => write!(f, ".")?,
          Tile::Elf => write!(f, "#")?,
        }
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}
pub fn run(input: String) {
  let mut map = Map::from(&input);
  let mut i = 0;
  print!("Day 23: ");
  loop {
    i += 1;
    let moved = map.tick();
    if i == 10 {
      let size = map.get_bounding_box();
      let elves = map.elf_count();
      print!("{}", (size.0 * size.1) - elves as usize);
    }
    if !moved {
      break;
    }
  }

  println!(" {}", i);
}
