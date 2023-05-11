pub const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}
