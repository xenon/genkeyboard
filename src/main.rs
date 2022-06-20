use std::fmt::Write;

use clap::Parser;
use convert_case::{Case, Casing};

use kbdlayout::Layout;
use kbdwriter::{Format, KbdWriter};

use crate::kbdwriter::{emacs::EmacsKbdWriter, list::ListKbdWriter, vimfn::VimFnKbdWriter};

mod compose;
mod kbdlayout;
mod kbdwriter;

#[derive(Parser, Debug)]
enum Command {
    ListLayouts,
    ListFormats,
    Generate(GenArgs),
}
#[derive(Parser, Default, Debug)]
struct GenArgs {
    #[clap(short, long, arg_enum, required = true)]
    layout: Option<Layout>,
    #[clap(short, long, arg_enum)]
    format: Format,
    #[clap(short, long, value_hint = clap::ValueHint::FilePath)]
    output_file: Option<std::path::PathBuf>,
}

fn main() {
    match Command::parse() {
        Command::ListLayouts => {
            for layout in Layout::Greek as u8..Layout::VARIANT_COUNT as u8 {
                println!(
                    "{}",
                    Layout::try_from(layout)
                        .unwrap()
                        .to_string()
                        .to_case(Case::Kebab)
                );
            }
        }
        Command::ListFormats => {
            for format in Format::List as u8..Format::VARIANT_COUNT as u8 {
                println!(
                    "{}",
                    Format::try_from(format)
                        .unwrap()
                        .to_string()
                        .to_case(Case::Kebab)
                );
            }
        }
        Command::Generate(args) => {
            let mut kbd = KbdWriter::new();
            match args.layout.unwrap() {
                Layout::Greek => {
                    kbdlayout::greek::gen(&mut kbd);
                }
                Layout::Latin => {
                    kbdlayout::latin::gen(&mut kbd, true);
                }
            }
            let mut output = String::new();
            match args.format {
                Format::List => {
                    let map = ListKbdWriter::new(&kbd);
                    output.write_fmt(format_args!("{}", map)).expect("");
                }
                Format::EmacsQuail => {
                    let map = EmacsKbdWriter::new(&kbd);
                    output.write_fmt(format_args!("{}", map)).expect("");
                }
                Format::VimFn => {
                    let map = VimFnKbdWriter::new(&kbd);
                    output.write_fmt(format_args!("{}", map)).expect("");
                }
            }
            if let Some(ofile) = args.output_file {
                std::fs::write(ofile, format!("{}", output)).expect("Unable to write file");
            } else {
                println!("{}", output);
            }
        }
    }
}
