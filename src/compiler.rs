use std::{
    env, fs,
    fs::File,
    io,
    io::{Read, Write},
    path,
};

use lalrpop_util::lalrpop_mod;

use crate::error::{Err, Result};
use crate::Language;

lalrpop_mod!(grammar);

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Compiler {
    target: Language,
    input:  Vec<path::PathBuf>,
    output: Output,
}

#[derive(Clone)]
pub enum Output {
    Stdout,
    Directory(path::PathBuf),
}

impl Compiler {
    pub fn new(target: Language, input: Vec<path::PathBuf>, output: Output) -> Self {
        Self {
            target,
            input,
            output,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let Self {
            target,
            ref input,
            ref output,
        } = self;

        for (i, input) in input.iter().enumerate() {
            let mut file = File::open(input).map_err(Err::IOError)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).map_err(Err::IOError)?;

            let ast = grammar::AstParser::new()
                .parse(&contents)
                .map_err(|err| Err::ParseAst(err.to_string()))?;

            let mut output_data = Vec::<u8>::new();
            target.generate(&mut output_data, &ast)?;

            let mut export_data = Vec::<u8>::new();
            target.export(
                &mut export_data,
                input.file_stem().unwrap().to_str().unwrap(),
            )?;

            match output {
                Output::Directory(ref out_dir) => {
                    let mut out_file = out_dir.to_owned();
                    out_file.push(input.file_stem().unwrap());
                    out_file.set_extension(target.extension());

                    let mut file_out = fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(&out_file)
                        .unwrap();
                    file_out.write_all(&output_data).unwrap();
                    file_out.flush().unwrap();

                    let mod_file = out_dir.join(target.lib());
                    let mut mode_file = if i == 0 {
                        fs::OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .open(&mod_file)
                            .unwrap()
                    } else {
                        fs::OpenOptions::new()
                            .append(true)
                            .open(&mod_file)
                            .unwrap()
                    };
                    mode_file.write_all(&export_data).unwrap();
                    mode_file.flush().unwrap();
                }
                Output::Stdout => {
                    let stdout = io::stdout();
                    let mut stdout_handle = stdout.lock();

                    stdout_handle.write_all(&output_data).unwrap();
                    stdout_handle.flush().unwrap();
                }
            }
        }

        Ok(())
    }
}
