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
            TopLevelDeclaration::VarDeclarations{declarations} => {
                interpret_var_declarations(declarations, env);
            },
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


pub fn env_get_value(env: &Env, s: &str) -> Option<Value> {
    //println!("{:?}", env.entries.borrow().keys());
    if let Some(declaration) = env.entries.borrow().get(s) {
        match declaration {
            Declaration::Variable(v) => {return Some(v.clone());}
            Declaration::Function(_) => {todo!("Haven't implemented taking functions as values");}
        }
    } else if let Some(parent) = env.parent {
        env_get_value(parent, s)
    } else {
        None
    }
}

// TODO: functions are strictly top-level in this subset, so I probably want to
// have a separate hashmap for functions, that is global, instead of one for each scope...
// This code here is strictly because it was easy to copy and paste...
pub fn env_get_function<'a,'b>(env: &'a Env<'a,'b>, s: &str) -> Option<&'b ast::Function> {
    //println!("{:?}", env.entries.borrow().keys());
    if let Some(declaration) = env.entries.borrow().get(s) {
        match declaration {
            Declaration::Variable(_) => {panic!("not a function")},
            Declaration::Function(f) => {return Some(*f);}
        }
    } else if let Some(parent) = env.parent {
        env_get_function(parent, s)
    } else {
        None
    }
}


/*
pub fn env_get_ref<'a,'b>(env: &'a Env<'a,'b>, s: &str) -> Option<RefMut<'a, Value>> {
    //println!("{:?}", env.entries.borrow().keys());

    let mut found = false;
    if let Some(declaration) = env.entries.borrow().get(s) {
        match declaration {
            Declaration::Variable(_) => {

                found = true;

            }
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
*/

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

pub fn env_declare_var(env: &Env, s: &str, v: Value) {
    env.entries.borrow_mut().insert(s.to_string(), Declaration::Variable(v));
}

pub fn env_set_var(env: &Env, s: &str, v: Value) {
    if let Some(declaration) = env.entries.borrow_mut().get_mut(s) {
        match declaration {
            Declaration::Variable(ref mut lv) => {
                *lv = v;
            },
            Declaration::Function(_) => todo!()
        }
    } else if let Some(parent) = env.parent {
        env_set_var(parent, s, v)
    } else {
        panic!("Variable not found");
    }
}

pub fn compute_binary_operation_int(op: BinaryOperator, l: i32, r: i32) -> Value {
    use self::BinaryOperator::*;
    match op {
        Eq => Value::Bool(l == r),
        Neq => Value::Bool(l != r),

        Lt => Value::Bool(l < r),
        Leq => Value::Bool(l <= r),
        Gt => Value::Bool(l > r),
        Geq => Value::Bool(l >= r),

        Add => Value::Int(l.wrapping_add(r)),
        Sub => Value::Int(l.wrapping_sub(r)),
        Mul => Value::Int(l.wrapping_mul(r)),
        Div => Value::Int(l.wrapping_div(r)),

        BwOr => Value::Int(l | r),
        BwXor => Value::Int(l ^ r),
        Mod => Value::Int(l.wrapping_rem(r)),
        BwAnd => Value::Int(l & r),
        BwAndNot => Value::Int(l & (! r)),

        LShift => Value::Int(l<<r),
        RShift => Value::Int(l>>r),

        _ => panic!("Should not have been computing this, bc or/and are shortcircuiting"),
    }
}

pub fn compute_binary_operation_float(op: BinaryOperator, l: f64, r: f64) -> Value {
    use self::BinaryOperator::*;
    match op {
        Eq => Value::Bool(l == r),
        Neq => Value::Bool(l != r),

        Lt => Value::Bool(l < r),
        Leq => Value::Bool(l <= r),
        Gt => Value::Bool(l > r),
        Geq => Value::Bool(l >= r),

        Add => Value::Float(l+r),
        Sub => Value::Float(l-r),
        Mul => Value::Float(l*r),
        Div => Value::Float(l/r),

        _ => panic!("Other operators not supported on floats."),
    }
}

pub fn compute_binary_operation(op: BinaryOperator, lv: Value, rv: Value) -> Value {
    match (lv, rv) {
        (Value::Int(li), Value::Int(ri)) => {
            compute_binary_operation_int(op, li, ri)
        },
        (Value::Float(li), Value::Float(ri)) => {
            compute_binary_operation_float(op, li, ri)
        },
        _ => todo!(),
    }
}

pub fn interpret_expression(expression_node: &ExpressionNode, env: & Env) -> Value {
    match &expression_node.expression {
        Expression::RawLiteral{value} => {value::parse_with_kind(&value, &expression_node.kind)}
        Expression::BinaryOperation { op, lhs, rhs } => {
            if let BinaryOperator::Or = op {
                //special case, short circuiting
                let l_ref = interpret_reference_expr(lhs, env);
                let lv = l_ref.get_value(env);

                if lv.get_boolean().unwrap() {
                    lv
                } else {
                    let r_ref = interpret_reference_expr(rhs, env);
                    let rv = r_ref.get_value(env);
                    rv
                }

            } else if let BinaryOperator::And = op {
                //special case, short circuiting
                let l_ref = interpret_reference_expr(lhs, env);
                let lv = l_ref.get_value(env);

                if lv.get_boolean().unwrap() {
                    let r_ref = interpret_reference_expr(rhs, env);
                    let rv = r_ref.get_value(env);
                    rv
                } else {
                    lv
                }

            } else {
                let l_ref = interpret_reference_expr(lhs, env);
                let r_ref = interpret_reference_expr(rhs, env);
                let lv = l_ref.get_value(env);
                let rv = r_ref.get_value(env);
                compute_binary_operation(*op, lv, rv)
            }

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
        Expression::Identifier{..} |
        Expression::Index { .. } |
        Expression::Selector { .. } => {
            let r = interpret_reference_expr(expression_node, env);
            r.get_value(env)
        }
        Expression::FunctionCall { primary, arguments } => {
            let f;
            match &primary.expression {
                Expression::Identifier{name, ..} => {
                    f = env_get_function(env, name).unwrap();
                },
                _ => todo!()
            }

            let refs: Vec<Reference> = arguments.iter().map(|arg| {
                interpret_reference_expr(arg, env)
            }).collect();

            let evaled_args: Vec<Value> = refs.into_iter().map(|r| {
                r.get_value(env)
            }).collect();

            interpret_function(f, env, evaled_args.into())
        }
        Expression::Append { lhs, rhs } => {
            let l_ref = interpret_reference_expr(lhs, env);
            let r_ref = interpret_reference_expr(rhs, env);
            let lv = l_ref.get_value(env);
            let rv = r_ref.get_value(env);
            builtins::append(lv,rv)
        }
        Expression::TypeCast {expr, ..} => {
            let kind = &expression_node.kind;
            let v = interpret_expression(expr, env);
            builtins::cast(kind, &v)
        }
    }
}


pub fn interpret_var_declarations(declarations: &[VarSpec], env: &Env) {
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
}

// Different "signals" that may be returned from a statement to control the program flow
#[derive(Debug)]
pub enum Signal {
    None,
    Return(Value),
    Break,
    Continue,
}

impl Signal {
    fn is_none(&self) -> bool {
        match self {
            Signal::None => true,
            _ => false,
        }
    }
}

pub enum ReferenceBase {
    Identifier(String),
    Value(Value),
    Underscore,
}

pub enum ReferenceModifier {
    Selector(String),
    Index(i32),
}

pub struct Reference {
    base: ReferenceBase,
    modifier_stack: Vec<ReferenceModifier>,
}

fn get_reference_value(base: &Value, modifier_stack: &[ReferenceModifier]) -> Value {
    if modifier_stack.len() == 0 {
        return base.clone();
    }
    match (base, &modifier_stack[0]) {
        (Value::Array(ref array), ReferenceModifier::Index(ii))  => {
            let i = *ii;
            // TODO: display the correct line number
            check_bounds(i, array.len(), 0);
            get_reference_value(& array[i as usize], &modifier_stack[1..])
        }
        (Value::Slice(ref slice), ReferenceModifier::Index(ii))  => {
            let i = *ii;
            // TODO: display the correct line number
            check_bounds(i, slice.length, 0);
            get_reference_value(& *slice.contents[i as usize].borrow(), &modifier_stack[1..])
        }
        (_, ReferenceModifier::Index(s))  => panic!("Trying to index something that isn't an array or slice."),
        (Value::Struct(ref hm), ReferenceModifier::Selector(s))  =>  {
            get_reference_value(hm.get(s).unwrap(), &modifier_stack[1..])
        }
        (_, ReferenceModifier::Selector(s))  => panic!("Trying to get a field of something that isn't a struct."),
        (_,_) => panic!("Invalid reference expression.")
    }
}

fn set_reference_value(base: &mut Value, modifier_stack: &[ReferenceModifier], value: Value) {
    if modifier_stack.len() == 0 {
        *base = value;
        return;
    }
    match (base, &modifier_stack[0]) {
        (Value::Array(ref mut array), ReferenceModifier::Index(ii))  => {
            let i = *ii;
            // TODO: display the correct line number
            check_bounds(i, array.len(), 0);
            set_reference_value(&mut array[i as usize], &modifier_stack[1..], value)
        }
        (Value::Slice(ref mut slice), ReferenceModifier::Index(ii))  => {
            let i = *ii;
            // TODO: display the correct line number
            check_bounds(i, slice.length, 0);
            set_reference_value(&mut *slice.contents[i as usize].borrow_mut(), &modifier_stack[1..], value)
        }
        (_, ReferenceModifier::Index(s))  => panic!("Trying to index something that isn't an array or slice."),
        (Value::Struct(ref mut hm), ReferenceModifier::Selector(s))  =>  {
            set_reference_value(hm.get_mut(s).unwrap(), &modifier_stack[1..], value)
        }
        (_, ReferenceModifier::Selector(s))  => panic!("Trying to get a field of something that isn't a struct."),
        (_,_) => panic!("Invalid reference expression.")
    }
}


impl Reference {
    pub fn get_value(&self, env: &Env) -> Value {
        let modifier_stack = &self.modifier_stack;
        match &self.base {
            ReferenceBase::Identifier(ref s) => {
                env_get_reference_value(env, s, modifier_stack).unwrap()
            },
            ReferenceBase::Value(base) => {
                get_reference_value(&base, modifier_stack)
            },
            ReferenceBase::Underscore => {
                panic!("Cannot get value from underscore.");
            },
        }
    }
    pub fn set_value(&self, env: &Env, value: Value) {
        let modifier_stack = &self.modifier_stack;
        match &self.base {
            ReferenceBase::Identifier(ref s) => {
                env_set_reference_value(env, s, modifier_stack, value);
            },
            ReferenceBase::Value(base) => {
                let mut copy = base.clone();
                set_reference_value(&mut copy, modifier_stack, value);
            }
            ReferenceBase::Underscore => {
                if modifier_stack.len() == 0 {
                    // Do nothing.
                }
                else {
                    panic!("Cannot do stuff with underscore");
                }

            },
        }
    }
}

pub fn env_set_reference_value(env: &Env, ident: &str, modifier_stack: &[ReferenceModifier], value: Value) {
    //println!("{:?}", env.entries.borrow().keys());
    if let Some(ref mut declaration) = env.entries.borrow_mut().get_mut(ident) {
        match declaration {
            Declaration::Variable(ref mut v) => {
                set_reference_value(v, modifier_stack, value);
            }
            Declaration::Function(_) => {todo!("Haven't implemented taking functions as values");}
        }
    } else if let Some(parent) = env.parent {
        env_set_reference_value(parent, ident, modifier_stack, value)
    } else {
        panic!("Variable does not exist in scope");
    }
}

pub fn env_get_reference_value(env: &Env, ident: &str, modifier_stack: &[ReferenceModifier]) -> Option<Value> {
    if let Some(ref declaration) = env.entries.borrow().get(ident) {
        match declaration {
            Declaration::Variable(ref v) => {
                Some(get_reference_value(v, modifier_stack))
            }
            Declaration::Function(_) => {todo!("Haven't implemented taking functions as values");}
        }
    } else if let Some(parent) = env.parent {
        env_get_reference_value(parent, ident, modifier_stack)
    } else {
        None
    }
}

pub fn interpret_reference_expr(expr: &ExpressionNode, env: &Env) -> Reference {
    match expr.expression {
        Expression::Identifier{ref name, ..} => {
            if name == "_" {
                Reference {
                    base: ReferenceBase::Underscore,
                    modifier_stack: Vec::new(),
                }
            } else {
                Reference {
                    base: ReferenceBase::Identifier(name.clone()),
                    modifier_stack: Vec::new(),
                }
            }
        },
        Expression::Index{ref primary,ref  index} => {
            let mut reference = interpret_reference_expr(&primary, env);
            let i = interpret_expression(&*index, env).get_integer().unwrap();
            reference.modifier_stack.push(ReferenceModifier::Index(i));
            reference
        }
        Expression::Selector{ref primary,ref name} => {
            let mut reference = interpret_reference_expr(&primary, env);
            reference.modifier_stack.push(ReferenceModifier::Selector(name.clone()));
            reference
        },
        _ => {
            Reference {
                base: ReferenceBase::Value(interpret_expression(expr, env)),
                modifier_stack: Vec::new(),
            }
        }
    }
}

#[must_use]
pub fn interpret_statement(statement: &Statement, env: & Env) -> Signal {
    match statement {
        Statement::Empty => {
            return Signal::None;
        },
        Statement::Block(statement_node_vec) => {
            let mut block_env = create_child_env(env);
            for sn in statement_node_vec {
                let s = interpret_statement(&sn.statement, &block_env);
                if (!s.is_none()) {
                    return s;
                }
            }
            return Signal::None;
        },
        Statement::Expression(expression_node) => {
            interpret_expression(&expression_node, env);
            return Signal::None;
        },
        Statement::Assignment{lhs, rhs} => {

            let mut references: Vec<Reference> = Vec::new();
            for le in lhs {
                let l_ref = interpret_reference_expr(le, env);
                references.push(l_ref);
            }

            let mut values: Vec<Value> = Vec::new();
            for re in rhs {
                let r_val = interpret_expression(re, env);
                values.push(r_val);
            }
            assert!(references.len() == values.len());

            for (l_ref, r_val) in references.into_iter().zip(values.into_iter()) {
                l_ref.set_value(env, r_val);
            }
            return Signal::None;
        },
        Statement::OpAssignment{lhs, rhs, operator} => {
            let mut l_ref = interpret_reference_expr(lhs, env);
            let rval = interpret_expression(rhs, env);

            let lval = l_ref.get_value(env);

            let result = compute_binary_operation(*operator, lval, rval);

            l_ref.set_value(env, result);
            return Signal::None;
        },
        Statement::VarDeclarations{declarations} => {
            interpret_var_declarations(declarations, env);
            return Signal::None;
        },
        Statement::TypeDeclarations{declarations} => {
            // nothing, we completely erase all types
            return Signal::None;
        },
        Statement::ShortVariableDeclaration{identifier_list, expression_list, is_assigning} => {
            let mut temp: Vec<Value> = Vec::new();
            for ee in expression_list {
                let ev = interpret_expression(ee, env);
                temp.push(ev);
            }
            assert!(identifier_list.len() == temp.len());
            assert!(is_assigning.len() == temp.len());
            for (i,ev) in temp.into_iter().enumerate() {
                let name = &identifier_list[i];
                if is_assigning[i] {

                    env_set_var(&env, name, ev);
                } else {
                    env_declare_var(&env, name, ev);
                }
            }
            return Signal::None;
        },
        Statement::IncDec{is_dec, expr} => {
            let is_dec = *is_dec;
            let r = interpret_reference_expr(expr, env);
            let v = r.get_value(env);

            let new_v =
            match v {
                Value::Int(i) => {
                    Value::Int(
                        if is_dec {i-1} else {i+1}
                    )
                },
                Value::Float(f) =>  {
                    Value::Float(
                        if is_dec {f-1.} else {f+1.}
                    )
                },
                _ => panic!("Shouldn't inc/dec this"),
            };

            // TODO: create some kind of Reference::transform_value, that takes a function
            r.set_value(env, new_v);
            return Signal::None;
        },
        Statement::Print{exprs} => {
            let refs: Vec<Reference> = exprs.iter().map(|expr| {
                interpret_reference_expr(expr, env)
            }).collect();

            for v_ref in refs {
                let value = v_ref.get_value(env);
                print!("{}",value);
            }
            return Signal::None;
        },
        Statement::Println{exprs} => {
            let len = exprs.len();

            let refs: Vec<Reference> = exprs.iter().map(|expr| {
                interpret_reference_expr(expr, env)
            }).collect();

            for (i,refv) in refs.into_iter().enumerate() {
                let value = refv.get_value(env);
                print!("{}",value);
                if i<len-1 {
                    print!(" ");
                }
            }
            print!("\n");
            return Signal::None;
        },
        Statement::If{init, condition, if_branch, else_branch} => {
            let is = interpret_statement(&init.statement, env);
            if (!is.is_none()) {
                return is;
            }

            let cv = interpret_expression(condition, env);
            if let Value::Bool(b) = cv {
                if b {
                    let new_env = create_child_env(env);
                    for sn in if_branch {
                        let s = interpret_statement(&sn.statement, &new_env);
                        if (!s.is_none()) {
                            return s;
                        }
                    }
                    return Signal::None;
                } else if let Some(s) = else_branch{
                    return interpret_statement(&s.statement, env);
                } else {
                    // condition was false and there is no else branch
                    return Signal::None;
                }

            } else {
                panic!("Condition passed to if statement is not a boolean type.");
            }
        },
        Statement::For{init, condition, post, body} => {
            let is = interpret_statement(&init.statement, env);
            if (!is.is_none()) {
                return is;
            }
            'external: loop {
                let new_env = create_child_env(env);

                let looping =
                if let Some(cond) = condition {
                    let cv = interpret_expression(cond, env);
                    if let Value::Bool(b) = cv {
                        b
                    } else {
                        panic!("Condition passed to if statement is not a boolean type.");
                    }
                } else {
                    true
                };

                if looping {
                    for sn in body {
                        let s = interpret_statement(&sn.statement, &new_env);
                        match (s) {
                            Signal::None => {},
                            Signal::Return(_) => {
                                return s;
                            },
                            Signal::Break => {
                                break 'external;
                            },
                            Signal::Continue => {
                                continue 'external;
                            },
                        }
                    }
                    let ps = interpret_statement(&post.statement, &env);
                    if (!ps.is_none()) {
                        return ps;
                    }
                } else {
                    break;
                }
            }
            return Signal::None;
        },     
        Statement::Switch{init, expr, body} => {
            todo!();
        },

        Statement::Break => {
            return Signal::Break;
        },

        Statement::Continue => {
            return Signal::Continue;
        },

        Statement::Return(opt_expr) => {
            if let Some(expr) = opt_expr {
                let v = interpret_expression(&*expr, env);
                return Signal::Return(v);
            } else {
                return Signal::Return(Value::Void);
            }
        },
    }

}

pub fn create_child_env<'a,'b>(env: &'a Env<'a,'b>) -> Env<'a,'b> {
    return Env{
        parent: Some(env),
        entries: RefCell::new(HashMap::new()),
    };
}

pub fn interpret_function<'a,'b>(f: &ast::Function, tl_env: &'a Env<'a,'b>, args: Box<[Value]>) -> Value {
    let mut env = create_child_env(tl_env);

    let mut flattened_parameters = Vec::new();
    for field in &f.parameters {
        for ident in &field.identifiers {
            flattened_parameters.push(ident.clone());
        }
    }

    assert!(args.len() == flattened_parameters.len());
    let args_huh: Vec<Value> = args.into(); // WHY? is it a bug that I need to do this?
    for (arg, param) in args_huh.into_iter().zip(flattened_parameters.into_iter()) {
        // Why is arg a &Value and not a Value when I use into_iter directly on the Box<[Value]>??????
        env.entries.borrow_mut().insert(param, Declaration::Variable(arg));
    }

    for statement_node in &f.body {
        let s = interpret_statement(&statement_node.statement,&mut env);
        match s {
            Signal::None => {},
            Signal::Return(v) => {return v},
            Signal::Continue | Signal::Break
                => {panic!("continue or break outside of loop")},
        }
    }
    return Value::Void;
}


pub fn interpret<'b>(root: &'b Program){
    let mut env = Env {parent:None, entries:RefCell::new(HashMap::new())};
    {
        env.entries.borrow_mut().insert("true".to_string(),Declaration::Variable(Value::Bool(true)));
        env.entries.borrow_mut().insert("false".to_string(),Declaration::Variable(Value::Bool(false)));
    }

    let init_functions = init_top_level(&env, root);

    for f in init_functions.iter() {
        let empty_args: Box<[Value]> = Vec::new().into();
        interpret_function(f, &mut env, empty_args);
    }

    let main = 
    if let Some(Declaration::Function(ref f)) = env.entries.borrow().get("main") {
        *f
    } else {
        panic!("no main function");
    };

    let empty_args: Box<[Value]> = Vec::new().into();
    interpret_function(main, &env, empty_args);


}
