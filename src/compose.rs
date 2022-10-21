use std::collections::HashMap;

pub struct CompositionMap {
    above: HashMap<&'static str, char>,
}

impl CompositionMap {
    pub fn new() -> Self {
        let above: HashMap<&str, char> = HashMap::from([
            ("grave", char::from_u32(0x0300).unwrap()),
            ("acute", char::from_u32(0x0301).unwrap()),
            ("circumflex", char::from_u32(0x302).unwrap()),
            ("tilde", char::from_u32(0x303).unwrap()),
            ("macron", char::from_u32(0x304).unwrap()),
            ("overline", char::from_u32(0x305).unwrap()),
            ("breve", char::from_u32(0x306).unwrap()),
            ("dot above", char::from_u32(0x307).unwrap()),
            ("diaresis", char::from_u32(0x308).unwrap()),
            ("turned comma", char::from_u32(0x312).unwrap()),
            ("comma", char::from_u32(0x313).unwrap()),
            ("reversed comma", char::from_u32(0x314).unwrap()),
            ("periposmeni", char::from_u32(0x342).unwrap()),
            ("iota", char::from_u32(0x345).unwrap()),
        ]);
        Self { above }
    }

    pub fn above(&self, name: &str) -> char {
        *self.above.get(name).unwrap()
    }
}

pub fn compose_vec(seq: Vec<char>) -> Vec<char> {
    if seq.is_empty() {
        return seq;
    }
    seq[1..]
        .iter()
        .fold(vec![seq[0]], |mut acc, next| -> Vec<char> {
            match unicode_normalization::char::compose(*acc.last().unwrap(), *next) {
                Some(out) => {
                    acc.pop();
                    acc.push(out);
                }
                None => {
                    acc.push(*next);
                }
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(compose_vec(vec![]), vec![]);
    }
    #[test]
    fn nop() {
        assert_eq!(compose_vec(vec!['a']), vec!['a']);
    }
    #[test]
    fn many_no_compose() {
        assert_eq!(compose_vec(vec!['a', 'b', 'c']), vec!['a', 'b', 'c']);
    }
    #[test]
    fn simple() {
        assert_eq!(compose_vec(vec!['α', '\u{304}']), vec!['ᾱ']);
    }
    #[test]
    fn cant_compose() {
        assert_eq!(compose_vec(vec!['ξ', '\u{304}']), vec!['ξ', '\u{304}']);
    }
    #[test]
    fn half_compose() {
        assert_eq!(
            compose_vec(vec!['α', '\u{304}', '\u{313}']),
            vec!['ᾱ', '\u{313}']
        );
    }
    #[test]
    fn many() {
        assert_eq!(
            compose_vec(vec!['α', '\u{304}', 'ι', '\u{304}', 'υ', '\u{304}']),
            vec!['ᾱ', 'ῑ', 'ῡ']
        );
    }
    #[test]
    fn double() {
        assert_eq!(
            compose_vec(vec!['α', '\u{313}', '\u{301}', 'ρ', 'χ', 'ω']),
            vec!['ἄ', 'ρ', 'χ', 'ω']
        );
    }
    #[test]
    fn iota() {
        assert_eq!(compose_vec(vec!['α', '\u{345}']), vec!['ᾳ'])
    }
    #[test]
    fn rough() {
        assert_eq!(compose_vec(vec!['α', '\u{314}']), vec!['ἁ'])
    }
    #[test]
    fn diaresis() {
        assert_eq!(
            compose_vec(vec!['υ', '\u{304}', '\u{308}']),
            vec!['ῡ', '\u{308}']
        );
    }
}
