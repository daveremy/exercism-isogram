pub fn check(candidate: &str) -> bool {
    let mut sorted: Vec<char> = candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<char>>();
    sorted.sort();
    for (i, c) in sorted.iter().enumerate().skip(1) {
        if c == &sorted[i - 1] {
            return false;
        }
    }
    true
}
