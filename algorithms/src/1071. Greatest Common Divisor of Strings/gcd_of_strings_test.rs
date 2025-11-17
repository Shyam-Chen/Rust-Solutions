use super::*;

#[test]
fn gcd_of_strings() {
    assert_eq!(
        Solution::gcd_of_strings(String::from("ABCABC"), String::from("ABC")),
        "ABC"
    );
    assert_eq!(
        Solution::gcd_of_strings(String::from("ABABAB"), String::from("ABAB")),
        "AB"
    );
    assert_eq!(
        Solution::gcd_of_strings(String::from("LEET"), String::from("CODE")),
        ""
    );
}
