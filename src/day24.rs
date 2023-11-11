#[derive(Clone)]
enum Tile {
  Empty,
  Blizzard,
}

#[derive(Clone)]
enum Direction {
  North,
  South,
  East,
  West,
}
struct Blizzard {
  position: (usize, usize),
  direction: Direction,
}
impl Blizzard {
  pub fn tick(&mut self, map_size: (usize, usize)) {
    match self.direction {
      Direction::North => self.position.0 = (map_size.0 + self.position.0 - 1) % map_size.0,
      Direction::South => self.position.0 = (self.position.0 + 1) % map_size.0,
      Direction::East => self.position.1 = (self.position.1 + 1) % map_size.1,
      Direction::West => self.position.1 = (map_size.1 + self.position.1 - 1) % map_size.1,
    }
  }
}
struct ValleyMap {
  map: Vec<Vec<Tile>>,
  blizzards: Vec<Blizzard>,
  map_size: (usize, usize),
}

impl ValleyMap {
  pub fn from(input: String) -> Self {
    let mut map = Vec::new();
    let mut blizzards = Vec::new();
    let row_count = input.lines().count();
    let col_count = input.lines().next().unwrap().chars().count();
    for (row_idx, line) in input.lines().enumerate() {
      if row_idx == 0 || row_idx == row_count - 1 {
        continue;
      }
      let row_idx = row_idx - 1;
      let mut row = Vec::new();
      for (col_idx, c) in line.chars().enumerate() {
        if col_idx == 0 || col_idx == col_count - 1 {
          continue;
        }
        let col_idx = col_idx - 1;
        match c {
          '.' => row.push(Tile::Empty),
          '^' => {
            row.push(Tile::Blizzard);
            blizzards.push(Blizzard {
              position: (row_idx, col_idx),
              direction: Direction::North,
            })
          }
          'v' => {
            row.push(Tile::Blizzard);
            blizzards.push(Blizzard {
              position: (row_idx, col_idx),
              direction: Direction::South,
            })
          }
          '>' => {
            row.push(Tile::Blizzard);
            blizzards.push(Blizzard {
              position: (row_idx, col_idx),
              direction: Direction::East,
            })
          }
          '<' => {
            row.push(Tile::Blizzard);
            blizzards.push(Blizzard {
              position: (row_idx, col_idx),
              direction: Direction::West,
            });
          }

          _ => panic!("Invalid character in input"),
        }
      }
      map.push(row);
    }
    Self {
      map,
      blizzards,
      map_size: (row_count - 2, col_count - 2),
    }
  }
  pub fn display(&self) {
    for (row_idx, row) in self.map.iter().enumerate() {
      for (col_idx, tile) in row.iter().enumerate() {
        match tile {
          Tile::Empty => print!("."),
          Tile::Blizzard => {
            let blizzards = self
              .blizzards
              .iter()
              .filter(|blizzard| blizzard.position == (row_idx, col_idx))
              .collect::<Vec<&Blizzard>>();
            if blizzards.len() == 0 {
              panic!("No blizzard found at position ({}, {})", row_idx, col_idx);
            } else if blizzards.len() == 1 {
              match blizzards[0].direction {
                Direction::North => print!("^"),
                Direction::South => print!("v"),
                Direction::East => print!(">"),
                Direction::West => print!("<"),
              }
            } else {
              print!("{}", blizzards.len())
            }
          }
        }
      }
      println!();
    }
  }
  pub fn tick(&mut self) {
    let height = self.map.len();
    let width = self.map[0].len();
    let mut new_map = vec![vec![Tile::Empty; width]; height];
    for blizzard in &mut self.blizzards {
      blizzard.tick(self.map_size);
      let (row, col) = blizzard.position;
      new_map[row][col] = Tile::Blizzard;
    }
    self.map = new_map;
  }
}

struct WalkableMap {
  next_valley_map: ValleyMap,
  pub pathway: Vec<Vec<Vec<u32>>>,
  count: u32,
  start_pos: (usize, usize),
}
impl WalkableMap {
  pub fn from(valley_map: ValleyMap, start_pos: (usize, usize)) -> Self {
    Self {
      count: 0,
      next_valley_map: valley_map,
      pathway: vec![],
      start_pos,
    }
  }

