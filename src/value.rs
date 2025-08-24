use kind;
use kind::Kind;
use kind::BasicKind;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use util::string_to_int;
use std::convert::TryFrom;
use util;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Slice {
    pub length: usize,
    pub contents: Rc<[RefCell<Value>]>,
    // Note, length might be less than the len(contents)
    // Used to represent a growable slice with a length and capacity
}

const ONE_OVER_MAX: u32 = 2147483648;
const SMALLEST_INT: i32 = -2147483648;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),

    OneOverMax, // A special value to deal with the expression -2147483648 (Since the biggest positive int is 2147483647)
    
    Array(Box<[Value]>),
    Slice(Slice),
    Struct(HashMap<String,Value>),
    Void,


    Undefined, // Used for initializing Slices (although maybe it shouldn't and I should just use the zero type?
}

impl Value {
    pub fn get_integer(&self) -> Option<i32> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None
        }
    }
    pub fn get_boolean(&self) -> Option<bool> {
        match self {
            Value::Bool(i) => Some(*i),
            _ => None
        }
    }
}

fn format_float(a: f64, f: &mut fmt::Formatter) -> fmt::Result {
    if a.is_finite() {
        let base = format!("{:.6e}", a);

        let parts = base.split("e").collect::<Vec<_>>();
        assert!(parts.len() == 2);

        let mut mantissa = parts[0];
        let mut exponent = parts[1];

        let neg_mant = &mantissa[0..1] == "-";
        mantissa = if neg_mant {&mantissa[1..]} else {mantissa};

        let neg_exp = &exponent[0..1] == "-";
        exponent = if neg_exp {&exponent[1..]} else {exponent};

        let mut len_exp = exponent.len();

        write!(f, "{}", if neg_mant { "-" } else {"+"})?;
        write!(f, "{}", mantissa)?;

        write!(f, "e")?;

        write!(f, "{}", if neg_exp { "-" } else {"+"})?;
        while len_exp<3 {
            write!(f, "0")?;
            len_exp += 1;
        }
        write!(f, "{}", exponent)
    } else if a.is_nan() {
        write!(f,"NaN")
    } else if a == f64::INFINITY {
        write!(f,"+Inf")
    } else if a == f64::NEG_INFINITY {
        write!(f,"-Inf")
    } else {
        panic!("Those are al the f64 options, there should be any others.");
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;

        match self {
            Int(a) => write!(f, "{}", a),
            Float(a) => format_float(*a, f),
            String(ref a) => write!(f, "{}", a), // prints the string without quotation marks
            Bool(a) => write!(f, "{}", a), // prints true or false
            
            Value::OneOverMax => write!(f, "2147483648"),

            Value::Void => write!(f, "()"),

            Value::Undefined => write!(f, "<undefined>"),

            Array(..) => write!(f, "<array>"),
            Slice(s) => {
                write!(f, "<slice> [")?;
                for v in s.contents.iter() {
                    write!(f, "{} ", v.borrow())?;
                }
                write!(f, "]")
            },

            Struct(..) => write!(f, "<struct>"),
        }
    }
}

pub fn parse_with_kind(s: &str,k: &Kind) -> Value {
    match k {
        Kind::Basic(BasicKind::Int) => {
            let base: u32 = string_to_int(s);
            if base == ONE_OVER_MAX {
                Value::OneOverMax
            } else {
                Value::Int(i32::try_from(base).unwrap())
            }
        }
        Kind::Basic(BasicKind::Float) => Value::Float(s.parse::<f64>().unwrap()),
        Kind::Basic(BasicKind::Rune) => Value::Int(util::parse_rune_literal(s)),
        Kind::Basic(BasicKind::String) => Value::String(util::parse_string_literal(s)),
        Kind::Basic(BasicKind::Bool) => Value::Bool(match s {
            "true" => true,
            "false" => false,
            _ => panic!("unrecognized bool literal"),
        }),
        _ => panic!("cannot parse non-basic literal")

    }
}

pub fn zero_array(k: &Kind, len: u32) -> Value {
    let mut v: Vec<Value> = Vec::new();
    for _ in 0..len {
        v.push(zero_value(k));
    }
    Value::Array(v.into())
}

