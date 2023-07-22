use thiserror::Error;

pub type Result<T> = std::result::Result<T, Err>;

#[derive(Error, Debug)]
pub enum Err {
    #[error("IO error: {0}")]
    IOError(std::io::Error),

    #[error("Failed to parse AST: {0}")]
    ParseAst(String),

    #[error("Failed to generate code: {0}")]
    CodeGen(std::io::Error),
}
