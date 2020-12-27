use std::cmp::Ordering;

const CONTAINERS: usize = 20;

const TARGET: u8 = 150;

fn recurse(remaining: u8, containers: &[u8], len: usize, part2: &mut (usize, usize)) -> usize {
    match containers.split_first().unwrap() {
        (&container, &[]) => {
            if remaining == container {
                if part2.0 > len {
                    *part2 = (len, 1);
                } else if len == part2.0 {
                    part2.1 += 1;
                }

                1
            } else {
                0
            }
        }

        (container, rest) => {
            // consider the case that we don't use this container
            recurse(remaining, rest, len, part2)
                + match remaining.cmp(container) {
                    // if we can't use this container at this point, stop
                    Ordering::Less => 0,
                    // if we can fill this container exactly, do so
                    Ordering::Equal => {
                        if part2.0 > len {
                            *part2 = (len, 1);
                        } else if len == part2.0 {
                            part2.1 += 1;
                        }

                        1
                    }
                    // otherwise, put all that we can in it and recurse
                    Ordering::Greater => recurse(remaining - container, rest, len + 1, part2),
                }
        }
    }
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut containers = [0; CONTAINERS];

    include_str!("input.txt")
        .lines()
        .map(|n| n.parse::<u8>().unwrap())
        .zip(containers.iter_mut())
        .for_each(|(val, dest)| *dest = val);

    let mut part2 = (usize::MAX, 0);

    (recurse(TARGET, &containers, 0, &mut part2), part2.1)
}
