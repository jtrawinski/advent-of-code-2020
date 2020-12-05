struct Slope {
    right: usize,
    down: usize,
}

impl From<(usize, usize)> for Slope {
    fn from(tuple: (usize, usize)) -> Slope {
        Slope {
            right: tuple.0,
            down: tuple.1,
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(map: &[String]) -> usize {
    solve_for_slope(map, &(3, 1).into())
}

#[aoc(day3, part2)]
pub fn solve_part2(map: &[String]) -> usize {
    let slopes: [Slope; 5] = [
        (1, 1).into(),
        (3, 1).into(),
        (5, 1).into(),
        (7, 1).into(),
        (1, 2).into(),
    ];
    slopes
        .iter()
        .map(|slope| solve_for_slope(map, slope))
        .product()
}

fn solve_for_slope(map: &[String], slope: &Slope) -> usize {
    map.iter()
        .enumerate()
        .skip(slope.down)
        .step_by(slope.down)
        .map(|(idx, line)| {
            let row_idx = (slope.right * idx / slope.down) % line.len();
            line.chars().nth(row_idx).unwrap()
        })
        .filter(|&c| c == '#')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let map = input_generator(input);
        let ans = solve_part2(&map);
        assert_eq!(ans, 336);
    }
}
