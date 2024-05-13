use std::collections::HashSet;

pub fn is_unique(s: &str) -> bool {
    println!("Word ....{}", s);
    let mut letters: HashSet<char> = HashSet::new();
    for c in s.chars() {
        println!("{:?}", c);
        if letters.contains(&c) {
            return false;
        }
        letters.insert(c);
    }
    true
}

#[test]
fn is_unique_test() {
    assert_eq!(is_unique("abcde"), true);
}

#[test]
fn is_not_unique_test() {
    assert_eq!(is_unique("abcded"), false);
}
