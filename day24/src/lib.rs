use std::{cmp::Ordering, collections::HashSet};

fn find_groups(packages: &[u8], remaining: u16, acc_mask: u32, callback: &mut impl FnMut(u32)) {
    if let Some((head, rest)) = packages.split_first() {
        let head = *head as u16;

        match remaining.cmp(&head) {
            Ordering::Equal => {
                let acc_mask = ((acc_mask << 1) | 1) << rest.len();
                callback(acc_mask)
            }

            Ordering::Greater => find_groups(rest, remaining - head, (acc_mask << 1) | 1, callback),

            Ordering::Less => {}
        }

        find_groups(rest, remaining, acc_mask << 1, callback)
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

    let mut groups = HashSet::with_capacity(500_000);
    find_groups(&packages, target, 0, &mut |m| {
        groups.insert(m);
    });

    let l = groups.iter().map(|g| g.count_ones()).min().unwrap();
    groups.retain(|g| g.count_ones() == l);

    groups.iter().map(|&a| qe(&packages, a)).min().unwrap()
}

#[inline]
pub fn solve_part2(packages: &[u8]) -> u64 {
    let target = packages.iter().map(|&n| n as u16).sum::<u16>() / 4;

    let mut groups = HashSet::with_capacity(500_000);
    find_groups(&packages, target, 0, &mut |m| {
        groups.insert(m);
    });

    let l = groups.iter().map(|g| g.count_ones()).min().unwrap();
    groups.retain(|g| g.count_ones() == l);

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
