use std::collections::HashMap;
#[derive(Debug, Clone)]
enum Monkey {
  Number {
    name: String,
    value: i64,
  },
  Operation {
    name: String,
    left: Box<Monkey>,
    right: Box<Monkey>,
    operation: Operation,
  },
  Human,
}
#[derive(Debug, Clone)]
enum Operation {
  Add,
  Subtract,
  Multiply,
  Divide,
}

enum IncompleteMonkey {
  Number {
    name: String,
    value: i64,
  },
  Operation {
    name: String,
    left: String,
    right: String,
    operation: Operation,
  },
  Human,
}
pub fn run(input: String) {
  print!("Day 21: ");
  solve_1(&input);
  print!(" ");
  solve_2(&input);
  println!("");
}
pub fn solve_1(input: &String) {
  let mut incomplete_monkey_library: HashMap<String, IncompleteMonkey> = HashMap::new();
  let mut monkey_library: HashMap<String, Monkey> = HashMap::new();
  input
    .split("\n")
    .map(|m_str| m_str.split(": "))
    .into_iter()
    .for_each(|mut m_str| {
      let name = m_str.next().unwrap();
      let value = m_str.next().unwrap();
      if value.contains(" ") {
        let mut value = value.split(" ");
        let left = value.next().unwrap();
        let operation = value.next().unwrap();
        let right = value.next().unwrap();
        let operation = match operation {
          "+" => Operation::Add,
          "-" => Operation::Subtract,
          "*" => Operation::Multiply,
          "/" => Operation::Divide,
          _ => panic!("Unknown operation: {}", operation),
        };
        let monkey = IncompleteMonkey::Operation {
          name: name.to_owned(),
          left: left.to_owned(),
          right: right.to_owned(),
          operation,
        };
        incomplete_monkey_library.insert(name.to_owned(), monkey);
      } else {
        let monkey = IncompleteMonkey::Number {
          name: name.to_owned(),
          value: value.parse::<i64>().unwrap(),
        };
        incomplete_monkey_library.insert(name.to_owned(), monkey);
      }
    });

  let complete_root = complete_monkey(&incomplete_monkey_library, &mut monkey_library, "root");
  let collapsed = collapse_monkey(&complete_root);
  match collapsed {
    Monkey::Number { value, .. } => print!("{}", value),
    _ => panic!("Collapsed monkey is not a number"),
  }
}

pub fn solve_2(input: &String) {
  let mut incomplete_monkey_library = HashMap::new();
  input
    .split("\n")
    .map(|m_str| m_str.split(": "))
    .into_iter()
    .for_each(|mut m_str| {
      let name = m_str.next().unwrap();
      let value: &str = m_str.next().unwrap();
      if name == "humn" {
        let monkey = IncompleteMonkey::Human;
        incomplete_monkey_library.insert(name.to_owned(), monkey);
      } else if value.contains(" ") {
        let mut value = value.split(" ");
        let left = value.next().unwrap();
        let operation = value.next().unwrap();
        let right = value.next().unwrap();
        let operation = match operation {
          "+" => Operation::Add,
          "-" => Operation::Subtract,
          "*" => Operation::Multiply,
          "/" => Operation::Divide,
          _ => panic!("Unknown operation: {}", operation),
        };
        let monkey = IncompleteMonkey::Operation {
          name: name.to_owned(),
          left: left.to_owned(),
          right: right.to_owned(),
          operation,
        };
        incomplete_monkey_library.insert(name.to_owned(), monkey);
      } else {
        let monkey = IncompleteMonkey::Number {
          name: name.to_owned(),
          value: value.parse::<i64>().unwrap(),
        };
        incomplete_monkey_library.insert(name.to_owned(), monkey);
      }
    });

  let mut monkey_library: HashMap<String, Monkey> = HashMap::new();
  let incomplete_root = incomplete_monkey_library.get("root").unwrap();
  if let IncompleteMonkey::Operation {
    name,
    left,
    right,
    operation,
  } = incomplete_root
  {
    let left = complete_monkey(&incomplete_monkey_library, &mut monkey_library, left);
    let right = complete_monkey(&incomplete_monkey_library, &mut monkey_library, right);
    let left = collapse_monkey(&left);
    let right = collapse_monkey(&right);
    // println!("Left: {:#?}", left);
    // println!("Right: {:#?}", right);
    let (human_side, number_side) = match (&left, &right) {
      (Monkey::Human, Monkey::Number { .. })
      | (Monkey::Operation { .. }, Monkey::Number { .. }) => (left, right),
      (Monkey::Number { .. }, Monkey::Human)
      | (Monkey::Number { .. }, Monkey::Operation { .. }) => (right, left),
      _ => panic!("Root must have one human side and one number side"),
    };
    let (human_side, number_side) = invert(&human_side, &number_side);
    // println!("Human side: {:#?}", human_side);
    // println!("Number side: {:#?}", number_side);
    let collapsed = collapse_monkey(&number_side);
    match collapsed {
      Monkey::Number { value, .. } => print!("{}", value),
      _ => panic!("Collapsed monkey is not a number"),
    }

    // monkey_library.insert(name.to_owned(), monkey);
  }
}

