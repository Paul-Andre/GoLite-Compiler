use std::collections::HashMap;
use std::process::exit;

pub struct SymbolTable<'a> {
    pub parent_scope: Option<&'a SymbolTable<'a>>,
    pub variables: HashMap<String, Symbol>,
    pub types: HashMap<String, Symbol>,
    pub return_type: Option<Type>
}

impl struct SymbolTable<'a>{
    
}


pub struct Symbol {
    pub line_number: u32,
    pub identifier: String,
    pub definition: Definition
}


#[derive(Clone)]
pub enum Definition {
    Variable(Type),
    Type(Type),
    Function{params: Vec<Definition>, return_type: Type}
}

#[derive(Clone)]
pub enum BaseType {
    Int = 0,
    Float = 1,
    Rune = 2,
    String = 3,
    Bool = 4
}

#[derive(Clone)]
pub enum StructureType {
    Array(Type),
    Struct(Vec<Field>),
    Slice(Type),
}

#[derive(Clone)]
pub struct Field {
    pub name: String,
    pub _type: Type
}

#[derive(Clone)]
pub enum Type {
    Base(BaseType),

    // TODO: When evaluating base type, will have to traverse table if referencing another defined type
    // NOTE: name and base cannot be the same!!!
    // Type equality only happens if bases are the same
    DefinedType{ name: String, base: BaseType},
    DataStructure(Box<StructureType>),
    Void
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
