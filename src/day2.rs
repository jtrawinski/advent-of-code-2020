pub struct Entry {
    range_btm: usize,
    range_top: usize,
    letter: char,
    password: String
}

impl Entry {
    fn valid_part1(&self) -> bool {
        let num_letter = self.password.chars().filter(|&c| c == self.letter).count();
        num_letter >= self.range_btm && num_letter <= self.range_top
    }

    fn valid_part2(&self) -> bool {
        let fst_in_pos = self.password.chars().nth(self.range_btm - 1).unwrap() == self.letter;
        let snd_in_pos = self.password.chars().nth(self.range_top - 1).unwrap() == self.letter;
        fst_in_pos != snd_in_pos
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    input.lines().map(parse).collect()
}

fn parse(input: &str) -> Entry {
    // expect input as [\d+]-[\d+] \w: \w+
    // use of take_while relies on the proper spacing since it will consume the first element for which
    // the predicate returns false
    let mut input_chars = input.chars();
    let range_btm = input_chars.by_ref()
                        .take_while(|&c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
    let range_top = input_chars.by_ref()
                        .take_while(|&c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
    let letter = input_chars.by_ref()
                    .take_while(|&c| c.is_ascii_alphabetic())
                    .next()
                    .unwrap();
    let password = input_chars.filter(|&c| c.is_ascii_alphabetic()).collect::<String>();
    Entry {
        range_btm,
        range_top,
        letter,
        password
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(inputs: &[Entry]) -> usize {
    inputs.iter().filter(|&entry| entry.valid_part1()).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(inputs: &[Entry]) -> usize {
    inputs.iter().filter(|&entry| entry.valid_part2()).count()
}

