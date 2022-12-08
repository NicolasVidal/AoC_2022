#[inline(always)]
fn parse_matrix(s: &str) -> Vec<Vec<u8>> {
    let mut trees = vec!();

    for line in s.lines() {
        let mut row = vec!();
        for c in line.chars() {
            row.push(c as u8);
        }
        trees.push(row);
    }

    trees
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let trees = parse_matrix(s);

    let height = trees.len();
    let width = trees.iter().next().unwrap().len();

    let mut total = 0;

    for row in 0..height {
        for col in 0..width {
            let tree_height = trees[row][col];
            if ((row + 1)..height).all(|l| trees[l][col] < tree_height) {
                total += 1;
                continue;
            }
            if (0..row).all(|l| trees[l][col] < tree_height) {
                total += 1;
                continue;
            }
            if ((col + 1)..width).all(|l| trees[row][l] < tree_height) {
                total += 1;
                continue;
            }
            if (0..col).all(|l| trees[row][l] < tree_height) {
                total += 1;
                continue;
            }
        }
    }

    total
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j8.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let trees = parse_matrix(s);

    let height = trees.len();
    let width = trees.iter().next().unwrap().len();

    let mut best_viewing_distance = 0;

    for row in 0..height {
        for col in 0..width {
            let tree_height = trees[row][col];
            let mut left = 1;
            let mut right = 1;
            let mut up = 1;
            let mut down = 1;

            up = ((row + 1)..(height - 1)).take_while(|l| trees[*l][col] < tree_height).count() + 1;
            down = (1..row).rev().take_while(|l| trees[*l][col] < tree_height).count() + 1;
            right = ((col + 1)..(width - 1)).take_while(|l| trees[row][*l] < tree_height).count() + 1;
            left = (1..col).rev().take_while(|l| trees[row][*l] < tree_height).count() + 1;

            let viewing_distance = right * left * down * up;

            best_viewing_distance = best_viewing_distance.max(viewing_distance);
        }
    }

    best_viewing_distance
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j8.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j8_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(21, _p1(include_str!("j8_test.txt")));
        assert_eq!(1690, _p1(include_str!("j8.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(16, _p2(include_str!("j8_test.txt")));
        assert_eq!(535680, _p2(include_str!("j8.txt")));
    }
}