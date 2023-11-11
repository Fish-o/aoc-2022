use colored::Colorize;

pub fn parse_snafu(snafu: &str) -> i64 {
  let mut val = 0;
  for (base, c) in snafu.chars().rev().enumerate() {
    val += 5_i64.pow(base.try_into().unwrap())
      * match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("Invalid character in snafu"),
      }
  }
  val
}

pub fn create_snafu(number: i64) -> String {
  let mut base_5 = vec![];
  let mut number = number;
  while number != 0 {
    let digit = number % 5;
    number /= 5;
    base_5.push(digit)
  }
  let mut next_value = 0;
  let mut snafu = "".to_owned();
  for (base, v) in base_5.iter().enumerate() {
    let mut v = *v + next_value;
    if v > 2 {
      v = v - 5;
      next_value = 1;
    } else {
      next_value = 0;
    }
    snafu.push(match v {
      -2 => '=',
      -1 => '-',
      0 => '0',
      1 => '1',
      2 => '2',
      _ => panic!("Invalid value in snafu"),
    })
  }
  if next_value != 0 {
    snafu.push(match next_value {
      1 => '1',
      2 => '2',
      _ => panic!("Invalid value in snafu next_value"),
    })
  }
  snafu.chars().rev().collect()
}
pub fn run(input: String) {
  let snafus = input.lines();
  let mut sum = 0;
  for snafu in snafus {
    let numb = parse_snafu(snafu);
    println!("{}", numb);
    sum += numb;
  }
  println!("Sum: {} {}", sum, create_snafu(sum));
}
