use std::collections::HashMap;
use std::process::exit;
use kind::Kind;
use kind::BasicKind;

pub struct SymbolTable<'a> {
    pub parent_scope: Option<&'a SymbolTable<'a>>,
    pub symbols: HashMap<String, Symbol>,
    pub return_type: Option<Kind>,
    pub in_function: bool,
    pub level: u32,
    pub print_table: bool,
}

impl<'a> SymbolTable<'a>{
    pub fn get_symbol<'b>(&'b self, identifier: &str, line_number: u32) -> &'b Symbol{

        let mut current_scope = Some(self);

        while let Some(x) = current_scope {
            let temp = x.symbols.get(identifier);

            match temp {
                Some(ref sym ) => {
                        return sym
                },
                None => current_scope = x.parent_scope
            }
        }

        eprintln!("Error: line {}: `{}` is undefined.", line_number, identifier);
        exit(1);
    }
    pub fn is_in_current_scope<'b>(&'b self, identifier: &str) -> bool {
        self.symbols.get(identifier).is_some()
    }
    pub fn new_scope<'b>(&'b self) -> SymbolTable<'b> {
        if (self.print_table) {
            indent(self.level + 1);
            println!("{{");
        }
        return SymbolTable {
            parent_scope: Some(self),
            symbols: HashMap::new(),
            return_type: self.return_type.clone(),
            in_function: self.in_function,
            level: self.level + 1,
            print_table: self.print_table
        }
    }
    pub fn add_declaration(&mut self, id: String, line_number: u32, decl: Declaration, inferred: bool) {
        use self::Declaration::*;

        if (&id == "_") {
            return;
        }

        let ilk = 
            match decl {
                Variable(..) => "variable",
                Constant(..) => "constant",
                Type(..) => "type",
                Function{..} => "function",
            };

        if (self.level <= 1 && &id == "init") {
            match decl {
                Function{ref params, ref return_kind} => {
                    if (params.len() != 0 || return_kind.is_some()) {
                        eprintln!("Error: line {}: `init` function must have type () -> void",
                                    line_number);

                        exit(1);
                    }
                },
                _ => {
                    eprintln!("Error: line {}: cannot have {} called `init` at top level",
                              line_number, ilk);
                    exit(1);
                }
            }
            return;
        }

        if let Some(&Symbol{line_number: l, ..}) = self.symbols.get(&id) {
            eprintln!("Error: line {}: `{}` was already declared in the current scope at line {}.",
                      line_number, id, l);
            exit(1);
        }

        if (self.print_table) {


            indent(self.level + 1);
            print!("{} [{}] = ", id, ilk);

            if inferred {
                println!("<infer>");
            } else {
                match decl {
                    Variable(ref k) | Constant(ref k) | Type(ref k) => {
                        println!("{}", k);
                    },
                    Function{ref params, ref return_kind}  => {
                        print!("( ");
                        for param in params {
                            print!("{}, ", param);
                        }
                        print!(") -> ");
                        if let &Some(ref ret) = return_kind {
                            print!("{}", ret);
                        } else {
                            print!("void");
                        }
                        println!();
                    }
                }
            }
        }
            
        self.symbols.insert(id, Symbol{
            line_number: line_number,
            declaration: decl
        });
    }
}

fn indent(level: u32) {
    for _ in 0..level {
        print!("\t"); // we use tabs now
    }
}

impl<'a> Drop for SymbolTable<'a> {
    fn drop(&mut self) {
        if (self.print_table) {
            indent(self.level);
            println!("}}");
        }
    }
}


pub struct Symbol {
    pub line_number: u32,
    pub declaration: Declaration,
}


pub enum Declaration {
    Variable(Kind),
    Constant(Kind),
    Type(Kind),
    Function{params: Vec<Kind>, return_kind: Option<Kind>}
}


/// Populates the symbol table with the Go defaul variables and types
pub fn create_root_symbol_table<'a>(print_table: bool) -> SymbolTable<'a>{
    if (print_table) {
        indent(0);
        println!("{{");
    }
    let mut root_scope = SymbolTable{
        parent_scope: None,
        symbols: HashMap::new(),
        return_type: None,
        in_function: false,
        level: 0,
        print_table: print_table
    };

    root_scope.symbols.insert("int".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Type(Kind::Basic(BasicKind::Bool))
    });
    root_scope.symbols.insert("float64".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Type(Kind::Basic(BasicKind::Bool))
    });
    root_scope.symbols.insert("rune".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Type(Kind::Basic(BasicKind::Bool))
    });
    root_scope.symbols.insert("bool".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Type(Kind::Basic(BasicKind::Bool))
    });
    root_scope.symbols.insert("string".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Type(Kind::Basic(BasicKind::Bool))
    });

    root_scope.symbols.insert("true".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Constant(Kind::Basic(BasicKind::Bool))
    });
    root_scope.symbols.insert("false".to_string(), Symbol{
        line_number: 0,
        declaration: Declaration::Constant(Kind::Basic(BasicKind::Bool))
    });

    return root_scope;
}
