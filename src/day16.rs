use std::{
  arch::x86_64, collections::HashMap, f32::consts::E, hash::Hash, mem::size_of_val, option,
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
  println!("Day 16: {} ???", best.0);
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
  /*fn get_all_pressures(self: &Self) -> Vec<u32> {
      let mut pressures = vec![];
      if self.paused {
        pressures.push(self.get_released_pressure(26));
      } else {
        for node in &self.nodes {
          pressures.append(&mut node.get_all_pressures());
        }
      }
      pressures
    }
    fn prune(self: &mut Self, below: u32) {
      self.nodes = self
        .nodes
        .iter()
        .filter(|n| {
          if n.paused {
            if n.get_released_pressure(26) < below {
              false
            } else {
              true
            }
          } else if n.nodes.len() == 0 {
            false
          } else {
            true
          }
        })
        .cloned()
        .collect::<Vec<_>>();
      self.nodes.iter_mut().for_each(|n| {
        if !n.paused {
          n.prune(below);
        }
      })
    }
    fn unpause(
      self: &mut Self,
      valves: &Vec<Valve>,
      tables: &HashMap<String, HashMap<String, (String, u32, u32)>>,
      prune: f32,
      max_t: u32,
      new_max_depth: u32,
    ) {
      for node in &mut self.nodes {
        if node.paused {
          let new_node = create_doubly_traversed_tree(
            valves,
            tables,
            &node.pause_data.as_ref().unwrap().0,
            &node.pause_data.as_ref().unwrap().1,
            node.opened.clone(),
            node.t,
            max_t,
            node.released_pressure,
            node.pressure_release_rate,
            prune,
            node.depth,
            new_max_depth,
          );
          *node = new_node;
        } else {
          node.unpause(valves, tables, prune, max_t, new_max_depth);
        }
      }
    }
  */
}

