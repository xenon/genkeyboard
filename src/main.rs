use kbdwriter::KbdWriter;

use crate::kbdwriter::{emacs::EmacsKbdWriter, vimfn::VimFnKbdWriter};

mod compose;
mod kbdlayout;
mod kbdwriter;
fn main() {
    let mut kbd = KbdWriter::new();
    kbdlayout::greek::gen(&mut kbd);

    //let emacs = EmacsKbdWriter::new(&kbd);
    //print!("{}", emacs);

    let vim = VimFnKbdWriter::new(&kbd);
    print!("{}", vim);
}
