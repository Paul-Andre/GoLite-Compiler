use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    pub parent_scope: Option<Box<SymbolTable>>,
    pub children_scopes: <Vec<SymbolTable>,
    pub symbols: HashMap<String, Symbol>,

    // TODO: On initial creation of symbol table need, to populate global space with predefined declarations (refer to page 3 of specs)
}

#[derive(Debug)]
pub struct Symbol {
    pub line_number: u32,
    pub identifier: String,
    pub definition: Definition
}

pub enum Definition {
    Variable(Type),
    Type(Type),
    Function{params: Vec<Definition>, scope: SymbolTable, return_type: Type}
}

pub enum BaseType {
    Int = 0,
    Float = 1,
    Rune = 2,
    String = 3,
    Bool = 4
}

pub enum StructureType {
    Array(Type),
    Struct(Vec<Field>),
    Slice(Type),
}

pub struct Field {
    name: String,
    _type: Type
}

pub enum Type {
    Base(BaseType),

    // TODO: When evaluating base type, will have to traverse table if referencing another defined type
    // NOTE: name and base cannot be the same!!!
    // Type equality only happens if bases are the same
    DefinedType{ name: String, base: BaseType},
    DataStructure(StructureType),
    Void
}