/*
fn create_doubly_traversed_tree(
  valves: &Vec<Valve>,
  tables: &HashMap<String, HashMap<String, (String, u32, u32)>>,
  current_valve_a: &String,
  current_valve_b: &String,
  opened: HashMap<String, u32>,
  t: u32,
  max_t: u32,
  released_pressure: u32,
  pressure_release_rate: u32,
  prune: f32,
  depth: u32,
  max_depth: u32,
) -> TreeNode {
  // println!("Depth: {}", opened.len());
  if depth >= max_depth {
    return TreeNode {
      nodes: vec![],
      opened,
      pressure_release_rate,
      released_pressure,
      t,
      depth,
      paused: true,
      pause_data: Some((current_valve_a.to_owned(), current_valve_b.to_owned())),
    };
  }
  // AAAAAAAAAAAAAAAAAAAA
  let table = tables.get(current_valve_a).unwrap();
  let mut options = vec![];
  for valve in valves {
    if opened.contains_key(&valve.id) {
      continue;
    }
    let (via, time_cost, flow_rate) = table.get(&valve.id).unwrap();
    if t + *time_cost <= max_t {
      options.push((
        current_valve_a.to_owned(),
        valve.id.clone(),
        1,
        time_cost.clone(),
        flow_rate.clone(),
      ));
    }
  }
  options.sort_by_key(|(_, _, potential, _, _)| *potential);
  options.reverse();
  let count: usize = ((options.len() as f32) * (1.0 - prune)).ceil() as usize;
  // println!("Pruned {}/{}", options.len() - count, options.len());
  let pruned_options = options.drain(0..count).collect::<Vec<_>>();
  let pruned_options_a = pruned_options;
  // BBBBBBBBBBBBBBB
  let table = tables.get(current_valve_b).unwrap();
  let mut options = vec![];
  for valve in valves {
    if opened.contains_key(&valve.id) {
      continue;
    }
    let (via, time_cost, flow_rate) = table.get(&valve.id).unwrap();
    if t + time_cost.clone() <= max_t {
      options.push((
        current_valve_b.to_owned(),
        valve.id.clone(),
        1,
        time_cost.clone(),
        flow_rate.clone(),
      ));
    }
  }
  options.sort_by_key(|(_, _, potential, _, _)| *potential);
  options.reverse();
  let count: usize = ((options.len() as f32) * (1.0 - prune)).ceil() as usize;
  let pruned_options = options.drain(0..count).collect::<Vec<_>>();
  let pruned_options_b = pruned_options;
  if pruned_options_a.len() == 0 && pruned_options_b.len() == 0 {
    return TreeNode {
      nodes: vec![],
      opened,
      pressure_release_rate,
      released_pressure,
      t,
      depth,
      paused: false,
      pause_data: None,
    };
  }
  // Get all combinations of the options a and options b
  // CURRENT ID POTENTIAL TIME_COST FLOW_RATE

  let mut combined_options: Vec<(
    (String, String, u32, u32, u32),
    (String, String, u32, u32, u32),
  )> = vec![];
  for option_a in &pruned_options_a {
    for option_b in &pruned_options_b {
      if option_a.3 > option_b.3 {
        if combined_options
          .iter()
          .any(|f| f.0 == *option_b && f.1 == *option_a)
        {
          continue;
        }
        combined_options.push((option_b.clone(), option_a.clone()));
      } else {
        if combined_options
          .iter()
          .any(|f| f.0 == *option_a && f.1 == *option_b)
        {
          continue;
        }
        combined_options.push((option_a.clone(), option_b.clone()));
      }
    }
  }

  let mut nodes = vec![];
  for option in &combined_options {
    let (a_orig, a_dest, a_pot, a_cost, a_flow_rate) = option.0.clone();
    let (b_orig, b_dest, b_pot, b_cost, b_flow_rate) = option.1.clone();

    // println!("A move takes {} from {}", a_cost, a_orig);
    // println!("B move takes {} from {}", b_cost, b_orig);

    // println!("A is at {}", a_orig);
    // println!("B is at {}", b_orig);
    // println!("B wants to go to {}", b_dest);

    let (b_end_location, b_opened) = {
      let mut b_location = &b_orig;
      let mut b_opened = false;
      for _ in 0..a_cost {
        if b_location == &b_dest {
          // println!("B is at {} and opened", b_location);
          b_opened = true;
          break;
        }
        let table = tables.get(b_location).unwrap();
        let (via, _, _) = table.get(&b_dest).unwrap();
        b_location = via;
        // println!("B is at {}", b_location)
      }
      (b_location, b_opened)
    };
    // println!("B ENDS AT {} {}", b_end_location, b_opened);
    let new_t = t + a_cost;

    let mut new_opened = opened.clone();
    new_opened.insert(a_dest.clone(), new_t);
    let new_released_pressure = released_pressure + (pressure_release_rate * a_cost);
    let mut new_pressure_release_rate = pressure_release_rate + a_flow_rate;
    if b_opened && !new_opened.contains_key(b_dest.as_str()) {
      new_opened.insert(b_dest, new_t);
      new_pressure_release_rate += b_flow_rate;
    }
    let node = create_doubly_traversed_tree(
      valves,
      tables,
      &a_dest,
      &b_end_location,
      new_opened,
      new_t,
      max_t,
      new_released_pressure,
      new_pressure_release_rate,
      prune,
      depth + 1,
      max_depth,
    );
    nodes.push(node);
  }
  if combined_options.len() == 0 {
    for opt in pruned_options_b {
      let (_, dest, _, cost, rate) = opt;
      let new_t = t + cost;
      let mut new_opened = opened.clone();
      new_opened.insert(dest.clone(), new_t);
      let new_released_pressure = released_pressure + (pressure_release_rate * cost);
      let new_pressure_release_rate = pressure_release_rate + rate;
      let node = create_doubly_traversed_tree(
        valves,
        tables,
        &dest,
        &current_valve_b,
        new_opened,
        new_t,
        max_t,
        new_released_pressure,
        new_pressure_release_rate,
        prune,
        depth + 1,
        max_depth,
      );
      nodes.push(node)
    }
    for opt in pruned_options_a {
      let (_, dest, _, cost, rate) = opt;
      let new_t = t + cost;
      let mut new_opened = opened.clone();
      new_opened.insert(dest.clone(), new_t);
      let new_released_pressure = released_pressure + (pressure_release_rate * cost);
      let new_pressure_release_rate = pressure_release_rate + rate;
      let node = create_doubly_traversed_tree(
        valves,
        tables,
        &current_valve_a,
        &dest,
        new_opened,
        new_t,
        max_t,
        new_released_pressure,
        new_pressure_release_rate,
        prune,
        depth + 1,
        max_depth,
      );
      nodes.push(node)
    }
  }
  TreeNode {
    opened,
    t,
    released_pressure,
    pressure_release_rate,
    nodes,
    depth,
    paused: false,
    pause_data: None,
  }
}*/
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