fn complete_monkey(
  incomplete_monkey_library: &HashMap<String, IncompleteMonkey>,
  monkey_library: &mut HashMap<String, Monkey>,
  name: &str,
) -> Monkey {
  if monkey_library.contains_key(name) {
    return monkey_library.get(name).unwrap().to_owned();
  }
  let incomplete_monkey = incomplete_monkey_library.get(name).unwrap();
  match incomplete_monkey {
    IncompleteMonkey::Number { name, value } => Monkey::Number {
      name: name.to_owned(),
      value: *value,
    },
    IncompleteMonkey::Operation {
      name,
      left,
      right,
      operation,
    } => {
      let left = complete_monkey(incomplete_monkey_library, monkey_library, left);
      let right = complete_monkey(incomplete_monkey_library, monkey_library, right);
      Monkey::Operation {
        name: name.to_owned(),
        left: Box::new(left),
        right: Box::new(right),
        operation: operation.to_owned(),
      }
    }
    IncompleteMonkey::Human => Monkey::Human,
  }
}

fn collapse_monkey(monkey: &Monkey) -> Monkey {
  match monkey {
    Monkey::Number { .. } => monkey.to_owned(),
    Monkey::Operation {
      name,
      left,
      right,
      operation,
    } => {
      let left = collapse_monkey(left);
      let right = collapse_monkey(right);
      match (&left, &right) {
        (Monkey::Number { value: left, .. }, Monkey::Number { value: right, .. }) => {
          Monkey::Number {
            name: name.to_owned(),
            value: collapse_operation(*left, *right, operation),
          }
        }
        _ => Monkey::Operation {
          name: name.to_owned(),
          left: Box::new(left),
          right: Box::new(right),
          operation: operation.to_owned(),
        },
      }
    }
    Monkey::Human => Monkey::Human,
  }
}

fn collapse_operation(left: i64, right: i64, operation: &Operation) -> i64 {
  match operation {
    Operation::Add => left + right,
    Operation::Subtract => left - right,
    Operation::Multiply => left * right,
    Operation::Divide => left / right,
  }
}

fn invert(from: &Monkey, onto: &Monkey) -> (Monkey, Monkey) {
  match from {
    Monkey::Number { .. } => (from.to_owned(), onto.to_owned()),
    Monkey::Operation {
      name,
      left,
      right,
      operation,
    } => {
      let (human_side, number_side, flipped) = match (&left.as_ref(), &right.as_ref()) {
        (Monkey::Human, Monkey::Number { .. })
        | (Monkey::Operation { .. }, Monkey::Number { .. }) => (left, right, false),
        (Monkey::Number { .. }, Monkey::Human)
        | (Monkey::Number { .. }, Monkey::Operation { .. }) => (right, left, true),
        _ => panic!("From must have one human side and one number side"),
      };
      let new_number_side = match (flipped, operation) {
        (_, Operation::Add) => Monkey::Operation {
          name: name.to_owned(),
          left: Box::new(onto.to_owned()),
          right: Box::new(number_side.as_ref().to_owned()),
          operation: Operation::Subtract,
        },
        (_, Operation::Multiply) => Monkey::Operation {
          name: name.to_owned(),
          left: Box::new(onto.to_owned()),
          right: Box::new(number_side.as_ref().to_owned()),
          operation: Operation::Divide,
        },
        (false, Operation::Subtract) => Monkey::Operation {
          name: name.to_owned(),
          left: Box::new(onto.to_owned()),
          right: Box::new(number_side.as_ref().to_owned()),
          operation: Operation::Add,
        },
        (false, Operation::Divide) => Monkey::Operation {
          name: name.to_owned(),
          right: Box::new(onto.to_owned()),
          left: Box::new(number_side.as_ref().to_owned()),
          operation: Operation::Multiply,
        },
        (true, Operation::Subtract) => Monkey::Operation {
          name: name.to_owned(),
          left: Box::new(number_side.as_ref().to_owned()),
          right: Box::new(onto.to_owned()),
          operation: Operation::Subtract,
        },
        (true, Operation::Divide) => Monkey::Operation {
          name: name.to_owned(),
          left: Box::new(number_side.as_ref().to_owned()),
          right: Box::new(onto.to_owned()),
          operation: Operation::Divide,
        },
      };
      invert(human_side.as_ref(), &new_number_side)
    }
    Monkey::Human => (from.to_owned(), onto.to_owned()),
  }
}
