use colored::Colorize;

const SIDE_LENGTH: usize = 50;

#[derive(Debug, Clone)]
enum Tile {
  Empty,
  Walkable,
  Wall,
}
struct Map {
  map: Vec<Vec<Tile>>,
}
impl Map {
  pub fn get_first_non_empty_row(&self, column: usize, reversed: bool) -> usize {
    let mut first_non_empty_row = self.map.iter();

    if !reversed {
      first_non_empty_row.position(|r| match r.iter().nth(column).unwrap() {
        Tile::Empty => false,
        _ => true,
      })
    } else {
      first_non_empty_row.rposition(|r| match r.iter().nth(column).unwrap() {
        Tile::Empty => false,
        _ => true,
      })
    }
    .unwrap()
  }
  pub fn get_first_non_empty_column(&self, row: usize, reversed: bool) -> usize {
    let mut first_non_empty_column = self.map[row].iter();
    if !reversed {
      first_non_empty_column.position(|c| match c {
        Tile::Empty => false,
        _ => true,
      })
    } else {
      first_non_empty_column.rposition(|c| match c {
        Tile::Empty => false,
        _ => true,
      })
    }
    .unwrap()
  }
}
#[derive(Clone, Debug)]
struct Pos {
  row: usize,
  column: usize,
}
impl Pos {
  pub fn new(row: usize, column: usize) -> Pos {
    Pos { row, column }
  }
  pub fn walk(&mut self, direction: &Direction, map: &Map) {
    let row = self.row;
    let column = self.column;

    match direction {
      Direction::Up => {
        let new_row = if row == 0
          || match map.map[row - 1][column] {
            Tile::Empty => true,
            _ => false,
          } {
          map.get_first_non_empty_row(column, true)
        } else {
          row - 1
        };
        self.row = match map.map[new_row][column] {
          Tile::Empty => panic!("Empty tile"),
          Tile::Wall => row,
          Tile::Walkable => new_row,
        }
      }
      Direction::Down => {
        let new_row = if row == map.map.len() - 1
          || match map.map[row + 1][column] {
            Tile::Empty => true,
            _ => false,
          } {
          map.get_first_non_empty_row(column, false)
        } else {
          row + 1
        };
        self.row = match map.map[new_row][column] {
          Tile::Empty => panic!("Empty tile"),
          Tile::Wall => row,
          Tile::Walkable => new_row,
        }
      }
      Direction::Left => {
        let new_column = if column == 0
          || match map.map[row][column - 1] {
            Tile::Empty => true,
            _ => false,
          } {
          map.get_first_non_empty_column(row, true)
        } else {
          column - 1
        };
        self.column = match map.map[row][new_column] {
          Tile::Empty => panic!("Empty tile"),
          Tile::Wall => column,
          Tile::Walkable => new_column,
        }
      }
      Direction::Right => {
        let new_column = if column == map.map[row].len() - 1
          || match map.map[row][column + 1] {
            Tile::Empty => true,
            _ => false,
          } {
          map.get_first_non_empty_column(row, false)
        } else {
          column + 1
        };
        self.column = match map.map[row][new_column] {
          Tile::Empty => panic!("Empty tile"),
          Tile::Wall => column,
          Tile::Walkable => new_column,
        }
      }
    };
  }

