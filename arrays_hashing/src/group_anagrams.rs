use std::collections::HashMap;

fn sort_vec<T: Ord>(vec: &mut Vec<Vec<T>>) {
    vec.iter_mut().for_each(|v| v.sort());
    vec.sort();
}

#[test]
fn test_group_anagrams() {
    let mut expected = vec![vec!["bat"], vec!["eat", "tea", "ate"], vec!["tan", "nat"]];
    sort_vec(&mut expected);

    let mut actual = group_anagrams(
        vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    );
    sort_vec(&mut actual);

    assert_eq!(expected, actual);
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len());

    for candidate in strs.into_iter() {
        let mut chars: [u8; 26] = [0; 26];
        candidate.bytes().for_each(|c| {
            chars[(c - b'a') as usize] += 1;
        });

        anagrams
            .entry(chars)
            .and_modify(|seen| seen.push(candidate.clone()))
            .or_insert(vec![candidate]);
    }

    anagrams.into_values().collect()
}
