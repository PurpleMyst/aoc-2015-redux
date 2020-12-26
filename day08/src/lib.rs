use memchr::memchr;

fn in_memory(line: &str) -> usize {
    debug_assert!(line.starts_with('"') && line.ends_with('"'));
    let mut line = line[1..line.len() - 1].as_bytes();

    let mut size = line.len();

    while let Some(idx) = memchr(b'\\', line) {
        size -= 1;
        line = if line[idx + 1] == b'x' {
            size -= 2;
            &line[idx + 4..]
        } else {
            &line[idx + 2..]
        };
    }

    size
}

fn escaped(line: &str) -> usize {
    let line = line.as_bytes();
    line.len() + 2 + bytecount::naive_count_32(line, b'\\') + bytecount::naive_count_32(line, b'"')
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;

    include_str!("input.txt").lines().for_each(|line| {
        part1 += line.len() - in_memory(line);
        part2 += escaped(line) - line.len();
    });

    (part1, part2)
}