  pub fn walk_wrapping_cube(&mut self, direction: &Direction, map: &Map) -> Direction {
    let row = self.row;
    let column = self.column;
    let new_direction: Direction = match direction {
      Direction::Up => {
        let (new_row, new_col, new_direction) = if row == 0 && column / SIDE_LENGTH == 1 {
          (
            (column - SIDE_LENGTH) + SIDE_LENGTH * 3,
            0,
            Direction::Right,
          )
        } else if row == 0 && column / SIDE_LENGTH == 2 {
          (
            SIDE_LENGTH * 4 - 1,
            column - (SIDE_LENGTH * 2),
            Direction::Up,
          )
        } else if row == SIDE_LENGTH * 2 && column / SIDE_LENGTH == 0 {
          (SIDE_LENGTH + column, SIDE_LENGTH, Direction::Right)
        } else {
          (row - 1, column, Direction::Up)
        };
        match map.map[new_row][new_col] {
          Tile::Empty => panic!(
            "Empty tile {:?} {} {}  new: {} {}",
            direction, self.row, self.column, new_row, new_col
          ),
          Tile::Wall => direction.clone(),
          Tile::Walkable => {
            self.row = new_row;
            self.column = new_col;
            new_direction
          }
        }
      }
      Direction::Down => {
        let (new_row, new_col, new_direction) = if row == SIDE_LENGTH * 4 - 1 {
          (0, column + (SIDE_LENGTH * 2), Direction::Down)
        } else if row == SIDE_LENGTH * 3 - 1 && column / SIDE_LENGTH == 1 {
          (
            SIDE_LENGTH * 3 + (column - SIDE_LENGTH),
            SIDE_LENGTH - 1,
            Direction::Left,
          )
        } else if row == SIDE_LENGTH - 1 && column / SIDE_LENGTH == 2 {
          (column - SIDE_LENGTH, SIDE_LENGTH * 2 - 1, Direction::Left)
        } else {
          (row + 1, column, Direction::Down)
        };
        match map.map[new_row][new_col] {
          Tile::Empty => panic!(
            "Empty tile {:?} {} {}  new: {} {}",
            direction, self.row, self.column, new_row, new_col
          ),
          Tile::Wall => direction.clone(),
          Tile::Walkable => {
            self.row = new_row;
            self.column = new_col;
            new_direction
          }
        }
      }
      Direction::Left => {
        let (new_row, new_col, new_direction) = if column == 0 && row / SIDE_LENGTH == 2 {
          (
            SIDE_LENGTH - (row - (SIDE_LENGTH * 2)) - 1,
            SIDE_LENGTH,
            Direction::Right,
          )
        } else if column == 0 && row / SIDE_LENGTH == 3 {
          (0, row - (SIDE_LENGTH * 2), Direction::Down)
        } else if column == SIDE_LENGTH && row / SIDE_LENGTH == 0 {
          (
            (SIDE_LENGTH * 2) + (SIDE_LENGTH - row - 1),
            0,
            Direction::Right,
          )
        } else if column == SIDE_LENGTH && row / SIDE_LENGTH == 1 {
          (SIDE_LENGTH * 2, row - SIDE_LENGTH, Direction::Down)
        } else {
          (row, column - 1, Direction::Left)
        };
        match map.map[new_row][new_col] {
          Tile::Empty => panic!(
            "Empty tile {:?} {} {}  new: {} {}",
            direction, self.row, self.column, new_row, new_col
          ),
          Tile::Wall => direction.clone(),
          Tile::Walkable => {
            self.row = new_row;
            self.column = new_col;
            new_direction
          }
        }
      }
      Direction::Right => {
        let (new_row, new_col, new_direction) = if column == SIDE_LENGTH * 3 - 1 {
          (
            (SIDE_LENGTH - row - 1) + SIDE_LENGTH * 2,
            SIDE_LENGTH * 2 - 1,
            Direction::Left,
          )
        } else if column == SIDE_LENGTH * 2 - 1 && row / SIDE_LENGTH == 1 {
          (
            SIDE_LENGTH - 1,
            SIDE_LENGTH * 2 + (row - SIDE_LENGTH),
            Direction::Up,
          )
        } else if column == SIDE_LENGTH * 2 - 1 && row / SIDE_LENGTH == 2 {
          (
            SIDE_LENGTH - (row - (SIDE_LENGTH * 2)) - 1,
            SIDE_LENGTH * 3 - 1,
            Direction::Left,
          )
        } else if column == SIDE_LENGTH - 1 && row / SIDE_LENGTH == 3 {
          (
            SIDE_LENGTH * 3 - 1,
            SIDE_LENGTH + (row % SIDE_LENGTH),
            Direction::Up,
          )
        } else {
          (row, column + 1, Direction::Right)
        };
        match map.map[new_row][new_col] {
          Tile::Empty => panic!(
            "Empty tile {:?} {} {}  new: {} {}",
            direction, self.row, self.column, new_row, new_col
          ),
          Tile::Wall => direction.clone(),
          Tile::Walkable => {
            self.row = new_row;
            self.column = new_col;
            new_direction
          }
        }
      }
    };
    new_direction
  }
}

