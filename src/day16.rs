use std::{
  arch::x86_64,
  collections::HashMap,
  f32::consts::E,
  hash::Hash,
  mem::size_of_val,
  option,
  sync::{Arc, Mutex},
  thread,
};

use itertools::Itertools;
use petgraph::{
  dot::{Config, Dot},
  graph,
  prelude::UnGraphMap,
};
#[derive(Debug, Clone)]
struct Valve {
  id: String,
  flow_rate: u32,
}

pub fn run(input: String) {
  let rows = input.lines().collect::<Vec<_>>();
  let mut valves = vec![];
  let mut tunnels = HashMap::new();
  rows.iter().for_each(|f| {
    let parts = f.split(';').collect::<Vec<_>>();
    let valve_part = parts[0];
    let id = &valve_part[6..8];
    let flow_rate = valve_part[23..].parse::<u32>().unwrap();
    let valve = Valve {
      id: id.to_owned(),
      flow_rate,
    };
    valves.push(valve);
    let tunnel_part = parts[1];
    let tunnel_valves = if tunnel_part.contains("valves") {
      tunnel_part[24..]
        .split(", ")
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
    } else {
      vec![tunnel_part[23..].to_string()]
    };
    tunnels.insert(id.to_string(), tunnel_valves);
  });

  let mut flow_rates = HashMap::new();
  let mut dot_graph = UnGraphMap::<(&str, u32), ()>::new();
  let mut graph = Graph::new(valves.clone(), tunnels.clone());
  // println!("Created graph");
  let tables = graph.generate_initial_tables();
  // println!("Created tables");
  for valve in valves.iter() {
    flow_rates.insert(valve.id.to_string(), valve.flow_rate);
    dot_graph.add_node((valve.id.as_str(), valve.flow_rate));
  }
  for tunnel in &tunnels {
    let from = tunnel.0;

    let to = tunnel.1;
    for to in to {
      dot_graph.add_edge(
        (from, *flow_rates.get(from).unwrap()),
        (to, *flow_rates.get(to).unwrap()),
        (),
      );
    }
  }

  // let dot = Dot::with_config(&dot_graph, &[Config::EdgeNoLabel]);
  // println!("{:?}", dot);
  // panic!();
  // println!("Creating tree...");
  let functional_valves = &valves
    .iter()
    .filter(|v| v.flow_rate > 0)
    .cloned()
    .collect::<Vec<_>>();
  let tree = create_tree(
    functional_valves,
    &tables,
    "AA",
    HashMap::new(),
    0,
    0,
    0,
    0.0,
  );
  let best = tree.get_highest(30);
  println!("Day 16: {} (takes ~15 mins)", best.0);
  // println!("Creating elephant tree");
  // let elephant_tree = top_level_create_elephant(
  //   functional_valves,
  //   &tables,
  //   0,
  //   26,
  //   "AA",
  //   "AA",
  //   None,
  //   None,
  //   vec![],
  //   0,
  //   0,
  // );
  // let best_eventual_pressure = elephant_tree.get_eventual_best_pressure(26);
  // println!("{:#?}", elephant_tree);
  // println!("Day 16: {}", best_eventual_pressure);
  // println!("Memory size of tree {}", tree.mem_size());
  // println!("Nodes in tree {}", tree.get_node_count());
  // println!("Creating elephant tree");

  // let hh_table = tables.get("HH").unwrap();
  // println!("{:?}", hh_table);
  // // panic!();
  // let mut elephant_tree = create_doubly_traversed_tree(
  //   functional_valves,
  //   &tables,
  //   &"AA".to_owned(),
  //   &"AA".to_owned(),
  //   HashMap::new(),
  //   0,
  //   26,
  //   0,
  //   0,
  //   0.5,
  //   0,
  //   3,
  // );
  // println!("Memory size of elephant tree {}", elephant_tree.mem_size());
  // println!("Nodes in elephant tree {}", elephant_tree.get_node_count());

  // let mut releases = elephant_tree.get_all_pressures();
  // releases.sort();
  // releases.reverse();
  // let below = releases[(releases.len() as f32 * 0.3).ceil() as usize];

  // println!("Pruning leaves below {}", below);
  // let mut size = elephant_tree.get_node_count();
  // elephant_tree.prune(below);
  // while size != elephant_tree.get_node_count() {
  //   size = elephant_tree.get_node_count();
  //   println!("Pruning leaves below {}", below);
  //   elephant_tree.prune(below);
  // }
  // println!("Memory size of elephant tree {}", elephant_tree.mem_size());
  // println!("Nodes in elephant tree {}", elephant_tree.get_node_count());
  // println!("Unpausing elephant tree");
  // elephant_tree.unpause(functional_valves, &tables, 0.5, 26, 5);
  // println!("Memory size of elephant tree {}", elephant_tree.mem_size());
  // println!("Nodes in elephant tree {}", elephant_tree.get_node_count());

  // let mut releases = elephant_tree.get_all_pressures();
  // releases.sort();
  // releases.reverse();
  // let below = releases[(releases.len() as f32 * 0.1).ceil() as usize];

  // println!("Pruning leaves below {}", below);
  // let mut size = elephant_tree.get_node_count();
  // elephant_tree.prune(below);
  // while size != elephant_tree.get_node_count() {
  //   size = elephant_tree.get_node_count();
  //   println!("Pruning leaves below {}", below);
  //   elephant_tree.prune(below);
  // }
  // println!("Memory size of elephant tree {}", elephant_tree.mem_size());
  // println!("Nodes in elephant tree {}", elephant_tree.get_node_count());
  // println!("Unpausing elephant tree");
  // elephant_tree.unpause(functional_valves, &tables, 0.5, 26, 6);
  // println!("Memory size of elephant tree {}", elephant_tree.mem_size());
  // println!("Nodes in elephant tree {}", elephant_tree.get_node_count());

  // let mut releases = elephant_tree.get_all_pressures();
  // releases.sort();
  // releases.reverse();
  // let below = releases[(releases.len() as f32 * 0.05).ceil() as usize];

  // println!("Pruning leaves below {}", below);
  // let mut size = elephant_tree.get_node_count();
  // elephant_tree.prune(below);
  // while size != elephant_tree.get_node_count() {
  //   size = elephant_tree.get_node_count();
  //   println!("Pruning leaves below {}", below);
  //   elephant_tree.prune(below);
  // }
  // println!("Memory size of elephant tree {}", elephant_tree.mem_size());
  // println!("Nodes in elephant tree {}", elephant_tree.get_node_count());
  // println!("Unpausing elephant tree");
  // elephant_tree.unpause(functional_valves, &tables, 0.5, 26, 7);
  // println!("Memory size of elephant tree {}", elephant_tree.mem_size());
  // println!("Nodes in elephant tree {}", elephant_tree.get_node_count());

  // let mut releases = elephant_tree.get_all_pressures();
  // releases.sort();
  // releases.reverse();
  // let below = releases[(releases.len() as f32 * 0.01).ceil() as usize];

  // println!("Pruning leaves below {}", below);
  // let mut size = elephant_tree.get_node_count();
  // elephant_tree.prune(below);
  // while size != elephant_tree.get_node_count() {
  //   size = elephant_tree.get_node_count();
  //   println!("Pruning leaves below {}", below);
  //   elephant_tree.prune(below);
  // }
  // println!("Memory size of elephant tree {}", elephant_tree.mem_size());
  // println!("Nodes in elephant tree {}", elephant_tree.get_node_count());
  // println!("Unpausing elephant tree");
  // elephant_tree.unpause(functional_valves, &tables, 0.5, 26, 20);

  // println!("Getting best elephant leaf");
  // let best = elephant_tree.get_highest(26);

  // println!("{:#?}", best);
}
#[derive(Debug, Clone)]
struct TreeNode {
  opened: HashMap<String, u32>,
  t: u32,
  released_pressure: u32,
  pressure_release_rate: u32,
  nodes: Vec<TreeNode>,
  // depth: u32,
  // paused: bool,
  // pause_data: Option<(String, String)>,
}
impl TreeNode {
  fn mem_size(self: &Self) -> usize {
    return size_of_val(self) + self.nodes.iter().map(|v| v.mem_size()).sum::<usize>();
  }
  fn get_node_count(self: &Self) -> usize {
    return self.nodes.len() + self.nodes.iter().map(|v| v.get_node_count()).sum::<usize>() + 1;
  }
  fn get_highest(self: &Self, max_t: u32) -> (u32, u32, HashMap<String, u32>, TreeNode) {
    if self.nodes.len() == 0 {
      let pressure = self.released_pressure + (self.pressure_release_rate * (max_t - self.t));
      (
        pressure,
        self.pressure_release_rate,
        self.opened.clone(),
        self.clone(),
      )
    } else {
      self
        .nodes
        .iter()
        .map(|n| n.get_highest(max_t))
        .max_by_key(|f| f.0)
        .unwrap()
    }
  }
  fn get_released_pressure(self: &Self, max_t: u32) -> u32 {
    self.released_pressure + (self.pressure_release_rate * (max_t - self.t))
  }
}

