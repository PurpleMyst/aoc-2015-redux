#[inline]
pub fn solve() -> (usize, usize) {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut dimensions = line.split("x").map(|p| p.parse::<usize>().unwrap());
            (
                dimensions.next().unwrap(),
                dimensions.next().unwrap(),
                dimensions.next().unwrap(),
            )
        })
        .fold((0, 0), |(part1, part2), (l, w, h)| {
            (
                part1 + 2 * l * w + 2 * w * h + 2 * h * l + (l * w).min(l * h).min(w * h),
                part2 + ((2 * (l + w)).min(2 * (l + h)).min(2 * (w + h))) + l * w * h,
            )
        })
}
