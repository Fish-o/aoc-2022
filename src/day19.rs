use std::cmp::max;
use std::sync::{Arc, Mutex};
use std::thread;

struct Blueprint {
  id: u32,
  pub ore_robot: Price,
  pub clay_robot: Price,
  pub obsidian_robot: Price,
  pub geode_robot: Price,
}

#[derive(Debug)]
struct Price {
  ore: u32,
  clay: u32,
  obsidian: u32,
}

impl Price {
  pub fn new(ore: u32, clay: u32, obsidian: u32) -> Price {
    Price {
      ore,
      clay,
      obsidian,
    }
  }
  pub fn can_afford(&self, ore: u32, clay: u32, obsidian: u32) -> bool {
    ore >= self.ore && clay >= self.clay && obsidian >= self.obsidian
  }
  pub fn resources_needed(&self, valuables: &(u32, u32, u32, u32)) -> (u32, u32, u32) {
    (
      if self.ore >= valuables.0 {
        self.ore - valuables.0
      } else {
        0
      },
      if self.clay >= valuables.1 {
        self.clay - valuables.1
      } else {
        0
      },
      if self.obsidian >= valuables.2 {
        self.obsidian - valuables.2
      } else {
        0
      },
    )
  }
}
#[derive(Debug, Clone)]
struct Tick {
  ore: u32,
  clay: u32,
  obsidian: u32,
  geode: u32,

  ore_robots: u32,
  clay_robots: u32,
  obsidian_robots: u32,
  geode_robots: u32,

  best_leaf_count: Option<u32>,
  t: u32,
  next: Vec<Tick>,
}

impl Tick {
  pub fn new(
    bp: &Blueprint,
    valuables: (u32, u32, u32, u32),
    robots: (u32, u32, u32, u32),
    t: u32,
    max_t: u32,
  ) -> Tick {
    // if t > 0 {
    //   panic!();
    // }
    // println!("Tick: {:?}", t);
    let (ore, clay, obsidian, geode) = valuables;
    let (ore_robots, clay_robots, obsidian_robots, geode_robots) = robots;

    // Possible buy actions:
    // 1. Buy ore robot
    // 2. Buy clay robot
    // 3. Buy obsidian robot
    // 4. Buy geode robot
    // 5. Do nothing

    let mut options = vec![];
    // ORE_ROBOT
    for (cost, new_robots) in [
      (
        &bp.clay_robot,
        (ore_robots, clay_robots + 1, obsidian_robots, geode_robots),
      ),
      (
        &bp.ore_robot,
        (ore_robots + 1, clay_robots, obsidian_robots, geode_robots),
      ),
      (
        &bp.obsidian_robot,
        (ore_robots, clay_robots, obsidian_robots + 1, geode_robots),
      ),
      (
        &bp.geode_robot,
        (ore_robots, clay_robots, obsidian_robots, geode_robots + 1),
      ),
    ] {
      let (ore_needed, clay_needed, obsidian_needed) = cost.resources_needed(&valuables);
      let mut ore_ticks = 0;
      let mut clay_ticks = 0;
      let mut obsidian_ticks = 0;
      if (ore_needed > 0 && ore_robots == 0)
        || (clay_needed > 0 && clay_robots == 0)
        || (obsidian_needed > 0 && obsidian_robots == 0)
      {
        continue;
      }
      if ore_needed > 0 {
        ore_ticks = (ore_needed + (ore_robots - 1)) / ore_robots
      }
      if clay_needed > 0 {
        clay_ticks = (clay_needed + (clay_robots - 1)) / clay_robots;
      }
      if obsidian_needed > 0 {
        obsidian_ticks = (obsidian_needed + (obsidian_robots - 1)) / obsidian_robots;
      }

      let time_spent = 1 + max(max(ore_ticks, clay_ticks), obsidian_ticks);

      if time_spent + t <= max_t {
        let new_valuables = (
          ore + (ore_robots * time_spent) - cost.ore,
          clay + (clay_robots * time_spent) - cost.clay,
          obsidian + (obsidian_robots * time_spent) - cost.obsidian,
          geode + (geode_robots * time_spent),
        );
        // println!(
        //   "Buying {:?} at t={} for {:?} (new_valuables={:?})",
        //   cost, t, time_spent, new_valuables
        // );
        options.push(Tick::new(
          bp,
          new_valuables,
          new_robots,
          t + time_spent,
          max_t,
        ))
      }
    }

    if options.len() == 0 && t < max_t {
      let time_left = max_t - t;
      let (new_ore, new_clay, new_obsidian, new_geode) = (
        ore + (ore_robots * time_left),
        clay + (clay_robots * time_left),
        obsidian + (obsidian_robots * time_left),
        geode + (geode_robots * time_left),
      );
      options.push(Tick {
        ore: new_ore,
        clay: new_clay,
        obsidian: new_obsidian,
        geode: new_geode,
        ore_robots,
        clay_robots,
        obsidian_robots,
        geode_robots,
        t: t + time_left,
        next: vec![],
        best_leaf_count: Some(new_geode),
      })
    }

    // Filter the worst options out
    // let options = options
    //   .into_iter()
    //   .filter(|tick| tick.t < 20 || tick.geode > 0)
    //   .collect::<Vec<_>>();
    if options.len() > 0 {
      let mut best_option = Tick::get_best_option(options);
      let best_leaf_count = best_option.get_best_leaf_count();
      Tick {
        ore,
        clay,
        obsidian,
        geode,
        ore_robots,
        clay_robots,
        obsidian_robots,
        geode_robots,
        t,
        next: vec![best_option],
        best_leaf_count: Some(best_leaf_count),
      }
    } else {
      Tick {
        ore,
        clay,
        obsidian,
        geode,
        ore_robots,
        clay_robots,
        obsidian_robots,
        geode_robots,
        t,
        next: vec![],
        best_leaf_count: Some(geode),
      }
    }
  }
  pub fn get_best_option(options: Vec<Tick>) -> Tick {
    let mut options = options;
    let mut best: Option<Tick> = None;
    let mut best_count = 0;
    for tick in &mut options {
      let best_leaf_count = tick.get_best_leaf_count();
      if best.is_none() || best_leaf_count > best_count {
        best = Some(tick.clone());
        best_count = best_leaf_count;
      }
    }
    return best.unwrap();
  }
  pub fn get_best_leaf(&self) -> &Tick {
    if self.next.len() == 0 {
      return self;
    } else {
      let mut best = self;
      for tick in &self.next {
        let best_leaf = tick.get_best_leaf();
        if best_leaf.geode > best.geode {
          best = &best_leaf;
        }
      }
      return best;
    }
  }
  pub fn get_leaf_count(&self) -> u32 {
    if self.next.len() == 0 {
      return 1;
    } else {
      let mut count = 0;
      for tick in &self.next {
        count += tick.get_leaf_count();
      }
      return count;
    }
  }
  pub fn get_best_leaf_count(&mut self) -> u32 {
    if self.next.len() == 0 {
      return self.geode;
    } else if self.best_leaf_count.is_some() {
      return self.best_leaf_count.unwrap();
    } else {
      let mut best = 0;
      for tick in &mut self.next {
        let best_leaf_count = tick.get_best_leaf_count();
        if best_leaf_count > best {
          best = best_leaf_count;
        }
      }
      self.best_leaf_count = Some(best);
      return best;
    }
  }
  pub fn get_all_robot_count(&self) -> Vec<u32> {
    let mut counts =
      vec![self.ore_robots + self.clay_robots + self.obsidian_robots + self.geode_robots];
    for tick in &self.next {
      let mut tick_counts = tick.get_all_robot_count();
      counts.append(&mut tick_counts);
    }
    counts
  }
}

