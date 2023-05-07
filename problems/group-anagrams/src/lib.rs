use std::collections::BTreeMap;

#[allow(dead_code)]
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: BTreeMap<BTreeMap<char, usize>, Vec<String>> = BTreeMap::new();

        fn get_chars(s: &str) -> BTreeMap<char, usize> {
            let mut chars = BTreeMap::new();

            for c in s.chars() {
                match chars.entry(c) {
                    std::collections::btree_map::Entry::Vacant(e) => {
                        e.insert(1);
                    }
                    std::collections::btree_map::Entry::Occupied(mut e) => {
                        *e.get_mut() += 1;
                    }
                }
            }

            chars
        }

        for str in strs {
            let chars = get_chars(&str);
            match groups.entry(chars) {
                std::collections::btree_map::Entry::Vacant(e) => {
                    e.insert(vec![str]);
                }
                std::collections::btree_map::Entry::Occupied(mut e) => {
                    e.get_mut().push(str);
                }
            };
        }

        groups.into_values().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn empty_string() {
        assert_eq!(
            Solution::group_anagrams(vec!["".to_string()]),
            vec![vec![""]]
        );
    }

    #[test]
    fn one_string() {
        assert_eq!(
            Solution::group_anagrams(vec!["a".to_string()]),
            vec![vec!["a"]]
        );
    }

    #[test]
    fn actual_test() {
        let groups = Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        assert!(groups.contains(&vec!["bat".to_string()]));
        assert!(groups
            .iter()
            .find(|group| group.contains(&"nat".to_string())
                && group.contains(&"tan".to_string())
                && group.len() == 2)
            .is_some());
        assert!(groups
            .iter()
            .find(|group| group.contains(&"ate".to_string())
                && group.contains(&"eat".to_string())
                && group.contains(&"tea".to_string())
                && group.len() == 3)
            .is_some());
        assert!(groups.len() == 3);
    }
}
