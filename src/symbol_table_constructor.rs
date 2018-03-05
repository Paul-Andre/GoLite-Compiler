use ast::*;
use symbol_table::*;
use std::process::exit;
use std::collections::HashMap;

pub fn construct_program_symbol_table(root: &Program) -> *mut SymbolTable {

    let mut root_scope: SymbolTable = Box::new(SymbolTable {
        parent_scope: None,
        children_scopes: Vec::new(),
        symbols: HashMap::new()
    });

    for decl in &root.declarations {
        evaluate_top_level_declaration(decl, &mut *root_scope.SymbolTable);
    }
}

fn populate_root_scope_with_defaults(root_scope: &SymbolTable){


}

fn evaluate_top_level_declaration(decl: &TopLevelDeclarationNode, table: &SymbolTable){
    match decl {
        TopLevelDeclaration::VarDeclarations { ref declarations } => {
            for var_spec in declarations.iter() {
                add_var_declaration_to_table(var_spec, &table)
            }
        },
        TopLevelDeclaration::TypeDeclarations { ref declarations } => {
            for type_spec in declarations.iter() {
                add_type_declaration_to_table(type_spec)
            }
        },
        TopLevelDeclaration::FunctionDeclarations { ref name, ref parameters, ref return_kind, ref body } => {
            evaluate_function_declaration(&name, &parameters, &return_kind, &body, &decl.line_number, &table)
        }
    }
}

fn evaluate_function_declaration(name: &String,
                                 params: &Vec<Field>,
                                 return_kind: &Option<Box<AstKindNode>>,
                                 body: &Vec<StatementNode>,
                                 line: &int,
                                 table: &SymbolTable){
    
    let mut p_vec = Vec::new();

    for p in params.iter() {
        p_vec.push(Definition::Variable(get_type(*p.kind)))
    }

    let mut t: Type;

    match return_kind {
        &Some(ref k) => t = get_type(&*k),
        &None => t = Type::Void
    }

    iterate_through_statements(&body, &table);

    // TODO: swap out scope for symbol table
    let fun_sym = Symbol {
        line_number: line,
        identifier: name,
        definition: Definition::Function {params: p_vec, scope: None, return_type: t}
    };
    
}

fn evaluate_statement(stmt: &StatementNode, table: &SymbolTable) {
    match stmt.Statement {
        Statement::Block( ref vec ) => iterate_through_statements(&vec, &table),
        Statement::VarDeclarations { ref declarations } => {
            for decl in declarations.iter(){
                add_var_declaration_to_table(decl, &table)
            }
        },
        Statement::TypeDeclarations { ref declarations } => {
            for decl in declarations.iter(){
                add_type_declaration_to_table(decl, &table)
            }
        },
        Statement:: ShortVariableDeclaration { ref identifer_list, ref expression_list } => {
            // TODO: For short variable declarations need decide if we want to determine type now or in typecheck
            return
        },
        Statement::If { ref init, ref condition, ref if_branch, ref else_branch } {
            evaluate_statement(&*init, &table);

           iterate_through_statements(if_branch, &table);

            match else_branch {
                &Some( ref s ) => evaluate_statement(&*s, &table),
                &None => return
            }
        },
        Statement::Loop { ref body } => iterate_through_statements(body, &table),
        Statement::While { ref condition, ref body } => iterate_through_statements(&body, &table),
        Statement::For { ref init, ref condition, ref post, ref body } => {
            evaluate_statement(&*init, &table);
            evaluate_statement(&*post, &table);
            iterate_through_statements(&body, &table);
        },
        Statement::Switch {ref init, ref expr, ref body} => {
            evaluate_statement(&*init, &table);
            evaluate_case_clause(&body, &table);
        },
        _ => return
    }
}

fn evaluate_case_clause(clauses: &Vec<CaseClause>, table: &table){
    for clause in clauses.iter(){
        iterate_through_statements(&clause.statements, &table)
    }
}

fn iterate_through_statements(stmts: &Vec<StatementNode>, table: &SymbolTable) {
    for s in stmts.iter() {
        evaluate_statement(s, &table)
    }
}


fn add_var_declaration_to_table(var_spec: &VarSpec, table: &SymbolTable){

    let t: Type;

    match var_spec.kind {
        &Some(ref k) => {
            t = get_type(k)
        },
        &None => {
            // TODO: determine if we want to evaluate RHS or put temporary void type until typechecking
        }
    }

    for var in var_spec.names.iter(){
        let sym = Symbol {
            line_number: var_spec.line_number,
            identifier: var,
            definition: Definition::Variable(t)
        };

        // TODO: Add symbol to symbol table
    }
}

fn get_type(ast_kind_node: &AstKindNode) -> Type{
    match *ast_kind_node.ast_kind {
        AstKind::Identifier { ref name } => {
            match name {
                "int" => return Type::Base(BaseType::Int),
                "float64" => return Type::Base(BaseType::Float),
                "rune" => return Type::Base(BaseType::Rune),
                "string" => return Type::Base(BaseType::String),
                "bool" => return Type::Base(BaseType::Bool)
                //TODO see if the type exists in current defined types
            }
        },
        AstKind::Slice { ref base } => {
            let t = get_type(*base.ast_kind);
            return Type::DataStructure(StructureType::Slice(t))
        },
        AstKind::Array { ref base, ref size } => {
            let t = get_type(*base.ast_kind);
            return Type::DataStructure(StructureType::Array(t))
        },
        AstKind::Struct { ref fields } => {

            let mut vec = Vec::new();

            for f in fields.iter() {
                let t = get_type(*f.ast_kind);

                for id in f.identifiers.iter() {
                    let field = symbol_table::Field{
                        name: id,
                        _type: t
                    };
                    vec.push(&field)
                }
            }
            return Type::DataStructure(StructureType::Struct(vec))
        }
    }
}