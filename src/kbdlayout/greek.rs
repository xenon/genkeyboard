use std::collections::HashMap;

use crate::{
    compose::{compose_vec, CompositionMap},
    kbdwriter::{KbdMap, KbdMetaData, KbdWriter},
};

pub fn gen(keyboard: &mut KbdWriter) {
    keyboard.set_metadata(KbdMetaData::new(
        "Ancient Greek".to_string(),
        "grc".to_string(),
        "Ancient Greek with extensions for macron and accents simultaneously".to_string(),
    ));

    // modifier keys
    let modifier_special: char = 'q';
    let modifier_macron: char = 'w';
    let modifier_acute: char = ';';
    let modifier_grave: char = '\'';
    let modifier_circumflex: char = '-';
    let modifier_smooth: char = ':';
    let modifier_rough: char = '\"';
    let modifier_iota: char = '[';
    let modifier_diaresis: char = ']';
    let modifiers: Vec<char> = vec![
        modifier_special,
        modifier_macron,
        modifier_acute,
        modifier_grave,
        modifier_circumflex,
        modifier_smooth,
        modifier_rough,
        modifier_iota,
        modifier_diaresis,
    ];

    let alphabet: Vec<(&str, char)> = vec![
        ("a", 'α'),
        ("b", 'β'),
        ("g", 'γ'),
        ("d", 'δ'),
        ("e", 'ε'),
        ("z", 'ζ'),
        ("h", 'η'), // non-phonetic
        ("th", 'θ'),
        ("i", 'ι'),
        ("k", 'κ'),
        ("l", 'λ'),
        ("m", 'μ'),
        ("n", 'ν'),
        ("ks", 'ξ'),
        ("o", 'ο'),
        ("p", 'π'),
        ("r", 'ρ'),
        ("s", 'σ'),
        ("t", 'τ'),
        ("u", 'υ'),
        ("ph", 'φ'),
        ("kh", 'χ'),
        ("ps", 'ψ'),
        ("v", 'ω'), // non-phonetic
    ];
    let alphabet_doubles: Vec<(String, char)> = vec![
        ("c".to_string(), 'κ'),
        (vec![modifier_macron, 'e'].iter().collect(), 'η'),
        ("f".to_string(), 'φ'),
        ("ch".to_string(), 'χ'),
        ("x".to_string(), 'χ'),
        (vec![modifier_macron, 'o'].iter().collect(), 'ω'),
    ];
    let final_consonants: HashMap<&str, char> = HashMap::from([("σ", 'ς')]);
    let vowels: Vec<char> = vec!['α', 'ε', 'η', 'ι', 'ο', 'υ', 'ω'];
    let punctuation: Vec<(char, char)> = vec![('.', '·'), ('<', '«'), ('>', '»'), ('?', ';')];

    // basic alphabet
    let mut alphabet_map = KbdMap::new();
    for (sequence, letter) in alphabet {
        // lowercase
        alphabet_map.add(sequence.to_string(), letter.to_string());
        // capital
        let mut cap_seq = sequence.to_string().to_uppercase();
        let cap_letter = letter.to_uppercase().to_string();
        alphabet_map.add(cap_seq.clone(), cap_letter.clone());
        if cap_seq.len() > 1 {
            let last = sequence.chars().nth(1).unwrap();
            // if the 2nd character is not a modifier
            if !modifiers.contains(&last) {
                cap_seq.pop();
                cap_seq.push(last);
                alphabet_map.add(cap_seq.clone(), cap_letter);
            }
        }
    }
    keyboard.write_section("alphabet".to_string(), alphabet_map);

    // doubled alphabet
    let mut alphabet_doubles_map = KbdMap::new();
    for (sequence, letter) in alphabet_doubles {
        // lowercase
        alphabet_doubles_map.add(sequence.clone(), letter.to_string());
        // capital
        let mut cap_seq = sequence.to_uppercase();
        let cap_letter = letter.to_uppercase().to_string();
        alphabet_doubles_map.add(cap_seq.clone(), cap_letter.clone());
        if cap_seq.len() > 1 {
            let last = sequence.chars().nth(1).unwrap();
            // if the 2nd character is not a modifier
            if !modifiers.contains(&last) {
                cap_seq.pop();
                cap_seq.push(last);
                alphabet_doubles_map.add(cap_seq.clone(), cap_letter);
            }
        }
    }
    keyboard.write_section("alphabet doubles".to_string(), alphabet_doubles_map);

    let mut punctuation_map = KbdMap::new();
    for (punct_letter, letter) in punctuation {
        punctuation_map.add(
            vec![modifier_special, punct_letter].iter().collect(),
            letter.to_string(),
        );
    }
    keyboard.write_section("punctuation".to_string(), punctuation_map);
}
