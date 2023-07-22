pub struct Ast {
    pub decls: Vec<Decl>,
}

pub enum Decl {
    Struct(Struct),
    // Tuple(Tuple),
    // Enum(Enum),
}

pub struct Struct {
    pub id:     String,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub id:    String,
    pub type_: Type,
}

pub enum Type {
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
