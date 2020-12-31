use std::cmp::Ordering;

fn find_groups(
    packages: &[u8],
    remaining: u16,
    group: u32,
    best_len: &mut u32,
    out: &mut Vec<u32>,
) {
    if group.count_ones() > *best_len {
        return;
    }

    if let Some((head, rest)) = packages.split_first() {
        let head = *head as u16;

        match remaining.cmp(&head) {
            Ordering::Equal => {
                let group = ((group << 1) | 1) << rest.len();
                let len = group.count_ones();

                if len < *best_len {
                    *best_len = len;
                    out.clear();
                }

                out.push(group)
            }

            Ordering::Greater => {
                find_groups(rest, remaining - head, (group << 1) | 1, best_len, out)
            }

            Ordering::Less => {}
        }

        find_groups(rest, remaining, group << 1, best_len, out)
    }
}

fn qe(packages: &[u8], mut mask: u32) -> u64 {
    let mut it = packages.iter();
    let mut res: u64 = 1;

    while mask != 0 {
        let val = *it.next_back().unwrap();

        if mask & 1 == 1 {
            res *= val as u64;
        }

        mask >>= 1;
    }

    res
}

#[inline]
pub fn solve_part1(packages: &[u8]) -> u64 {
    let target = packages.iter().map(|&n| n as u16).sum::<u16>() / 3;

    let mut groups = Vec::with_capacity(500_000);
    let mut best_len = packages.len() as u32 / 3;
    find_groups(&packages, target, 0, &mut best_len, &mut groups);

    groups.iter().map(|&a| qe(&packages, a)).min().unwrap()
}

#[inline]
pub fn solve_part2(packages: &[u8]) -> u64 {
    let target = packages.iter().map(|&n| n as u16).sum::<u16>() / 4;

    let mut groups = Vec::with_capacity(500_000);
    let mut best_len = packages.len() as u32 / 4;
    find_groups(&packages, target, 0, &mut best_len, &mut groups);

    groups.iter().map(|&a| qe(&packages, a)).min().unwrap()
}

#[inline]
pub fn load_input() -> Vec<u8> {
    include_str!("input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[inline]
pub fn solve() -> (u64, u64) {
    let packages = load_input();
    debug_assert!(packages.len() <= 32);
    (solve_part1(&packages), solve_part2(&packages))
}
