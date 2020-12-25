#[inline]
pub fn solve() -> (i16, usize) {
    let mut pos = 0;

    let mut it = include_str!("input.txt").trim().bytes().map(|ch| {
        pos = match ch {
            b'(' => pos + 1,
            b')' => pos - 1,
            _ => unreachable!(),
        };

        pos
    });

    let part2 = it.by_ref().position(|pos| pos < 0).unwrap() + 1;

    let part1 = it.last().unwrap();

    (part1, part2)
}
