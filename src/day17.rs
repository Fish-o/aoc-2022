use std::{cmp::max, collections::HashMap};

/*
  ####

  .#.
  ###
  .#.

  ..#
  ..#
  ###

  #
  #
  #
  #

  ##
  ##
*/
const ROCKS: [Rock; 5] = [
  Rock {
    shape: [0b0000, 0b0000, 0b0000, 0b1111],
    width: 4,
    height: 1,
  },
  Rock {
    shape: [0b0000, 0b0100, 0b1110, 0b0100],
    width: 3,
    height: 3,
  },
  Rock {
    shape: [0b0000, 0b0010, 0b0010, 0b1110],
    width: 3,
    height: 3,
  },
  Rock {
    shape: [0b1000, 0b1000, 0b1000, 0b1000],
    width: 1,
    height: 4,
  },
  Rock {
    shape: [0b0000, 0b0000, 0b1100, 0b1100],
    width: 2,
    height: 2,
  },
];
struct Rock {
  shape: [u8; 4],
  width: usize,
  height: usize,
}

struct CacheObject {
  heights: [u32; 7],
  started_at_i: usize,
  started_at_block: usize,
  height_added: [u32; 7],
}

const CAVE_HEIGHT: usize = 1000;
pub fn run(input: String) {
  print!("Day 17: ");
  solve(&input, 2022);
  println!(" (takes ~10 hours)");
  // solve(&input, 1000000000000);
}
pub fn solve(input: &String, blocks: usize) {
  // 2022 rocks, avg height of ~3
  // 7 units wide, 6066 tall
  /*
    Each rock appears so that its left edge is two units away from the left wall
    and its bottom edge is three units above the highest rock in the room
    (or the floor, if there isn't one).
  */
  let mut cave = [0u8; CAVE_HEIGHT];
  let mut highest_local_rock = 0;
  let mut lines_shifted = 0;

  let pulses = input.as_bytes();
  let pulse_count = pulses.len();
  let mut i = 0;
  // Time how long it takes
  let start = std::time::Instant::now();
  let up_to = blocks;
  for rock_i in 0..up_to {
    if rock_i % 10_000_000 == 0 && rock_i > 0 {
      println!(
        "still calculating rocks... {:?} seconds, {}% done",
        start.elapsed(),
        ((rock_i as f64) / (up_to as f64)) * 100.0
      );
    }
    if highest_local_rock > CAVE_HEIGHT - 100 {
      // Shift the cave down so that the top 100 rows still exist
      let lines_to_clear = highest_local_rock - 100;
      // println!("{} {}", rock_i, lines_to_clear);
      if lines_to_clear > 0 {
        for i in 0..(CAVE_HEIGHT - lines_to_clear) {
          cave[i] = cave[i + lines_to_clear];
        }
        for i in (CAVE_HEIGHT - lines_to_clear)..CAVE_HEIGHT {
          cave[i] = 0;
        }
        highest_local_rock -= lines_to_clear;
        lines_shifted += lines_to_clear;
      }
    }

    let mut y = highest_local_rock + 3;
    let mut x = 2;
    let rock = &ROCKS[rock_i % 5];
    'outer: loop {
      // println!("Loop{}", rock_i);
      // display_cave(rock, x, y, &cave);

      // Move rock left/right
      let pulse = pulses[i % pulse_count];
      let old_x = x;
      if pulse == b'<' {
        if x > 0 {
          x -= 1
        }
      } else if pulse == b'>' {
        if x + rock.width < 7 {
          x += 1
        }
      }

      // println!("Pulse {}: {} ({} -> {})", i, pulse, x, new_x);

      let mut collided = false;
      // Check for collisions
      for i in 0..4 {
        let rock_slice = rock.shape[i] << 3;
        let rock_slice = rock_slice >> x;
        let cave_slice = cave[y + (3 - i)];
        let intersect = cave_slice & rock_slice;
        if intersect > 0 {
          x = old_x;
          break;
        }
      }
      i += 1;

      if y == 0 {
        break;
      }

      let old_y = y;
      y = y - 1;
      for i in 0..4 {
        let rock_slice = (rock.shape[i] << 3) >> x;
        let cave_slice = cave[y + (3 - i)];
        let intersect: u8 = cave_slice & rock_slice;
        if intersect > 0 {
          y = old_y;
          break 'outer;
        }
      }
    }
    highest_local_rock = max(y + rock.height as usize, highest_local_rock);
    // Store rock
    for i in 0..4 {
      let rock_slice = rock.shape[i] << 3;
      let rock_slice = rock_slice >> x;
      let cave_slice = cave[y + (3 - i)];
      cave[y + (3 - i)] = cave_slice | rock_slice;
    }
  }
  let elapsed = start.elapsed();
  // println!("Time: {:?}", elapsed);
  // println!("Height: {}", lines_shifted + highest_local_rock);
  print!("{}", lines_shifted + highest_local_rock);
}

