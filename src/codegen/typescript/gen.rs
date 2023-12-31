use std::io;

use crate::codegen::{Exporter, LanguageGenerator};
use crate::compiler::VERSION;
use crate::error::{Err, Result};
use crate::parser::ast::*;

pub struct Generator;

impl LanguageGenerator for Generator {
    fn generate<W: io::Write>(writer: &mut W, ast: &Ast) -> Result<()> {
        ast.generate(writer, 0).map_err(Err::CodeGen)?;
        Ok(())
    }
}

impl Exporter for Generator {
    fn export<W: io::Write>(writer: &mut W, lib_name: &str) -> Result<()> {
        writeln!(writer, "export * from './{}';", lib_name).map_err(Err::IOError)?;
        Ok(())
    }
}

pub trait Generate {
    type Out;

    fn generate<W: io::Write>(&self, writer: &mut W, mode: u8) -> io::Result<Self::Out>;
}

impl Generate for Ast {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W, mode: u8) -> io::Result<Self::Out> {
        writeln!(writer, "// Generated by Binserde {}\n", VERSION)?;
        writeln!(writer, "import {{ Arr, Base, Bool, Enum, Int8, Int16, Int32, Int64, Int128, Option, String, Uint8, Uint16, Uint32, Uint64, Uint128, Float32, Float64, Vec }} from 'bincoder';")?;

        for decl in &self.decls {
            writeln!(writer)?;
            decl.generate(writer, mode)?;
        }

        Ok(())
    }
}

impl Generate for Decl {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W, mode: u8) -> io::Result<Self::Out> {
        match self {
            Decl::Struct(s) => s.generate(writer, mode),
            Decl::Enum(e) => e.generate(writer, mode),
        }
    }
}

impl Generate for Struct {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W, mode: u8) -> io::Result<Self::Out> {
        writeln!(writer, "export class {} extends Base {{", self.id)?;
        for field in &self.fields {
            write!(writer, "  {}: ", field.id)?;
            field.type_.generate(writer, mode)?;
            writeln!(writer, ";")?;
        }

        write!(writer, "\n  constructor(")?;
        for (i, field) in self.fields.iter().enumerate() {
            write!(writer, "{}: ", field.id)?;
            field.type_.generate(writer, 0)?;
            write!(writer, " = new ")?;
            field.type_.generate(writer, 1)?;

            if i < self.fields.len() - 1 {
                write!(writer, ", ")?;
            }
        }
        writeln!(writer, ") {{")?;

        writeln!(writer, "    super()")?;

        for field in &self.fields {
            writeln!(writer, "    this.{} = {};", field.id, field.id)?;
        }
        writeln!(writer, "  }}")?;

        writeln!(writer, "}}")?;
        Ok(())
    }
}

impl Generate for Enum {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W, _mode: u8) -> io::Result<Self::Out> {
        writeln!(writer, "export enum {}Enum {{", self.id)?;
        for field in self.fields.iter() {
            writeln!(writer, "  {},", field.id)?;
        }
        writeln!(writer, "}}\n")?;

        writeln!(writer, "export class {} extends Enum {{", self.id)?;
        writeln!(writer, "  init() {{")?;
        writeln!(writer, "    return {{")?;
        for field in self.fields.iter() {
            write!(writer, "      [{}Enum.{}]: new ", self.id, field.id)?;
            field.type_.generate(writer, 1)?;
            writeln!(writer, ",")?;
        }
        writeln!(writer, "    }}")?;
        writeln!(writer, "  }}")?;
        writeln!(writer, "}}")?;
        Ok(())
    }
}

impl Generate for Type {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W, mode: u8) -> io::Result<Self::Out> {
        match self {
            Type::BasicType(t) => t.generate(writer, mode)?,
            Type::ArrayType(t) => t.generate(writer, mode)?,
            Type::VecType(t) => t.generate(writer, mode)?,
            Type::StructOrEnum(t) => {
                if mode == 0 {
                    write!(writer, "{}", t)?;
                } else {
                    write!(writer, "{}()", t)?;
                }
            }
            Type::OptionType(t) => t.generate(writer, mode)?,
        }
        Ok(())
    }
}

impl Generate for OptionType {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W, mode: u8) -> io::Result<Self::Out> {
        if mode == 0 {
            write!(writer, "Option<")?;
            self.type_.generate(writer, mode)?;
            write!(writer, ">")?;
        } else {
            write!(writer, "Option(new ")?;
            self.type_.generate(writer, mode)?;
            write!(writer, ")")?;
        }
        Ok(())
    }
}

impl Generate for ArrayType {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W, mode: u8) -> io::Result<Self::Out> {
        if mode == 0 {
            write!(writer, "Arr<")?;
            self.type_.generate(writer, 0)?;
            write!(writer, ", {}", self.len)?;
            write!(writer, ">")?;
        } else {
            write!(writer, "Arr(")?;
            self.type_.generate(writer, 0)?;
            write!(writer, ", {}", self.len)?;
            write!(writer, ")")?;
        }
        Ok(())
    }
}

impl Generate for VecType {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W, mode: u8) -> io::Result<Self::Out> {
        if mode == 0 {
            write!(writer, "Vec<")?;
            self.type_.generate(writer, 0)?;
            write!(writer, ">")?;
        } else {
            write!(writer, "Vec(")?;
            self.type_.generate(writer, 0)?;
            write!(writer, ")")?;
        }
        Ok(())
    }
}

impl Generate for VecItemType {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W, mode: u8) -> io::Result<Self::Out> {
        match self {
            Self::BasicType(t) => t.generate(writer, mode)?,
            Self::StructOrEnum(t) => write!(writer, "{}", t)?,
        }
        Ok(())
    }
}

impl Generate for BasicType {
    type Out = ();

    fn generate<W: io::Write>(&self, writer: &mut W, mode: u8) -> io::Result<Self::Out> {
        match self {
            BasicType::Bool => write!(writer, "Bool")?,
            BasicType::Uint8 => write!(writer, "Uint8")?,
            BasicType::Uint16 => write!(writer, "Uint16")?,
            BasicType::Uint32 => write!(writer, "Uint32")?,
            BasicType::Uint64 => write!(writer, "Uint64")?,
            BasicType::Uint128 => write!(writer, "Uint128")?,
            BasicType::Int8 => write!(writer, "Int8")?,
            BasicType::Int16 => write!(writer, "Int16")?,
            BasicType::Int32 => write!(writer, "Int32")?,
            BasicType::Int64 => write!(writer, "Int64")?,
            BasicType::Int128 => write!(writer, "Int128")?,
            BasicType::Float32 => write!(writer, "Float32")?,
            BasicType::Float64 => write!(writer, "Float64")?,
            BasicType::String => write!(writer, "String")?,
        }
        if mode == 1 {
            write!(writer, "()")?;
        }
        Ok(())
    }
}
