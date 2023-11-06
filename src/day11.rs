use itertools::Itertools;

struct Monkey {
  inspect_count: usize,
  inventory: Vec<usize>,
  operation: String,
  test: usize,
  success: u8,
  failure: u8,
}
// #[derive(Debug, Clone)]
// struct Item {
//   remainder: usize,
//   prime_parts: Vec<usize>,
// }

// fn primes_up_to(n: usize) -> Vec<usize> {
//   let mut primes = Vec::new();
//   let mut is_prime = vec![true; n + 1];
//   is_prime[0] = false;
//   is_prime[1] = false;
//   for i in 2..=n {
//     if is_prime[i] {
//       primes.push(i);
//       for j in (i..=n).step_by(i) {
//         is_prime[j] = false;
//       }
//     }
//   }
//   primes
// }
// impl Item {
//   fn new(worry: usize) -> Item {
//     let mut worry_divs = HashMap::new();
//     let mut worry = worry;
//     for prime in [17, 13, 19, 23] {
//       if worry % prime == 0 {
//         *worry_divs.entry(prime).or_insert(0) += 1;
//         worry /= prime;
//       }
//     }
//     Item {
//       worry_divs,
//       remainder: worry,
//     }
//   }
//   fn add(&mut self, number: u32) {
//     let new_remainder = self.remainder + number as usize;
//     for prime in [17, 13, 19, 23] {
//       if new_remainder % prime == 0 {
//         *self.worry_divs.entry(prime).or_insert(0) += 1;
//         self.remainder /= prime;
//       }
//     }
//   }
//   fn mul(&mut self, number: u32) {
//     let new_remainder = self.remainder * number as usize;
//     for prime in [17, 13, 19, 23] {
//       if new_remainder % prime == 0 {
//         *self.worry_divs.entry(prime).or_insert(0) += 1;
//         self.remainder /= prime;
//       }
//     }
//   }
//   fn pow(&mut self, number: u32) {
//     let new_remainder = self.remainder.pow(number as u32);
//     for prime in [17, 13, 19, 23] {
//       if new_remainder % prime == 0 {
//         *self.worry_divs.entry(prime).or_insert(0) += 1;
//         self.remainder /= prime;
//       }
//     }
//   }

//   fn test(&self, test: usize) -> bool {
//     if self.remainder % test == 0 {
//       return true;
//     }
//     false
//   }
// }
impl Monkey {
  fn new(
    inventory: Vec<usize>,
    operation: String,
    test: usize,
    success: u8,
    failure: u8,
  ) -> Monkey {
    Monkey {
      inspect_count: 0,
      inventory,
      operation,
      test,
      success,
      failure,
    }
  }
  pub fn throw(&mut self) -> (usize, u8) {
    let item = self.inventory.remove(0);
    // Inspect
    let inspect = self.operation.split(" ").collect::<Vec<&str>>();
    let right = inspect[2];
    let right = if right == "old" {
      item
    } else {
      right.parse::<usize>().unwrap()
    };
    let mut left = item;
    match inspect[1] {
      "+" => left += right,
      "*" => left *= right,
      _ => panic!("Unknown operation"),
    };
    self.inspect_count += 1;
    // Didn't destroy it
    // let worry = new_worry / 3;

    // Test
    let throw = if left % self.test == 0 {
      self.success
    } else {
      self.failure
    };
    (left, throw)
  }
  pub fn catch(&mut self, worry: usize) {
    self.inventory.push(worry);
  }
  pub fn throw_count(&self) -> usize {
    self.inventory.len()
  }
}

pub fn run(input: String) {
  let mut monkeys_input = input.split("\n\n");

  let mut monkeys = Vec::new();
  while let Some(monkey_input) = monkeys_input.next() {
    let mut monkey_input = monkey_input.split("\n");
    monkey_input.next();

    let inventory = monkey_input.next().unwrap().trim()["Starting items: ".len()..]
      .split(", ")
      .map(|x| x.parse::<usize>().unwrap())
      .collect::<Vec<usize>>();
    // println!("{:?}", inventory);
    let operation = monkey_input.next().unwrap()["  Operation: new = ".len()..].to_string();
    let test = monkey_input.next().unwrap()["  Test: divisible by ".len()..]
      .parse::<usize>()
      .unwrap();
    let success = monkey_input.next().unwrap()["    If true: throw to monkey ".len()..]
      .parse::<u8>()
      .unwrap();
    let failure = monkey_input.next().unwrap()["    If false: throw to monkey ".len()..]
      .parse::<u8>()
      .unwrap();
    let monkey = Monkey::new(inventory, operation, test, success, failure);
    monkeys.push(monkey);
  }
  let tests_prod = monkeys.iter().map(|x| x.test).product::<usize>();
  // println!("Tests prod: {}", tests_prod);
  let mut prod = 1;
  for round in 0..10000 {
    // println!("Round {}", round);
    for monkey_index in 0..monkeys.len() {
      let monkey = &mut monkeys[monkey_index];
      let throw_count = monkey.throw_count();
      for _ in 0..throw_count {
        let monkey = &mut monkeys[monkey_index];
        let (worry, throw) = monkey.throw();
        // println!(
        //   "Monkey {} threw {} to monkey {}",
        //   monkey_index, throw, worry
        // );
        let worry = worry % tests_prod;
        monkeys[throw as usize].catch(worry);
      }
    }

    for (i, monkey) in &mut monkeys.iter().enumerate() {
      // println!(
      //   "Monkey id: {} c: {:<5} has {:?}",
      //   i, monkey.inspect_count, monkey.inventory
      // );
    }
    let all_inspect_counts = monkeys
      .iter()
      .map(|x| x.inspect_count)
      .sorted_by(|a, b| b.cmp(a))
      .collect::<Vec<_>>();
    let biggest_1 = all_inspect_counts[0];
    let biggest_2 = all_inspect_counts[1];
    prod = biggest_1 * biggest_2;

    // println!("Biggest 1: {}", biggest_1);
    // println!("Biggest 2: {}", biggest_2);
    // println!("Product: {}", prod);
  }
  println!("Day 11: ??? {}", prod);
  // println!("Tests prod: {}", tests_prod);
}
