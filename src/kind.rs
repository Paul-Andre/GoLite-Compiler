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
    Func{params: Vec<Kind>, return_kind: Option<Box<Kind>>},
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
            Func{ref params, ref return_kind}  => {
                write!(f, "(")?;
                for param in params {
                    write!(f, "{}, ", param)?;
                }
                write!(f, ") -> ")?;
                if let &Some(ref ret) = return_kind {
                    write!(f, "{}", ret)
                } else {
                    write!(f, "void")
                }
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
        (&Func{..}, &Func{..}) => {
            panic!("Cannot check if function types are identical; Should not happen.");
        },
        (&Underscore, _) => true, // Ugly hack
        _ => false
    }
}

impl Kind {
    pub fn resolve<'a>(&'a self) -> &'a Kind {
        match self {
            &Kind::Defined(ref r) => &(r.kind).resolve(),
            something_else => something_else
        }
    }

    pub fn is_comparable(&self) -> bool {
        //TODO
        false
    }

    pub fn is_ordered(&self) -> bool {
        //TODO
        false
    }

    pub fn is_numeric(&self) -> bool {
        //TODO
        false
    }

    pub fn is_integer(&self) -> bool {
        //TODO
        false
    }
}

