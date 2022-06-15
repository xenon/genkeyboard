use std::fs;

use kbdwriter::KbdWriter;

use crate::kbdwriter::{emacs::EmacsKbdWriter, list::ListKbdWriter, vimfn::VimFnKbdWriter};

mod compose;
mod kbdlayout;
mod kbdwriter;
fn main() {
    let mut kbd = KbdWriter::new();
    kbdlayout::greek::gen(&mut kbd);

    //let emacs = EmacsKbdWriter::new(&kbd);
    //print!("{}", emacs);

    let list = ListKbdWriter::new(&kbd);
    print!("{}", list);
    fs::write("kbd.txt", format!("{}", list)).expect("Unable to write file");
}
