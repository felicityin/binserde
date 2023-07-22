use std::io;

use crate::error::Result;
use crate::parser::ast::Ast;

mod rust;
mod typescript;

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Rust,
    TypeScript,
}

pub trait LanguageGenerator {
    fn generate<W: io::Write>(writer: &mut W, ast: &Ast) -> Result<()>;
}

pub trait Exporter {
    fn export<W: io::Write>(writer: &mut W, lib_name: &str) -> Result<()>;
}

impl Language {
    pub fn extension(&self) -> &'static str {
        match *self {
            Self::Rust => "rs",
            Self::TypeScript => "ts",
        }
    }

    pub fn generate<W: io::Write>(&self, writer: &mut W, ast: &Ast) -> Result<()> {
        match *self {
            Self::Rust => rust::Generator::generate(writer, ast),
            Self::TypeScript => typescript::Generator::generate(writer, ast),
        }
    }

    pub fn lib(&self) -> &'static str {
        match *self {
            Self::Rust => "mod.rs",
            Self::TypeScript => "index.ts",
        }
    }

    pub fn export<W: io::Write>(&self, writer: &mut W, lib_name: &str) -> Result<()> {
        match *self {
            Self::Rust => rust::Generator::export(writer, lib_name),
            Self::TypeScript => typescript::Generator::export(writer, lib_name),
        }
    }
}