impl Map {
  pub fn from(input: &str) -> Map {
    let max_width = input
      .split("\n")
      .map(|line| line.len())
      .max()
      .expect("No max width");

    let map = input
      .split("\n")
      .map(|line| {
        line
          .chars()
          .map(|c| match c {
            ' ' => Tile::Empty,
            '#' => Tile::Wall,
            '.' => Tile::Walkable,
            _ => panic!("Unknown tile"),
          })
          .collect::<Vec<_>>()
      })
      .into_iter()
      .map(|mut line| {
        while line.len() < max_width {
          line.push(Tile::Empty);
        }
        line
      })
      .collect();
    Map { map }
  }
  pub fn from_empty(input: &str) -> Map {
    let max_width = input
      .split("\n")
      .map(|line| line.len())
      .max()
      .expect("No max width");

    let map = input
      .split("\n")
      .map(|line| {
        line
          .chars()
          .map(|c| match c {
            ' ' => Tile::Empty,
            '#' => Tile::Walkable,
            '.' => Tile::Walkable,
            _ => panic!("Unknown tile"),
          })
          .collect::<Vec<_>>()
      })
      .into_iter()
      .map(|mut line| {
        while line.len() < max_width {
          line.push(Tile::Empty);
        }
        line
      })
      .collect();
    Map { map }
  }
  pub fn get_start_pos(&self) -> Pos {
    let row = 0;
    let column = self.map[row]
      .iter()
      .position(|tile| match tile {
        Tile::Walkable => true,
        _ => false,
      })
      .expect("No start position found");
    Pos::new(row, column)
  }
}

enum Instruction {
  Move(u32),
  RotateL,
  RotateR,
}
struct InstructionSet {
  pub instructions: Vec<Instruction>,
}
impl InstructionSet {
  pub fn from(input: &str) -> InstructionSet {
    let mut distance = 0;
    let mut instructions = vec![];
    for char in input.chars() {
      match char {
        'R' => {
          if distance > 0 {
            instructions.push(Instruction::Move(distance));
          }
          instructions.push(Instruction::RotateR);
          distance = 0;
        }
        'L' => {
          if distance > 0 {
            instructions.push(Instruction::Move(distance));
          }
          instructions.push(Instruction::RotateL);
          distance = 0;
        }
        '0'..='9' => {
          distance *= 10;
          distance += char.to_digit(10).unwrap();
        }
        _ => panic!("Unknown instruction"),
      };
    }
    if distance > 0 {
      instructions.push(Instruction::Move(distance));
    }
    InstructionSet { instructions }
  }
}
#[derive(Clone)]
struct Player {
  pos: Pos,
  direction: Direction,
}
impl Player {
  pub fn walk(&mut self, map: &Map, instruction_set: &InstructionSet) {
    for instruction in instruction_set.instructions.iter() {
      match instruction {
        Instruction::RotateL => {
          self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
          };
        }
        Instruction::RotateR => {
          self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
          };
        }
        Instruction::Move(distance) => {
          for _ in 0..*distance {
            self.pos.walk(&self.direction, map);
            match map.map[self.pos.row][self.pos.column] {
              Tile::Empty => panic!("Empty tile"),
              Tile::Wall => panic!("Wall tile"),
              Tile::Walkable => {}
            }
          }
        }
      }
    }
  }

  pub fn walk_wrapping(&mut self, map: &Map, instruction_set: &InstructionSet) -> Vec<Player> {
    let mut players = vec![];
    for instruction in instruction_set.instructions.iter() {
      match instruction {
        Instruction::RotateL => {
          self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
          };
        }
        Instruction::RotateR => {
          self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
          };
        }
        Instruction::Move(distance) => {
          for _ in 0..*distance {
            players.push(self.clone());
            self.direction = self.pos.walk_wrapping_cube(&self.direction, map);
            match map.map[self.pos.row][self.pos.column] {
              Tile::Empty => panic!("Empty tile"),
              Tile::Wall => panic!("Wall tile"),
              Tile::Walkable => {}
            }
          }
        }
      }
    }
    players
  }
  pub fn new(map: &Map) -> Player {
    Player {
      pos: map.get_start_pos(),
      direction: Direction::Right,
    }
  }
}

