use std::collections::HashMap;
use std::str::Chars;

struct Trie {
    chars: HashMap<char, Trie>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            chars: HashMap::new(),
        }
    }

    fn _insert(&mut self, mut chars: Chars) {
        if let Some(c) = chars.next() {
            let trie = self.chars.entry(c).or_insert(Trie::new());
            trie._insert(chars);
        } else {
            self.chars.entry('\0').or_insert(Trie::new());
        }
    }

    fn insert(&mut self, word: String) {
        self._insert(word.chars());
    }

    fn _search(&self, mut chars: Chars) -> bool {
        if let Some(c) = chars.next() {
            return match self.chars.get(&c) {
                Some(trie) => trie._search(chars),
                None => false,
            };
        }
        self.chars.contains_key(&'\0')
    }

    fn search(&self, word: String) -> bool {
        self._search(word.chars())
    }

    fn _starts_with(&self, mut chars: Chars) -> bool {
        if let Some(c) = chars.next() {
            return match self.chars.get(&c) {
                Some(trie) => trie._starts_with(chars),
                None => false,
            };
        }
        true
    }

    fn starts_with(&self, prefix: String) -> bool {
        self._starts_with(prefix.chars())
    }
}
