#[inline]
pub fn solve() -> (usize, usize) {
    let mut state = include_str!("input.txt").trim().to_string();
    let mut next_state = String::with_capacity(state.len() * 2);

    let mut part1 = 0;

    for i in 0..50 {
        next_state.clear();

        let mut it = state.chars();
        let first = it.next().unwrap();
        let (cnt, last) = it.fold((1, first), |(cnt, group), digit| {
            if digit == group {
                (cnt + 1, group)
            } else {
                next_state.push_str(&format!("{}{}", cnt, group));
                (1, digit)
            }
        });
        next_state.push_str(&format!("{}{}", cnt, last));

        state.clone_from(&next_state);

        if i == 39 {
            part1 = state.len();
        }
    }

    (part1, state.len())
}
