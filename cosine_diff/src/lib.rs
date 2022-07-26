use std::collections::HashSet;

/// Returns the case-insensitive bigram cosine similarity between two strings.
/// A `1` means the strings are identical, while a `0` means they are completely
/// different. Returns `NaN` if one of the strings has < 2 characters.
pub fn str_diff(a: &str, b: &str) -> f32 {
    cos_sim(&ngram(&a.to_lowercase(), &b.to_lowercase(), 2))
}

/// Returns the case-insensitive n-gram cosine similarity between two strings.
/// A `1` means the strings are identical, while a `0` means they are completely
/// different.
pub fn str_diff_n(a: &str, b: &str, n: u32) -> f32 {
    cos_sim(&ngram(&a.to_lowercase(), &b.to_lowercase(), n))
}

// Returns the term frequency of `n` consecutive characters between two strings.
// The order of the terms is not guarenteed, but will always be consistent
// between the two returned vectors (order could be guarenteed with a BTreeSet,
// but that is slower).
pub fn ngram(s1: &str, s2: &str, n: u32) -> (Vec<u32>, Vec<u32>) {
    let n = n as usize;

    let mut grams = HashSet::<&str>::new();
    for i in 0..((s1.len() + 1).saturating_sub(n)) {
        grams.insert(&s1[i..(i + n)]);
    }
    for i in 0..((s2.len() + 1).saturating_sub(n)) {
        grams.insert(&s2[i..(i + n)]);
    }

    let mut q1 = Vec::new();
    let mut q2 = Vec::new();
    for i in grams {
        q1.push(s1.matches(i).count() as u32);
        q2.push(s2.matches(i).count() as u32);
    }
    (q1, q2)
}

// Returns the dot product of two slices of equal length. Returns an `Err` if
// the slices are not of equal length.
fn dot_prod(a: &[u32], b: &[u32]) -> Result<u32, &'static str> {
    if a.len() != b.len() {
        return Err("Vectors must be of equal length");
    }

    let mut v = Vec::new();
    for i in 0..a.len() {
        v.push(a[i] * b[i]);
    }
    Ok(v.iter().sum())
}

// Returns the cosine similarity between two vectors of equal length.
// `S_c(A, B) = (A · B) / (||A|| ||B||)`
fn cos_sim((a, b): &(Vec<u32>, Vec<u32>)) -> f32 {
    let a_mag = (dot_prod(a, a).unwrap() as f32).sqrt();
    let b_mag = (dot_prod(b, b).unwrap() as f32).sqrt();

    // use `min` and `max` to constrain floating point errors within 0..=1
    (dot_prod(a, b).unwrap() as f32 / (a_mag * b_mag).max(0.0)).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same() {
        let a = "abc";
        let b = "abc";
        assert_eq!(str_diff(a, b), 1.0);
    }

    #[test]
    fn diff() {
        let a = "abc";
        let b = "def";

        assert_eq!(str_diff(a, b), 0.0);
    }

    #[test]
    fn close() {
        let a = "abcde";
        let b = "abdcde";

        assert_eq!(str_diff(a, b), 0.67082036);
    }
}
