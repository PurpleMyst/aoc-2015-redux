use rustc_hash::FxHashSet as HashSet;

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

#[inline]
pub fn solve_part1(molecule: &'static str, reactions: &[Reaction]) -> usize {
    products(molecule, reactions).collect::<HashSet<_>>().len()
}

#[inline]
pub fn solve_part2(target: &'static str, _reactions: &[Reaction]) -> usize {
    let target = target.as_bytes();

    target.iter().filter(|&&ch| ch.is_ascii_uppercase()).count()
        - 2 * bytecount::naive_count_32(target, b'R')
        - 2 * bytecount::naive_count_32(target, b'Y')
        - 1
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
