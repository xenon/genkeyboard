use std::collections::{HashMap, VecDeque};

use crate::{
    compose::{compose_vec, CompositionMap},
    kbdwriter::{KbdMap, KbdMetaData, KbdWriter},
};

pub fn gen(keyboard: &mut KbdWriter) {
    fn gen_vowels(
        map: &mut KbdMap,
        modifiers: Vec<&str>,
        vowel_key: &Vec<char>,
        short_vowels: &Vec<char>,
        long_vowels: &Vec<char>,
        modifier_map: &HashMap<&str, (char, Option<&str>)>,
        compositions: &CompositionMap,
    ) {
        let modifier_keys: VecDeque<char> = modifiers
            .iter()
            .map(|str| modifier_map.get(str).unwrap().0)
            .collect();
        let modifier_strs: VecDeque<&str> = modifiers
            .iter()
            .map(|str| modifier_map.get(str).unwrap().1)
            .flatten()
            .collect();
        for (index, vowel) in short_vowels.iter().enumerate() {
            let mut modifier = modifier_keys.clone();
            modifier.push_front(vowel_key[index]);

            let mut output: Vec<char> = vec![*vowel];
            for modifier in modifier_strs.iter() {
                output.push(compositions.above(modifier));
            }
            map.add(
                modifier.iter().collect(),
                compose_vec(output).iter().collect(),
            );
        }
        for (index, vowel) in long_vowels.iter().enumerate() {
            let mut modifier = modifier_keys.clone();
            modifier.push_front(modifier_map.get("macron").unwrap().0);
            modifier.push_front(vowel_key[index]);

            let mut output: Vec<char> = vec![*vowel];
            for modifier in modifier_strs.iter() {
                println!(
                    "modifier: {}, unicode: {}",
                    modifier.clone().to_string(),
                    compositions.above(modifier)
                );
                output.push(compositions.above(modifier));
            }
            map.add(
                modifier.iter().collect(),
                compose_vec(output).iter().collect(),
            );
        }
    }
    let compositions = CompositionMap::new();

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
    let modifier_breve: char = '-';
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
        modifier_breve,
    ];
    let modifier_map: HashMap<&str, (char, Option<&str>)> = HashMap::from([
        ("special", (modifier_special, None)),
        ("macron", (modifier_macron, Some("macron"))),
        ("acute", (modifier_acute, Some("acute"))),
        ("grave", (modifier_grave, Some("grave"))),
        ("circumflex", (modifier_circumflex, Some("circumflex"))),
        ("smooth", (modifier_smooth, Some("comma"))),
        ("rough", (modifier_rough, Some("reversed comma"))),
        ("iota", (modifier_iota, Some("iota"))),
        ("diaresis", (modifier_diaresis, Some("diaresis"))),
        ("breve", (modifier_breve, Some("breve"))),
    ]);

    let alphabet: HashMap<&str, char> = HashMap::from([
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
    ]);
    let alphabet_doubles: Vec<(String, char)> = vec![
        ("c".to_string(), 'κ'),
        (vec!['e', modifier_macron].iter().collect(), 'η'),
        ("f".to_string(), 'φ'),
        ("ch".to_string(), 'χ'),
        ("x".to_string(), 'ξ'),
        (vec!['o', modifier_macron].iter().collect(), 'ω'),
    ];
    let final_consonants: Vec<(char, char)> = vec![('σ', 'ς')];
    let all_vowels: Vec<char> = vec!['α', 'ε', 'η', 'ι', 'ο', 'υ', 'ω'];

    let vowel_key: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    let short_vowels: Vec<char> = vec!['α', 'ε', 'ι', 'ο', 'υ'];
    let long_vowels: Vec<char> = vec!['ᾱ', 'η', 'ῑ', 'ω', 'ῡ'];

    let punctuation: Vec<(char, char)> = vec![('.', '·'), ('<', '«'), ('>', '»'), ('?', ';')];

    // basic alphabet
    let mut alphabet_map = KbdMap::new();
    for (sequence, letter) in alphabet.iter() {
        // lowercase
        alphabet_map.add(sequence.to_string(), letter.to_string());
        // capital
        let mut cap_seq = sequence.to_string().to_uppercase();
        let cap_letter = letter.to_uppercase().to_string();
        if cap_seq.len() > 1 {
            let last = sequence.chars().nth(1).unwrap();
            //println!("long: {} -> {}, last char: {}", sequence, letter, last);
            // if the 2nd character is not a modifier
            if modifiers.contains(&last) {
                //println!("  modifiers contains {}", last);
                cap_seq.pop();
                cap_seq.push(last);
                alphabet_map.add(cap_seq, cap_letter);
            } else {
                alphabet_map.add(cap_seq, cap_letter);
            }
        } else {
            alphabet_map.add(cap_seq, cap_letter);
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
        if cap_seq.len() > 1 {
            let last = sequence.chars().nth(1).unwrap();
            //println!("long: {} -> {}, last char: {}", sequence, letter, last);
            // if the 2nd character is not a modifier
            if modifiers.contains(&last) {
                //println!("  modifiers contains {}", last);
                cap_seq.pop();
                cap_seq.push(last);
                alphabet_doubles_map.add(cap_seq, cap_letter);
            } else {
                alphabet_doubles_map.add(cap_seq, cap_letter);
            }
        } else {
            alphabet_doubles_map.add(cap_seq, cap_letter);
        }
    }
    keyboard.write_section("alphabet doubles".to_string(), alphabet_doubles_map);

    // final consonants
    let mut final_consonants_map = KbdMap::new();
    for (base_letter, final_letter) in final_consonants {
        for (key_seq, letter) in alphabet.iter() {
            if base_letter == *letter {
                let mut new_seq: String = (*key_seq).to_string();
                new_seq.push(' ');
                final_consonants_map.add(new_seq, final_letter.to_string());
            }
        }
    }
    keyboard.write_section("final consonants".to_string(), final_consonants_map);

    // punctuation
    let mut punctuation_map = KbdMap::new();
    for (punct_letter, letter) in punctuation {
        punctuation_map.add(
            vec![modifier_special, punct_letter].iter().collect(),
            letter.to_string(),
        );
    }
    keyboard.write_section("punctuation".to_string(), punctuation_map);

    // smooth breathing
    let mut smooth_breathing_map = KbdMap::new();
    gen_vowels(
        &mut smooth_breathing_map,
        vec!["smooth"],
        &vowel_key,
        &short_vowels,
        &long_vowels,
        &modifier_map,
        &compositions,
    );
    keyboard.write_section("smooth breathing".to_string(), smooth_breathing_map);

    // rough breathing
    let mut rough_breathing_map = KbdMap::new();
    gen_vowels(
        &mut rough_breathing_map,
        vec!["rough"],
        &vowel_key,
        &short_vowels,
        &long_vowels,
        &modifier_map,
        &compositions,
    );
    keyboard.write_section("rough breathing".to_string(), rough_breathing_map);
}
