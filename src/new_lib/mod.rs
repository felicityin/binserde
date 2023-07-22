use std::path;

use crate::error::Result;

mod rust;
mod typescript;

#[derive(Debug, Clone, Copy)]
pub enum Target {
    Rust,
    TypeScript,
}

pub trait TemplateGenerator {
    fn generate(lib_name: &str, output_dir: path::PathBuf) -> Result<()>;
}

impl Target {
    pub fn generate(&self, lib_name: &str, output_dir: path::PathBuf) -> Result<()> {
        match *self {
            Self::Rust => rust::Template::generate(lib_name, output_dir),
            Self::TypeScript => typescript::Template::generate(lib_name, output_dir),
        }
    }
}
