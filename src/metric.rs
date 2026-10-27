pub fn hamming_metric(lhs: &str, rhs: &str) -> usize {
    let mut distance = lhs.len().max(rhs.len())-lhs.len().min(rhs.len());
    for (a, b) in lhs.chars().zip(rhs.chars()) {
        if a != b {
            distance += 1
        }
    }
    distance
}