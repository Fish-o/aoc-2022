use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Pos {
  x: i32,
  y: i32,
}
impl Pos {
  pub fn distance_to(&self, other: &Pos) -> i32 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }

  pub fn from(x: i32, y: i32) -> Pos {
    Pos { x, y }
  }
}

struct Sensor {
  pos: Pos,
  beacon: Pos,
  range: i32,
}
impl Sensor {
  pub fn get_edges(&self) -> HashSet<(i32, i32)> {
    let mut edges = HashSet::new();
    for x_disp in (-self.range - 1)..=self.range + 1 {
      let y_disp_1 = (self.range + 2) - x_disp;
      let y_disp_2 = -(self.range + 2) + x_disp;
      edges.insert((self.pos.x + x_disp, self.pos.y + y_disp_1));
      edges.insert((self.pos.x + x_disp, self.pos.y + y_disp_2));
    }
    edges
  }
}
impl Sensor {
  pub fn new(pos: Pos, beacon: Pos) -> Sensor {
    Sensor {
      range: pos.distance_to(&beacon),
      pos,
      beacon,
    }
  }
  pub fn get_row_no_beacon_spots(&self, y: i32) -> Option<(i32, i32)> {
    let y_diff = (self.pos.y - y).abs();
    if y_diff > self.range {
      return None;
    }
    let x_range = self.range - y_diff;
    Some((self.pos.x - x_range, self.pos.x + x_range))
  }
}
fn print(sensors: &Vec<Sensor>) {
  let min_x = sensors
    .iter()
    .flat_map(|sensor| vec![sensor.pos.x, sensor.beacon.x])
    .min()
    .unwrap();
  let max_x = sensors
    .iter()
    .flat_map(|sensor| vec![sensor.pos.x, sensor.beacon.x])
    .max()
    .unwrap();
  let min_y = sensors
    .iter()
    .flat_map(|sensor| vec![sensor.pos.y, sensor.beacon.y])
    .min()
    .unwrap();
  let max_y = sensors
    .iter()
    .flat_map(|sensor| vec![sensor.pos.y, sensor.beacon.y])
    .max()
    .unwrap();
  println!("{} {} {} {}", min_x, max_x, min_y, max_y);

  let x_range = (max_x - min_x) as usize + 5;
  let y_range = (max_y - min_y) as usize + 5;
  let min_x = min_x - 2;
  let min_y = min_y - 2;

  let mut grid = vec![vec!['.'; x_range]; y_range];
  for sensor in sensors {
    for y in (sensor.pos.y - sensor.range)..=(sensor.pos.y + sensor.range) {
      for x in (sensor.pos.x - sensor.range)..=(sensor.pos.x + sensor.range) {
        if sensor.pos.distance_to(&Pos::from(x, y)) <= sensor.range {
          if x < min_x || x > max_x || y < min_y || y > max_y {
            continue;
          }
          grid[(y - min_y) as usize][(x - min_x) as usize] = '#';
        }
      }
    }
    grid[(sensor.pos.y - min_y) as usize][(sensor.pos.x - min_x) as usize] = 'S';
    grid[(sensor.beacon.y - min_y) as usize][(sensor.beacon.x - min_x) as usize] = 'B';
  }
  println!();
  let numbs = (min_x..=max_x)
    .map(|x| format!("{:02}", x))
    .collect::<Vec<_>>();
  for i in 0..numbs[0].len() {
    print!("   ");
    for numb in &numbs {
      if numb.parse::<i32>().unwrap() % 5 == 0 {
        let digit = numb.chars().nth(i).unwrap();
        if digit == '0' && i == 0 {
          print!(" ");
        } else {
          print!("{}", digit);
        }
      } else {
        print!(" ");
      }
    }
    println!();
  }
  println!();
  for (i, row) in grid.iter().enumerate() {
    print!("{:02} ", min_y + i as i32);
    for col in row {
      print!("{}", col);
    }
    println!();
  }
}

