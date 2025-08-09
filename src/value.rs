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


#[derive(Debug,Clone,PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    Rune(u32),
    String(String),
    Bool(bool),
    
    Array(Box<[Value]>),
    Slice(usize, Rc<RefCell<[Value]>>),
    Struct(HashMap<String,Value>),
    Void,
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;

        match *self {
            Int(a) => write!(f, "{}", a),
            Float(a) => write!(f, "{}", a),
            Rune(a) => write!(f, "{}", a),
            String(ref a) => write!(f, "{}", a), // prints the string without quotation marks
            Bool(a) => write!(f, "{}", a), // prints true or false
            _ => write!(f, "<unimplemented>"),
        }
    }
}

fn parse_with_kind(s: &str,k: &Kind) -> Value {
    match (k) {
        Kind::Basic(Int) => Value::Int(i32::try_from(string_to_int(s)).unwrap()),
        Kind::Basic(Float) => Value::Float(s.parse::<f64>().unwrap()),
        Kind::Basic(Rune) => Value::Rune(util::parse_rune_literal(s)),
        Kind::Basic(String) => Value::String(util::parse_string_literal(s)),
        Kind::Basic(Bool) => Value::Bool(match s {
            "true" => true,
            "false" => false,
            _ => panic!("unrecognized bool literal"),
        }),
        _ => panic!("cannot parse non-basic literal")

    }
}

enum ErrorCode {
    NotFound,
    PermissionDenied,
    Unknown,
}

fn explain_error_code(code: ErrorCode) -> &'static str {
    match code {
        ErrorCode::NotFound => "The requested item was not found.",
        ErrorCode::PermissionDenied => "You do not have permission to perform this action.",
        ErrorCode::Unknown => "An unknown error has occurred.",
    }
}

