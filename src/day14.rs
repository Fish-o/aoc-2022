use std::collections::HashMap;
#[derive(PartialEq)]
enum Tile {
  Stone,
  Sand,
}
fn print_map(map: &HashMap<(u32, u32), Tile>, width: (u32, u32), depth: (u32, u32)) {
  let floor_depth = depth.1 + 2;
  let bottom_width = (floor_depth * 2) + 1;
  let left_width = width.0.min(500 - floor_depth);
  let right_width = width.1.max(500 + floor_depth);

  println!();
  for y in depth.0..=floor_depth {
    for x in left_width..=right_width {
      if y == 0 && x == 500 {
        print!("+");
        continue;
      } else if y == floor_depth {
        print!("#");
      } else {
        match map.get(&(x, y)) {
          Some(Tile::Stone) => print!("#"),
          Some(Tile::Sand) => print!("o"),
          _ => print!(" "),
        }
      }
    }
    println!();
  }
}
pub fn run(data: String) {
  let mut map: HashMap<(u32, u32), Tile> = HashMap::new();
  for line in data.lines() {
    let mut parts = line.split(" -> ").map(|s| {
      let mut pair = s.split(",").map(|s| s.trim().parse::<u32>());
      let x = pair.next().unwrap().unwrap();
      let y = pair.next().unwrap().unwrap();
      (x, y)
    });

    let mut pos = parts.next().unwrap();
    for part in parts {
      let dif = (part.0 as i32 - pos.0 as i32, part.1 as i32 - pos.1 as i32);
      let dif_dir = (dif.0.signum(), dif.1.signum());
      while pos != part {
        map.insert(pos, Tile::Stone);
        pos = (
          (pos.0 as i32 + dif_dir.0) as u32,
          (pos.1 as i32 + dif_dir.1) as u32,
        );
      }
      map.insert(pos, Tile::Stone);
      pos = part;
    }
  }
  let mut width = (500, 500);
  let mut depth = (0, 0);
  // let first_key = map.keys().next().unwrap();
  // width.0 = first_key.0;
  // width.1 = first_key.0;
  // hight.0 = first_key.1;
  // hight.1 = first_key.1;
  for key in map.keys() {
    if key.0 < width.0 {
      width.0 = key.0;
    }
    if key.0 > width.1 {
      width.1 = key.0;
    }
    if key.1 < depth.0 {
      depth.0 = key.1;
    }
    if key.1 > depth.1 {
      depth.1 = key.1;
    }
  }

  let mut done = false;
  let mut units = 0;
  while !done {
    let mut spawn_pos = (500, 0);
    loop {
      let mut new_pos = (spawn_pos.0, spawn_pos.1 + 1);
      // println!("Loop:   ");
      // println!("NewPos: {:?}", new_pos);
      if new_pos.1 > depth.1 + 1 {
        // println!("Out of bounds");
        break;
      } else {
        match map.get(&new_pos) {
          Some(Tile::Stone) => {
            // Check if left is empty
            let left_pos = (new_pos.0 - 1, new_pos.1);
            let left = map.get(&left_pos);
            let right_pos = (new_pos.0 + 1, new_pos.1);
            let right = map.get(&right_pos);
            if left.is_none() {
              new_pos = left_pos;
            } else if right.is_none() {
              new_pos = right_pos;
            } else {
              // println!("Blocked");
              break;
            }
          }
          Some(Tile::Sand) => {
            // Check if left is empty
            let left_pos = (new_pos.0 - 1, new_pos.1);
            let left = map.get(&left_pos);
            let right_pos = (new_pos.0 + 1, new_pos.1);
            let right = map.get(&right_pos);
            if left.is_none() {
              new_pos = left_pos;
            } else if right.is_none() {
              new_pos = right_pos;
            } else {
              break;
            }
          }
          _ => (),
        }
      }

      spawn_pos = new_pos;
    }

    if spawn_pos.0 == 500 && spawn_pos.1 == 0 {
      map.insert(spawn_pos, Tile::Sand);
      done = true;
      break;
    }
    // println!("{}: {:?}", units, spawn_pos);
    if done {
      // print_map(&map, width, depth);
    } else {
      // print!("{:?}: ", spawn_pos);
      map.insert(spawn_pos, Tile::Sand);
      units += 1;
    }
  }
  // print_map(&map, width, depth);
  let count = map.values().filter(|t| **t == Tile::Sand).count();
  println!("Day 14: ??? {}", count);
}
