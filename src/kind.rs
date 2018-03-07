#[derive(Debug,Clone)]

type DefinitionId = usize;

pub enum BaseKind {
    Int = 0,
    Float = 1,
    Rune = 2,
    String = 3,
    Bool = 4
}

pub enum Kind {
    Base(BaseKind),
    Defined(DefinitionId),
    Array(Box<Kind>,u32),
    Struct(Vec<Field>),
    Slice(Box<Kind>),
}

pub struct Field {
    name: String,
    kind: Kind
}

pub struct KindDefinition {
    line_number: u32,
    name: string,
    baseKind: Kind
}