pub fn zero_slice(k: &Kind) -> Value {
    _ = k; // interestingly, I don't even need to know the kind.
    let v: Vec<RefCell<Value>> = Vec::new();
    Value::Slice(Slice{length:0, contents:v.into()})
}
pub fn zero_struct(fields: &[kind::Field]) -> Value {
    let mut ret = HashMap::<String, Value>::new();
    for kind::Field{name, kind} in fields {
        ret.insert(name.clone(), zero_value(kind));
    }
    Value::Struct(ret)
}

pub fn zero_value(k: &Kind) -> Value {
    use self::Kind::*;
    match k {
        Undefined => Value::Undefined,
        Basic(BasicKind::Int) => {
            Value::Int(0)
        }
        Basic(BasicKind::Float) => {
            Value::Float(0.0)
        }
        Basic(BasicKind::Rune) => {
            Value::Int(0)
        }
        Basic(BasicKind::String) => {
            Value::String("".to_string())
        }
        Basic(BasicKind::Bool) => {
            Value::Bool(false)
        }
        Defined(ref def) => {
            zero_value(&def.borrow().kind)
        },
        Array(k, s) => {
            zero_array(k, *s)
        }
        Slice(ref k) => {
            zero_slice(k)
        }
        Struct(ref fields) => {
            zero_struct(fields)
        },
        Underscore => panic!("It does not make sense to instantiate the underscore type"),
        Void => Value::Void,
    }

}

pub mod builtins {
    use value;
    use value::*;

    pub fn cast(kk: &Kind, v: &Value) -> Value {
        let k = kk.resolve();
        if let Kind::Basic(bk) = k {
                //dbg!(bk,v);
            match (bk, v) {
                (BasicKind::String, Value::Int(i)) => {
                    // In modern Go, this will work only for runes, it is deprecated for other ints
                    let i = *i;
                    let c = char::from_u32(i as u32).unwrap();
                    return Value::String(format!("{}",c))
                },
                (BasicKind::Int, Value::Float(f)) => {
                    let f = *f;
                    return Value::Int(f as i32);
                },
                (BasicKind::Rune, Value::Float(f)) => {
                    let f = *f;
                    return Value::Int(f as i32);
                },
                (BasicKind::Float, Value::Int(i)) => {
                    let i = *i;
                    return Value::Float(i as f64);
                },
                (_,_) => {},
            }
        }
        // Assume most of the heavy lifting was done by the typechecker.
        return v.clone();
    }

    pub fn append(l: Value, r: Value) -> Value {
        if let Value::Slice(slice) = l {
            let length = slice.length;
            let new_l = length+1;
            if new_l <= slice.contents.len() {
                *slice.contents[new_l-1].borrow_mut() = r;
                Value::Slice(value::Slice {
                    length: new_l,
                    contents: slice.contents
                })
            } else {
                let new_capacity = if length==0 {1} else {length*2};
                let mut new_contents: Vec<RefCell<Value>> = Vec::with_capacity(new_capacity);
                for a in slice.contents.iter() {
                    new_contents.push(RefCell::new(a.borrow_mut().clone()));
                }
                new_contents.push(RefCell::new(r));
                while new_contents.len()<new_capacity {
                    new_contents.push(RefCell::new(Value::Undefined));
                }

                Value::Slice(value::Slice {
                    length: new_l,
                    contents: new_contents.into(),
                })
            }

        } else {
            panic!("Cannot append to non-slice");
        }
    }

    pub fn plus(v: &Value) -> Value {
        v.clone()
    }
    pub fn neg(v: &Value) -> Value {
        use value::Value::*;
        match v {
            Int(a) => Int(-a),
            OneOverMax => Int(SMALLEST_INT),
            Float(f) => Float(-f),
            _ => panic!("Cannot negate"),
        }
    }
    pub fn bw_compl(v: &Value) -> Value {
        use value::Value::*;
        match v {
            Int(a) => Int(!a),
            _ => panic!("Cannot take complement"),
        }
    }
    pub fn not(v: &Value) -> Value {
        use value::Value::*;
        match v {
            Bool(a) => Bool(!a),
            _ => panic!("Cannot take not"),
        }
    }

}
