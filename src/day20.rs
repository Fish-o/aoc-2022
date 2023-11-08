struct Mixer {
  nodes: Vec<i64>,
  forward_links: Vec<usize>,
  backward_links: Vec<usize>,
}
impl Mixer {
  pub fn new(arr: Vec<i64>) -> Self {
    let mut forward_links = vec![0; arr.len()];
    let mut backward_links = vec![0; arr.len()];
    let max: usize = arr.len();
    for i in 0..max {
      forward_links[i] = (i + 1) % max;
      backward_links[i] = (i + max - 1) % max;
    }
    Mixer {
      nodes: arr,
      forward_links,
      backward_links,
    }
  }

  pub fn mix(&mut self) {
    for (index, node) in self.nodes.iter().enumerate() {
      let steps = node.unsigned_abs() as usize % (self.nodes.len() - 1);
      let mut displaced_node_idx = index;
      if *node > 0 {
        for _ in 0..steps {
          displaced_node_idx = self.forward_links[displaced_node_idx];
        }
      } else if *node < 0 {
        for _ in 0..(steps + 1) {
          displaced_node_idx = self.backward_links[displaced_node_idx];
        }
      } else {
        continue;
      }
      let moved_node_next = self.forward_links[index];
      let moved_node_prev = self.backward_links[index];
      // Connect the nodes around the node that moved
      self.forward_links[moved_node_prev] = moved_node_next;
      self.backward_links[moved_node_next] = moved_node_prev;

      // Connect the new surrounding nodes to the displaced node
      let displaced_node_next = self.forward_links[displaced_node_idx];
      self.forward_links[displaced_node_idx] = index;
      self.backward_links[displaced_node_next] = index;

      // Update current node to point to the new neighbors
      self.forward_links[index] = displaced_node_next;
      self.backward_links[index] = displaced_node_idx;
    }
  }
  pub fn get_ordered(&self) -> Vec<i64> {
    let mut ordered = vec![0; self.nodes.len()];
    let mut current = 0;
    for i in 0..self.nodes.len() {
      ordered[i] = self.nodes[current];
      current = self.forward_links[current];
    }
    ordered
  }
  pub fn zero_index(&self) -> usize {
    self.nodes.iter().position(|v| *v == 0).unwrap()
  }
  pub fn get_grove_coords_sum(&self) -> i64 {
    let zero_index: usize = self.zero_index();
    let mut current = zero_index;
    let mut x = 0;
    for i in 1..=3000 {
      current = self.forward_links[current];
      if i % 1000 == 0 {
        x += self.nodes[current]
      }
    }
    x
  }
  pub fn decrypt(&mut self, key: i64) {
    self.nodes = self.nodes.iter().map(|v| v * key).collect();
  }
}

pub fn run(input: String) {
  let numbers = input.split("\n").map(|v| v.parse::<i64>().unwrap());
  let mut mixer1 = Mixer::new(numbers.clone().collect());
  mixer1.mix();
  print!("Day 20: ");
  print!("{} ", mixer1.get_grove_coords_sum());
  let mut mixer2 = Mixer::new(numbers.clone().collect());
  mixer2.decrypt(811589153);
  for _ in 0..10 {
    mixer2.mix();
  }
  println!("{}", mixer2.get_grove_coords_sum());
}
