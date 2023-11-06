use std::io::Read;

macro_rules! run {
  ($($day:ident),*) => {
    $( mod $day; )*
    fn main() {
      $(
        let input = fetch_file(stringify!($day));
        $day::run(input.to_owned());
      )*
    }
  };
}
run!(
  day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14, day15
);
// run!(day15);
//
fn fetch_file(day: &str) -> String {
  // remove first 3 chars from day
  let day = &day[3..];

  let mut input = String::new();
  std::fs::File::open(format!("./input/input-{}.txt", day))
    .unwrap()
    .read_to_string(&mut input)
    .unwrap();
  input
}
