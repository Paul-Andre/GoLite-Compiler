
pub type DefinitionId = usize;


#[repr(C)]
#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum BaseKind {
    Int = 0,
    Float = 1,
    Rune = 2,
    String = 3,
    Bool = 4
}

#[derive(Debug,Clone)]
pub enum Kind {
    Base(BaseKind),
    Defined(DefinitionId),
    Array(Box<Kind>,u32),
    Slice(Box<Kind>),
    Struct(Vec<Field>),
}

#[derive(Debug,Clone)]
pub struct Field {
    name: String,
    kind: Kind
}

#[derive(Debug,Clone)]
pub struct Definition {
    line_number: u32,
    name: String,
    kind: Kind
}

pub fn are_identical(a: &Kind, b: &Kind) -> bool {
    use self::Kind::*;
    match (a, b) {
        (&Base(a_kind), &Base(b_kind)) => a_kind == b_kind,
        (&Defined(a_id), &Defined(b_id)) => a_id == b_id,
        (&Array(ref a_base, a_size), &Array(ref b_base, b_size)) => {
            are_identical(a_base, b_base) && a_size == b_size
        },
        (&Slice(ref a_base), &Slice(ref b_base)) => {
            are_identical(a_base, b_base)
        },
        (&Struct(ref a_fields), &Struct(ref b_fields)) => {
            a_fields.len() == b_fields.len() &&
                a_fields.iter().zip(b_fields).all(|(a_field, b_field)| {
                    &a_field.name == &b_field.name &&
                        are_identical(&a_field.kind,&b_field.kind)
                })
        },
        _ => false
    }
}

pub fn resolve<'a>(k : &'a Kind, definitions: &'a [Definition]) -> &'a Kind {
    match k {
        &Kind::Defined(id) => &definitions[id].kind,
        something_else => something_else
    }
}

