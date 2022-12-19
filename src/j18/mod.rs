use std::str::FromStr;

use smallvec::{SmallVec, smallvec};

pub fn check_neighbour((x1, y1, z1): (i8, i8, i8),
                       compute_check_trapped: bool,
                       world: &WorldType) -> usize {
    if x1 < 0 || y1 < 0 || z1 < 0 ||
        x1 as usize >= EXPECTED_CUBE_SIDE ||
        y1 as usize >= EXPECTED_CUBE_SIDE ||
        z1 as usize >= EXPECTED_CUBE_SIDE {
        return 1;
    }

    match world[x1 as usize][y1 as usize][z1 as usize] {
        CellType::Inside => {
            usize::from(!compute_check_trapped)
        }
        CellType::Rock => { 0 }
        CellType::Outside => { 1 }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CellType {
    Inside,
    Rock,
    Outside,
}

const EXPECTED_CUBE_SIDE: usize = 20;
const EXPECTED_NUMBER_OF_LAVA_CUBES: usize = 2043;

pub type WorldType = [[[CellType; EXPECTED_CUBE_SIDE]; EXPECTED_CUBE_SIDE]; EXPECTED_CUBE_SIDE];

pub fn try_expand_outside_from((x, y, z): (usize, usize, usize), world: &mut WorldType) {
    if world[x][y][z] != CellType::Inside {
        return;
    }

    world[x][y][z] = CellType::Outside;

    if x > 0 {
        try_expand_outside_from((x - 1, y, z), world);
    }
    if x < EXPECTED_CUBE_SIDE - 1 {
        try_expand_outside_from((x + 1, y, z), world);
    }
    if y > 0 {
        try_expand_outside_from((x, y - 1, z), world);
    }
    if y < EXPECTED_CUBE_SIDE - 1 {
        try_expand_outside_from((x, y + 1, z), world);
    }
    if z > 0 {
        try_expand_outside_from((x, y, z - 1), world);
    }
    if z < EXPECTED_CUBE_SIDE - 1 {
        try_expand_outside_from((x, y, z + 1), world);
    }
}

pub fn compute_grid_world(cubes: &SmallVec<[(i8, i8, i8); EXPECTED_NUMBER_OF_LAVA_CUBES]>) -> WorldType {
    let mut world: WorldType = [[[CellType::Inside; EXPECTED_CUBE_SIDE]; EXPECTED_CUBE_SIDE]; EXPECTED_CUBE_SIDE];

    for &(x, y, z) in cubes {
        world[x as usize][y as usize][z as usize] = CellType::Rock;
    }

    for it1 in 0..EXPECTED_CUBE_SIDE {
        for it2 in 0..EXPECTED_CUBE_SIDE {
            try_expand_outside_from((it1, it2, 0), &mut world);
            try_expand_outside_from((it1, it2, EXPECTED_CUBE_SIDE - 1), &mut world);
            try_expand_outside_from((it1, 0, it2), &mut world);
            try_expand_outside_from((it1, EXPECTED_CUBE_SIDE - 1, it2), &mut world);
            try_expand_outside_from((0, it1, it2), &mut world);
            try_expand_outside_from((EXPECTED_CUBE_SIDE - 1, it1, it2), &mut world);
        }
    }

    world
}

pub fn parse_exposed_faces(s: &str, compute_trapped_cubes: bool) -> usize {
    let mut cubes: SmallVec<[(i8, i8, i8); EXPECTED_NUMBER_OF_LAVA_CUBES]> = smallvec![];

    let mut min = (i8::MAX, i8::MAX, i8::MAX);
    let mut max = (0, 0, 0);

    for line in s.lines() {
        let mut coords = line.split(',');
        let cube_coord = (
            i8::from_str(coords.next().unwrap()).unwrap(),
            i8::from_str(coords.next().unwrap()).unwrap(),
            i8::from_str(coords.next().unwrap()).unwrap(),
        );
        min.0 = min.0.min(cube_coord.0);
        min.1 = min.1.min(cube_coord.1);
        min.2 = min.2.min(cube_coord.2);
        max.0 = max.0.max(cube_coord.0);
        max.1 = max.1.max(cube_coord.1);
        max.2 = max.2.max(cube_coord.2);
        cubes.push(cube_coord);
    }

    let world = compute_grid_world(&cubes);

    let mut exposed_sides = 0usize;
    for &(x1, y1, z1) in cubes.iter() {
        exposed_sides +=
            check_neighbour((x1 + 1, y1, z1), compute_trapped_cubes, &world) +
                check_neighbour((x1 - 1, y1, z1), compute_trapped_cubes, &world) +
                check_neighbour((x1, y1 + 1, z1), compute_trapped_cubes, &world) +
                check_neighbour((x1, y1 - 1, z1), compute_trapped_cubes, &world) +
                check_neighbour((x1, y1, z1 + 1), compute_trapped_cubes, &world) +
                check_neighbour((x1, y1, z1 - 1), compute_trapped_cubes, &world);
    }

    exposed_sides
}


#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    parse_exposed_faces(s, false)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j18.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    parse_exposed_faces(s, true)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j18.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j18_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(64, _p1(include_str!("j18_test.txt")));
        assert_eq!(3522, _p1(include_str!("j18.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(58, _p2(include_str!("j18_test.txt")));
        assert_eq!(2074, _p2(include_str!("j18.txt")));
    }
}