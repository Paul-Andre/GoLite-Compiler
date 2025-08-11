use std::fmt::Write;
use util::*;
use ast;
use ast::*;
use kind::*;
use std::fs::File;
use std::io::prelude::*;
use value;
use value::Value;
use value::builtins;
use std::collections::HashMap;
use std::cell::RefCell;
use std::cell::RefMut;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug)]
pub enum Declaration<'b> {
    Variable(Value),
    Type(Kind),
    Function(&'b ast::Function),
}

#[derive(Debug)]
pub struct Env<'a,'b> {
    parent: Option<&'a Env<'a,'b>>,
    entries: RefCell<HashMap<String, Declaration<'b> >>
}


// Adds appropriate entries to env, and returns a list of init functions
fn init_top_level<'a,'b>(env: &Env<'a, 'b>, root: &'b Program) -> Box<[&'b ast::Function]> {
    let mut init_functions = Vec::<&'b ast::Function>::new();
    for decl in &root.declarations {
        match &decl.top_level_declaration {
            TopLevelDeclaration::VarDeclarations{declarations} => {},
            TopLevelDeclaration::TypeDeclarations{declarations} => {},
            TopLevelDeclaration::FunctionDeclaration(function) => {
                let name = &function.name;
                if name == "init" {
                    init_functions.push(function);
                } else {
                    env.entries.borrow_mut().insert(function.name.clone(), Declaration::Function(function));
                }
            }
        }
    }
    return init_functions.into();
}


pub fn env_get(env: &Env, s: &str) -> Option<Value> {
    //println!("{:?}", env.entries.borrow().keys());
    if let Some(declaration) = env.entries.borrow().get(s) {
        match declaration {
            Declaration::Variable(v) => Some(v.clone()),
            Declaration::Type(_) => todo!(),
            Declaration::Function(_) => todo!()
        }
    } else if let Some(parent) = env.parent {
        env_get(parent, s)
    } else {
        None
    }
}

pub fn env_get_ref<'a,'b>(env: &'a Env<'a,'b>, s: &str) -> Option<RefMut<'a, Value>> {
    //println!("{:?}", env.entries.borrow().keys());

    let mut found = false;
    if let Some(declaration) = env.entries.borrow().get(s) {
        match declaration {
            Declaration::Variable(_) => {

                found = true;

            }
            Declaration::Type(_) => todo!(),
            Declaration::Function(_) => todo!()
        }
    }

    if (found) {
        Some( RefMut::map(env.entries.borrow_mut(), |hm| {
            if let Some(declaration) = hm.get_mut(s) {
                if let Declaration::Variable(v) = declaration {
                    return v;
                }
            } 
            panic!("it was here just before!");
        }))
    }else if let Some(parent) = env.parent {
        env_get_ref(parent, s)
    } else {
        None
    }
}

pub fn check_bounds(a: i32, length: usize, line_number: u32) {
    if (a < 0) {
        eprintln!("Error: line {}: trying to index an array or slice with negative number.", line_number);
        std::process::exit(1);
    }
    if (a as usize >= length) {
        eprintln!("Error: line {}: index {} out of range. Should be in range of {}", line_number, a, length);
        std::process::exit(1);
    }
}

pub fn slice_get_index_ref<'a>(s: &'a value::Slice, i: i32) -> RefMut<'a, Value> {
    s.contents[i as usize].borrow_mut()
}

pub fn array_get_index_ref(a: &mut[Value], i: i32) -> &mut Value {
    &mut a[i as usize]
}


pub fn env_declare_var(env: &Env, s: &str, v: Value) {
    env.entries.borrow_mut().insert(s.to_string(), Declaration::Variable(v));
}

/*
pub fn env_set_var(env: &Env, s: &str, v: Value) {
    env.entries.borrow_mut().insert(s.to_string(), Declaration::Variable(v));
    if let Some(declaration) = env.entries.borrow().get(s) {
        match declaration {
            Declaration::Variable(_) => {
                //env.entries.borrow_mut().insert(s.to_string(), Declaration::Variable(v));
                env.entries.borrow_mut()[s] = Declaration::Variable(v);
            },
            Declaration::Type(_) => todo!(),
            Declaration::Function(_) => todo!()
        }
    } else if let Some(parent) = env.parent {
        env_set_var(parent, s, v)
    } else {
        panic!("Variable not found");
    }
}
*/

