use std::rc::Rc;
use std::fmt::Display;
use std::fmt;

#[repr(C)]
#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum BasicKind {
    Int = 0,
    Float = 1,
    Rune = 2,
    String = 3,
    Bool = 4
}

#[derive(Debug,Clone)]
pub enum Kind {
    Undefined,
    Basic(BasicKind),
    Defined(Rc<Definition>),
    Array(Box<Kind>,u32),
    Slice(Box<Kind>),
    Struct(Vec<Field>),
    Underscore,
}

impl fmt::Display for BasicKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match *self {
                   BasicKind::Int => "int",
                   BasicKind::Float => "float64",
                   BasicKind::Rune => "rune",
                   BasicKind::String => "string",
                   BasicKind::Bool => "bool",
               })
    }
}


impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Kind::*;
        match *self {
            Undefined => write!(f, "<undefined>"),
            Basic(b) => write!(f, "{}", b),
            Defined(ref def) => {
                write!(f, "{}", def.name)
            },
            Array(ref k, s) => write!(f, "[{}]{}", s, k),
            Slice(ref k) => write!(f, "[]{}", k),
            Struct(ref fields) => {
                write!(f, "{{")?;
                for &Field{ref name, ref kind} in fields {
                    write!(f, "{} {}; ", name, kind)?;
                }
                write!(f, "}}")
            },
            Underscore => write!(f, "_"),

        }
    }
}

#[derive(Debug,Clone)]
pub struct Field {
    pub name: String,
    pub kind: Kind
}

#[derive(Debug,Clone)]
pub struct Definition {
    pub line_number: u32,
    pub name: String,
    pub kind: Kind
}

pub fn are_identical(a: &Kind, b: &Kind) -> bool {
    use self::Kind::*;
    match (a, b) {
        (&Basic(a_kind), &Basic(b_kind)) => a_kind == b_kind,
        (&Defined(ref a), &Defined(ref b)) => {
            Rc::ptr_eq(a,b)
        },
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
        (&Underscore, _) => true, // Ugly hack
        _ => false
    }
}

pub fn are_comparable(a: &Kind, b: &Kind) -> bool {
    return are_identical(a, b) && a.is_comparable()
}

pub fn are_ordered(a: &Kind, b: &Kind) -> bool {
    return are_identical(a, b) && a.is_ordered()
}

pub fn are_numeric(a: &Kind, b: &Kind) -> bool{
    return a.is_numeric() && b.is_numeric()
}

pub fn are_integers(a: &Kind, b: &Kind) -> bool {
    return a.is_integer() && b.is_integer()
}


impl Kind {
    pub fn resolve<'a>(&'a self) -> &'a Kind {
        match self {
            &Kind::Defined(ref r) => &(r.kind).resolve(),
            something_else => something_else
        }
    }

    pub fn is_comparable(&self) -> bool {
        match self.resolve() {
            &Kind::Struct(ref fields) => {
                for f in fields.iter(){
                    if !f.kind.is_comparable() {
                        return false
                    }
                }
                return true
            },
            &Kind::Array(ref kind, ..) => {
                return kind.is_comparable()
            },
            &Kind::Slice(..) => false,
            _ => true
        }
    }

    pub fn is_ordered(&self) -> bool {
        match self.resolve() {
            &Kind::Basic(BasicKind::Bool) | &Kind::Slice(..)
            | &Kind::Struct(..) => false,
            _ => true
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self.resolve() {
            &Kind::Basic(t) => {
                t == BasicKind::Int || t == BasicKind::Rune || t == BasicKind::Float
            }
            _ => false
        }
    }

    pub fn is_integer(&self) -> bool {
        match self.resolve() {
            &Kind::Basic(t) => {
                t == BasicKind::Int || t == BasicKind::Rune
            }
            _ => false
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self.resolve() {
            &Kind::Basic(BasicKind::Bool) => true,
            _ => false
        }
    }

    pub fn is_string(&self) -> bool {
        match self.resolve() {
            &Kind::Basic(BasicKind::String) => true,
            _ => false
        }
    }
}

