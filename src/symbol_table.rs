use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    pub parent_scope: Option<Box<SymbolTable>>,
    pub children_scopes: <Vec<SymbolTable>,
    pub variables: HashMap<String, Symbol>,
    pub return_type: Option<Type>

    // TODO: On initial creation of symbol table need, to populate global space with predefined declarations (refer to page 3 of specs)
}

#[derive(Debug)]
pub struct Symbol {
    pub line_number: u32,
    pub identifier: String,
    pub definition: Definition
}

pub enum Definition {
    Variable(Type),
    Type(Type),
    Function{params: Vec<Definition>, scope: SymbolTable, return_type: Type}
}

pub enum BaseType {
    Int = 0,
    Float = 1,
    Rune = 2,
    String = 3,
    Bool = 4
}

pub enum StructureType {
    Array(Type),
    Struct(Vec<Field>),
    Slice(Type),
}

pub struct Field {
    name: String,
    _type: Type
}

pub enum Type {
    Base(BaseType),

    // TODO: When evaluating base type, will have to traverse table if referencing another defined type
    // NOTE: name and base cannot be the same!!!
    // Type equality only happens if bases are the same
    DefinedType{ name: String, base: BaseType},
    DataStructure(StructureType),
    Void
}

fn find_variable_definition(identifier: String, scope: &SymbolTable) -> Option(Symbol){

    let mut current_scope: SymbolTable = scope;

    while current_scope != None {
        let temp = current_scope.variables.get(identifier);

        match temp {
            &Some(ref sym ) => {
                if sym.identifier == identifier {
                    return Some(sym)
                } else {
                    current_scope = current_scope.parent_scope
                }
            },
            &None => current_scope.parent_scope
        }
    }

    return None
}

fn find_type_definition(identifier: String, scope: &SymbolTable) -> Option(Symbol){

    let mut current_scope: SymbolTable = scope;

    while current_scope != None {
        let temp = current_scope.types.get(identifier);

        match temp {
            &Some(ref sym ) => {
                if sym.identifier == identifier {
                    return Some(sym)
                } else {
                    current_scope = current_scope.parent_scope
                }
            },
            &None => current_scope.parent_scope
        }
    }

    return None
}

// Looks up identifier in context. Returns type if identifier is in current or parent
fn get_var_type(identifier: String, scope: &SymbolTable) -> Option(Definition::Variable(Type)) {
    let mut current_scope: SymbolTable = scope;

    while current_scope != None {
        let temp = current_scope.variables.get(identifier);

        match temp {
            &Some(ref sym ) => {
                if sym.identifier == identifier {
                    match sym.definition {
                        Definition::Variable( ref t) => return Some(t)
                    }
                } else {
                    current_scope = current_scope.parent_scope
                }
            },
            &None => current_scope.parent_scope
        }
    }

    return None
}

// Adds symbol to symbol table. We need to check duplicates at this point.
fn add_variable_symbol(identifier: String, definition: Definition::Variable(Type), scope: &SymbolTable) {

    let temp = scope.variables.get(identifier);

    match temp {
        &Some(ref var) => {
            if var.identifier == identifier {
                // TODO: error message with line number
            } else {
                let sym = Symbol { line_number: 0, identifier, definition };
                scope.variables.insert(identifier, sym)
            }
        },
        &None => {
            let sym = Symbol { line_number: 0, identifier, definition };
            scope.variables.insert(identifier, sym)
        }
    }
}

// Adds symbol to symbol table. We need to check duplicates at this point.
fn add_type_symbol(identifier: String, definition: Definition::Type(Type), scope: &SymbolTable) {
    let temp = scope.types.get(identifier);

    match temp {
        &Some(ref var) => {
            if var.identifier == identifier {
                // TODO: error message with line number
            } else {
                let sym = Symbol { line_number: 0, identifier, definition };
                scope.types.insert(identifier, sym)
            }
        },
        &None => {
            let sym = Symbol { line_number: 0, identifier, definition };
            scope.types.insert(identifier, sym)
        }
    }
}

// Creates new scope
fn add_new_scope(return_type: Type, table: &SymbolTable) -> &SymbolTable {

}

// Checks equality of two types
fn types_are_equal(a: Type, b: Type) -> bool {

}