pub fn run(input: String) {
  let lines = input.lines();
  // Match the X and Y coordinates in "Sensor at x=2388052, y=2201955: closest beacon is at x=2163809, y=1961540"
  let regex = regex::Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();

  let sensors: Vec<Sensor> = lines
    .map(|line| {
      let mut caps = regex.captures_iter(line);
      let sensor = caps.next().unwrap();
      // println!("Sensor: {:?}", sensor);
      let beacon = caps.next().unwrap();
      // println!("Beacon: {:?}", beacon);
      let sensor = Pos::from(sensor[1].parse().unwrap(), sensor[2].parse().unwrap());
      let beacon = Pos::from(beacon[1].parse().unwrap(), beacon[2].parse().unwrap());
      Sensor::new(sensor, beacon)
    })
    .collect();
  // print(&sensors);
  let mut taken_spaces = sensors
    .iter()
    .map(|sensor| sensor.get_row_no_beacon_spots(2000000))
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect::<Vec<_>>();

  taken_spaces.sort_by(|(min_1, _), (min_2, _)| min_1.cmp(min_2));
  // println!("{:?}", taken_spaces);

  let mut to_delete = Vec::new();
  let taken_spaces2 = taken_spaces.clone();
  for i in 0..taken_spaces.len() {
    let cur = taken_spaces[i];
    let next = taken_spaces2.get(i + 1);
    if let Some(next) = next {
      if cur.1 >= next.0 {
        to_delete.push(i);
        taken_spaces[i + 1] = (cur.0, next.1.max(cur.1));
      }
    }
  }
  // println!("{:?}", to_delete);
  let taken_spaces = taken_spaces
    .into_iter()
    .enumerate()
    .filter(|(i, _)| !to_delete.contains(i))
    .map(|(_, x)| x)
    .collect::<Vec<_>>();
  // println!("{:?}", taken_spaces);
  let beacons: Vec<_> = sensors
    .iter()
    .map(|sensor| sensor.beacon)
    .filter(|beacon| beacon.y == 2000000)
    .map(|beacon| beacon.x)
    .collect::<std::collections::BTreeSet<_>>()
    .into_iter()
    .collect();

  let mut tot_spaces = 0;

  for x in beacons {
    if !taken_spaces.iter().any(|(min, max)| x >= *min && x <= *max) {
      // println!("{} is not taken", x);
    } else {
      // println!("{} is taken", x);
      tot_spaces -= 1;
    }
  }
  for (min, max) in &taken_spaces {
    // println!("{} - {}", min, max);
    tot_spaces += (max - min) + 1;
  }

  // println!("{:?}", taken_spaces);

  print!("Day 15: {}", tot_spaces);
  solve2(&sensors);
  // taken_spaces.sort_by(|(min_1, _), (min_2, _)| min_1.cmp(min_2));
}

pub fn get_row_ranges(row: i32, sensors: &Vec<Sensor>) -> Vec<(i32, i32)> {
  let mut taken_spaces = sensors
    .iter()
    .map(|sensor| sensor.get_row_no_beacon_spots(row))
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect::<Vec<_>>();

  taken_spaces.sort_by(|(min_1, _), (min_2, _)| min_1.cmp(min_2));
  // println!("{:?}", taken_spaces);

  let mut to_delete = Vec::new();
  let taken_spaces2 = taken_spaces.clone();
  for i in 0..taken_spaces.len() {
    let cur = taken_spaces[i];
    let next = taken_spaces2.get(i + 1);
    if let Some(next) = next {
      if cur.1 >= next.0 {
        to_delete.push(i);
        taken_spaces[i + 1] = (cur.0, next.1.max(cur.1));
      }
    }
  }
  // println!("{:?}", to_delete);
  let taken_spaces = taken_spaces
    .into_iter()
    .enumerate()
    .filter(|(i, _)| !to_delete.contains(i))
    .map(|(_, x)| x)
    .collect::<Vec<_>>();
  return taken_spaces;
}
// Withing 0 <= x <= 4_000_000 and 0 <= y <= 4_000_000
// Get all spaces that are not taken

// Does it make sense to rotate the board 45 degrees?
fn solve2(sensors: &Vec<Sensor>) {
  for row in 0..4_000_000 {
    let ranges = get_row_ranges(row, sensors);
    if ranges.len() > 1 {
      let y = row as i64;
      let x = ranges[0].1 as i64 + 1;
      // println!("{} {:?}", row, ranges)
      let tuning_freq = 4000000 as i64 * x + y;
      print!(" {tuning_freq}");
    }
  }
}
