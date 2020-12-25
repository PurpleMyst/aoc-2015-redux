use rayon::prelude::*;

fn starts_with_5_zeroes(prefix: &'static str, n: usize) -> bool {
    let mut ctx = md5::Context::new();
    ctx.consume(prefix);
    ctx.consume(n.to_string());
    let digest: [u8; 16] = ctx.compute().into();

    digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0
}

fn starts_with_6_zeroes(prefix: &'static str, n: usize) -> bool {
    let mut ctx = md5::Context::new();
    ctx.consume(prefix);
    ctx.consume(n.to_string());
    let digest: [u8; 16] = ctx.compute().into();

    digest[0] == 0 && digest[1] == 0 && digest[2] == 0
}

#[inline]
pub fn solve() -> (usize, usize) {
    let prefix = include_str!("input.txt").trim();

    let part1 = (0..=200_000usize)
        .into_par_iter()
        .find_first(|&n| starts_with_5_zeroes(prefix, n))
        .unwrap();

    let part2 = (part1..=5_000_000)
        .into_par_iter()
        .find_first(|&n| starts_with_6_zeroes(prefix, n))
        .unwrap();

    (part1, part2)
}
