use std::collections::HashMap;
#[derive(Debug)]
struct Dir {
  size: u32,
  children: Option<HashMap<String, Dir>>,
}

impl Dir {
  fn new() -> Self {
    Dir {
      size: 0,
      children: Some(HashMap::new()),
    }
  }
  fn new_file(size: u32) -> Self {
    Dir {
      size,
      children: None,
    }
  }
  pub fn add_file(&mut self, path: &[String], size: u32, name: String) {
    self.size += size;
    if path.is_empty() {
      self
        .children
        .as_mut()
        .unwrap()
        .insert(name, Dir::new_file(size));
    } else {
      let child = self
        .children
        .as_mut()
        .unwrap()
        .entry(path[0].clone())
        .or_insert_with(|| Dir::new());
      child.add_file(&path[1..], size, name);
    }
  }

  pub fn get_small_dirs(&self, max_size: u32) -> Vec<u32> {
    let mut res = Vec::new();
    if let Some(children) = &self.children {
      if self.size <= max_size {
        res.push(self.size);
      }
      for child in children.values() {
        res.extend(child.get_small_dirs(max_size));
      }
    }
    res
  }

  pub fn get_closest_to(&self, size: i64) -> i64 {
    let mut res = self.size as i64;
    if let Some(children) = &self.children {
      for child in children.values() {
        let child_res = child.get_closest_to(size);
        if child_res > size && child_res < res {
          res = child_res;
        }
      }
    }
    res
  }
}
pub fn run(input: String) {
  let commands = input.split("\n$ ");
  let mut file_sizes = Dir {
    size: 0,
    children: Some(HashMap::new()),
  };

  let mut path = Vec::<String>::new();
  for command in commands {
    let mut lines = command.lines();
    let command_line = lines.next().unwrap().split_whitespace().collect::<Vec<_>>();
    let _res = match command_line[..] {
      ["cd", new_wd] => match new_wd {
        ".." => {
          path.pop();
        }
        "/" => {
          path.clear();
        }
        _ => {
          path.push(new_wd.to_string());
          // file_sizes.add_dir(&path, new_wd.to_string());
        }
      },
      ["ls"] => {
        let files = lines
          .map(|line| line.splitn(2, " "))
          .map(|mut line| (line.next().unwrap(), line.next().unwrap().to_string()))
          .filter(|(size, _name)| {
            if size == &"dir" {
              // file_sizes.add_dir(&path, name.clone());
              false
            } else {
              true
            }
          })
          .map(|(size, name)| (size.parse::<u32>().unwrap(), name))
          .collect::<Vec<_>>();
        for (size, name) in files {
          file_sizes.add_file(&path, size, name);
        }
      }

      _ => {
        println!("Unknown command");
      }
    };
  }

  // println!("{:#?}", file_sizes);
  let small_dirs = file_sizes.get_small_dirs(100000);
  // println!("{:?}", small_dirs.iter().sum::<u32>());
  let total_usage = file_sizes.size;
  // println!("Total: {}", total_usage);
  let free = 70000000 - total_usage;
  // println!("Free: {}", free);
  let needed = 30000000 - free;
  // println!("Needed: {}", needed);
  let closest = file_sizes.get_closest_to(needed as i64);
  // println!("Closest: {}", closest);
  println!("Day 7: {} {}", small_dirs.iter().sum::<u32>(), closest);
}
