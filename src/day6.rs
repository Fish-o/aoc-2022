use std::io::{BufReader, Read};

use itertools::Itertools;

pub fn run(input: String) {
  print!("Day 6: ");

  let buffer = BufReader::new(input.as_bytes());
  let mut chars = buffer.bytes().enumerate();
  let mut memory3: [char; 3] = [' '; 3];
  let mut i = 0;
  while let Some((index, Ok(c))) = chars.next() {
    if !memory3.contains(&(c as char)) {
      let mut has_doubles = false;
      for (a, b) in memory3.iter().tuple_combinations() {
        if a == b {
          has_doubles = true;
          break;
        }
      }
      if !has_doubles && index > 3 {
        print!("{}", index + 1);
        break;
      }
    }
    memory3[i] = c as char;
    i = (i + 1) % 3;
  }
  print!(" ");
  let buffer = BufReader::new(input.as_bytes());
  let mut chars = buffer.bytes().enumerate();
  let mut memory13: [char; 13] = [' '; 13];
  let mut i = 0;
  while let Some((index, Ok(c))) = chars.next() {
    if !memory13.contains(&(c as char)) {
      // Check if memory has doubles
      let mut has_doubles = false;
      for (a, b) in memory13.iter().tuple_combinations() {
        if a == b {
          has_doubles = true;
          break;
        }
      }
      if !has_doubles {
        // println!("{} {}", index + 1, i);
        // println!("{} {:?}", c as char, memory);
        print!("{}", index + 1);
        break;
      }
    }
    memory13[i] = c as char;
    i = (i + 1) % 13;
  }
  println!();
}
