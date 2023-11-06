use itertools::Itertools;
use regex::Regex;
const WIDTH: usize = 3;
const PADDING: usize = 1;

fn parse_begin_drawing(input: &str) -> Vec<Vec<char>> {
  let mut lines = input.lines().peekable();
  let last_line = lines.peek().unwrap();
  let columns = (last_line.len() + PADDING) / (WIDTH + PADDING);
  let mut hash_set = vec![vec![]; columns];
  for line in lines.rev().skip(1) {
    let line = " ".to_string() + line;
    let line = line.chars().chunks(WIDTH + PADDING);
    let line = line.into_iter().map(|x| x.collect::<String>());
    // (WIDTH + PADDING);
    for (i, c) in line.enumerate() {
      if c.starts_with("  ") {
        continue;
      }
      // println!("{} {}", i, c);
      let char = c.chars().skip(2).next().unwrap();
      hash_set[i].push(char);
    }
  }
  hash_set
}

fn move_crates(amount: usize, from: usize, to: usize, stacks: &mut Vec<Vec<char>>) {
  let len = stacks[from - 1].len();
  let mut crates = stacks[from - 1].drain((len - amount)..).collect::<Vec<_>>();
  // crates.reverse();
  stacks[to - 1].append(&mut crates);
}

pub fn run(input: String) {
  let mut input = input.split("\n\n");
  let begin_drawing = input.next().unwrap();
  let rearrangement_procedure = input.next().unwrap();
  let mut stacks = parse_begin_drawing(begin_drawing);
  // println!("{:#?}", stacks);
  let re = Regex::new(r"move (\d{1,2}) from (\d{1}) to (\d{1})").unwrap();

  rearrangement_procedure.lines().for_each(|line| {
    let capture = re.captures_iter(line).next().unwrap();
    let (amount, from, to) = (
      capture[1].parse::<usize>().unwrap(),
      capture[2].parse::<usize>().unwrap(),
      capture[3].parse::<usize>().unwrap(),
    );
    move_crates(amount, from, to, &mut stacks);
  });
  print!("Day 5: ??? ");
  for stack in stacks {
    let last = stack.last().unwrap();
    print!("{}", last)
  }
  println!();
}
