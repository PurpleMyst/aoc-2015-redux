use rustc_hash::FxHashSet as HashSet;

#[inline]
pub fn solve() -> (usize, usize) {
    let mut part1_visited = HashSet::default();
    let mut part2_human_visited = HashSet::default();
    let mut part2_robo_visited = HashSet::default();
    part1_visited.insert((0, 0));
    part2_human_visited.insert((0, 0));
    part2_robo_visited.insert((0, 0));

    let mut part1_pos = (0, 0);
    let mut part2_human_pos = (0, 0);
    let mut part2_robo_pos = (0, 0);

    for (i, b) in include_str!("input.txt").trim().bytes().enumerate() {
        let (part2_pos, part2_visited) = if i % 2 == 0 {
            (&mut part2_human_pos, &mut part2_human_visited)
        } else {
            (&mut part2_robo_pos, &mut part2_robo_visited)
        };

        match b {
            b'<' => {
                part1_pos.0 -= 1;
                part2_pos.0 -= 1;
            }

            b'>' => {
                part1_pos.0 += 1;
                part2_pos.0 += 1;
            }

            b'^' => {
                part1_pos.1 -= 1;
                part2_pos.1 -= 1;
            }

            b'v' => {
                part1_pos.1 += 1;
                part2_pos.1 += 1;
            }

            _ => unreachable!(),
        }

        part1_visited.insert(part1_pos);
        part2_visited.insert(*part2_pos);
    }

    let mut part2_visited = part2_human_visited;
    part2_visited.extend(part2_robo_visited.into_iter());

    (part1_visited.len(), part2_visited.len())
}
