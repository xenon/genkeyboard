use std::collections::{btree_map::Iter, BTreeMap};

pub mod emacs;
pub mod list;
pub mod vimfn;

#[derive(Clone)]
pub struct KbdMetaData {
    pub language: String,
    pub language_code: String,
    pub description: String,
}

impl KbdMetaData {
    pub fn new(language: String, language_code: String, description: String) -> Self {
        Self {
            language,
            language_code,
            description,
        }
    }
}

impl Default for KbdMetaData {
    fn default() -> Self {
        Self {
            language: "ERROR: Unknown".to_string(),
            language_code: "???".to_string(),
            description: "ERROR: Unknown".to_string(),
        }
    }
}

pub struct KbdWriter {
    metadata: Option<KbdMetaData>,
    sections: Vec<(String, KbdMap)>,
}

impl KbdWriter {
    pub fn new() -> Self {
        Self {
            metadata: None,
            sections: Vec::new(),
        }
    }
    pub fn set_metadata(&mut self, metadata: KbdMetaData) {
        self.metadata = Some(metadata);
    }
    pub fn write_section(&mut self, title: String, keymap: KbdMap) {
        self.sections.push((title, keymap));
    }
    pub fn write_sections(&mut self, sections: Vec<(String, KbdMap)>) {
        for (title, keymap) in sections {
            self.write_section(title, keymap);
        }
    }
}

pub struct KbdMap {
    keymap: BTreeMap<String, String>,
}

impl KbdMap {
    pub fn new() -> Self {
        Self {
            keymap: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, key_sequence: String, mapped_value: String) -> bool {
        if !self.keymap.contains_key(&key_sequence) {
            self.keymap.insert(key_sequence, mapped_value);
            return true;
        } else {
            eprintln!(
                "Duplicated key sequence: {} for value {}",
                key_sequence, mapped_value
            );
        }
        return false;
    }

    pub fn iter(&self) -> Iter<'_, String, String> {
        self.keymap.iter()
    }
}
