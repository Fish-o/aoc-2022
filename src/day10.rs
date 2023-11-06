pub fn run(input: String) {
  // let input = include_str!("input.txt");
  let lines = input.lines();
  let mut cycles = 0;
  let mut register: i64 = 1;
  print!("Day 10: ");
  let interesting_cycles = [20, 60, 100, 140, 180, 220];
  let mut interesting_cycles_res = vec![];
  for (i, line) in lines.enumerate() {
    let mut parts = line.split_whitespace();
    let command = parts.next().unwrap();

    let mut tick = || {
      let sprite_pos = register;
      if (sprite_pos - cycles).abs() <= 1 {
        print!("##")
      } else {
        print!("  ")
      }
      // print!("{} ", cycles);
      if interesting_cycles.contains(&cycles) {
        // println!("aCycle {}: {} {}", cycles, register * cycles, register);
        interesting_cycles_res.push(register * cycles);
      }
      cycles += 1;
      if cycles >= 40 {
        if i != input.lines().count() - 1 {
          print!("\n        ");
        } else {
          print!("\n");
        }
        cycles = 0;
      }
    };

    match command {
      "addx" => {
        tick();
        tick();
        register += parts.next().unwrap().parse::<i64>().unwrap();
      }
      "noop" => {
        tick();
      }
      _ => panic!("Unknown command: {}", command),
    }
  }
  // println!("Res: {:?}", interesting_cycles_res);
  // println!("Answer: {}", answer);
  // println!("Sum: {}", interesting_cycles_res.iter().sum::<i64>());
}
