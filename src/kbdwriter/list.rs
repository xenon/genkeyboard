use std::{cell::RefCell, fmt::Display};

use super::{KbdMetaData, KbdWriter};

pub struct ListKbdWriter<'a> {
    kbd: RefCell<&'a KbdWriter>,
}

impl<'a> ListKbdWriter<'a> {
    pub fn new(kbd: &'a KbdWriter) -> Self {
        Self {
            kbd: RefCell::new(kbd),
        }
    }
}

impl<'a> Display for ListKbdWriter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kbd = self.kbd.borrow();

        let metadata = match kbd.metadata.is_some() {
            true => kbd.metadata.as_ref().unwrap().clone(),
            false => KbdMetaData::default(),
        };
        // LANGUAGE DEFINE
        writeln!(f, "{} ({})", metadata.language, metadata.language_code)?;
        writeln!(f, "{}", metadata.description)?;
        writeln!(f, "----------------")?;

        // KEYMAP
        // header
        // sections
        for (section, keymap) in kbd.sections.iter() {
            writeln!(f, "  {}", section)?;
            for (seq, mapped) in keymap.iter() {
                writeln!(f, "    〈{}〉 → 〈{}〉", seq, mapped)?;
            }
        }
        Ok(())
    }
}
