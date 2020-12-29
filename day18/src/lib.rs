use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

const GENERATIONS: usize = 100;
const GRID_SIDE: i8 = 100;

#[inline]
pub fn parse_input() -> HashSet<(i8, i8)> {
    include_str!("input.txt")
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes().enumerate().filter_map(move |(x, ch)| {
                if ch == b'#' {
                    Some((x as i8, y as i8))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<_>>()
}

#[inline]
pub fn run(mut state: HashSet<(i8, i8)>, part2: bool) -> usize {
    let mut next_state = Vec::new();
    let mut inactive: HashMap<_, usize> = HashMap::default();

    for _ in 0..GENERATIONS {
        if part2 {
            state.insert((0, 0));
            state.insert((GRID_SIDE - 1, 0));
            state.insert((0, GRID_SIDE - 1));
            state.insert((GRID_SIDE - 1, GRID_SIDE - 1));
        }

        for &(x, y) in &state {
            let mut active_neighbors = 0;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let neighbor = (x + dx, y + dy);

                    if state.contains(&neighbor) {
                        active_neighbors += 1;
                    } else {
                        *inactive.entry(neighbor).or_default() += 1;
                    }
                }
            }

            if active_neighbors == 2 || active_neighbors == 3 {
                next_state.push((x, y))
            }
        }

        next_state.extend(
            inactive
                .drain()
                .filter(|&(_, neighbors)| neighbors == 3)
                .map(|(pos, _)| pos),
        );

        state.clear();
        state.extend(next_state.drain(..));
        state.retain(|(x, y)| (0..GRID_SIDE).contains(x) && (0..GRID_SIDE).contains(y));
    }

    if part2 {
        state.insert((0, 0));
        state.insert((GRID_SIDE - 1, 0));
        state.insert((0, GRID_SIDE - 1));
        state.insert((GRID_SIDE - 1, GRID_SIDE - 1));
    }

    state.len()
}

#[inline]
pub fn solve() -> (usize, usize) {
    let input = parse_input();
    (run(input.clone(), false), run(input, true))
}
