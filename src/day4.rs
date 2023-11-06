pub fn run(input: String) {
  let p = |elf: &str| {
    let mut elf = elf.split("-");
    (
      elf.next().unwrap().parse::<usize>().unwrap(),
      elf.next().unwrap().parse::<usize>().unwrap(),
    )
  };
  let res1 = input
    .split("\n")
    .map(|pair| {
      let mut pair = pair.split(",");

      let elf1 = p(pair.next().unwrap());
      let elf2 = p(pair.next().unwrap());
      let (first_elf, last_elf) = if elf1.0 == elf2.0 {
        return Some((elf1, elf2));
      } else if elf1.0 < elf2.0 {
        (elf1, elf2)
      } else {
        (elf2, elf1)
      };
      if first_elf.1 >= last_elf.1 {
        Some((first_elf, last_elf))
      } else {
        None
      }
    })
    .map(|pair| if pair.is_some() { 1 } else { 0 })
    .sum::<usize>();
  let res2 = input
    .split("\n")
    .map(|pair| {
      let mut pair = pair.split(",");

      let elf1 = p(pair.next().unwrap());
      let elf2 = p(pair.next().unwrap());
      let (small_elf, big_elf) = if elf1.0 == elf2.0 {
        return Some((elf1, elf2));
      } else if elf1.0 < elf2.0 {
        (elf1, elf2)
      } else {
        (elf2, elf1)
      };
      if small_elf.1 >= big_elf.0 {
        Some((small_elf, big_elf))
      } else {
        None
      }
    })
    .map(|pair| if pair.is_some() { 1 } else { 0 })
    .sum::<usize>();
  println!("Day 4: {} {}", res1, res2);
}
