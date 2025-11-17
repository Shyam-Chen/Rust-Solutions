use super::*;

#[test]
fn trie() {
    let mut trie = Trie::new();

    trie.insert("apple".into());
    assert_eq!(trie.search("apple".into()), true);
    assert_eq!(trie.search("app".into()), false);
    assert_eq!(trie.starts_with("app".into()), true);

    trie.insert("app".into());
    assert_eq!(trie.search("app".into()), true);
}
