#[cfg(test)]
mod tests {
    use crate::solutions::*;

    #[test]
    fn test_contains_duplicate() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }

    #[test]
    fn test_is_anagram() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_group_anagrams() {
        assert_eq!(
            group_anagrams(
                vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            vec![vec!["bat"], vec!["eat", "tea", "ate"], vec!["tan", "nat"]]
        );
    }

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 2), vec![1]);
    }
}
