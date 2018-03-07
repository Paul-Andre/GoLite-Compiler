use std::collections::HashMap;
use std::process::exit;
use kind::Kind;

pub struct SymbolTable<'a> {
    pub parent_scope: Option<&'a SymbolTable<'a>>,
    pub symbols: HashMap<String, Symbol>,
    pub return_type: Option<Kind>,
    pub in_function: bool
}

impl<'a> SymbolTable<'a>{
    fn get_symbol<'b>(&'b self, identifier: &str) -> Option<&'b Symbol>{

        let mut current_scope = Some(self);

        while let Some(x) = current_scope {
            let temp = x.symbols.get(identifier);

            match temp {
                Some(ref sym ) => {
                    if &sym.identifier == identifier {
                        return Some(sym)
                    } else {
                        current_scope = x.parent_scope
                    }
                },
                None => current_scope = x.parent_scope
            }
        }

    return None

    }
    fn is_in_current_scope<'b>(&'b self, identifier: &str) -> bool {
        self.symbols.get(identifier).is_some()
    }
    fn new_scope<'b>(&'b self) -> SymbolTable<'b> {
        return SymbolTable {
            parent_scope: Some(self),
            symbols: HashMap::new(),
            return_type: self.return_type.clone(),
            in_function: self.in_function
        }
    }
}


pub struct Symbol {
    pub line_number: u32,
    pub identifier: String,
    pub declaration: Declaration
}


pub enum Declaration {
    Variable(Kind),
    Type(Kind),
    Function{params: Vec<Kind>, return_type: Option<Kind>}
}
