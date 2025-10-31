struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        loop {
            match (chars1.next(), chars2.next()) {
                (Some(c1), Some(c2)) => {
                    result.push(c1);
                    result.push(c2);
                }
                (Some(c1), None) => result.push(c1),
                (None, Some(c2)) => result.push(c2),
                (None, None) => break,
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_alternately() {
        assert_eq!(
            Solution::merge_alternately(String::from("abc"), String::from("pqr")),
            "apbqcr"
        );
        assert_eq!(
            Solution::merge_alternately(String::from("ab"), String::from("pqrs")),
            "apbqrs"
        );
        assert_eq!(
            Solution::merge_alternately(String::from("abcd"), String::from("pq")),
            "apbqcd"
        );
    }
}
