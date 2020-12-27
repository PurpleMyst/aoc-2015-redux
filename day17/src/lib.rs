const CONTAINERS: usize = 20;
const TARGET: u8 = 150;

#[inline]
pub fn solve() -> (usize, usize) {
    let mut counts = [[0; CONTAINERS + 1]; TARGET as usize + 1];
    counts[0][0] = 1;

    for container in include_str!("input.txt")
        .lines()
        .map(|n| n.parse::<u8>().unwrap())
    {
        for remaining in (0..=TARGET - container).rev() {
            for i in (1..=CONTAINERS).rev() {
                counts[(remaining + container) as usize][i] += counts[remaining as usize][i - 1];
            }
        }
    }

    let part1 = counts[TARGET as usize].iter().sum();

    let part2 = counts[TARGET as usize]
        .iter()
        .copied()
        .find(|&n| n != 0)
        .unwrap();

    (part1, part2)
}
