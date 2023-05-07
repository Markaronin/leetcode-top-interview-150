use std::collections::HashMap;

#[allow(dead_code)]
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_isomorphic(mut s: String, mut t: String) -> bool {
        if s.len() != t.len() {
            false
        } else {
            let mut seen_chars: HashMap<char, usize> = HashMap::new();
            s = s
                .chars()
                .map(|c| {
                    let seen_chars_len = seen_chars.len();
                    match seen_chars.entry(c) {
                        std::collections::hash_map::Entry::Occupied(e) => e.get().to_string(),
                        std::collections::hash_map::Entry::Vacant(e) => {
                            let v = seen_chars_len;
                            e.insert(v);
                            v.to_string()
                        }
                    }
                })
                .collect();

            seen_chars.clear();

            t = t
                .chars()
                .map(|c| {
                    let seen_chars_len = seen_chars.len();
                    match seen_chars.entry(c) {
                        std::collections::hash_map::Entry::Occupied(e) => e.get().to_string(),
                        std::collections::hash_map::Entry::Vacant(e) => {
                            let v = seen_chars_len;
                            e.insert(v);
                            v.to_string()
                        }
                    }
                })
                .collect();

            println!("{s}, {t}");

            s == t
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn empty_strings() {
        assert!(Solution::is_isomorphic("".to_string(), "".to_string()));
    }

    #[test]
    fn different_sized_strings() {
        assert!(!Solution::is_isomorphic("a".to_string(), "ab".to_string()));
    }

    #[test]
    fn same_sized_not_isomorphic() {
        assert!(!Solution::is_isomorphic("ab".to_string(), "aa".to_string()));
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));
    }

    #[test]
    fn same_sized_isomorphic() {
        assert!(Solution::is_isomorphic(
            "title".to_string(),
            "paper".to_string()
        ));
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));
    }
}
