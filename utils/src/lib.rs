pub type Coord = (usize, usize);

pub fn parse_with_lens<'a, V, F>(
    lines: &'a str,
    f: &'a F,
) -> (Coord, impl Iterator<Item = (Coord, V)> + 'a)
where
    F: Fn(u8) -> V,
{
    let y_len = lines.lines().count();
    let x_len = lines.lines().next().map(|s| s.trim().len()).unwrap();
    let it = lines.lines().enumerate().flat_map(move |(y, line)| {
        line.trim()
            .bytes()
            .enumerate()
            .map(move |(x, b)| ((x, y), f(b)))
    });
    ((x_len, y_len), it)
}

pub const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub const fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}
