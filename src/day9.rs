use std::collections::HashSet;
pub fn run(input: String) {
  let mut pos_visited: HashSet<(i32, i32)> = HashSet::new();
  pos_visited.insert((0, 0));
  // println!("{}", pos_visited.len());
  let mut head = (0i32, 0i32);
  let mut positions = vec![(0i32, 0i32); 9];

  for line in input.lines() {
    let mut line = line.split_whitespace();
    let dir = line.next().unwrap();
    let amount = line.next().unwrap().parse::<i32>().unwrap();
    let dir = match dir {
      "R" => (1, 0),
      "L" => (-1, 0),
      "U" => (0, 1),
      "D" => (0, -1),
      _ => panic!("Invalid direction"),
    };
    for _ in 0..amount {
      let mut old_pos_head = head;
      head = (head.0 + dir.0, head.1 + dir.1);
      let mut cur_pos_head = head;
      // println!("{:?} {:?}", cur_pos_head, old_pos_head);
      for i in 0..positions.len() {
        let mut cur_pos_tail = positions[i];
        let old_pos_tail = cur_pos_tail;
        // println!("{:?} {:?}", cur_pos_head, cur_pos_tail);
        if cur_pos_head == cur_pos_tail {
          continue;
        }
        let dif: (u32, u32) = (
          cur_pos_head.0.abs_diff(cur_pos_tail.0),
          cur_pos_head.1.abs_diff(cur_pos_tail.1),
        );
        match dif {
          (0, 0) | (0, 1) | (1, 0) => {}
          (1, 1) => {}
          _ => {
            let dif = (
              cur_pos_head.0 - cur_pos_tail.0,
              cur_pos_head.1 - cur_pos_tail.1,
            );
            let fix = match dif {
              (2, 0) => (1, 0),
              (-2, 0) => (-1, 0),
              (0, 2) => (0, 1),
              (0, -2) => (0, -1),

              (2, 1) => (1, 1),
              (2, -1) => (1, -1),
              (1, 2) => (1, 1),
              (-1, 2) => (-1, 1),

              (-2, 1) => (-1, 1),
              (-2, -1) => (-1, -1),
              (1, -2) => (1, -1),
              (-1, -2) => (-1, -1),

              (2, 2) => (1, 1),
              (2, -2) => (1, -1),
              (-2, 2) => (-1, 1),
              (-2, -2) => (-1, -1),
              _ => panic!("Invalid dif: {:?}", dif),
            };
            cur_pos_tail = (cur_pos_tail.0 + fix.0, cur_pos_tail.1 + fix.1);
          }
        }
        positions[i] = cur_pos_tail;
        old_pos_head = old_pos_tail;
        cur_pos_head = cur_pos_tail;
      }
      // println!("Prev dir {:?} tail pos{:?}", dir, positions[8]);
      pos_visited.insert(positions[8]);
    }
  }
  println!("Day 9: ??? {}", pos_visited.len());
}
