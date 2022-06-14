use std::{cell::RefCell, fmt::Display};

use convert_case::{Case, Casing};

use super::{KbdMetaData, KbdWriter};

pub struct VimFnKbdWriter<'a> {
    kbd: RefCell<&'a KbdWriter>,
}

impl<'a> VimFnKbdWriter<'a> {
    pub fn new(kbd: &'a KbdWriter) -> Self {
        Self {
            kbd: RefCell::new(kbd),
        }
    }
}

impl<'a> Display for VimFnKbdWriter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kbd = self.kbd.borrow();

        let metadata = match kbd.metadata.is_some() {
            true => kbd.metadata.as_ref().unwrap().clone(),
            false => KbdMetaData::default(),
        };
        // Comment
        writeln!(f, "\" {} ({})", metadata.language, metadata.language_code)?;
        writeln!(f, "\" {}", metadata.description)?;

        // KEYMAP
        // function decl
        writeln!(
            f,
            "function kbdmap_{}()",
            metadata.language.to_case(Case::Snake)
        )?;
        // sections
        for (section, keymap) in kbd.sections.iter() {
            writeln!(f, "   \" {}", section)?;
            for (seq, mapped) in keymap.iter() {
                writeln!(f, "   imap {} {}", seq, mapped)?;
            }
        }
        // footer
        writeln!(f, "endfunction")?;
        Ok(())
    }
}
