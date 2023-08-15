use std::io;

use crate::codegen::{Exporter, LanguageGenerator};
use crate::compiler::VERSION;
use crate::error::{Err, Result};
use crate::parser::ast::*;

pub struct Generator;

impl LanguageGenerator for Generator {
    fn generate<W: io::Write>(writer: &mut W, ast: &Ast) -> Result<()> {
        ast.generate(writer).map_err(Err::CodeGen)?;
        Ok(())
    }
}

impl Exporter for Generator {
    fn export<W: io::Write>(writer: &mut W, mod_name: &str) -> Result<()> {
        writeln!(writer, "pub mod {};", mod_name).map_err(Err::IOError)?;
        Ok(())
    }
}

trait Generate {
    type Out;

    fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<Self::Out>;
}

impl Generate for Ast {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<Self::Out> {
        writeln!(writer, "// Generated by Binserde {}\n", VERSION)?;
        writeln!(writer, "use bincode::{{error, Decode, Encode}};")?;
        writeln!(writer, "use bincode_macro::Serde;")?;

        for decl in &self.decls {
            writeln!(writer)?;
            decl.generate(writer)?;
        }

        Ok(())
    }
}

impl Generate for Decl {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<Self::Out> {
        match self {
            Decl::Struct(s) => s.generate(writer),
            Decl::Enum(e) => e.generate(writer),
        }
    }
}

impl Generate for Struct {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<Self::Out> {
        writeln!(
            writer,
            "#[derive(Serde, Encode, Decode, PartialEq, Clone, Debug, Default)]"
        )?;
        writeln!(writer, "pub struct {} {{", self.id)?;
        for field in &self.fields {
            write!(writer, "\tpub {}: ", field.id)?;
            field.type_.generate(writer)?;
            writeln!(writer, ",")?;
        }
        writeln!(writer, "}}")?;
        Ok(())
    }
}

impl Generate for Enum {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<Self::Out> {
        writeln!(
            writer,
            "#[derive(Serde, Encode, Decode, PartialEq, Clone, Debug)]"
        )?;
        writeln!(writer, "pub enum {} {{", self.id)?;
        for field in &self.fields {
            write!(writer, "\t{}(", field.id)?;
            field.type_.generate(writer)?;
            writeln!(writer, "),")?;
        }
        writeln!(writer, "}}")?;
        Ok(())
    }
}

impl Generate for Type {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<Self::Out> {
        match self {
            Type::BasicType(t) => t.generate(writer)?,
            Type::ArrayType(t) => t.generate(writer)?,
            Type::VecType(t) => t.generate(writer)?,
            Type::StructOrEnum(t) => write!(writer, "{}", t)?,
        }
        Ok(())
    }
}

impl Generate for ArrayType {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<Self::Out> {
        write!(writer, "[")?;
        self.type_.generate(writer)?;
        write!(writer, "; {}", self.len)?;
        write!(writer, "]")?;
        Ok(())
    }
}

impl Generate for VecType {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<Self::Out> {
        write!(writer, "Vec<")?;
        self.type_.generate(writer)?;
        write!(writer, ">")?;
        Ok(())
    }
}

impl Generate for VecItemType {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<Self::Out> {
        match self {
            Self::BasicType(t) => t.generate(writer)?,
            Self::StructOrEnum(t) => write!(writer, "{}", t)?,
        }
        Ok(())
    }
}

impl Generate for BasicType {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<Self::Out> {
        match self {
            BasicType::Bool => write!(writer, "bool")?,
            BasicType::Uint8 => write!(writer, "u8")?,
            BasicType::Uint16 => write!(writer, "u16")?,
            BasicType::Uint32 => write!(writer, "u32")?,
            BasicType::Uint64 => write!(writer, "u64")?,
            BasicType::Uint128 => write!(writer, "u128")?,
            BasicType::Int8 => write!(writer, "i8")?,
            BasicType::Int16 => write!(writer, "i16")?,
            BasicType::Int32 => write!(writer, "i32")?,
            BasicType::Int64 => write!(writer, "i64")?,
            BasicType::Int128 => write!(writer, "i128")?,
            BasicType::Float32 => write!(writer, "f32")?,
            BasicType::Float64 => write!(writer, "f64")?,
            BasicType::String => write!(writer, "String")?,
        }
        Ok(())
    }
}