pub fn run(input: String) {
  let blueprints = generate_blueprints(&input);
  let bp1 = &blueprints[0];
  let now = std::time::Instant::now();
  let tick = Tick::new(bp1, (0, 0, 0, 0), (1, 0, 0, 0), 0, 28);
  println!("Took {:?}", now.elapsed());
  println!("Best leaf: {:?}", tick.get_best_leaf());
  panic!();
  let blueprint_count = blueprints.len();
  // Spawn a process thread for each blueprint and return the best geode count
  let arc = Arc::new(Mutex::new(0));
  let finished_count = Arc::new(Mutex::new(0));
  let threads = blueprints
    .into_iter()
    .map(|bp| {
      let arc_clone = Arc::clone(&arc);
      let finished_count = Arc::clone(&finished_count);
      thread::spawn(move || {
        let best = Tick::new(&bp, (0, 0, 0, 0), (1, 0, 0, 0), 0, 24).get_best_leaf_count();
        println!("Best for {:?} is {}", bp.id, best);

        let mut arc = arc_clone.lock().unwrap();
        (*arc) += bp.id * best;

        let mut finished_count = finished_count.lock().unwrap();
        (*finished_count) += 1;
        println!("Finished {}/{}", *finished_count, blueprint_count);
      })
    })
    .collect::<Vec<_>>();
  for handle in threads {
    let res = handle.join();
    if res.is_err() {
      println!("Error: {:?}", res);
    }
  }
  println!("Result is: {:?}", arc.lock().unwrap());
}

fn generate_blueprints(input: &String) -> Vec<Blueprint> {
  input
    .split("\n")
    .map(|bp_str| bp_str.split(": "))
    .into_iter()
    .map(|mut bp_parts| (bp_parts.next().unwrap(), bp_parts.next().unwrap()))
    .map(|(id_str, prices_str)| (id_str.split(" ").last().unwrap(), prices_str.split(". ")))
    .map(|(id_str, mut prices)| {
      let id = id_str.parse::<u32>().unwrap();
      let ore_robot = prices
        .next()
        .unwrap()
        .split(" ")
        .skip(4)
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
      let clay_robot = prices
        .next()
        .unwrap()
        .split(" ")
        .skip(4)
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
      let mut obsidian_robot_parts = prices.next().unwrap().split(" and ");
      let obsidian_ore = obsidian_robot_parts
        .next()
        .unwrap()
        .split(" ")
        .skip(4)
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
      let obsidian_clay = obsidian_robot_parts
        .next()
        .unwrap()
        .split(" ")
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
      let mut geode_robot_parts = prices.next().unwrap().split(" and ");
      let geode_ore = geode_robot_parts
        .next()
        .unwrap()
        .split(" ")
        .skip(4)
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
      let geode_obsidian = geode_robot_parts
        .next()
        .unwrap()
        .split(" ")
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();

      Blueprint {
        id,
        ore_robot: Price::new(ore_robot, 0, 0),
        clay_robot: Price::new(clay_robot, 0, 0),
        obsidian_robot: Price::new(obsidian_ore, obsidian_clay, 0),
        geode_robot: Price::new(geode_ore, 0, geode_obsidian),
      }
    })
    .collect()
}