  pub fn add_floor(&mut self) {
    self.count += 1;
    let mut walkable = vec![];
    for row in &self.next_valley_map.map {
      let mut floor_row = vec![];
      for tile in row {
        match tile {
          Tile::Empty => floor_row.push(true),
          Tile::Blizzard => floor_row.push(false),
        }
      }
      walkable.push(floor_row);
    }
    if self.pathway.len() == 0 {
      let mut pathway = vec![vec![0; walkable[0].len()]; walkable.len()];
      if walkable[self.start_pos.0][self.start_pos.1] {
        pathway[self.start_pos.0][self.start_pos.1] = self.count;
      }
      self.pathway.push(pathway);
    } else {
      let mut next_pathway = vec![vec![0; walkable[0].len()]; walkable.len()];
      let prev_pathway = &self.pathway[self.pathway.len() - 1];
      for (row_idx, row) in prev_pathway.iter().enumerate() {
        for (col_idx, distance) in row.iter().enumerate() {
          //
          if distance == &0 {
            continue;
          }
          let mut movable_positions = vec![];

          if walkable[row_idx][col_idx] {
            movable_positions.push((row_idx, col_idx));
          }
          if row_idx > 0 && walkable[row_idx - 1][col_idx] {
            movable_positions.push((row_idx - 1, col_idx));
          };
          if row_idx < walkable.len() - 1 && walkable[row_idx + 1][col_idx] {
            movable_positions.push((row_idx + 1, col_idx));
          };
          if col_idx > 0 && walkable[row_idx][col_idx - 1] {
            movable_positions.push((row_idx, col_idx - 1));
          };
          if col_idx < walkable[0].len() - 1 && walkable[row_idx][col_idx + 1] {
            movable_positions.push((row_idx, col_idx + 1));
          };

          for (row_idx, col_idx) in movable_positions {
            let current_val = next_pathway[row_idx][col_idx];
            if current_val == 0 || current_val > distance + 1 {
              next_pathway[row_idx][col_idx] = distance + 1;
            }
          }
        }
      }
      if walkable[self.start_pos.0][self.start_pos.1] {
        next_pathway[self.start_pos.0][self.start_pos.1] = self.count;
      }
      self.pathway.push(next_pathway);
    }
    self.next_valley_map.tick();
  }

  pub fn display(&self) {
    for (floor_idx, floor) in self.pathway.iter().enumerate() {
      println!("Floor {}", floor_idx);
      for (row_idx, row) in floor.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
          print!("{:>3?}", tile);
        }
        println!();
      }
    }
  }
  pub fn has_arrived_at(&self, pos: (usize, usize)) -> bool {
    let last_path = &self.pathway[self.pathway.len() - 1];
    last_path[pos.0][pos.1] != 0
  }
  pub fn change(&mut self, pos: (usize, usize)) {
    self.start_pos = pos;
    self.pathway = vec![];
  }
}

pub fn run(input: String) {
  let map = ValleyMap::from(input);
  let mut walkable_map = WalkableMap::from(map, (0, 0));
  let map_size = walkable_map.next_valley_map.map_size;
  let goal = (map_size.0 - 1, map_size.1 - 1);
  print!("Day 25: ");
  loop {
    walkable_map.add_floor();
    if walkable_map.has_arrived_at(goal) {
      break;
    }
  }

  print!("{} ", walkable_map.count);
  walkable_map.add_floor();
  walkable_map.add_floor();
  walkable_map.change(goal);
  let goal = (0, 0);
  loop {
    walkable_map.add_floor();
    if walkable_map.has_arrived_at(goal) {
      break;
    }
  }
  walkable_map.add_floor();
  walkable_map.add_floor();
  walkable_map.change(goal);
  let goal = (map_size.0 - 1, map_size.1 - 1);

  loop {
    walkable_map.add_floor();
    if walkable_map.has_arrived_at(goal) {
      break;
    }
  }
  let time_consumed = walkable_map.count;
  println!("{}", time_consumed);
  // walkable_map.display();
}
