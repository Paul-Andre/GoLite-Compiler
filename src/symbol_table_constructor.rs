use symbol_table;
use ast;
use ast::*;
use symbol_table::*;
use std::collections::HashMap;

/*
 SYMBOL TABLE CONSTRUCTOR
 ========================================= */

/// Main method for constructing a symbol table from an inputted AST

/// Populates the symbol table with the Go defaul variables and types
pub fn populate_root_scope_with_defaults<'a>(root_scope: & mut SymbolTable<'a>){
    symbol_table::add_variable_symbol(String::from("true"), Definition::Variable(Type::Base(BaseType::Bool)), root_scope);
    symbol_table::add_variable_symbol(String::from("false"), Definition::Variable(Type::Base(BaseType::Bool)), root_scope);
    symbol_table::add_type_symbol(String::from("int"), Definition::Type(Type::Base(BaseType::Int)), root_scope);
    symbol_table::add_type_symbol(String::from("float64"), Definition::Type(Type::Base(BaseType::Float)), root_scope);
    symbol_table::add_type_symbol(String::from("rune"), Definition::Type(Type::Base(BaseType::Rune)), root_scope);
    symbol_table::add_type_symbol(String::from("bool"), Definition::Type(Type::Base(BaseType::Bool)), root_scope);
    symbol_table::add_type_symbol(String::from("string"), Definition::Type(Type::Base(BaseType::String)), root_scope);
}

/*
 EVALUATION METHODS
 ========================================= */

/// Evaluates a top level declaration in order to add the definition to the symbol table
fn evaluate_top_level_declaration<'a>(decl: &TopLevelDeclarationNode, table: &'a mut SymbolTable<'a>){
    match decl.top_level_declaration {
        TopLevelDeclaration::VarDeclarations { ref declarations } => {
            for var_spec in declarations.iter() {
                add_var_declaration_to_table(var_spec, table)
            }
        },
        TopLevelDeclaration::TypeDeclarations { ref declarations } => {
            for type_spec in declarations.iter() {
                add_type_declaration_to_table(type_spec, &table)
            }
        },
        TopLevelDeclaration::FunctionDeclaration { ref name, ref parameters, ref return_kind, ref body } => {
            evaluate_function_declaration(&name, &parameters, &return_kind, &body, &decl.line_number, table)
        }
    }
}

/// Evaluates a function declaration by adding the identifier + params + return kind to
/// function table and then evaluating the inner scope
fn evaluate_function_declaration<'a>(name: &String,
                                 params: &Vec<ast::Field>,
                                 return_kind: &Option<Box<AstKindNode>>,
                                 body: &Vec<StatementNode>,
                                 line: &u32,
                                 table: &'a mut SymbolTable<'a>){
    
    let mut p_vec = Vec::new();

    for p in params.iter() {
        p_vec.push(Definition::Variable(evaluate_type(&*p.kind)))
    }

    let t: Type;

    match return_kind {
        &Some(ref k) => t = evaluate_type(&*k),
        &None => t = Type::Void
    }

    iterate_through_statements(&body, table);

    // TODO: swap out scope for symbol table
//    let fun_sym = Symbol {
//        line_number: line,
//        identifier: name,
//        definition: Definition::Function {params: p_vec, scope: None, return_type: t}
//    };

    // TODO: add symbol to symbol table
    
}

fn evaluate_statement<'a>(stmt: &StatementNode, table: & mut SymbolTable<'a>) {
    match stmt.statement {
        Statement::Block( ref vec ) => iterate_through_statements(&vec, table),
        Statement::VarDeclarations { ref declarations } => {
            for decl in declarations.iter(){
                add_var_declaration_to_table(decl, table)
            }
        },
        Statement::TypeDeclarations { ref declarations } => {
            for decl in declarations.iter(){
                add_type_declaration_to_table(decl, table)
            }
        },
        Statement:: ShortVariableDeclaration { ref identifier_list, ref expression_list } => {
            // TODO: For short variable declarations need decide if we want to determine type now or in typecheck
            return
        },
        Statement::If { ref init, ref condition, ref if_branch, ref else_branch } => {
            evaluate_statement(&*init, table);

            iterate_through_statements(if_branch, table);

            match else_branch {
                &Some( ref s ) => evaluate_statement(&*s, table),
                &None => return
            }
        },
        Statement::Loop { ref body } => iterate_through_statements(body, table),
        Statement::While { ref condition, ref body } => iterate_through_statements(&body, table),
        Statement::For { ref init, ref condition, ref post, ref body } => {
            evaluate_statement(&*init, table);
            evaluate_statement(&*post, table);
            iterate_through_statements(&body, table);
        },
        Statement::Switch {ref init, ref expr, ref body} => {
            evaluate_statement(&*init, table);
            evaluate_case_clause(&body, table);
        },
        _ => return
    }
}

fn evaluate_case_clause<'a>(clauses: &Vec<CaseClause>, table: & mut SymbolTable<'a>){
    for clause in clauses.iter(){
        iterate_through_statements(&clause.statements, table)
    }
}

fn iterate_through_statements<'a>(stmts: &Vec<StatementNode>, table: & mut SymbolTable<'a>) {
    for s in stmts.iter() {
        evaluate_statement(s, table)
    }
}


fn add_var_declaration_to_table<'a>(var_spec: &VarSpec, table: & mut SymbolTable<'a>){

    let t: Type;

    match var_spec.kind {
        Some(ref k) => {
            t = evaluate_type(k)
        },
        None => {
            // TODO: determine if we want to evaluate RHS or put temporary void type until typechecking
//            t = evaluate_type(k)
            return
        }
    }

    for var in var_spec.names.iter(){
        symbol_table::add_variable_symbol(var.clone(), Definition::Variable(t.clone()), table)
    }
}

fn add_type_declaration_to_table(type_sepc: &TypeSpec, table: &SymbolTable){

}

fn evaluate_type(ast_kind_node: &AstKindNode) -> Type{
    match ast_kind_node.ast_kind {
        AstKind::Identifier { ref name } => {
            // TODO: add look up and replace the following
            return Type::DataStructure(Box::new(StructureType::Slice(Type::Base(BaseType::Bool))))
        },
        AstKind::Slice { ref base } => {
            let t = evaluate_type(&base);
            return Type::DataStructure(Box::new(StructureType::Slice(t)))
        },
        AstKind::Array { ref base, ref size } => {
            let t = evaluate_type(&base);
            return Type::DataStructure(Box::new(StructureType::Array(t)))
        },
        AstKind::Struct { ref fields } => {

            let mut vec: Vec<symbol_table::Field> = Vec::new();

            for f in fields.iter() {
                let t = evaluate_type(&*f.kind);

                for id in f.identifiers.iter() {
                    let field = symbol_table::Field{
                        name: id.clone(),
                        _type: t.clone()
                    };
                    vec.push(field)
                }
            }
            return Type::DataStructure(Box::new(StructureType::Struct(vec)))
        }
    }
}
