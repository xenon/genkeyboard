use std::{fmt::Write, io::BufRead, process::exit};

use automaton::Automaton;
use clap::Parser;
use convert_case::{Case, Casing};

use kbdlayout::Layout;
use kbdwriter::{graphviz::GraphVizKbdWriter, Format, KbdWriter};

use crate::kbdwriter::{emacs::EmacsKbdWriter, list::ListKbdWriter, vimfn::VimFnKbdWriter};

mod automaton;
mod compose;
mod kbdlayout;
mod kbdwriter;

#[derive(Parser, Debug)]
enum Command {
    ListLayouts,
    ListFormats,
    ListSubgraphs(ListSubgraphArgs),
    Generate(GenArgs),
    Automaton(AutomatonArgs),
}

#[derive(Parser, Default, Debug)]
struct ListSubgraphArgs {
    #[clap(short, long, arg_enum, required = true)]
    layout: Option<Layout>,
}
#[derive(Parser, Default, Debug)]
struct GenArgs {
    #[clap(short, long, arg_enum, required = true)]
    layout: Option<Layout>,
    #[clap(short, long, arg_enum)]
    format: Format,
    #[clap(short, long)]
    subgraph: Option<String>,
    #[clap(short, long, value_hint = clap::ValueHint::FilePath)]
    output_file: Option<std::path::PathBuf>,
}

#[derive(Parser, Debug, Default)]
enum AutomatonCommand {
    #[default]
    Repl,
    Test {
        test: String,
    },
    CodeGen,
}

#[derive(Parser, Default, Debug)]
struct AutomatonArgs {
    #[clap(subcommand)]
    command: AutomatonCommand,
    #[clap(short, long, arg_enum, required = true)]
    layout: Option<Layout>,
    #[clap(short, long)]
    subgraph: Option<String>,
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
        Command::ListSubgraphs(args) => {
            let mut kbd = KbdWriter::new();
            match args.layout.unwrap() {
                Layout::Greek => {
                    kbdlayout::greek::gen(&mut kbd);
                }
                Layout::Latin => {
                    kbdlayout::latin::gen(&mut kbd, true);
                }
            }
            for (name, _) in kbd.sections.iter() {
                println!("{}", name);
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
            // subgraph
            if let Some(section) = args.subgraph {
                if let Some(sub_writer) = kbd.subsection_writer(&section) {
                    eprintln!("Found subgraph: '{}'", section);
                    kbd = sub_writer;
                } else {
                    eprintln!("Invalid subgraph.");
                    exit(-1);
                }
            }
            // write output
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
                Format::GraphViz => {
                    let map = GraphVizKbdWriter::new(&kbd);
                    output.write_fmt(format_args!("{}", map)).expect("");
                }
            }
            if let Some(ofile) = args.output_file {
                std::fs::write(ofile, &output).expect("Unable to write file");
            } else {
                println!("{}", output);
            }
        }
        Command::Automaton(args) => {
            let mut kbd = KbdWriter::new();
            match args.layout.unwrap() {
                Layout::Greek => {
                    kbdlayout::greek::gen(&mut kbd);
                }
                Layout::Latin => {
                    kbdlayout::latin::gen(&mut kbd, true);
                }
            }
            if let Some(section) = args.subgraph {
                if let Some(sub_writer) = kbd.subsection_writer(&section) {
                    eprintln!("Found subgraph: '{}'", section);
                    kbd = sub_writer;
                } else {
                    eprintln!("Invalid subgraph.");
                    exit(-1);
                }
            }

            let automaton = Automaton::from_writer(&kbd).0;

            // test or enter repl
            match args.command {
                AutomatonCommand::Repl => {
                    let stdin = std::io::stdin();
                    for line in stdin.lock().lines() {
                        match line {
                            Ok(l) => {
                                if let Some(mapped) = automaton.run(&l) {
                                    println!("Found match: {}", mapped);
                                } else {
                                    println!("No match!");
                                }
                            }
                            Err(e) => println!("Line Error?: {}", e),
                        }
                    }
                }
                AutomatonCommand::Test { test } => {
                    if let Some(mapped) = automaton.run(&test) {
                        println!("Found match: {}", mapped);
                    } else {
                        println!("No match!");
                    }
                }
                AutomatonCommand::CodeGen => {
                    let mut output = String::new();
                    automaton
                        .json_codegen(&mut output)
                        .expect("Codegen shouldn't fail");
                    println!("{}", output);
                }
            }
        }
    }
}
