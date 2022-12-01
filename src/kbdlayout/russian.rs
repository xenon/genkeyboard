use std::collections::HashMap;

use crate::{
    compose::{compose_vec, CompositionMap},
    kbdwriter::{KbdMap, KbdMetaData, KbdWriter},
};

pub fn gen(keyboard: &mut KbdWriter) {
    let compositions = CompositionMap::new();
    keyboard.set_metadata(KbdMetaData::new(
        "Russian Cyrillic".to_string(),
        "rus".to_string(),
        "Russian phonetic layout".to_string(),
    ));

    // modifier keys
    let modifier_sign: char = 'q';
    let modifier_soft: char = 'j';
    let modifier_acute: char = ';';

    let consonants: HashMap<&str, char> = HashMap::from([
        ("b", 'б'),
        ("v", 'в'),
        ("g", 'г'),
        ("d", 'д'),
        ("zh", 'ж'),
        ("z", 'з'),
        ("j", 'й'),
        ("k", 'к'),
        ("l", 'л'),
        ("m", 'м'),
        ("n", 'н'),
        ("p", 'п'),
        ("r", 'р'),
        ("s", 'с'),
        ("t", 'т'),
        ("f", 'ф'),
        ("h", 'х'),
        ("ts", 'ц'),
        ("ch", 'ч'),
        ("sh", 'ш'),
        ("sch", 'щ'),
    ]);

    let i_vowel_key: Vec<char> = vec!['i', 'y'];
    let i_vowels: Vec<char> = vec!['и', 'ы'];

    let vowel_key: Vec<char> = vec!['a', 'e', 'o', 'u'];
    let hard_vowels: Vec<char> = vec!['а', 'э', 'о', 'у'];
    let soft_vowels: Vec<char> = vec!['я', 'е', 'ё', 'ю'];

    let sign_key: Vec<char> = vec!['s', 'h'];
    let signs: Vec<char> = vec!['ь', 'ъ'];

    let mut consonant_map = KbdMap::new();
    for (sequence, letter) in consonants.iter() {
        consonant_map.add(sequence.to_string(), letter.to_string());
        consonant_map.add(
            sequence.to_string().to_uppercase(),
            letter.to_uppercase().to_string(),
        );
    }
    keyboard.write_section("consonants".to_string(), consonant_map);

    let mut hard_vowel_map = KbdMap::new();
    for (vowel, mapped) in vowel_key
        .iter()
        .zip(hard_vowels.iter())
        .chain(i_vowel_key.iter().zip(i_vowels.iter()))
    {
        hard_vowel_map.add(vowel.to_string(), mapped.to_string());
        hard_vowel_map.add(
            vowel.to_string().to_uppercase(),
            mapped.to_uppercase().to_string(),
        );
    }
    keyboard.write_section("vowels".to_string(), hard_vowel_map);

    let mut soft_vowel_map = KbdMap::new();
    for (vowel, mapped) in vowel_key.iter().zip(soft_vowels.iter()) {
        let mut seq = modifier_soft.to_string();
        seq.push(*vowel);
        soft_vowel_map.add(seq.clone(), mapped.to_string());
        seq.pop();
        seq = seq.to_uppercase();
        seq.push_str(&vowel.to_uppercase().to_string());
        soft_vowel_map.add(seq.clone(), mapped.to_uppercase().to_string());
        seq.pop();
        seq.push(*vowel);
        soft_vowel_map.add(seq, mapped.to_uppercase().to_string());
    }
    keyboard.write_section("soft vowels".to_string(), soft_vowel_map);

    let mut acute_hard_vowel_map = KbdMap::new();
    for (vowel, mapped) in vowel_key
        .iter()
        .zip(hard_vowels.iter())
        .chain(i_vowel_key.iter().zip(i_vowels.iter()))
    {
        acute_hard_vowel_map.add(
            [*vowel, modifier_acute].iter().collect(),
            compose_vec(vec![*mapped, compositions.above("acute")])
                .iter()
                .collect(),
        );
        let in_str: Vec<char> = vowel.to_uppercase().collect();
        let map_str: Vec<char> = mapped.to_uppercase().collect();
        acute_hard_vowel_map.add(
            [in_str[0], modifier_acute].iter().collect(),
            compose_vec(vec![map_str[0], compositions.above("acute")])
                .iter()
                .collect(),
        );
    }
    keyboard.write_section("acute hard vowels".to_string(), acute_hard_vowel_map);

    let mut acute_soft_vowel_map = KbdMap::new();
    for (vowel, mapped) in vowel_key.iter().zip(soft_vowels.iter()) {
        acute_soft_vowel_map.add(
            [modifier_soft, *vowel, modifier_acute].iter().collect(),
            compose_vec(vec![*mapped, compositions.above("acute")])
                .iter()
                .collect(),
        );
        let mod_str: Vec<char> = modifier_soft.to_uppercase().collect();
        let in_str: Vec<char> = vowel.to_uppercase().collect();
        let map_str: Vec<char> = mapped.to_uppercase().collect();

        acute_soft_vowel_map.add(
            [mod_str[0], in_str[0], modifier_acute].iter().collect(),
            compose_vec(vec![map_str[0], compositions.above("acute")])
                .iter()
                .collect(),
        );
        acute_soft_vowel_map.add(
            [mod_str[0], *vowel, modifier_acute].iter().collect(),
            compose_vec(vec![map_str[0], compositions.above("acute")])
                .iter()
                .collect(),
        );
    }
    keyboard.write_section("acute soft vowels".to_string(), acute_soft_vowel_map);

    let mut sign_map = KbdMap::new();
    for (key, sign) in sign_key.iter().zip(signs.iter()) {
        let mut seq = modifier_sign.to_string();
        seq.push(*key);
        sign_map.add(seq.clone(), sign.to_string());
        seq.pop();
        seq.push_str(&key.to_uppercase().to_string());
        sign_map.add(seq, sign.to_string());
    }
    keyboard.write_section("signs".to_string(), sign_map);
}
