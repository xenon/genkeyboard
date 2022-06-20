use std::collections::{HashMap, VecDeque};

use crate::{
    compose::{compose_vec, CompositionMap},
    kbdwriter::{KbdMap, KbdMetaData, KbdWriter},
};

pub fn gen(keyboard: &mut KbdWriter) {
    fn gen_vowels(
        map: &mut KbdMap,
        modifiers: Vec<&str>,
        vowel_key_short: &Vec<char>,
        short_vowels: &Vec<char>,
        vowel_key_long: &Vec<char>,
        long_vowels: &Vec<char>,
        modifier_map: &HashMap<&str, (char, Option<&str>)>,
        compositions: &CompositionMap,
        short_or_long_selector: Option<(bool, bool)>,
        capitals: bool,
    ) {
        let mut local_short_vowels = short_vowels.clone();
        let mut local_vowel_key_short = vowel_key_short.clone();
        let mut local_long_vowels = long_vowels.clone();
        let mut local_vowel_key_long = vowel_key_long.clone();

        let (short, long) = short_or_long_selector.unwrap_or((true, true));
        let modifier_keys: VecDeque<char> = modifiers
            .iter()
            .map(|str| modifier_map.get(str).unwrap().0)
            .collect();
        let modifier_strs: VecDeque<&str> = modifiers
            .iter()
            .map(|str| modifier_map.get(str).unwrap().1)
            .flatten()
            .collect();
        if short {
            if capitals {
                for letter in local_vowel_key_short.iter_mut() {
                    *letter = letter.to_uppercase().to_string().chars().nth(0).unwrap();
                }
                for letter in local_short_vowels.iter_mut() {
                    *letter = letter.to_uppercase().to_string().chars().nth(0).unwrap();
                }
            }
            for (index, vowel) in local_short_vowels.iter().enumerate() {
                let mut modifier = modifier_keys.clone();
                modifier.push_front(local_vowel_key_short[index]);

                let mut output: Vec<char> = vec![*vowel];
                for modifier in modifier_strs.iter() {
                    output.push(compositions.above(modifier));
                }
                map.add(
                    modifier.iter().collect(),
                    compose_vec(output).iter().collect(),
                );
            }
        }
        if long {
            if capitals {
                for letter in local_vowel_key_long.iter_mut() {
                    *letter = letter.to_uppercase().to_string().chars().nth(0).unwrap();
                }
                for letter in local_long_vowels.iter_mut() {
                    *letter = letter.to_uppercase().to_string().chars().nth(0).unwrap();
                }
            }
            for (index, vowel) in local_long_vowels.iter().enumerate() {
                let mut modifier = modifier_keys.clone();
                modifier.push_front(modifier_map.get("macron").unwrap().0);
                modifier.push_front(local_vowel_key_long[index]);

                let mut output: Vec<char> = vec![*vowel];
                for modifier in modifier_strs.iter() {
                    output.push(compositions.above(modifier));
                }
                map.add(
                    modifier.iter().collect(),
                    compose_vec(output).iter().collect(),
                );
            }
        }
    }
    fn gen_vowels_both(
        map: &mut KbdMap,
        modifiers: Vec<&str>,
        vowel_key_short: &Vec<char>,
        short_vowels: &Vec<char>,
        vowel_key_long: &Vec<char>,
        long_vowels: &Vec<char>,
        modifier_map: &HashMap<&str, (char, Option<&str>)>,
        compositions: &CompositionMap,
        short_or_long_selector: Option<(bool, bool)>,
    ) {
        gen_vowels(
            map,
            modifiers.clone(),
            vowel_key_short,
            short_vowels,
            vowel_key_long,
            long_vowels,
            modifier_map,
            compositions,
            short_or_long_selector,
            false,
        );
        gen_vowels(
            map,
            modifiers,
            vowel_key_short,
            short_vowels,
            vowel_key_long,
            long_vowels,
            modifier_map,
            compositions,
            short_or_long_selector,
            true,
        );
    }

    fn gen_class(
        map: &mut KbdMap,
        base_modifiers: Vec<&str>,
        vowel_key: &Vec<char>,
        vowel_key_ambiguous: &Vec<char>,
        vowel_key_iotable_ambiguous: &Vec<char>,
        short_vowels: &Vec<char>,
        ambiguous_vowels: &Vec<char>,
        iotable_vowels_ambiguous: &Vec<char>,
        vowel_key_iotable_long: &Vec<char>,
        long_vowels: &Vec<char>,
        iotable_vowels_long: &Vec<char>,
        modifier_map: &HashMap<&str, (char, Option<&str>)>,
        compositions: &CompositionMap,
        iotable: bool,
        capitals: bool,
    ) {
        gen_vowels(
            map,
            base_modifiers.clone(),
            &vowel_key,
            &short_vowels,
            &vowel_key,
            &long_vowels,
            &modifier_map,
            &compositions,
            None,
            capitals,
        );
        if iotable && !capitals {
            gen_vowels(
                map,
                base_modifiers
                    .iter()
                    .chain(vec!["iota"].iter())
                    .cloned()
                    .collect(),
                &vowel_key_iotable_ambiguous,
                &iotable_vowels_ambiguous,
                &vowel_key_iotable_long,
                &iotable_vowels_long,
                &modifier_map,
                &compositions,
                None,
                capitals,
            );
        }
        gen_vowels(
            map,
            base_modifiers
                .iter()
                .chain(vec!["acute"].iter())
                .cloned()
                .collect(),
            &vowel_key,
            &short_vowels,
            &vowel_key,
            &long_vowels,
            &modifier_map,
            &compositions,
            None,
            capitals,
        );
        if iotable && !capitals {
            gen_vowels(
                map,
                base_modifiers
                    .iter()
                    .chain(vec!["acute", "iota"].iter())
                    .cloned()
                    .collect(),
                &vowel_key_iotable_ambiguous,
                &iotable_vowels_ambiguous,
                &vowel_key_iotable_long,
                &iotable_vowels_long,
                &modifier_map,
                &compositions,
                None,
                capitals,
            );
        }
        gen_vowels(
            map,
            base_modifiers
                .iter()
                .chain(vec!["grave"].iter())
                .cloned()
                .collect(),
            &vowel_key,
            &short_vowels,
            &vowel_key,
            &long_vowels,
            &modifier_map,
            &compositions,
            None,
            capitals,
        );
        if iotable && !capitals {
            gen_vowels(
                map,
                base_modifiers
                    .iter()
                    .chain(vec!["grave", "iota"].iter())
                    .cloned()
                    .collect(),
                &vowel_key_iotable_ambiguous,
                &iotable_vowels_ambiguous,
                &vowel_key_iotable_long,
                &iotable_vowels_long,
                &modifier_map,
                &compositions,
                None,
                capitals,
            );
        }
        gen_vowels(
            map,
            base_modifiers
                .iter()
                .chain(vec!["circumflex"].iter())
                .cloned()
                .collect(),
            &vowel_key_ambiguous,
            &ambiguous_vowels,
            &vowel_key,
            &long_vowels,
            &modifier_map,
            &compositions,
            None,
            capitals,
        );
        if iotable && !capitals {
            gen_vowels(
                map,
                base_modifiers
                    .iter()
                    .chain(vec!["circumflex", "iota"].iter())
                    .cloned()
                    .collect(),
                &vowel_key_iotable_ambiguous,
                &iotable_vowels_ambiguous,
                &vowel_key_iotable_long,
                &iotable_vowels_long,
                &modifier_map,
                &compositions,
                None,
                capitals,
            );
        }
    }

    fn gen_class_both(
        map: &mut KbdMap,
        base_modifiers: Vec<&str>,
        vowel_key: &Vec<char>,
        vowel_key_ambiguous: &Vec<char>,
        vowel_key_iotable_ambiguous: &Vec<char>,
        short_vowels: &Vec<char>,
        ambiguous_vowels: &Vec<char>,
        iotable_vowels_ambiguous: &Vec<char>,
        vowel_key_iotable_long: &Vec<char>,
        long_vowels: &Vec<char>,
        iotable_vowels_long: &Vec<char>,
        modifier_map: &HashMap<&str, (char, Option<&str>)>,
        compositions: &CompositionMap,
        iotable: bool,
    ) {
        gen_class(
            map,
            base_modifiers.clone(),
            vowel_key,
            vowel_key_ambiguous,
            vowel_key_iotable_ambiguous,
            short_vowels,
            ambiguous_vowels,
            iotable_vowels_ambiguous,
            vowel_key_iotable_long,
            long_vowels,
            iotable_vowels_long,
            modifier_map,
            compositions,
            iotable,
            false,
        );
        gen_class(
            map,
            base_modifiers,
            vowel_key,
            vowel_key_ambiguous,
            vowel_key_iotable_ambiguous,
            short_vowels,
            ambiguous_vowels,
            iotable_vowels_ambiguous,
            vowel_key_iotable_long,
            long_vowels,
            iotable_vowels_long,
            modifier_map,
            compositions,
            iotable,
            true,
        );
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
    let modifier_circumflex: char = '[';
    let modifier_smooth: char = ':';
    let modifier_rough: char = '\"';
    let modifier_iota: char = ']';
    let modifier_diaresis: char = '{';
    let modifier_breve: char = '}';
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
        ("circumflex", (modifier_circumflex, Some("periposmeni"))),
        ("smooth", (modifier_smooth, Some("comma"))),
        ("rough", (modifier_rough, Some("reversed comma"))),
        ("iota", (modifier_iota, Some("iota"))),
        ("diaresis", (modifier_diaresis, Some("diaresis"))),
        ("breve", (modifier_breve, Some("breve"))),
    ]);

    let consonants: HashMap<&str, char> = HashMap::from([
        ("b", 'β'),
        ("g", 'γ'),
        ("d", 'δ'),
        ("z", 'ζ'),
        ("th", 'θ'),
        ("k", 'κ'),
        ("l", 'λ'),
        ("m", 'μ'),
        ("n", 'ν'),
        ("ks", 'ξ'),
        ("p", 'π'),
        ("r", 'ρ'),
        ("s", 'σ'),
        ("t", 'τ'),
        ("ph", 'φ'),
        ("kh", 'χ'),
        ("ps", 'ψ'),
    ]);
    let alphabet_doubles: Vec<(String, char)> = vec![
        ("h".to_string(), 'η'), // non-phonetic
        ("c".to_string(), 'κ'),
        ("f".to_string(), 'φ'),
        ("ch".to_string(), 'χ'),
        ("x".to_string(), 'ξ'),
        ("v".to_string(), 'ω'), // non-phonetic
    ];
    let final_consonants: Vec<(char, char)> = vec![('σ', 'ς')];

    let vowel_key: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    let short_vowels: Vec<char> = vec!['α', 'ε', 'ι', 'ο', 'υ'];
    let long_vowels: Vec<char> = vec!['ᾱ', 'η', 'ῑ', 'ω', 'ῡ'];

    let vowel_key_ambiguous: Vec<char> = vec!['a', 'i', 'u'];
    let ambiguous_vowels: Vec<char> = vec!['α', 'ι', 'υ'];

    let vowel_key_iotable_long: Vec<char> = vec!['a', 'e', 'o'];
    let iotable_vowels_long: Vec<char> = vec!['ᾱ', 'η', 'ω'];

    let vowel_key_iotable_ambiguous: Vec<char> = vec!['a'];
    let iotable_vowels_ambiguous: Vec<char> = vec!['α'];

    let vowel_key_diaresis: Vec<char> = vec!['i', 'u'];
    let diaresis_vowels: Vec<char> = vec!['ι', 'υ'];
    let diaresis_long_vowels: Vec<char> = vec!['ῑ', 'ῡ'];

    let punctuation: Vec<(char, char)> = vec![('.', '·'), ('<', '«'), ('>', '»'), ('?', ';')];

    // basic alphabet
    let mut alphabet_map = KbdMap::new();
    for (sequence, letter) in consonants.iter() {
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
    keyboard.write_section("consonants".to_string(), alphabet_map);

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
    keyboard.write_section(
        "alphabet/consonant doubles".to_string(),
        alphabet_doubles_map,
    );

    // final consonants
    let mut final_consonants_map = KbdMap::new();
    for (base_letter, final_letter) in final_consonants {
        for (key_seq, letter) in consonants.iter() {
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

    // accents
    let mut accents_map: KbdMap = KbdMap::new();
    gen_class_both(
        &mut accents_map,
        vec![],
        &vowel_key,
        &vowel_key_ambiguous,
        &vowel_key_iotable_ambiguous,
        &short_vowels,
        &ambiguous_vowels,
        &iotable_vowels_ambiguous,
        &vowel_key_iotable_long,
        &long_vowels,
        &iotable_vowels_long,
        &modifier_map,
        &compositions,
        true,
    );

    keyboard.write_section("accents without breathings".to_string(), accents_map);

    let mut accents_nobreath_map: KbdMap = KbdMap::new();
    // breve
    gen_vowels_both(
        &mut accents_nobreath_map,
        vec!["breve"],
        &vowel_key_ambiguous,
        &ambiguous_vowels,
        &vowel_key_ambiguous,
        &ambiguous_vowels,
        &modifier_map,
        &compositions,
        Some((true, false)),
    );
    // diaresis
    gen_class_both(
        &mut accents_nobreath_map,
        vec!["diaresis"],
        &vowel_key_diaresis,
        &vowel_key_diaresis,
        &vowel_key_iotable_ambiguous,
        &diaresis_vowels,
        &diaresis_vowels,
        &iotable_vowels_ambiguous,
        &vowel_key_iotable_long,
        &diaresis_long_vowels,
        &iotable_vowels_long,
        &modifier_map,
        &compositions,
        false,
    );
    keyboard.write_section(
        "accents exclusively without breathings".to_string(),
        accents_nobreath_map,
    );

    // smooth breathing
    let mut smooth_breathing_map = KbdMap::new();
    gen_class_both(
        &mut smooth_breathing_map,
        vec!["smooth"],
        &vowel_key,
        &vowel_key_ambiguous,
        &vowel_key_iotable_ambiguous,
        &short_vowels,
        &ambiguous_vowels,
        &iotable_vowels_ambiguous,
        &vowel_key_iotable_long,
        &long_vowels,
        &iotable_vowels_long,
        &modifier_map,
        &compositions,
        true,
    );
    keyboard.write_section("smooth breathing".to_string(), smooth_breathing_map);

    // rough breathing
    let mut rough_breathing_map = KbdMap::new();
    gen_class_both(
        &mut rough_breathing_map,
        vec!["rough"],
        &vowel_key,
        &vowel_key_ambiguous,
        &vowel_key_iotable_ambiguous,
        &short_vowels,
        &ambiguous_vowels,
        &iotable_vowels_ambiguous,
        &vowel_key_iotable_long,
        &long_vowels,
        &iotable_vowels_long,
        &modifier_map,
        &compositions,
        true,
    );
    keyboard.write_section("rough breathing".to_string(), rough_breathing_map);
}
