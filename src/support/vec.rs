pub fn vov<T, const A: usize, const B: usize>(x: [[T; A]; B]) -> Vec<Vec<T>> {
    x.into_iter().map(|a| a.into_iter().collect()).collect()
}
pub fn vos<const A: usize>(x: [&str; A]) -> Vec<String> {
    x.into_iter().map(|a| a.chars().collect()).collect()
}
