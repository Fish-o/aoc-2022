use std::{collections::HashSet, os::unix::raw::gid_t};

use itertools::Itertools;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
  Elf,
  Empty,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
  North,
  South,
  East,
  West,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Elf {
  pos: (usize, usize),
  directions: [Direction; 4],
}
impl Elf {
  pub fn new(pos: (usize, usize), dir: [Direction; 4]) -> Self {
    Elf {
      pos,
      directions: dir,
    }
  }
  pub fn update_pos(&mut self, pos: &(usize, usize)) {
    self.pos = *pos;
  }
}
const PADDING: usize = 10;

const MAP_WIDTH: usize = 12;
const MAP_HEIGHT: usize = 12;
const EMPTY_MAP: [[Tile; MAP_WIDTH + (PADDING * 2)]; MAP_HEIGHT + (PADDING * 2)] =
  [[Tile::Empty; MAP_WIDTH + (PADDING * 2)]; MAP_HEIGHT + (PADDING * 2)];
struct ElfGrounds {
  pub map: [[Tile; MAP_WIDTH + (PADDING * 2)]; MAP_HEIGHT + (PADDING * 2)],
  elfs: Vec<Elf>,
}
impl ElfGrounds {
  pub fn from(map: &String) -> Self {
    let mut grounds = EMPTY_MAP.clone();
    let mut elfs = Vec::new();
    for (row, line) in map.lines().enumerate() {
      for (col, c) in line.chars().enumerate() {
        println!("{} {} {}", c, row, col);
        grounds[row + PADDING][col + PADDING] = match c {
          '#' => Tile::Elf,
          '.' => Tile::Empty,
          _ => panic!("Invalid char in map: {}", c),
        };
        if c == '#' {
          elfs.push(Elf::new(
            (row + PADDING, col + PADDING),
            [
              Direction::North,
              Direction::South,
              Direction::East,
              Direction::West,
            ],
          ));
        }
      }
    }
    ElfGrounds {
      map: grounds,
      elfs: elfs,
    }
  }
  pub fn simulate_elfs(&mut self) {
    let mut proposed_positions = self
      .elfs
      .iter()
      .map(|elf| (elf.pos, get_new_elf_pos(&elf, &self))) // Remove collisions
      .collect::<Vec<_>>();
    proposed_positions.sort_by_key(|f| f.1.pos);

    let mut new_elfs: Vec<Elf> = proposed_positions
      .clone()
      .iter()
      .tuple_windows::<(_, _, _)>()
      .map(
        |((_, prev_new_pos), (pos, mut new_pos), (_, next_new_pos))| {
          if prev_new_pos.pos == new_pos.pos || next_new_pos.pos == new_pos.pos {
            // collision
            new_pos.update_pos(pos)
          }
          new_pos
        },
      )
      .collect::<Vec<_>>();

    // Add first and last elf back
    let (first_elf_old, first_elf) = proposed_positions.first().unwrap();
    let (_, second_elf) = proposed_positions.get(1).unwrap();
    if first_elf.pos == second_elf.pos {
      new_elfs.push(Elf::new(*first_elf_old, first_elf.directions));
    } else {
      new_elfs.push(*first_elf);
    }
    let (last_elf_old, last_elf) = proposed_positions.last().unwrap();
    let (_, second_last_elf) = proposed_positions
      .get(proposed_positions.len() - 2)
      .unwrap();
    if last_elf.pos == second_last_elf.pos {
      new_elfs.push(Elf::new(*last_elf_old, last_elf.directions));
    } else {
      new_elfs.push(*last_elf);
    }
    self.elfs = new_elfs;
    // Remove collisions

    self.map = EMPTY_MAP.clone();
    for (row, col) in self.elfs.iter().map(|e| e.pos) {
      self.map[row][col] = Tile::Elf;
    }
  }
  pub fn display(&self) {
    for row in &self.map {
      for tile in row {
        match tile {
          Tile::Elf => print!("#"),
          Tile::Empty => print!("."),
        }
      }
      println!();
    }
  }
}
pub fn run(input: String) {
  let mut elf_grounds = ElfGrounds::from(&input);
  elf_grounds.display();

  for i in 1..=10 {
    elf_grounds.simulate_elfs();
    println!("\nElf ground: {}", i);
    elf_grounds.display();
  }
}

fn get_new_elf_pos(elf: &Elf, map: &ElfGrounds) -> Elf {
  let row = elf.pos.0;
  let col = elf.pos.1;

  let n = map.map[row - 1][col];
  let ne = map.map[row - 1][col + 1];
  let e = map.map[row][col + 1];
  let se = map.map[row + 1][col + 1];
  let s = map.map[row + 1][col];
  let sw = map.map[row + 1][col - 1];
  let w = map.map[row][col - 1];
  let nw = map.map[row - 1][col - 1];

  let dirs: [Direction; 4] = [
    elf.directions[1],
    elf.directions[2],
    elf.directions[3],
    elf.directions[0],
  ];
  if [n, ne, e, se, s, sw, w, nw]
    .iter()
    .all(|tile| *tile == Tile::Empty)
  {
    return Elf::new((elf.pos.0, elf.pos.1), dirs);
  }

  for dir in &elf.directions {
    match dir {
      Direction::North => {
        if n == Tile::Empty && ne == Tile::Empty && nw == Tile::Empty {
          return Elf::new((row - 1, col), dirs);
        }
      }
      Direction::South => {
        if s == Tile::Empty && se == Tile::Empty && sw == Tile::Empty {
          return Elf::new((row + 1, col), dirs);
        }
      }
      Direction::West => {
        if w == Tile::Empty && nw == Tile::Empty && sw == Tile::Empty {
          return Elf::new((row, col - 1), dirs);
        }
      }
      Direction::East => {
        if e == Tile::Empty && ne == Tile::Empty && se == Tile::Empty {
          return Elf::new((row, col + 1), dirs);
        }
      }
    }
  }
  Elf::new((elf.pos.0, elf.pos.1), dirs)
}

// 2670 elves

// 73x73

/*
  If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
  If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
  If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
  If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
*/
