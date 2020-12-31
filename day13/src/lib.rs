use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

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
pub fn solve() -> (i64, i64) {
    let mut relations = HashMap::default();
    let mut people = HashSet::default();

    include_str!("input.txt").lines().for_each(|line| {
        let mut words = line.split(' ');

        let subject = words.next().unwrap();

        let sign = match words.nth(1).unwrap() {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!(),
        };

        let units = words.next().unwrap().parse::<i64>().unwrap();

        let delta = sign * units;

        let object = words.nth(6).unwrap().trim_end_matches('.');

        people.insert(subject);
        relations.insert((subject, object), delta);
    });

    let mut people = people.into_iter().collect::<Vec<_>>();

    let mut p1 = 0;
    let mut p2 = 0;

    for_all_permutations(&mut people, |people| {
        // Iterate over all the people
        let mut it = people.iter().copied();

        // Save the first person in the list, we'll need them later
        let first = it.next().unwrap();

        // Fold over the remaining people, adding to the total the mutual change
        // between the previous and the current person each time
        let mut total = 0;
        let last = it.fold(first, |prev, cur| {
            total += relations[&(prev, cur)] + relations[&(cur, prev)];

            cur
        });

        // Now, we've considered every relationship *except* the one between the
        // first person seated at the table and the last one. Here's the trick:
        // If we place ourselves between those two, then that interaction
        // doesn't exist, and that counts as a permutation for part 2
        p2 = p2.max(total);

        // Now we "remove" ourselves from the table by adding to the total the
        // relation between the last person and the first one and consider that
        // as a permutation for part 1
        total += relations[&(first, last)] + relations[&(last, first)];
        p1 = p1.max(total);
    });

    (p1, p2)
}
