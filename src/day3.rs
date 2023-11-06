use itertools::Itertools;
use std::collections::HashSet;

pub fn run(input: String) {
  let res1 = input
    .split("\n")
    .map(|rucksack| {
      let length = rucksack.len();

      let rucksack = rucksack.chars().map(|c| {
        if c >= 'a' {
          c as u8 - 'a' as u8 + 1
        } else {
          c as u8 - 'A' as u8 + 27
        }
      });
      let comp1 = rucksack.clone().take(length / 2).collect::<HashSet<_>>();
      let comp2 = rucksack.skip(length / 2);

      // Get the number thats in both
      for c in comp2 {
        if comp1.contains(&c) {
          return c as u32;
        }
      }
      unreachable!()
    })
    .sum::<u32>();

  let res2: u32 = input
    .split("\n")
    .chunks(3)
    .into_iter()
    .map(|group| {
      let (elf1, elf2, elf3) = group.collect_tuple().unwrap();
      let elf1 = elf1.chars().collect::<HashSet<_>>();
      let elf2 = elf2.chars().collect::<HashSet<_>>();

      for char in elf3.chars() {
        if elf1.contains(&char) && elf2.contains(&char) {
          return char;
        }
      }
      unreachable!()
    })
    .map(|c| {
      if c >= 'a' {
        c as u32 - 'a' as u32 + 1
      } else {
        c as u32 - 'A' as u32 + 27
      }
    })
    .sum::<u32>();

  println!("Day 3: {} {}", res1, res2);
}