pub fn interpret_expression(expression_node: &ExpressionNode, env: & Env) -> Value {
    match &expression_node.expression {
        Expression::Identifier{name, ..} => {
            //println!("{}", name);
            //println!("{:?}", env);
            let v = env_get(env, name).unwrap();
            return v;
        },
        Expression::RawLiteral{value} => {value::parse_with_kind(&value, &expression_node.kind)}
        Expression::BinaryOperation { op, lhs, rhs } => {
            todo!();
        }
        Expression::UnaryOperation { op, rhs } => {
            let rv = interpret_expression(rhs, env);
            match op {
                UnaryOperator::Plus => builtins::plus(&rv),
                UnaryOperator::Neg => builtins::neg(&rv),
                UnaryOperator::BwCompl => builtins::bw_compl(&rv),
                UnaryOperator::Not => builtins::not(&rv),
            }
        }
        Expression::Index { primary, index } => {
            let mut pv = interpret_expression(primary, env);
            let iv = interpret_expression(index, env);
            match pv {
                Value::Array(ref mut array) => {
                    match iv {
                        Value::Int(i) => {
                            check_bounds(i, array.len(), expression_node.line_number);
                            array_get_index_ref(&mut *array, i).clone()
                        },
                        _ => {
                            panic!("Non integer as index")
                        }
                    }

                }, 
                Value::Slice(ref mut slice) => {
                    match iv {
                        Value::Int(i) => {
                            check_bounds(i, slice.length, expression_node.line_number);
                            slice_get_index_ref(slice, i).clone()
                        },
                        _ => {
                            panic!("Non integer as index")
                        }
                    }
                }, 
                _ => {
                    panic!("Non array or slice being indexed")

                }
            }
        }
        Expression::Selector { primary, name } => {
            todo!()
        }
        Expression::FunctionCall { primary, arguments } => {
            todo!()
        }
        Expression::Append { lhs, rhs } => {
            let lv = interpret_expression(lhs, env);
            let rv = interpret_expression(rhs, env);
            builtins::append(lv,rv)
        }
        Expression::TypeCast { name, expr } => {
            todo!()
        }
    }
}

pub fn interpret_statement(statement: &Statement, env: & Env) {
    match statement {
        Statement::Empty => {},
        Statement::Block(statement_node_vec) => {
            let mut block_env = create_child_env(env);
            for sn in statement_node_vec {
                interpret_statement(&sn.statement, &block_env);
            }
        },
        Statement::Expression(expression_node) => {
            interpret_expression(&expression_node, env);
        },
        
        Statement::Assignment{lhs, rhs} => {
            for (i,le) in lhs.iter().enumerate() {
                let re = &rhs[i];
                match &le.expression {
                    Expression::Identifier{ref name, ..} => {
                        let rv = interpret_expression(re, env);
                        let mut l_ref = env_get_ref(env, &name).unwrap();
                        *l_ref = rv;
                    },
                    Expression::Index{primary, index} => {
                        todo!();
                    }
                    Expression::Selector{primary, name} => {
                        todo!();
                    },
                    _ => {
                        panic!("Invalid lhs of an assignment");
                    },
                }
            }
        },
        Statement::VarDeclarations{declarations} => {
            //let new_env = create_child_env(env);
            for var_spec in declarations {
                for (i,name) in var_spec.names.iter().enumerate() {
                    let rv;
                    if let Some(ref exprs) = var_spec.rhs {
                        rv = interpret_expression(&exprs[i], env);
                    } else {
                        rv = value::zero_value(&var_spec.evaluated_kind);
                    }
                    env_declare_var(&env, name, rv);
                }
            }
        },
        Statement::Print{exprs} => {
            for expression_node in exprs {
                let value = interpret_expression(&expression_node, env);
                print!("{}",value);
            }
        },
        Statement::Println{exprs} => {
            let len = exprs.len();
            for (i,expression_node) in exprs.iter().enumerate() {
                let value = interpret_expression(expression_node, env);
                print!("{}",value);
                if i<len-1 {
                    print!(" ");
                }
            }
            print!("\n");
        },
        _ => todo!(),

    }

}

pub fn create_child_env<'a,'b>(env: &'a Env<'a,'b>) -> Env<'a,'b> {
    return Env{
        parent: Some(env),
        entries: RefCell::new(HashMap::new()),
    };
}

pub fn interpret_function<'a,'b>(f: &Function, tl_env: &'a Env<'a,'b>, args: &[Value]) {
    let mut env = create_child_env(tl_env);

    // TODO: Add params:args to env
    assert!(args.len() == 0);

    for statement_node in &f.body {
        interpret_statement(&statement_node.statement,&mut env);
    }

}


pub fn interpret<'b>(root: &'b Program){
    let mut env = Env {parent:None, entries:RefCell::new(HashMap::new())};
    let init_functions = init_top_level(&env, root);
    {
        env.entries.borrow_mut().insert("true".to_string(),Declaration::Variable(Value::Bool(true)));
        env.entries.borrow_mut().insert("false".to_string(),Declaration::Variable(Value::Bool(false)));

    }

    let empty_args: &[Value] = &[];

    for f in init_functions.iter() {
        interpret_function(f, &mut env, empty_args);
    }

    let main_e = &env.entries.borrow()["main"];
    let main =  {
    if let Declaration::Function(f) = main_e {
        f
    } else {
        panic!("no main function");
    }

    };

    interpret_function(main, &env, empty_args);

}