#[derive(Clone)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}
impl std::fmt::Debug for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Direction::Up => write!(f, "^"),
      Direction::Down => write!(f, "v"),
      Direction::Left => write!(f, "<"),
      Direction::Right => write!(f, ">"),
    }
  }
}

pub fn run(input: String) {
  print!("Day 22: ");
  solve1(&input);
  print!(" ");
  solve2(&input);
  println!("");
}
fn debug(input: &String) {
  let mut input = input.split("\n\n");
  let input_map_part: &str = input.next().unwrap();
  // let input_instruction_part = input.next().unwrap();
  let map = Map::from_empty(input_map_part);
  let instruction_set = InstructionSet::from("200");
  loop {
    let start_pos = Pos::new(
      rand::random::<usize>() % map.map.len(),
      rand::random::<usize>() % map.map[0].len(),
    );
    if match map.map[start_pos.row][start_pos.column] {
      Tile::Empty => true,
      Tile::Wall => true,
      Tile::Walkable => false,
    } {
      continue;
    }
    let direction = match rand::random::<usize>() % 4 {
      0 => Direction::Up,
      1 => Direction::Down,
      2 => Direction::Left,
      3 => Direction::Right,
      _ => panic!("Unknown direction"),
    };
    let mut player = Player {
      pos: start_pos.clone(),
      direction: direction.clone(),
    };
    let steps = player.walk_wrapping(&map, &instruction_set);
    if player.pos.row == start_pos.row && player.pos.column == start_pos.column {
      continue;
    }
    // Debug all info
    println!("start: {:?} {:?}", direction, start_pos);
    println!("end: {:?} {:?}", player.direction, player.pos);
    display(&map, &player, &steps);
    panic!();
  }
}
fn solve1(input: &String) {
  let mut input = input.split("\n\n");
  let input_map_part: &str = input.next().unwrap();
  let input_instruction_part = input.next().unwrap();
  let map = Map::from(input_map_part);
  let instruction_set = InstructionSet::from(input_instruction_part);
  let mut player = Player::new(&map);
  player.walk(&map, &instruction_set);
  let facing = match player.direction {
    Direction::Right => 0,
    Direction::Down => 1,
    Direction::Left => 2,
    Direction::Up => 3,
  };
  let row = player.pos.row + 1;
  let col = player.pos.column + 1;
  let sum = (1000 * row) + (4 * col) + facing;
  print!("{:?}", sum);
}

fn solve2(input: &String) {
  let mut input = input.split("\n\n");
  let input_map_part: &str = input.next().unwrap();
  let input_instruction_part = input.next().unwrap();
  let map = Map::from(input_map_part);
  let instruction_set = InstructionSet::from(input_instruction_part);
  let mut wrapping_player = Player::new(&map);
  wrapping_player.walk_wrapping(&map, &instruction_set);
  let wrapping_facing = match wrapping_player.direction {
    Direction::Right => 0,
    Direction::Down => 1,
    Direction::Left => 2,
    Direction::Up => 3,
  };
  let wrapping_row = wrapping_player.pos.row + 1;
  let wrapping_col = wrapping_player.pos.column + 1;
  let wrapping_sum = (1000 * wrapping_row) + (4 * wrapping_col) + wrapping_facing;
  print!("{:?}", wrapping_sum);
}

fn display(map: &Map, player: &Player, prev: &Vec<Player>) {
  for (row_index, row) in map.map.iter().enumerate() {
    for (col_index, col) in row.iter().enumerate() {
      if row_index == player.pos.row && col_index == player.pos.column {
        print!("{}", (format!("{:?}", player.direction)).bold().red())
      } else if let Some(index) = prev
        .iter()
        .position(|p| p.pos.row == row_index && p.pos.column == col_index)
      {
        let gradient = index as f32 / prev.len() as f32;
        let dir = format!("{:?}", prev[index].direction);
        // Turn gradient into a value between 100-255
        let color = (gradient * 255.0) as u8;
        print!("{}", dir.bold().truecolor(color, color, 0))
      } else {
        match col {
          Tile::Empty => print!(" "),
          Tile::Wall => print!("#"),
          Tile::Walkable => print!("."),
        }
      }
    }
    println!();
  }
}
