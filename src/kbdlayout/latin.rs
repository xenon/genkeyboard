use crate::{
    compose::{compose_vec, CompositionMap},
    kbdwriter::{KbdMap, KbdMetaData, KbdWriter},
};

pub fn gen(keyboard: &mut KbdWriter, postfix: bool) {
    let compositions = CompositionMap::new();
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'y'];

    keyboard.set_metadata(KbdMetaData::new(
        "Lingua Latina".to_string(),
        "la".to_string(),
        "Latin with macrons and breve".to_string(),
    ));

    let mut macron_map = KbdMap::new();
    let macron_combine = ';';
    for letter_lower in vowels.iter() {
        let lowercase = match postfix {
            true => vec![*letter_lower, macron_combine],
            false => vec![macron_combine, *letter_lower],
        }
        .iter()
        .collect();
        macron_map.add(
            lowercase,
            compose_vec(vec![*letter_lower, compositions.above("macron")])
                .iter()
                .collect(),
        );
        let u_str: Vec<char> = letter_lower.to_uppercase().collect();
        let letter_upper = u_str[0];
        let uppercase = match postfix {
            true => vec![letter_upper, macron_combine],
            false => vec![macron_combine, letter_upper],
        }
        .iter()
        .collect();
        macron_map.add(
            uppercase,
            compose_vec(vec![letter_upper, compositions.above("macron")])
                .iter()
                .collect(),
        );
    }
    keyboard.write_section("macrons".to_string(), macron_map);

    let mut breve_map = KbdMap::new();
    let breve_combine = '-';
    for letter_lower in vowels.iter() {
        let lowercase = match postfix {
            true => vec![*letter_lower, breve_combine],
            false => vec![breve_combine, *letter_lower],
        }
        .iter()
        .collect();
        breve_map.add(
            lowercase,
            compose_vec(vec![*letter_lower, compositions.above("breve")])
                .iter()
                .collect(),
        );
        let u_str: Vec<char> = letter_lower.to_uppercase().collect();
        let letter_upper = u_str[0];
        let uppercase = match postfix {
            true => vec![letter_upper, breve_combine],
            false => vec![breve_combine, letter_upper],
        }
        .iter()
        .collect();
        breve_map.add(
            uppercase,
            compose_vec(vec![letter_upper, compositions.above("breve")])
                .iter()
                .collect(),
        );
    }
    keyboard.write_section("breve".to_string(), breve_map);
}
