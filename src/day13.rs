#[derive(Debug)]
struct Packet {
  value: Option<u32>,
  children: Vec<Packet>,
}

impl Packet {
  pub fn parse(data: &str) -> Packet {
    let mut packet = Packet {
      value: None,
      children: Vec::new(),
    };

    let mut iter = data.chars().peekable();

    while let Some(c) = iter.next() {
      match c {
        '[' => {
          let mut depth = 1;
          let mut buffer = String::new();
          while let Some(c) = iter.next() {
            match c {
              '[' => {
                depth += 1;
              }
              ']' => {
                depth -= 1;
                if depth == 0 {
                  break;
                }
              }
              _ => {}
            }
            buffer.push(c);
          }
          packet.children.push(Packet::parse(&buffer));
        }
        '0'..='9' => {
          let mut val = c.to_string();
          while let Some(c) = iter.peek() {
            match c {
              '0'..='9' => {
                val.push(iter.next().unwrap());
              }
              ',' => {
                iter.next();
                break;
              }
              ']' => {
                break;
              }
              _ => panic!("Unexpected character"),
            }
          }
          packet.children.push(Packet::value(val.parse().unwrap()));
        }
        ',' => {}
        ']' => {
          break;
        }
        _ => {
          packet.value = Some(c.to_digit(10).unwrap());
        }
      }
    }

    packet
  }
  pub fn value(value: u32) -> Packet {
    Packet {
      value: Some(value),
      children: Vec::new(),
    }
  }
  pub fn into_list(self) -> Packet {
    Packet {
      value: None,
      children: vec![self],
    }
  }
  pub fn correct_order(&self, other: &Packet) -> Option<bool> {
    match (self.value, other.value) {
      (Some(a), Some(b)) => {
        if a < b {
          return Some(true);
        } else if a > b {
          return Some(false);
        } else {
          return None;
        }
      }
      (Some(a), None) => {
        let a_list = Packet::value(a).into_list();
        let b_list = other;
        return a_list.correct_order(b_list);
      }
      (None, Some(b)) => {
        let a_list = self;
        let b_list = Packet::value(b).into_list();
        return a_list.correct_order(&b_list);
      }
      (None, None) => {}
    }
    let longest_list = self.children.len().max(other.children.len());

    for i in 0..longest_list {
      let a = self.children.get(i);
      let b = other.children.get(i);
      match (a, b) {
        (None, Some(_)) => return Some(true),
        (Some(_), None) => return Some(false),
        (Some(a), Some(b)) => {
          let result = a.correct_order(b);
          if result.is_some() {
            return result;
          }
        }
        (None, None) => unreachable!(),
      }
    }
    None
  }
  pub fn to_string(&self) -> String {
    let mut result = String::new();
    if let Some(value) = self.value {
      result.push_str(&value.to_string());
    }
    if !self.children.is_empty() {
      result.push('[');
      for (i, child) in self.children.iter().enumerate() {
        if i > 0 {
          result.push(',');
        }
        result.push_str(&child.to_string());
      }
      result.push(']');
    }
    result
  }
}

pub fn run(data: String) {
  let packet_pairs = data.split("\n\n").map(|p_pair| {
    let mut packets = p_pair.split("\n");
    let left = packets.next().unwrap();
    let left = Packet::parse(&left[1..]);
    let right = packets.next().unwrap();
    let right = Packet::parse(&right[1..]);
    // println!("Parsed packet pair");
    // println!("Left: {:#?}", left);
    // println!("Right: {:#?}", right);
    (left, right)
  });

  let mut correct_order = 0;

  for (i, (left, right)) in packet_pairs.enumerate() {
    let result = left.correct_order(&right);
    // println!("Result: {:#?}", result);
    match result {
      Some(true) => {
        correct_order += i + 1;
      }
      Some(false) => {}
      None => println!("??? Order"),
    }
  }
  // println!("Correct order sum: {}", correct_order);

  let mut all_packets = data
    .replace("\n\n", "\n")
    .lines()
    .map(|line| Packet::parse(&line[1..]))
    .collect::<Vec<_>>();
  all_packets.push(Packet::parse("[2]]"));
  all_packets.push(Packet::parse("[6]]"));
  all_packets.sort_by(|a, b| {
    match a
      .correct_order(b)
      .unwrap_or_else(|| panic!("Failed to compare: {:#?} {:#?}", a, b))
    {
      true => std::cmp::Ordering::Less,
      false => std::cmp::Ordering::Greater,
    }
  });
  let div1 = all_packets
    .iter()
    .enumerate()
    .find(|(i, p)| p.to_string() == "[[2]]")
    .map(|(i, _)| i + 1)
    .unwrap();
  let div2 = all_packets
    .iter()
    .enumerate()
    .find(|(i, p)| p.to_string() == "[[6]]")
    .map(|(i, _)| i + 1)
    .unwrap();

  // println!("Div1: {}", div1);
  // println!("Div2: {}", div2);
  println!("Day 13: {} {}", correct_order, div1 * div2);
}
