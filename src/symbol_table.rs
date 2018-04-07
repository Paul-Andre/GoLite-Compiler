use std::collections::HashMap;
use std::process::exit;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Cell;
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
    pub id_counter: Rc<Cell<u32>>,
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
    pub fn new_scope<'b>(&'b mut self) -> SymbolTable<'b> {
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
            print_table: self.print_table,
            id_counter: self.id_counter.clone()
        }
    }

    /// returns the name that the identifier should be renamed to
    fn add_declaration(&mut self, name: String, line_number: u32, decl: Declaration, rename: bool)
        -> String {

        if (&name == "init" || &name == "main") && self.level == 1 {
            match decl {
                Declaration::Function{..} | Declaration::Dummy => {},
                _ => {
                    // If declaration is anythig other than a dummy or a function
                    eprintln!("Error: line {}: only functions can be called `{}` at global \
                    scope.", line_number, name);
                    exit(1);
                }
            }
        }

        if &name == "_" || (&name == "init" && self.level == 1) {
            return name;
        }

        match self.symbols.get(&name) {
            Some(&Symbol{declaration: Declaration::Dummy, ..})  => {},
            Some(&Symbol{line_number: l, ..}) =>  {
                eprintln!("Error: line {}: `{}` was already declared in the current scope at line {}.",
                          line_number, name, l);
                exit(1);
            }
            None => {},
        }

        let new_name = if rename && !(&name == "main" && self.level == 1) {
            self.id_counter.set(self.id_counter.get() + 1);
            format!("{}_{}", name, &self.id_counter.get().to_string())
        } else {
            name.clone()
        };
        self.symbols.insert(name, Symbol{
            line_number,
            declaration: decl,
            new_name: new_name.clone()
        });
        return new_name;
    }

    pub fn define_type(&mut self, name: String, line_number: u32, kind: Kind) -> String {
        self.add_declaration( name.clone(), line_number, Declaration::Type(
                Kind::Defined(Rc::new(RefCell::new(
                        kind::Definition { line_number, name, kind } ) ))),
                /*rename*/ true)
    }

    pub fn print_type_definition(&mut self, name: &str, kind: &Kind) {
        if self.print_table {
            indent(self.level + 1);
            println!("{} [type] = {} -> {}", name, name, kind);
        }
    }


    pub fn add_initial_type(&mut self, name: String, kind: Kind) {
        if self.print_table {
            indent(self.level + 1);
            println!("{} [type] = {}", name, kind);
        }

        self.add_declaration(name.clone(), 0, Declaration::Type(kind), /*rename*/ false);
    }

    pub fn replace_dummy_by_function(&mut self, name: String, line_number: u32,
                        params: Vec<Kind>, return_kind: Option<Kind>) {


        if self.print_table {
            indent(self.level + 1);
            print!("{} [function] = ", name);

            if &name == "_" || &name == "init" {
                print!("<unmapped>");
            }
            else {
                print!("(");
                for (i,param) in params.iter().enumerate() {
                    if i<params.len()-1 {
                        print!("{}, ", param);
                    } else {
                        print!("{}", param);
                    }
                }
                print!(") -> ");
                if let &Some(ref ret) = &return_kind {
                    print!("{}", ret);
                } else {
                    print!("void");
                }
            }
            println!();
        }


        if (&name == "init" || &name == "main") &&
            (params.len() != 0 || return_kind.is_some()) {
            eprintln!("Error: line {}: {} function must have type () -> void",
                        line_number, name);
            exit(1);
        }

        if let Some(sym) = self.symbols.get_mut(&name){
            sym.declaration = Declaration::Function{
                                 params: params,
                                 return_kind: return_kind.clone()
                             };
        };
    }

    pub fn add_variable(&mut self, name: String, line_number: u32, kind: Kind, is_inferred: bool)  -> String {
        

        if self.print_table && &name != "_" {
            indent(self.level + 1);
            if !is_inferred {
                println!("{} [variable] = {}", name, kind);
            } else {
                println!("{} [variable] = <infer>", name);
            }
        }

        self.add_declaration(name, line_number, Declaration::Variable(kind), true)

    }

    pub fn add_constant(&mut self, name: String, line_number: u32, kind: Kind) {

        if self.print_table && &name != "_" {
            indent(self.level + 1);
            println!("{} [constant] = {}", name, kind);
        }
        
        self.add_declaration(name, line_number, Declaration::Constant(kind), false);

    }

    pub fn add_dummy(&mut self, name: String, line_number: u32) {

        self.add_declaration(name, line_number,
                             Declaration::Dummy, true);

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
    pub new_name: String,
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
        print_table,
        id_counter: Rc::new(Cell::new(0))
    };

    root_scope.add_initial_type("int".to_string(), Kind::Basic(BasicKind::Int));
    root_scope.add_initial_type("float64".to_string(), Kind::Basic(BasicKind::Float));
    root_scope.add_initial_type("bool".to_string(), Kind::Basic(BasicKind::Bool));
    root_scope.add_initial_type("rune".to_string(), Kind::Basic(BasicKind::Rune));
    root_scope.add_initial_type("string".to_string(), Kind::Basic(BasicKind::String));
        
    root_scope.add_constant("true".to_string(), 0, Kind::Basic(BasicKind::Bool));
    root_scope.add_constant("false".to_string(), 0, Kind::Basic(BasicKind::Bool));
        

    return root_scope;
}
