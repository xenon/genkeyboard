use std::{
    collections::{
        btree_map::{Entry, Iter},
        BTreeMap,
    },
    fmt,
};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use variant_count::VariantCount;

pub mod emacs;
pub mod graphviz;
pub mod list;
pub mod vimfn;

#[derive(
    clap::ArgEnum,
    Clone,
    Debug,
    enum_utils::FromStr,
    Eq,
    IntoPrimitive,
    PartialEq,
    TryFromPrimitive,
    VariantCount,
)]
#[repr(u8)]
pub enum Format {
    List,
    EmacsQuail,
    VimFn,
    GraphViz,
}

impl Default for Format {
    fn default() -> Self {
        Format::List
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
    pub(crate) sections: Vec<(String, KbdMap)>,
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

    pub fn subsection_writer(&self, section: &String) -> Option<KbdWriter> {
        for (name, kbdmap) in self.sections.iter() {
            if name == section {
                let mut new_writer = KbdWriter::new();
                new_writer.write_section(name.clone(), kbdmap.clone());
                if let Some(metadata) = &self.metadata {
                    new_writer.set_metadata(metadata.clone());
                }
                return Some(new_writer);
            }
        }
        None
    }
}

#[derive(Clone)]
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
        if let Entry::Vacant(e) = self.keymap.entry(key_sequence.clone()) {
            e.insert(mapped_value);
            true
        } else {
            eprintln!(
                "Duplicated key sequence: {} for value {}",
                key_sequence, mapped_value
            );
            false
        }
    }

    pub fn iter(&self) -> Iter<'_, String, String> {
        self.keymap.iter()
    }
}
