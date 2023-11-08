use std::collections::HashSet;

pub fn run(input: String) {
  let voxels = input.split("\n").map(|v| {
    let mut parts = v.split(",");
    (
      parts.next().unwrap().parse::<i32>().unwrap(),
      parts.next().unwrap().parse::<i32>().unwrap(),
      parts.next().unwrap().parse::<i32>().unwrap(),
    )
  });
  let mut x_range: (i32, i32) = (100, 0);
  let mut y_range: (i32, i32) = (100, 0);
  let mut z_range: (i32, i32) = (100, 0);
  voxels.clone().for_each(|voxel| {
    x_range = (x_range.0.min(voxel.0 as i32), x_range.1.max(voxel.0 as i32));
    y_range = (y_range.0.min(voxel.1 as i32), y_range.1.max(voxel.1 as i32));
    z_range = (z_range.0.min(voxel.2 as i32), z_range.1.max(voxel.2 as i32));
  });
  // println!("X {:?}", x_range);
  // println!("Y {:?}", y_range);
  // println!("Z {:?}", z_range);
  let mut voxel_space = [[[false; 20]; 20]; 20];
  let mut unique_voxels = HashSet::new();
  voxels.for_each(|voxel| {
    unique_voxels.insert(voxel);
    voxel_space[voxel.0 as usize][voxel.1 as usize][voxel.2 as usize] = true;
  });
  let mut exterior_voxels = HashSet::new();
  let mut queue = vec![(0, 0, 0)];
  while !queue.is_empty() {
    let (x, y, z) = queue.pop().unwrap();
    if exterior_voxels.contains(&(x, y, z))
      || x < 0
      || x > 19
      || y < 0
      || y > 19
      || z < 0
      || z > 19
      || voxel_space[x as usize][y as usize][z as usize]
    {
      continue;
    }
    exterior_voxels.insert((x, y, z));

    queue.push((x - 1, y, z));
    queue.push((x + 1, y, z));
    queue.push((x, y - 1, z));
    queue.push((x, y + 1, z));
    queue.push((x, y, z - 1));
    queue.push((x, y, z + 1));
  }

  let mut surface_area = 0;
  let mut external_surface_area = 0;
  for voxel in unique_voxels {
    // Check around the voxel for other voxels
    let mut unconnected_sides = 0;
    for (x, y, z) in vec![
      (-1, 0, 0),
      (1, 0, 0),
      (0, -1, 0),
      (0, 1, 0),
      (0, 0, -1),
      (0, 0, 1),
    ] {
      let new_x = voxel.0 + x;
      let new_y = voxel.1 + y;
      let new_z = voxel.2 + z;
      if new_x < 0
        || new_x > 19
        || new_y < 0
        || new_y > 19
        || new_z < 0
        || new_z > 19
        || !voxel_space[new_x as usize][new_y as usize][new_z as usize]
      {
        unconnected_sides += 1;
      }
      if exterior_voxels.contains(&(new_x, new_y, new_z))
        || new_x < 0
        || new_x > 19
        || new_y < 0
        || new_y > 19
        || new_z < 0
        || new_z > 19
      {
        external_surface_area += 1;
      }
    }
    surface_area += unconnected_sides;
  }
  // println!("Surface area: {}", surface_area);
  // println!("External surface area: {}", external_surface_area);
  println!("Day 18: {} {}", surface_area, external_surface_area);
}
