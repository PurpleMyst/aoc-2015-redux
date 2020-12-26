use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

fn parse_line(line: &str) -> (&str, &str, usize) {
    let mut it = line.split(' ');
    let from = it.next().unwrap();
    it.next().unwrap();
    let to = it.next().unwrap();
    it.next().unwrap();
    let dist = it.next().unwrap().parse().unwrap();
    (from, to, dist)
}

fn for_all_permutations(a: &mut [&'static str], mut f: impl FnMut(&[&'static str])) {
    let mut p = (0..=a.len()).collect::<Vec<_>>();

    let mut i = 1;

    f(&*a);

    while i < a.len() {
        p[i] -= 1;
        let j = if i % 2 == 1 { p[i] } else { 0 };
        a.swap(i, j);

        f(&*a);

        i = 1;
        while p[i] == 0 {
            p[i] = i;
            i += 1;
        }
    }

    f(&*a);
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut edges = HashMap::default();
    let mut cities = HashSet::default();

    include_str!("input.txt")
        .lines()
        .map(parse_line)
        .for_each(|(from, to, dist)| {
            cities.reserve(2);
            cities.insert(from);
            cities.insert(to);
            edges.reserve(2);
            edges.insert((from, to), dist);
            edges.insert((to, from), dist);
        });

    let mut cities = cities.into_iter().collect::<Vec<_>>();

    let mut part1 = usize::MAX;
    let mut part2 = 0;

    for_all_permutations(&mut cities[..], |perm| {
        let mut it = perm.iter().copied();
        let init = it.next().unwrap();

        let (total_distance, _) = it.fold((0, init), |(dist, prev), cur| {
            (dist + edges[&(prev, cur)], cur)
        });

        part1 = part1.min(total_distance);
        part2 = part2.max(total_distance);
    });

    (part1, part2)
}
