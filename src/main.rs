use std::{ffi::OsString, path::PathBuf};

use crate::codegen::Language;
use crate::compiler::Compiler;
use crate::new_lib::Target;

mod codegen;
mod compiler;
mod error;
mod new_lib;
mod parser;

fn main() {
    let cmd = clap::Command::new("binserde")
        .version(clap::crate_version!())
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("new-lib")
                .about("Compile Binserde source files")
                .arg(
                    clap::Arg::new("name")
                        .short('n')
                        .required(false)
                        .num_args(1)
                        .value_parser(clap::builder::ValueParser::string())
                        .default_value("types")
                        .help("lib name"),
                )
                .arg(
                    clap::Arg::new("target")
                        .short('t')
                        .required(false)
                        .num_args(1)
                        .value_parser(["rs", "ts"])
                        .default_value("rs")
                        .help("Target to build for [possible values: rs, ts]"),
                )
                .arg(
                    clap::Arg::new("output")
                        .short('o')
                        .required(true)
                        .num_args(1)
                        .value_parser(clap::builder::ValueParser::os_string())
                        .help("output directory"),
                ),
        )
        .subcommand(
            clap::Command::new("compile")
                .about("Compile Binserde source files")
                .arg(
                    clap::Arg::new("input")
                        .short('i')
                        .required(true)
                        .num_args(1..)
                        .value_parser(clap::builder::ValueParser::os_string())
                        .help("input files"),
                )
                .arg(
                    clap::Arg::new("output")
                        .short('o')
                        .required(false)
                        .num_args(1)
                        .value_parser(clap::builder::ValueParser::os_string())
                        .default_value("stdout")
                        .help("output directory"),
                )
                .arg(
                    clap::Arg::new("target")
                        .short('t')
                        .required(false)
                        .num_args(1)
                        .value_parser(["rs", "ts"])
                        .default_value("rs")
                        .help("target to build"),
                ),
        );

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("new-lib", matches)) => new_lib(matches),
        Some(("compile", matches)) => compile(matches),
        _ => unreachable!(),
    }
}

fn new_lib(matches: &clap::ArgMatches) {
    let target = match matches.get_one::<String>("target").unwrap().as_str() {
        "rs" => Target::Rust,
        "ts" => Target::TypeScript,
        _ => unimplemented!(),
    };

    let name = matches.get_one::<String>("name").unwrap().as_str();

    let output = matches.get_one::<OsString>("output").unwrap();
    let out_dir = PathBuf::from(output).canonicalize().unwrap();

    target.generate(name, out_dir).unwrap();
}

fn compile(matches: &clap::ArgMatches) {
    let target = match matches.get_one::<String>("target").unwrap().as_str() {
        "rs" => Language::Rust,
        "ts" => Language::TypeScript,
        _ => unimplemented!(),
    };

    let mut input = Vec::new();
    for filename in matches.get_many::<OsString>("input").unwrap() {
        input.push(PathBuf::from(filename).canonicalize().unwrap());
    }

    let output = matches.get_one::<OsString>("output").unwrap();
    let output = if output == "stdout" {
        compiler::Output::Stdout
    } else {
        compiler::Output::Directory(PathBuf::from(output).canonicalize().unwrap())
    };

    Compiler::new(target, input, output).run().unwrap();
}
