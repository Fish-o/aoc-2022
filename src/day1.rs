pub fn run(input: String) {
  let mut elf_foods = input
    .split("\n\n")
    .map(|elf| {
      elf
        .split("\n")
        .map(|line| line.parse::<u32>().unwrap())
        .reduce(|acc, x| acc + x)
        .unwrap()
    })
    .collect::<Vec<_>>();

  elf_foods.sort_by(|a, b| b.cmp(a));

  println!(
    "Day 1: {} {}",
    elf_foods[0],
    elf_foods[0] + elf_foods[1] + elf_foods[2]
  );
}
