use keyed_priority_queue::KeyedPriorityQueue;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[derive(Debug, Clone, Copy)]
pub struct Reaction {
    reagent: &'static str,
    product: &'static str,
}

fn products<'a>(molecule: &'a str, reactions: &'a [Reaction]) -> impl Iterator<Item = String> + 'a {
    reactions.iter().flat_map(move |reaction| {
        let mut start = 0;
        std::iter::from_fn(move || {
            let idx = molecule[start..]
                .find(reaction.reagent)
                .map(|offset| start + offset)?;

            let mut product = String::with_capacity(molecule.len() + reaction.reagent.len());
            product.push_str(&molecule[..idx]);
            product.push_str(reaction.product);
            product.push_str(&molecule[idx + reaction.reagent.len()..]);

            start = idx + reaction.reagent.len();
            Some(product)
        })
    })
}

fn products2<'a>(
    molecule: &'a str,
    reactions: &'a [Reaction],
    correct: usize,
) -> impl Iterator<Item = String> + 'a {
    let start = {
        let mut start = correct;

        start = start.saturating_sub(1);

        if molecule.as_bytes()[start].is_ascii_lowercase() {
            start = start.saturating_sub(1);
        }

        start
    };

    reactions.iter().flat_map(move |reaction| {
        let mut start = start;

        std::iter::from_fn(move || {
            let idx = molecule[start..]
                .find(reaction.reagent)
                .map(|offset| start + offset)?;

            let mut product = String::with_capacity(molecule.len() + reaction.reagent.len());
            product.push_str(&molecule[..idx]);
            product.push_str(reaction.product);
            product.push_str(&molecule[idx + reaction.reagent.len()..]);

            start = idx + reaction.reagent.len();
            Some(product)
        })
    })
}

#[inline]
pub fn solve_part1(molecule: &'static str, reactions: &[Reaction]) -> usize {
    // a trie might be useful?
    // could group together by product
    products(molecule, reactions).collect::<HashSet<_>>().len()
}

struct Priority {
    g: usize,
    f: usize,

    correct: usize,
}

impl PartialEq for Priority {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f && self.correct == other.correct
    }
}

impl Eq for Priority {}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // less f score = good
        // more correct = good

        (self.f.cmp(&other.f).reverse()).then(self.correct.cmp(&other.correct))
    }
}

fn compute_correct(target: &'static str, molecule: &str) -> usize {
    target
        .chars()
        .zip(molecule.chars())
        .take_while(|(a, b)| a == b)
        .count()
}

#[inline]
pub fn solve_part2(target: &'static str, reactions: &[Reaction]) -> usize {
    let start = String::from("e");

    let mut open = KeyedPriorityQueue::new();
    open.push(
        start.clone(),
        Priority {
            f: 0,
            g: 0,
            correct: 0,
        },
    );

    // g score is the cost of the cheapest (currently known) path from start to the given node
    let mut gs = HashMap::default();
    gs.insert(start, 0);

    while let Some((parent, parent_p)) = open.pop() {
        if parent == target {
            return parent_p.g;
        }

        for neighbor in products2(&parent, reactions, parent_p.correct) {
            let tentative_g = parent_p.g + 1;
            let correct = compute_correct(target, &neighbor);

            if correct < parent_p.correct {
                continue;
            }

            if gs.get(&neighbor).map_or(true, |&g| tentative_g < g) {
                // We consider the heuristic to be "how many characters *aren't* correct?"
                let f = tentative_g + (target.len() - correct);
                gs.insert(neighbor.clone(), tentative_g);
                open.push(
                    neighbor,
                    Priority {
                        g: tentative_g,
                        f,
                        correct,
                    },
                );
            }
        }
    }

    unreachable!()
}

#[inline]
pub fn load_input() -> (&'static str, Vec<Reaction>) {
    let mut input = include_str!("input.txt").trim().lines();

    let molecule = input.next_back().unwrap();

    // waste empty
    input.next_back();

    let reactions = input
        .map(|reaction| {
            let mut it = reaction.splitn(2, " => ");

            let reagent = it.next().unwrap();
            let product = it.next().unwrap();

            Reaction { reagent, product }
        })
        .collect::<Vec<_>>();

    (molecule, reactions)
}

#[inline]
pub fn solve() -> (usize, usize) {
    let (molecule, reactions) = load_input();

    (
        solve_part1(molecule, &reactions),
        solve_part2(molecule, &reactions),
    )
}
