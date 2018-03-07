use std::collections::HashMap;
use std::process::exit;
use kind::Kind;

pub struct SymbolTable<'a> {
    pub parent_scope: Option<&'a SymbolTable<'a>>,
    pub symbols: HashMap<String, Symbol>,
    pub return_type: Option<Type>,
    pub in_function: bool
}

impl struct SymbolTable<'a>{
    fn get_symbol<'b>(&'b Self, s: &str) -> Option<&'b Symbol>{

    }
    fn is_in_current_scope<'b>(&'b Self, s: &str) -> bool {

    }
    fn new_scope<'b>(&'b Self) -> &'a SymbolTable<'b> {

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


fn find_variable_definition<'a>(identifier: String, scope: &'a SymbolTable<'a>) -> Option<&'a Symbol>{

    let mut current_scope = Some(scope);

    while let Some(x) = current_scope {
        let temp = x.variables.get(&identifier);

        match temp {
            Some(ref sym ) => {
                if sym.identifier == identifier {
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

fn find_type_definition<'a>(identifier: String, scope: &'a SymbolTable<'a>) -> Option<&'a Symbol>{

    let mut current_scope = Some(scope);

    while let Some(x) = current_scope {
        let temp = x.types.get(&identifier);

        match temp {
            Some(ref sym ) => {
                if sym.identifier == identifier {
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

// Looks up identifier in context. Returns type if identifier is in current or parent
fn get_var_type<'a>(identifier: String, scope: &'a SymbolTable<'a>) -> Option<&Type> {
    let mut current_scope = Some(scope);

    while let Some(x) = current_scope {
        let var = x.variables.get(&identifier);

        match var {
            Some(ref sym) => {
                if sym.identifier == identifier {
                    match sym.definition {
                        Definition::Variable(ref t) => return Some(t),
                        _ => {
                            eprintln!("Error: trying to get var type on a type definition");
                            exit(1);
                        }
                    }

                } else {
                    current_scope = x.parent_scope
                }
            },
            None => current_scope = x.parent_scope
        }
    }

    return None
}

// Adds symbol to symbol table. We need to check duplicates at this point.
pub fn add_variable_symbol<'a>(identifier: String,
                       definition: Definition, scope: &mut SymbolTable<'a>) {

    let temp = scope.variables.get(&identifier);

    match temp {
        Some(ref var) => {
            if var.identifier == identifier {
                // TODO: error message with line number
            } else {
                let sym = Symbol { line_number: 0, identifier, definition };
                scope.variables.insert(identifier.clone(), sym);
            }
        },
        None => {
            let sym = Symbol { line_number: 0, identifier, definition };
            scope.variables.insert(identifier.clone(), sym);
        }
    }
}

// Adds symbol to symbol table. We need to check duplicates at this point.
pub fn add_type_symbol<'a>(identifier: String,
                   definition: Definition, scope: & mut SymbolTable<'a>) {

    let temp = scope.types.get(&identifier);

    match temp {
        Some(ref var) => {
            if var.identifier == identifier {
                // TODO: error message with line number
            } else {
                let sym = Symbol { line_number: 0, identifier: identifier.clone(), definition };
                scope.types.insert(identifier, sym);
            }
        },
        None => {
            let sym = Symbol { line_number: 0, identifier: identifier.clone(), definition };
            scope.types.insert(identifier, sym);
        }
    }
}

// Creates new scope
//fn add_new_scope(return_type: Type, table: &SymbolTable) -> &SymbolTable {
//    // TODO
//}

// Checks equality of two types
fn types_are_equal(a: Type, b: Type) -> bool {
    // TODO
    return false
}