fn create_tree(
  valves: &Vec<Valve>,
  tables: &HashMap<String, HashMap<String, (String, u32, u32)>>,
  current_valve: &str,
  opened: HashMap<String, u32>,
  t: u32,
  released_pressure: u32,
  pressure_release_rate: u32,
  prune: f32,
) -> TreeNode {
  // println!(
  //   "Creating tree node {} at depth {}",
  //   current_valve,
  //   opened.len()
  // );
  let table = tables.get(current_valve).unwrap();
  let mut options = vec![];
  for valve in valves {
    if valve.id == current_valve || opened.contains_key(&valve.id) || valve.flow_rate == 0 {
      continue;
    }
    let (via, time_cost, flow_rate) = table.get(&valve.id).unwrap();
    // println!(
    //   "Currently at {} going to {}, at t={}",
    //   current_valve, valve.id, t
    // );
    let potential = get_potential(*time_cost, *flow_rate, t, 30);
    if potential > 0 {
      options.push((valve.id.clone(), potential, time_cost, flow_rate));
    }
  }
  options.sort_by_key(|(_, potential, _, _)| *potential);
  options.reverse();
  let count = ((options.len() as f32) * (1.0 - prune)).ceil() as usize;
  // println!("Taking {} from {}", count, options.len());
  let pruned_options = options.drain(0..count);
  // println!("{:#?}", options);
  // panic!();
  let mut nodes = vec![];
  for option in pruned_options {
    let (id, potential, time_cost, flow_rate) = option;
    let new_t = t + time_cost;
    let mut new_opened = opened.clone();
    new_opened.insert(id.clone(), new_t);
    let new_pressure_release_rate = pressure_release_rate + flow_rate;
    let new_released_pressure = released_pressure + (pressure_release_rate * time_cost);
    let node = create_tree(
      valves,
      tables,
      id.as_str(),
      new_opened,
      new_t,
      new_released_pressure,
      new_pressure_release_rate,
      prune,
    );
    nodes.push(node);
  }

  TreeNode {
    opened,
    t,
    released_pressure,
    pressure_release_rate,
    nodes,
  }
}
struct Graph {
  valves: Vec<Valve>,
  edges: HashMap<String, Vec<String>>,
  tables: HashMap<String, HashMap<String, (String, u32, u32)>>,
}
impl Graph {
  pub fn new(valves: Vec<Valve>, edges: HashMap<String, Vec<String>>) -> Self {
    Self {
      valves,
      edges,
      tables: HashMap::new(),
    }
  }
  fn get_shortest_distance(
    self: &Self,
    n_1: &str,
    n_2: &str,
    current_path: Vec<String>,
  ) -> (String, u32) {
    // Use dijkstra's algorithm to find the shortest path between n_1 and n_2
    let distances = self.edges.get(n_1).unwrap().iter().map(|f| {
      let f = f.clone();
      if current_path.contains(&f) {
        return (f, u32::MAX);
      } else if f == n_2 {
        return (f, 1);
      }
      let mut new_path = current_path.clone();
      new_path.push(n_1.to_string());
      let (via, dist) = self.get_shortest_distance(&f, n_2, new_path);
      if dist == u32::MAX {
        return (f, u32::MAX);
      }
      (f, dist + 1)
    });
    distances.min_by_key(|f| f.1).unwrap()
  }
  fn get_routing_table(self: &Self, id: &str) -> HashMap<String, (String, u32, u32)> {
    let mut table = HashMap::new();
    for valve in &self.valves {
      if valve.flow_rate == 0 {
        continue;
      } else if valve.id == id {
        table.insert(valve.id.clone(), (valve.id.clone(), 1, valve.flow_rate));
      } else {
        let (via, dist) = self.get_shortest_distance(id, &valve.id, vec![]);
        table.insert(valve.id.clone(), (via, dist + 1, valve.flow_rate));
      }
    }
    table
  }
  pub fn generate_initial_tables(
    self: &mut Self,
    // starting_node: &str,
  ) -> HashMap<String, HashMap<String, (String, u32, u32)>> {
    let mut tables = HashMap::new();
    for valve in &self.valves {
      if valve.flow_rate != 0 || valve.id == "AA" {
        // println!("Creating routing table for valve {}", valve.id);
        tables.insert(valve.id.clone(), self.get_routing_table(&valve.id));
      }
    }
    self.tables = tables.clone();
    tables
  }
}

