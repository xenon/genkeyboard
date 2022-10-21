use std::{cell::RefCell, fmt::Display};

use convert_case::{Case, Casing};

use super::{KbdMetaData, KbdWriter};

pub struct EmacsKbdWriter<'a> {
    kbd: RefCell<&'a KbdWriter>,
}

impl<'a> EmacsKbdWriter<'a> {
    pub fn new(kbd: &'a KbdWriter) -> Self {
        Self {
            kbd: RefCell::new(kbd),
        }
    }
}

impl<'a> Display for EmacsKbdWriter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kbd = self.kbd.borrow();

        let metadata = match kbd.metadata.is_some() {
            true => kbd.metadata.as_ref().unwrap().clone(),
            false => KbdMetaData::default(),
        };
        // LANGUAGE DEFINE
        writeln!(f, "(quail-define-package")?;
        writeln!(f, "  \"{}\"", metadata.language.to_case(Case::Kebab))?;
        writeln!(f, "  \"{}\"", metadata.language_code.to_case(Case::Lower))?;
        writeln!(f, "  \"{}\"", metadata.language)?;
        writeln!(f, "  t")?;
        writeln!(f, "  \"{}\"", metadata.description)?;
        writeln!(f, "  nil t nil nil nil nil nil nil nil nil t")?;
        writeln!(f, ")")?;
        // KEYMAP
        // header
        writeln!(f, "(quail-define-rules")?;
        // sections
        for (section, keymap) in kbd.sections.iter() {
            writeln!(f, "  ;; {}", section)?;
            for (seq, mapped) in keymap.iter() {
                if mapped.chars().count() != 1 {
                    writeln!(f, "  (\"{}\", ?\"{}\")", seq.replace('\"', "\\\""), mapped)?;
                } else {
                    writeln!(f, "  (\"{}\", ?{})", seq.replace('\"', "\\\""), mapped)?;
                }
            }
        }
        // footer
        writeln!(f, ")")?;
        Ok(())
    }
}
