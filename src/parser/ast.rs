pub struct Ast {
    pub decls: Vec<Decl>,
}

pub enum Decl {
    Struct(Struct),
    Enum(Enum),
}

pub struct Struct {
    pub id:     String,
    pub fields: Vec<Field>,
}

pub struct Enum {
    pub id:     String,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub id:    String,
    pub type_: Type,
}

#[allow(clippy::enum_variant_names)]
pub enum Type {
    BasicType(BasicType),
    ArrayType(ArrayType),
    VecType(VecType),
    StructOrEnum(String),
    OptionType(OptionType),
}

pub struct OptionType {
    pub type_: Box<Type>,
}

pub struct ArrayType {
    pub type_: VecItemType,
    pub len:   u64,
}

pub struct VecType {
    pub type_: VecItemType,
}

pub enum VecItemType {
    BasicType(BasicType),
    StructOrEnum(String),
}

pub enum BasicType {
    Bool,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Float32,
    Float64,
    String,
}
