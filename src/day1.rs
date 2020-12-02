use itertools::Itertools;

const TARGET: i32 = 2020;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(inputs: &[i32]) -> i32 {
    // Vec is 25% faster than a HashSet here
    let set: Vec<_> = inputs.iter().collect();
    *inputs.iter().find(|&i| set.contains(&&(TARGET-i))).unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(inputs: &[i32]) -> i32 {
    // Vec is 25% faster than a HashSet here
    let set: Vec<_> = inputs.iter().collect();
    let (&a, &b) = inputs.iter().tuple_combinations().find(|(&a, &b)| set.contains(&&(TARGET - a - b))).unwrap();
    a*b*(TARGET-a-b)
}