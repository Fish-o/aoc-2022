use std::cmp::max;

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
#[derive(Debug)]
struct Tick {
  ore: u32,
  clay: u32,
  obsidian: u32,
  geode: u32,

  ore_robots: u32,
  clay_robots: u32,
  obsidian_robots: u32,
  geode_robots: u32,

  t: u32,
  next: Vec<Tick>,
}

impl Tick {
  pub fn new(
    bp: &Blueprint,
    valuables: (u32, u32, u32, u32),
    robots: (u32, u32, u32, u32),
    t: u32,
    pause_t: u32,
    max_t: u32,
  ) -> Tick {
    // println!("Tick: {:?}", t);
    if t > pause_t {
      return Tick {
        ore: valuables.0,
        clay: valuables.1,
        obsidian: valuables.2,
        geode: valuables.3,
        ore_robots: robots.0,
        clay_robots: robots.1,
        obsidian_robots: robots.2,
        geode_robots: robots.3,
        t,
        next: vec![],
      };
    }
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
          pause_t,
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
      })
    }

    // Filter the worst options out
    // let options = options
    //   .into_iter()
    //   .filter(|tick| tick.t < 20 || tick.geode > 0)
    //   .collect::<Vec<_>>();
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
      next: options,
    }
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

  pub fn get_all_robot_count(&self) -> Vec<u32> {
    let mut counts =
      vec![self.ore_robots + self.clay_robots + self.obsidian_robots + self.geode_robots];
    for tick in &self.next {
      let mut tick_counts = tick.get_all_robot_count();
      counts.append(&mut tick_counts);
    }
    counts
  }

  // pub fn prune(&mut self, below: u32) {
  //   let mut new_next = vec![];
  //   for tick in self.next. {
  //     tick.prune(below);

  //     if tick.ore_robots + tick.clay_robots + tick.obsidian_robots + tick.geode_robots >= below
  //       || tick.geode_robots > 0
  //     {
  //       new_next.push(tick);
  //     }
  //   }
  //   self.next = new_next;
  // }
}

pub fn run(input: String) {
  let bp1 = Blueprint {
    id: 1,
    ore_robot: Price::new(4, 0, 0),
    clay_robot: Price::new(2, 0, 0),
    obsidian_robot: Price::new(3, 14, 0),
    geode_robot: Price::new(2, 0, 7),
  };
  let bp2 = Blueprint {
    id: 2,
    ore_robot: Price::new(2, 0, 0),
    clay_robot: Price::new(3, 0, 0),
    obsidian_robot: Price::new(3, 8, 0),
    geode_robot: Price::new(3, 0, 12),
  };
  println!("Creating tick 1...");
  let time = std::time::Instant::now();
  let tick = Tick::new(&bp2, (0, 0, 0, 0), (1, 0, 0, 0), 0, 26, 23);

  println!("Created tick 1 in {:?}", time.elapsed());
  let leaf_count = tick.get_leaf_count();
  println!("Leaf count: {}", leaf_count);
  let mut all_counts = tick.get_all_robot_count();
  all_counts.sort();
  let best_20 = all_counts[all_counts.len() - (all_counts.len() / 5)..].to_vec();

  // let best1 = tick.get_best_leaf();
  // println!("Best tick 1: {:?}", best1);

  /*
    println!("Creating tick 2...");
    let time = std::time::Instant::now();
    let tick2 = Tick::new(&bp2, (0, 0, 0, 0), (1, 0, 0, 0), 0, 24);
    println!("Created tick 2 in {:?}", time.elapsed());
    let best2 = tick2.get_best_leaf();
    println!("Best tick 2: {:?}", best2);
  */
}
// 19 ->   8595059     536.085193ms
// 20 ->  32076315   1.887915484s
// 21 -> 122891409  14.880998946s
