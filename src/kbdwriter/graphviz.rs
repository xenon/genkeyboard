use std::{cell::RefCell, fmt::Display};

use crate::automaton::{Automaton, AutomatonStyle};

use super::KbdWriter;

pub struct GraphVizKbdWriter<'a> {
    kbd: RefCell<&'a KbdWriter>,
}

impl<'a> GraphVizKbdWriter<'a> {
    pub fn new(kbd: &'a KbdWriter) -> Self {
        Self {
            kbd: RefCell::new(kbd),
        }
    }
}

impl<'a> Display for GraphVizKbdWriter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kbd = self.kbd.borrow();
        let mut output = String::new();

        // Generate automaton and use it to generate graph
        let (automaton, ranges) = Automaton::from_writer(&kbd);
        let style = AutomatonStyle::default();
        automaton.write(&mut output, ranges, *kbd, style)?;
        write!(f, "{}", output)
    }
}
