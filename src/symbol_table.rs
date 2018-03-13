use std::collections::HashMap;
use std::process::exit;
use std::rc::Rc;
use kind;
use kind::Kind;
use kind::BasicKind;

pub struct SymbolTable<'a> {
    pub parent_scope: Option<&'a SymbolTable<'a>>,
    pub symbols: HashMap<String, Symbol>,
    pub return_kind: Option<Kind>,
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
        if self.print_table {
            indent(self.level + 1);
            println!("{{");
        }
        return SymbolTable {
            parent_scope: Some(self),
            symbols: HashMap::new(),
            return_kind: self.return_kind.clone(),
            in_function: self.in_function,
            level: self.level + 1,
            print_table: self.print_table
        }
    }

    pub fn add_declaration(&mut self, id: String, line_number: u32, decl: Declaration, inferred: bool) {
        use self::Declaration::*;

        let ilk = 
            match decl {
                Variable(..) => "variable",
                Constant(..) => "constant",
                Type(..) => "type",
                Function{..} => "function",
                Dummy => panic!("Dummy should not be added using add_declaration."),
            };


        if self.level <= 1 && &id == "init" {
            match decl {
                Function{ref params, ref return_kind} => {
                    if params.len() != 0 || return_kind.is_some() {
                        eprintln!("Error: line {}: `init` function must have type () -> void",
                                    line_number);

                        exit(1);
                    }
                },
                _ => {
                    eprintln!("Error: line {}: cannot have {} called `init` at top level.\
                    `init` must be a function",
                              line_number, ilk);
                    exit(1);
                }
            }
            if(self.print_table) {
                indent(self.level + 1);
                println!("init [function] = <unmapped>");
            }
            return;
        }

        let mut is_main = false;
        if self.level <= 1 && &id == "main" {
            match decl {
                Function{ref params, ref return_kind} => {
                    if params.len() != 0 || return_kind.is_some() {
                        eprintln!("Error: line {}: `main` function must have type () -> void",
                                    line_number);

                        exit(1);
                    }
                },
                _ => {
                    eprintln!("Error: line {}: cannot have {} called `main` at top level. \
                    `main` must be a function",
                              line_number, ilk);
                    exit(1);
                }
            }

            if(self.print_table) {
                indent(self.level + 1);
                println!("init [function] = <unmapped>");
            }
            is_main = true;
        }
        
        if &id == "_" && !match decl { Function{..} => true, _ => false} {
            return;
        }

    
        if self.print_table && !is_main {

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
                        print!("(");
                        for (i,param) in params.iter().enumerate() {
                            if i<params.len()-1 {
                                print!("{}, ", param);
                            } else {
                                print!("{}", param);
                            }
                        }
                        print!(") -> ");
                        if let &Some(ref ret) = return_kind {
                            print!("{}", ret);
                        } else {
                            print!("void");
                        }
                        println!();
                    },
                    Dummy => unreachable!()
                }
            }
        }

        if &id == "_" {
            return;
        }

        match self.symbols.get(&id) {
            Some(&Symbol{declaration: Declaration::Dummy, ..})  => {},
            Some(&Symbol{line_number: l, ..}) =>  {
                eprintln!("Error: line {}: `{}` was already declared in the current scope at line {}.",
                          line_number, id, l);
                exit(1);
            }
            None => {},
        }
            
        self.symbols.insert(id, Symbol{
            line_number,
            declaration: decl
        });
    }

    pub fn define_type(&mut self, name: String, line_number: u32, kind: Kind) {
        self.add_declaration( name.clone(), line_number, Declaration::Type(
                Kind::Defined(Rc::new(
                        kind::Definition { line_number, name, kind } ) )),
                        false);
    }

    pub fn add_dummy(&mut self, name: String, line_number: u32) {

        match self.symbols.get(&name) {
            Some(&Symbol{declaration: Declaration::Dummy, ..})  => {},
            Some(&Symbol{line_number: l, ..}) =>  {
                eprintln!("Error: line {}: `{}` was already declared in the current scope at line {}.",
                          line_number, name, l);
                exit(1);
            }
            None => {},
        }
            
        self.symbols.insert(name, Symbol{
            line_number,
            declaration: Declaration::Dummy
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
        if self.print_table {
            indent(self.level);
            println!("}}");
        }
    }
}


pub struct Symbol {
    pub line_number: u32,
    pub declaration: Declaration,
}


#[derive(Clone)]
pub enum Declaration {
    Variable(Kind),
    Constant(Kind),
    Type(Kind),
    Function{params: Vec<Kind>, return_kind: Option<Kind>},
    Dummy,
}


/// Populates the symbol table with the Go default variables and types
pub fn create_root_symbol_table<'a>(print_table: bool) -> SymbolTable<'a>{
    if print_table {
        indent(0);
        println!("{{");
    }
    let mut root_scope = SymbolTable{
        parent_scope: None,
        symbols: HashMap::new(),
        return_kind: None,
        in_function: false,
        level: 0,
        print_table
    };

    root_scope.add_declaration("int".to_string(), 0,
        Declaration::Type(Kind::Basic(BasicKind::Int)),
        false);
    root_scope.add_declaration("float64".to_string(), 0,
        Declaration::Type(Kind::Basic(BasicKind::Float)),
        false);
    root_scope.add_declaration("bool".to_string(), 0,
        Declaration::Type(Kind::Basic(BasicKind::Bool)),
        false);
    root_scope.add_declaration("rune".to_string(), 0,
        Declaration::Type(Kind::Basic(BasicKind::Rune)),
        false);
    root_scope.add_declaration("string".to_string(), 0,
        Declaration::Type(Kind::Basic(BasicKind::String)),
        false);

    root_scope.add_declaration("true".to_string(), 0,
        Declaration::Constant(Kind::Basic(BasicKind::Bool)),
        false);
    root_scope.add_declaration("false".to_string(), 0,
        Declaration::Constant(Kind::Basic(BasicKind::Bool)),
        false);

    return root_scope;
}