fn display_cave(rock: &Rock, x: usize, y: usize, cave: &[u32; 6066]) {
  // println!("Cave: {rock_i}");
  for (disp_y, slice) in cave.iter().take(10).enumerate().rev() {
    println!(
      "|{}|",
      format!("{:07b}", slice)
        .chars()
        .enumerate()
        .map(|(disp_x, c)| match c {
          '0' => {
            // Get the rock tile at this pos
            if disp_y < y || disp_y >= y + rock.height as usize {
              " "
            } else {
              let slice = rock.shape[3 - (disp_y - y)] << 3;
              let slice = slice >> x;
              match slice & (1 << (6 - disp_x)) {
                0 => ".",
                _ => "o",
              }
            }
          }
          '1' => "#",
          _ => unreachable!(),
        })
        .collect::<String>()
    );
  }
}
// 1000 -> ms
// 1000_000 -> s
// 1000_000_000 -> hour
// 1000_000_000_000 -> weeks

// 1000 -> µs
// 1000_000 -> ms
// 1000_000_000 -> s
// 1000_000_000_000 -> hours
/*
Cave after 3 rocks
|       | 10
|       | 9
|       | 8
|       | 7
|  #    | 6
|  #    | 5
|####   | 4
|  ###  | 3
|   #   | 2
|  #### | 1
Loop3
|..o....|
|  1    |
|  2    |
|  3    |
|  #    |
|  #    |
|####   |
|  ###  |
|   #   |
|  #### |
Pulse: > (2 -> 3)
Loop3
|...o...|
|...o...|
|   1   |
|   2   |
|  #    |
|  #    |
|####   |
|  ###  |
|   #   |
|  #### |
Pulse: > (3 -> 4)
Loop3
|....o..|
|....o..|
|....o..|
|    1  |
|  #    |
|  #    |
|####   |
|  ###  |
|   #   |
|  #### |
Pulse: < (4 -> 3)
Loop3
|...o...|
|...o...|
|...o...|
|...o...|
|  #    |
|  #    |
|####   |
|  ###  |
|   #   |
|  #### |
Pulse: > (3 -> 4)
Loop3
|       |
|....o..|
|....o..|
|....o..|
|..#.o..|
|  #    |
|####   |
|  ###  |
|   #   |
|  #### |
Pulse: > (4 -> 5)
Loop3
|       |
|       |
|.....o.|
|.....o.|
|..#..o.|
|..#..o.|
|####   |
|  ###  |
|   #   |
|  #### |
Pulse: > (5 -> 6)
Loop3
|       |
|       |
|       |
|......o|
|..#...o|
|..#...o|
|####..o|
|  ###  |
|   #   |
|  #### |
Pulse: < (6 -> 5)
Loop3
|       |
|       |
|       |
|       |
|..#..o.|
|..#..o.|
|####.o.|
|..###o.|
|   #   |
|  #### |
Pulse: < (5 -> 4)
Cave after 4 rocks
|       | 10
|       | 9
|       | 8
|       | 7
|  #  # | 6
|  #  # | 5
|#### # | 4
|  #### | 3
|   #   | 2
|  #### | 1
Loop4
|..oo...|
|       |
|       |
|       |
|  #  # |
|  #  # |
|#### # |
|  #### |
|   #   |
|  #### |
Pulse: < (2 -> 1)
Loop4
|.oo....|
|.oo....|
|       |
|       |
|  #  # |
|  #  # |
|#### # |
|  #### |
|   #   |
|  #### |
Pulse: < (1 -> 0)
Loop4
|       |
|oo.....|
|oo.....|
|       |
|  #  # |
|  #  # |
|#### # |
|  #### |
|   #   |
|  #### |
Pulse: > (0 -> 1)
Loop4
|       |
|       |
|.oo....|
|.oo....|
|  #  # |
|  #  # |
|#### # |
|  #### |
|   #   |
|  #### |
Pulse: > (1 -> 2)
Cave after 5 rocks
|       | 10
|       | 9
|  ##   | 8
|  ##   | 7
|  #  # | 6
|  #  # | 5
|#### # | 4
|  #### | 3
|   #   | 2
|  #### | 1
Height: 8

 */
