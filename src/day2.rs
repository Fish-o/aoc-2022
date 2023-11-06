pub fn run(input: String) {
  let round_scores = input
    .split("\n")
    .map(|round| {
      let mut round = round.split(" ");
      let opponent = match round.next().unwrap() {
        "A" => "R",
        "B" => "P",
        "C" => "S",
        _ => panic!("Invalid input"),
      };

      let you = match (opponent, round.next().unwrap()) {
        ("R", "X") => "S",
        ("R", "Y") => "R",
        ("R", "Z") => "P",
        ("P", "X") => "R",
        ("P", "Y") => "P",
        ("P", "Z") => "S",
        ("S", "X") => "P",
        ("S", "Y") => "S",
        ("S", "Z") => "R",
        _ => panic!("Invalid input"),
      };

      let win_score = match (opponent, you) {
        ("R", "R") => 3,
        ("R", "P") => 6,
        ("R", "S") => 0,
        ("P", "R") => 0,
        ("P", "P") => 3,
        ("P", "S") => 6,
        ("S", "R") => 6,
        ("S", "P") => 0,
        ("S", "S") => 3,
        _ => panic!("Invalid input"),
      };

      let play_score = match you {
        "R" => 1,
        "P" => 2,
        "S" => 3,
        _ => panic!("Invalid input"),
      };
      play_score + win_score
    })
    .collect::<Vec<_>>();
  let tot_score = round_scores.iter().sum::<i32>();
  println!("Day 2: ?? {}", tot_score);
}