fn get_potential(time_cost: u32, flow_rate: u32, t: u32, max_t: u32) -> u32 {
  // println!("Path: {:?}", path);
  // println!("Done at t={}", t);
  if t + time_cost >= max_t {
    // println!("Potential 0");
    return 0;
  }
  let time_left = max_t - t;

  let potential = (time_left - time_cost) * flow_rate;
  // println!("Potential: {}", potential);
  potential
}
#[derive(Debug, Clone)]
struct ElephantTree {
  t: u32,
  pos_a: String,
  pos_b: String,
  pressure: u32,
  pressure_release_rate: u32,
  opened: Vec<String>,
  opening_a: Option<(u32, String, u32)>,
  opening_b: Option<(u32, String, u32)>,
  best_child: Option<Box<ElephantTree>>,
}
impl ElephantTree {
  pub fn get_eventual_best_pressure(self: &Self, max_t: u32) -> u32 {
    if self.t == max_t {
      return self.pressure;
    } else if let Some(best_child) = &self.best_child {
      return best_child.get_eventual_best_pressure(max_t);
    } else {
      return self.pressure + (self.pressure_release_rate * (max_t - self.t));
    }
  }
}

fn top_level_create_elephant(
  valves: &Vec<Valve>,
  tables: &HashMap<String, HashMap<String, (String, u32, u32)>>,
  t: u32,
  max_t: u32,
  pos_a: &str,
  pos_b: &str,
  // Dist, valve, flow_rate
  opening_a: Option<(u32, String, u32)>,
  // Dist, valve, flow_rate
  opening_b: Option<(u32, String, u32)>,
  opened: Vec<String>,
  pressure: u32,
  pressure_release_rate: u32,
) -> ElephantTree {
  if t == max_t {
    return ElephantTree {
      t,
      pos_a: pos_a.to_owned(),
      pos_b: pos_b.to_owned(),
      pressure,
      pressure_release_rate,
      opened,
      opening_a,
      opening_b,
      best_child: None,
    };
  }
  let a_goals = if let Some(valve) = opening_a.clone() {
    vec![valve.clone()]
  } else {
    // Add all unopened valves in range
    valves
      .iter()
      .filter(|v| {
        v.flow_rate > 0
          && !opened.contains(&v.id)
          && opening_b.clone().unwrap_or((0, "none".to_owned(), 0)).1 != v.id
      })
      .map(|v| {
        let time = tables.get(pos_a).unwrap().get(&v.id).unwrap();
        (time.1, v.id.clone(), v.flow_rate)
      })
      .filter(|v| v.0 > 0 && v.0 < max_t - t)
      .collect::<Vec<_>>()
  };
  let b_goals = if let Some(valve) = opening_b.clone() {
    vec![valve.clone()]
  } else {
    // Add all unopened valves in range
    valves
      .iter()
      .filter(|v| {
        v.flow_rate > 0
          && !opened.contains(&v.id)
          && opening_a.clone().unwrap_or((0, "none".to_owned(), 0)).1 != v.id
      })
      .map(|v| {
        let time = tables.get(pos_b).unwrap().get(&v.id).unwrap();
        (time.1, v.id.clone(), v.flow_rate)
      })
      .filter(|v| v.0 > 0 && v.0 < max_t - t)
      .collect::<Vec<_>>()
  };

  let mut best_child: Arc<Mutex<Option<ElephantTree>>> = Arc::new(Mutex::new(None));
  let mut best_eventual_pressure = Arc::new(Mutex::new(0));
  let mut total_goals = a_goals.len() * b_goals.len();
  let mut i = 0;
  let mut start = std::time::Instant::now();
  let mut now = std::time::Instant::now();
  let mut handles = vec![];
  let mut total_done = Arc::new(Mutex::new(0));
  for a_goal in &a_goals {
    for b_goal in &b_goals {
      i += 1;
      let estimated_end = (start.elapsed() / i) * total_goals as u32;
      println!("{i}/{total_goals} in {:?}", now.elapsed());
      println!("Estimated end in {:?}", estimated_end);
      now = std::time::Instant::now();
      if a_goal.1 == b_goal.1 {
        continue;
      }
      let a_goal = a_goal.clone();
      let b_goal = b_goal.clone();
      let valves = valves.clone();
      let tables = tables.clone();
      let pos_a = pos_a.to_owned();
      let pos_b = pos_b.to_owned();
      let best_eventual_pressure = best_eventual_pressure.clone();
      let best_child = best_child.clone();
      let opened = opened.clone();
      let i = i.clone();
      let total_done = total_done.clone();
      let total_goals = total_goals.clone();
      let handle = thread::spawn(move || {
        let child = if a_goal.0 == b_goal.0 {
          let time_traveled = a_goal.0;
          let new_t = t + time_traveled;
          let new_pos_a = a_goal.1.clone();
          let new_pos_b = b_goal.1.clone();
          let new_opening_a = None;
          let new_opening_b = None;
          let new_pressure = pressure + (pressure_release_rate * time_traveled);
          let new_pressure_release_rate = pressure_release_rate + a_goal.2 + b_goal.2;
          let new_opened = {
            let mut new_opened = opened.clone();
            new_opened.push(a_goal.1.clone());
            new_opened.push(b_goal.1.clone());
            new_opened
          };
          create_elephant_tree(
            &valves,
            &tables,
            new_t,
            max_t,
            new_pos_a.as_str(),
            new_pos_b.as_str(),
            new_opening_a,
            new_opening_b,
            new_opened,
            new_pressure,
            new_pressure_release_rate,
          )
        } else if a_goal.0 < b_goal.0 {
          let time_traveled = a_goal.0;
          let new_t = t + time_traveled;
          let new_pos_a = a_goal.1.clone();
          let new_pos_b = pos_b;
          let new_opening_a = None;
          let new_opening_b = Some((b_goal.0 - time_traveled, b_goal.1.clone(), b_goal.2));
          let new_pressure = pressure + (pressure_release_rate * time_traveled);
          let new_pressure_release_rate = pressure_release_rate + a_goal.2;
          let new_opened = {
            let mut new_opened = opened.clone();
            new_opened.push(a_goal.1.clone());
            new_opened
          };
          create_elephant_tree(
            &valves,
            &tables,
            new_t,
            max_t,
            new_pos_a.as_str(),
            &new_pos_b,
            new_opening_a,
            new_opening_b,
            new_opened,
            new_pressure,
            new_pressure_release_rate,
          )
        } else {
          let time_traveled = b_goal.0;
          let new_t = t + time_traveled;
          let new_pos_a = pos_a;
          let new_pos_b = b_goal.1.clone();
          let new_opening_a = Some((a_goal.0 - time_traveled, a_goal.1.clone(), a_goal.2));
          let new_opening_b = None;
          let new_pressure = pressure + (pressure_release_rate * time_traveled);
          let new_pressure_release_rate = pressure_release_rate + b_goal.2;
          let new_opened = {
            let mut new_opened = opened.clone();
            new_opened.push(b_goal.1.clone());
            new_opened
          };
          create_elephant_tree(
            &valves,
            &tables,
            new_t,
            max_t,
            &new_pos_a,
            new_pos_b.as_str(),
            new_opening_a,
            new_opening_b,
            new_opened,
            new_pressure,
            new_pressure_release_rate,
          )
        };
        let eventual_pressure = child.get_eventual_best_pressure(max_t);
        let mut best_eventual_pressure = best_eventual_pressure.lock().unwrap();
        if *best_eventual_pressure < eventual_pressure {
          *best_eventual_pressure = eventual_pressure;
          let mut best_child = best_child.lock().unwrap();
          *best_child = Some(child);
        }
        let mut total_done = total_done.lock().unwrap();
        *total_done += 1;
        println!("[{}] {}/{} Done!", i, total_done, total_goals);
      });
      handles.push(handle);
    }
  }
  for handle in handles {
    handle.join().unwrap();
  }
  let best_child: ElephantTree = best_child.lock().unwrap().clone().unwrap();
  ElephantTree {
    t,
    pos_a: pos_a.to_owned(),
    pos_b: pos_b.to_owned(),
    pressure,
    pressure_release_rate,
    opened,
    opening_a,
    opening_b,
    best_child: Some(Box::new(best_child)),
  }
}
fn create_elephant_tree(
  valves: &Vec<Valve>,
  tables: &HashMap<String, HashMap<String, (String, u32, u32)>>,
  t: u32,
  max_t: u32,
  pos_a: &str,
  pos_b: &str,
  // Dist, valve, flow_rate
  opening_a: Option<(u32, String, u32)>,
  // Dist, valve, flow_rate
  opening_b: Option<(u32, String, u32)>,
  opened: Vec<String>,
  pressure: u32,
  pressure_release_rate: u32,
) -> ElephantTree {
  if t == max_t {
    return ElephantTree {
      t,
      pos_a: pos_a.to_owned(),
      pos_b: pos_b.to_owned(),
      pressure,
      pressure_release_rate,
      opened,
      opening_a,
      opening_b,
      best_child: None,
    };
  }
  let a_goals = if let Some(valve) = opening_a.clone() {
    vec![valve.clone()]
  } else {
    // Add all unopened valves in range
    valves
      .iter()
      .filter(|v| {
        v.flow_rate > 0
          && !opened.contains(&v.id)
          && opening_b.clone().unwrap_or((0, "none".to_owned(), 0)).1 != v.id
      })
      .map(|v| {
        let time = tables.get(pos_a).unwrap().get(&v.id).unwrap();
        (time.1, v.id.clone(), v.flow_rate)
      })
      .filter(|v| v.0 > 0 && v.0 < max_t - t)
      .collect::<Vec<_>>()
  };
  let b_goals = if let Some(valve) = opening_b.clone() {
    vec![valve.clone()]
  } else {
    // Add all unopened valves in range
    valves
      .iter()
      .filter(|v| {
        v.flow_rate > 0
          && !opened.contains(&v.id)
          && opening_a.clone().unwrap_or((0, "none".to_owned(), 0)).1 != v.id
      })
      .map(|v| {
        let time = tables.get(pos_b).unwrap().get(&v.id).unwrap();
        (time.1, v.id.clone(), v.flow_rate)
      })
      .filter(|v| v.0 > 0 && v.0 < max_t - t)
      .collect::<Vec<_>>()
  };

  let mut best_child: Option<ElephantTree> = None;
  let mut best_eventual_pressure = 0;
  for a_goal in &a_goals {
    for b_goal in &b_goals {
      if a_goal.1 == b_goal.1 {
        continue;
      }

      let child = if a_goal.0 == b_goal.0 {
        let time_traveled = a_goal.0;
        let new_t = t + time_traveled;
        let new_pos_a = a_goal.1.clone();
        let new_pos_b = b_goal.1.clone();
        let new_opening_a = None;
        let new_opening_b = None;
        let new_pressure = pressure + (pressure_release_rate * time_traveled);
        let new_pressure_release_rate = pressure_release_rate + a_goal.2 + b_goal.2;
        let new_opened = {
          let mut new_opened = opened.clone();
          new_opened.push(a_goal.1.clone());
          new_opened.push(b_goal.1.clone());
          new_opened
        };
        create_elephant_tree(
          valves,
          tables,
          new_t,
          max_t,
          new_pos_a.as_str(),
          new_pos_b.as_str(),
          new_opening_a,
          new_opening_b,
          new_opened,
          new_pressure,
          new_pressure_release_rate,
        )
      } else if a_goal.0 < b_goal.0 {
        let time_traveled = a_goal.0;
        let new_t = t + time_traveled;
        let new_pos_a = a_goal.1.clone();
        let new_pos_b = pos_b;
        let new_opening_a = None;
        let new_opening_b = Some((b_goal.0 - time_traveled, b_goal.1.clone(), b_goal.2));
        let new_pressure = pressure + (pressure_release_rate * time_traveled);
        let new_pressure_release_rate = pressure_release_rate + a_goal.2;
        let new_opened = {
          let mut new_opened = opened.clone();
          new_opened.push(a_goal.1.clone());
          new_opened
        };
        create_elephant_tree(
          valves,
          tables,
          new_t,
          max_t,
          new_pos_a.as_str(),
          new_pos_b,
          new_opening_a,
          new_opening_b,
          new_opened,
          new_pressure,
          new_pressure_release_rate,
        )
      } else {
        let time_traveled = b_goal.0;
        let new_t = t + time_traveled;
        let new_pos_a = pos_a;
        let new_pos_b = b_goal.1.clone();
        let new_opening_a = Some((a_goal.0 - time_traveled, a_goal.1.clone(), a_goal.2));
        let new_opening_b = None;
        let new_pressure = pressure + (pressure_release_rate * time_traveled);
        let new_pressure_release_rate = pressure_release_rate + b_goal.2;
        let new_opened = {
          let mut new_opened = opened.clone();
          new_opened.push(b_goal.1.clone());
          new_opened
        };
        create_elephant_tree(
          valves,
          tables,
          new_t,
          max_t,
          new_pos_a,
          new_pos_b.as_str(),
          new_opening_a,
          new_opening_b,
          new_opened,
          new_pressure,
          new_pressure_release_rate,
        )
      };
      let eventual_pressure = child.get_eventual_best_pressure(max_t);
      if best_child.is_none() || eventual_pressure > best_eventual_pressure {
        best_child = Some(child);
        best_eventual_pressure = eventual_pressure;
      }
    }
  }
  if a_goals.len() == 1 && b_goals.len() == 1 {
    let a_goal = a_goals[0].clone();
    let b_goal = b_goals[0].clone();
    if a_goal.1 == b_goal.1 {
      let (shortest, longest) = if a_goal.0 < b_goal.0 {
        (a_goal, b_goal)
      } else {
        (b_goal, a_goal)
      };
      let time_traveled = shortest.0;
      let new_t = t + time_traveled;
      let new_pos_a = shortest.1.clone();
      let new_pos_b = longest.1.clone();
      let new_opening_a = None;
      let new_opening_b = None;
      let new_pressure = pressure + (pressure_release_rate * time_traveled);
      let new_pressure_release_rate = pressure_release_rate + shortest.2;
      let new_opened = {
        let mut new_opened = opened.clone();
        new_opened.push(shortest.1.clone());
        new_opened
      };
      let child = create_elephant_tree(
        valves,
        tables,
        new_t,
        max_t,
        new_pos_a.as_str(),
        new_pos_b.as_str(),
        new_opening_a,
        new_opening_b,
        new_opened,
        new_pressure,
        new_pressure_release_rate,
      );
      let eventual_pressure = child.get_eventual_best_pressure(max_t);
      if best_child.is_none() || eventual_pressure > best_eventual_pressure {
        best_child = Some(child);
        best_eventual_pressure = eventual_pressure;
      }
    }
  }
  if a_goals.len() == 0 && b_goals.len() > 0 || a_goals.len() > 0 || b_goals.len() == 0 {
    let (goals, pos, other_pos) = if a_goals.len() > 0 {
      (a_goals, pos_a, pos_b)
    } else {
      (b_goals, pos_b, pos_a)
    };
    for goal in goals {
      let time_traveled = goal.0;
      let new_t = t + time_traveled;
      let new_pos_a = goal.1.clone();
      let new_pos_b = other_pos;
      let new_opening_a = None;
      let new_opening_b = None;
      let new_pressure = pressure + (pressure_release_rate * time_traveled);
      let new_pressure_release_rate = pressure_release_rate + goal.2;
      let new_opened = {
        let mut new_opened = opened.clone();
        new_opened.push(goal.1.clone());
        new_opened
      };
      let child = create_elephant_tree(
        valves,
        tables,
        new_t,
        max_t,
        &new_pos_a,
        new_pos_b,
        new_opening_a,
        new_opening_b,
        new_opened,
        new_pressure,
        new_pressure_release_rate,
      );
      let eventual_pressure = child.get_eventual_best_pressure(max_t);
      if best_child.is_none() || eventual_pressure > best_eventual_pressure {
        best_child = Some(child);
        best_eventual_pressure = eventual_pressure;
      }
    }
  }

  ElephantTree {
    t,
    pos_a: pos_a.to_owned(),
    pos_b: pos_b.to_owned(),
    pressure,
    pressure_release_rate,
    opened,
    opening_a,
    opening_b,
    best_child: best_child.map(|v| Box::new(v)),
  }
}
