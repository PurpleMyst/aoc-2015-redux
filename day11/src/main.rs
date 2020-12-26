fn main() {
    let (part1, part2) = day11::solve();
    println!("{}", std::str::from_utf8(&part1[..]).unwrap());
    println!("{}", std::str::from_utf8(&part2[..]).unwrap());
}
