use std::collections::HashMap;
use std::process::exit;
use kind::Kind;
use kind::BasicKind;

pub struct SymbolTable<'a> {
    pub parent_scope: Option<&'a SymbolTable<'a>>,
    pub symbols: HashMap<String, Symbol>,
    pub return_type: Option<Kind>,
    pub in_function: bool
}

impl<'a> SymbolTable<'a>{
    pub fn get_symbol<'b>(&'b self, identifier: &str) -> Option<&'b Symbol>{

        let mut current_scope = Some(self);

        while let Some(x) = current_scope {
            let temp = x.symbols.get(identifier);

            match temp {
                Some(ref sym ) => {
                        return Some(sym)
                },
                None => current_scope = x.parent_scope
            }
        }

    return None

    }
    pub fn is_in_current_scope<'b>(&'b self, identifier: &str) -> bool {
        self.symbols.get(identifier).is_some()
    }
    pub fn new_scope<'b>(&'b self) -> SymbolTable<'b> {
        return SymbolTable {
            parent_scope: Some(self),
            symbols: HashMap::new(),
            return_type: self.return_type.clone(),
            in_function: self.in_function
        }
    }
    pub fn add_symbol(&mut self, id: String, kind: Kind) {
        panic!("unimplemented");
    }


}


pub struct Symbol {
    pub line_number: u32,
    pub declaration: Declaration,
}


pub enum Declaration {
    Variable(Kind),
    Constant(Kind),
    Kind(Kind),
}


/// Populates the symbol table with the Go defaul variables and types
pub fn create_root_symbol_table<'a>() -> SymbolTable<'a>{
    let mut root_scope = SymbolTable{
        parent_scope: None,
        symbols: HashMap::new(),
        return_type: None,
        in_function: false
    };

    root_scope.symbols.insert("true".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Constant(Kind::Basic(BasicKind::Bool))
    });
    root_scope.symbols.insert("false".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Constant(Kind::Basic(BasicKind::Bool))
    });

    root_scope.symbols.insert("int".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Kind(Kind::Basic(BasicKind::Bool))
    });
    root_scope.symbols.insert("float64".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Kind(Kind::Basic(BasicKind::Bool))
    });
    root_scope.symbols.insert("rune".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Kind(Kind::Basic(BasicKind::Bool))
    });
    root_scope.symbols.insert("bool".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Kind(Kind::Basic(BasicKind::Bool))
    });
    root_scope.symbols.insert("string".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Kind(Kind::Basic(BasicKind::Bool))
    });
    return root_scope;
}
