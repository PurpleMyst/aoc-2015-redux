const PRESENTS_PART1: usize = 10;
const PRESENTS_PART2: usize = 11;
const LIMIT_PART2: usize = 50;

const HOUSES: usize = 850_000;

#[inline]
pub fn solve_part1(input: usize) -> usize {
    let mut houses = vec![0usize; HOUSES].into_boxed_slice();

    for elf in 1..input / PRESENTS_PART1 {
        for house in (elf..HOUSES).step_by(elf) {
            houses[house] += PRESENTS_PART1 * elf;
        }
    }

    houses
        .iter()
        .position(|&presents| presents >= input)
        .unwrap()
}

#[inline]
pub fn solve_part2(input: usize) -> usize {
    let mut houses = vec![0usize; HOUSES].into_boxed_slice();

    for elf in 1..input / PRESENTS_PART2 {
        for house in (elf..HOUSES).step_by(elf).take(LIMIT_PART2) {
            houses[house] += PRESENTS_PART2 * elf;
        }
    }

    houses
        .iter()
        .position(|&presents| presents >= input)
        .unwrap()
}

#[inline]
pub fn load_input() -> usize {
    include_str!("input.txt").trim().parse::<usize>().unwrap()
}

#[inline]
pub fn solve() -> (usize, usize) {
    let input = load_input();
    (solve_part1(input), solve_part2(input))
}
