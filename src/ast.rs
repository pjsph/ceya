use crate::{scanner::{Token, TokenType}, environment::EnvironmentArena};
use std::{fmt::{Debug, Formatter, Error, Display, Write}, rc::Rc, str::FromStr};
use rand::Rng;

#[derive(Clone)]
pub enum Fun { // TODO: make this an enum with 1 variant with a callee, so we can execute native functions
    Code    { name: String, params: Vec<Rc<Token>>, body: Rc<Stmt>, closure: usize },
    Native  { name: String, params: Vec<Rc<Token>>, callee: Rc<dyn Fn(Vec<Value>) -> Value> }
}

impl Fun {
    fn call(&self, arguments: Vec<Value>, env_arena: &mut EnvironmentArena) -> Value {
        match self {
            Self::Code { name: _, params, body, closure } => {
                let env = env_arena.add(Some(*closure));
                for (i, el) in params.iter().enumerate() {
                    env_arena.define(env, &el.lexeme, arguments.get(i).unwrap().clone());
                }
                if let Some(v) = body.execute(env_arena, env) {
                    return v;
                }
                Value::Null
            },
            Self::Native { name: _, params: _, callee } => {
                (callee)(arguments)
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    // Fun(Fun)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match *self {
            Value::String(ref s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            // Value::Fun(ref fun) => write!(f, "fun {}", match fun { 
            //     Fun::Code { ref name, params: _, body: _, closure: _ } => name,
            //     Fun::Native { ref name, params: _, callee: _ } => name
            //  })
        }
    }
}

pub enum Expr {
   //Assign   { name: Rc<Token>, value: Box<Expr> },
   Binary   { left: Box<Expr>, operator: Rc<Token>, right: Box<Expr> },
   //Logical  { left: Box<Expr>, operator: Rc<Token>, right: Box<Expr> },
   Grouping { expression: Box<Expr> },
   Literal  { value: Value },
   Unary    { operator: Rc<Token>, right: Box<Expr> },
   Soro,
   //Variable { name: Rc<Token> },
   //Call     { callee: Box<Expr>, paren: Rc<Token>, arguments: Vec<Box<Expr>> }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.fmt_output())
    }
}

impl Expr {
    //TODO: compiling errors instead of just returning null
    pub fn evaluate(&self, env_arena: &mut EnvironmentArena, environment: usize) -> Value {
        match self {
            // Self::Assign { name, value } => {
            //     let v = value.evaluate(env_arena, environment);
            //     if let Err(e) = env_arena.assign(environment, name, v.clone()) {
            //         eprintln!("{}", e);
            //     }
            //     v
            // }
            Self::Binary { left, operator, right } => {
                let l = left.evaluate(env_arena, environment);
                let r = right.evaluate(env_arena, environment);

                match operator.typ {
                    TokenType::Minus => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
                         _ => Value::Null
                    },
                    TokenType::Slash => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
                        _ => Value::Null
                    },
                    TokenType::Star => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
                        _ => Value::Null
                    },
                    TokenType::Plus => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                        (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", &a, &b)),
                        (Value::String(a), Value::Number(b)) => Value::String(format!("{}{}", &a, b)),
                        (Value::Number(a), Value::String(b)) => Value::String(format!("{}{}", a, &b)),
                        _ => Value::Null
                    },
                    TokenType::Greater => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Boolean(a > b),
                        _ => Value::Boolean(false)
                    },
                    TokenType::GreaterEqual => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Boolean(a >= b),
                        _ => Value::Boolean(false)
                    },
                    TokenType::Less => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Boolean(a < b),
                        _ => Value::Boolean(false)
                    },
                    TokenType::LessEqual => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Boolean(a <= b),
                        _ => Value::Boolean(false)
                    },
                    TokenType::BangEqual => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Boolean(a != b),
                        (Value::String(a), Value::String(b)) => Value::Boolean(a != b),
                        (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a != b),
                        (Value::Null, Value::Null) => Value::Boolean(!true),
                        _ => Value::Boolean(!false)
                    },
                    TokenType::EqualEqual => match (l, r) {
                        (Value::Number(a), Value::Number(b)) => Value::Boolean(a == b),
                        (Value::String(a), Value::String(b)) => Value::Boolean(a == b),
                        (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a == b),
                        (Value::Null, Value::Null) => Value::Boolean(true),
                        _ => Value::Boolean(false)
                    },
                    _ => Value::Null
                }
            },
            // Self::Logical { left, operator, right } => {
            //     let value = left.evaluate(env_arena, environment);

            //     match operator.typ {
            //         TokenType::Or => {
            //             if left.is_true(env_arena, environment) {
            //                 return value;
            //             }
            //         },
            //         TokenType::And => {
            //             if !left.is_true(env_arena, environment) {
            //                 return value;
            //             }
            //         },
            //         _ => ()
            //     };

            //     right.evaluate(env_arena, environment)
            // }
            Self::Grouping { expression } => {
                expression.evaluate(env_arena, environment)
            },
            Self::Literal { value } => {
                value.clone()
            },
            Self::Unary { operator, right } => {
                let r = right.evaluate(env_arena, environment);

                match operator.typ {
                    TokenType::Minus => match r {
                        Value::Number(n) => Value::Number(-n),
                        _ => Value::Null
                    },
                    TokenType::Bang => Value::Boolean(!match r {
                        Value::Null => false,
                        Value::Boolean(b) => b,
                        _ => true
                    }),
                    _ => Value::Null
                }
            },
            // Self::Variable { name } => {
            //     if let Ok(res) = env_arena.get(environment, name) {
            //         return res.clone();
            //     }

            //     Value::Null
            // },
            // Self::Call { ref callee, paren: _, ref arguments } => {
            //     let call = callee.evaluate(env_arena, environment);

            //     match call {
            //         Value::Fun(ref fun) => {

            //             let mut exe = |fun: &Fun, params: &Vec<Rc<Token>>| -> Value {
            //                 if params.len() != arguments.len() {
            //                     eprintln!("Expected {} arguments, but found {}.", params.len(), arguments.len());
            //                     return Value::Null;
            //                 }
    
            //                 let mut args: Vec<Value> = vec![];
            //                 for arg in arguments {
            //                     args.push(arg.evaluate(env_arena, environment));
            //                 }
            //                 fun.call(args, env_arena)
            //             };

            //             match fun {
            //                 Fun::Code { name: _, params, body: _, closure: _ } => {
            //                     exe(fun, params)
            //                 },
            //                 Fun::Native { name: _, params, callee: _ } => {
            //                     exe(fun, params)
            //                 }
            //             }
                        
            //         },
            //         _ => {
            //             call
            //         }
            //     }
            // },
            Self::Soro => Value::Null
        }
    }

    pub fn compile(&self) -> String {
        let mut res = String::new();

        match self {
            Self::Literal { value } => {
                writeln!(&mut res, "   ; {}", self.fmt_output()).unwrap();

                match value {
                    Value::Null => {
                        writeln!(&mut res, "   ; not implemented yet!").unwrap();
                    },
                    Value::Number(n) => {
                        writeln!(&mut res, "   push {}", n).unwrap();
                    },
                    Value::Boolean(b) => {
                        writeln!(&mut res, "   push {}", if *b { "1" } else { "0" }).unwrap();
                    },
                    Value::String(_) => {
                        writeln!(&mut res, "   ; not implemented yet!\n").unwrap(); // TODO
                    }
                }
            },
            Self::Unary { operator, right } => {
                writeln!(&mut res, "   ; {}", self.fmt_output()).unwrap();

                match operator.typ {
                    TokenType::Minus => {
                        write!(&mut res, "{}", right.compile()).unwrap();
                        writeln!(&mut res, "   pop rax").unwrap();
                        writeln!(&mut res, "   neg rax").unwrap();
                        writeln!(&mut res, "   push rax").unwrap();
                    },
                    TokenType::Bang => {
                        writeln!(&mut res, "not implemented yet!").unwrap();
                    },
                    _ => {
                        writeln!(&mut res, "error").unwrap();
                    }
                }
            },
            Self::Binary { left, operator, right } => {
                write!(&mut res, "{}", left.compile()).unwrap();
                write!(&mut res, "{}", right.compile()).unwrap();

                writeln!(&mut res, "   ; {}", self.fmt_output()).unwrap();

                match operator.typ {
                    TokenType::Plus => {
                        writeln!(&mut res, "   pop rbx").unwrap();
                        writeln!(&mut res, "   pop rax").unwrap();
                        writeln!(&mut res, "   add eax, ebx").unwrap();
                        writeln!(&mut res, "   push rax").unwrap();
                    },
                    TokenType::Star => {
                        writeln!(&mut res, "   pop rbx").unwrap();
                        writeln!(&mut res, "   pop rax").unwrap();
                        writeln!(&mut res, "   imul ebx").unwrap();
                        writeln!(&mut res, "   push rax").unwrap();
                    },
                    TokenType::Minus => {
                        writeln!(&mut res, "   pop rbx").unwrap();
                        writeln!(&mut res, "   pop rax").unwrap();
                        writeln!(&mut res, "   sub eax, ebx").unwrap();
                        writeln!(&mut res, "   push rax").unwrap();
                    },
                    TokenType::Slash => {
                        writeln!(&mut res, "   pop rbx").unwrap();
                        writeln!(&mut res, "   pop rax").unwrap();
                        writeln!(&mut res, "   cqo").unwrap();
                        writeln!(&mut res, "   idiv rbx").unwrap();
                        writeln!(&mut res, "   push rax").unwrap();
                    },
                    TokenType::Less => {
                        writeln!(&mut res, "   pop rbx").unwrap();
                        writeln!(&mut res, "   pop rax").unwrap();
                        writeln!(&mut res, "   cmp rax, rbx").unwrap();
                        writeln!(&mut res, "   setl al").unwrap();
                        writeln!(&mut res, "   movzx rax, al").unwrap();
                        writeln!(&mut res, "   push rax").unwrap();
                    },
                    TokenType::LessEqual => {
                        writeln!(&mut res, "   pop rbx").unwrap();
                        writeln!(&mut res, "   pop rax").unwrap();
                        writeln!(&mut res, "   cmp rax, rbx").unwrap();
                        writeln!(&mut res, "   setle al").unwrap();
                        writeln!(&mut res, "   movzx rax, al").unwrap();
                        writeln!(&mut res, "   push rax").unwrap();
                    },
                    TokenType::Greater => {
                        writeln!(&mut res, "   pop rbx").unwrap();
                        writeln!(&mut res, "   pop rax").unwrap();
                        writeln!(&mut res, "   cmp rax, rbx").unwrap();
                        writeln!(&mut res, "   setg al").unwrap();
                        writeln!(&mut res, "   movzx rax, al").unwrap();
                        writeln!(&mut res, "   push rax").unwrap();
                    },
                    TokenType::GreaterEqual => {
                        writeln!(&mut res, "   pop rbx").unwrap();
                        writeln!(&mut res, "   pop rax").unwrap();
                        writeln!(&mut res, "   cmp rax, rbx").unwrap();
                        writeln!(&mut res, "   setge al").unwrap();
                        writeln!(&mut res, "   movzx rax, al").unwrap();
                        writeln!(&mut res, "   push rax").unwrap();
                    },
                    TokenType::EqualEqual => {
                        writeln!(&mut res, "   pop rbx").unwrap();
                        writeln!(&mut res, "   pop rax").unwrap();
                        writeln!(&mut res, "   cmp rax, rbx").unwrap();
                        writeln!(&mut res, "   sete al").unwrap();
                        writeln!(&mut res, "   movzx rax, al").unwrap();
                        writeln!(&mut res, "   push rax").unwrap();
                    },
                    TokenType::BangEqual => {
                        writeln!(&mut res, "   pop rbx").unwrap();
                        writeln!(&mut res, "   pop rax").unwrap();
                        writeln!(&mut res, "   cmp rax, rbx").unwrap();
                        writeln!(&mut res, "   setne al").unwrap();
                        writeln!(&mut res, "   movzx rax, al").unwrap();
                        writeln!(&mut res, "   push rax").unwrap();
                    },
                    _ => {
                        writeln!(&mut res, "    ; not implemented yet!").unwrap();
                    }
                }
            },
            Expr::Grouping { expression } => {
                writeln!(&mut res, "   ; {}", self.fmt_output()).unwrap();

                write!(&mut res, "{}", expression.compile()).unwrap();
            },
            Expr::Soro => {
                writeln!(&mut res, "   ; {}", self.fmt_output()).unwrap();
                // Do nothing
            }
        }

        res
    }

    fn fmt_output(&self) -> String {
        match self {
            Self::Binary { left, operator, right } => {
                Expr::parenthesize(&operator.lexeme, vec![left, right])
            },
            Self::Grouping { expression } => {
                Expr::parenthesize("group", vec![expression])
            },
            Self::Literal { value } => {
                format!("{}", value)
            },
            // Self::Logical { left, operator, right } => {
            //     Expr::parenthesize(&operator.lexeme, vec![left, right])
            // }
            Self::Unary { operator, right } => {
                Expr::parenthesize(&operator.lexeme, vec![right])
            },
            // Self::Variable { name } => {
            //     format!("{}", &name.lexeme)
            // },
            // Self::Assign { name, value } => {
            //     Expr::parenthesize(&format!("{}=", name.lexeme), vec![value])
            // },
            // Self::Call { callee, paren: _, arguments } => {
            //     let mut args = vec![];
            //     for expr in arguments {
            //         args.push(expr);
            //     }
            //     Expr::parenthesize(&format!("{}()", callee.fmt_output()), args)
            // },
            Self::Soro => {
                String::from_str("soro").unwrap()
            }
        }
    }

    fn parenthesize(name: &str, exprs: Vec<&Box<Expr>>) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.fmt_output());
        }
        builder.push(')');

        builder
    }

    fn is_true(&self, env_arena: &mut EnvironmentArena, environment: usize) -> bool {
        match self.evaluate(env_arena, environment) {
            Value::Boolean(b) => b,
            Value::Null => false,
            Value::String(s) => !s.is_empty(),
            Value::Number(n) => n != 0.,
            // Value::Fun(_fun) => true
        }
    }
}

pub enum Stmt {
    Block       { statements: Vec<Stmt> },
    Expression  { expression: Box<Expr> },
    Print       { expression: Box<Expr> },  
    Faran,
    Ke,
    // Let         { name: Rc<Token>, initializer: Box<Expr> },
    If          { condition: Box<Expr>, then: Box<Stmt>, els: Option<Box<Stmt>> },
    While       { condition: Box<Expr>, body: Box<Stmt> },
    // Fun         { name: Rc<Token>, params: Vec<Rc<Token>>, body: Rc<Stmt> },
    // Return      { keyword: Rc<Token>, value: Box<Expr> }
 }

 impl Stmt {
    pub fn execute(&self, env_arena: &mut EnvironmentArena, environment: usize) -> Option<Value> {
        match *self {
            Stmt::Block { ref statements } => {
                let new_env = env_arena.add(Some(environment));
                for stmt in statements {
                    if let Some(v) = stmt.execute(env_arena, new_env) {
                        return Some(v);
                    }
                }
                None
            }
            Stmt::Expression { ref expression } => { 
                expression.evaluate(env_arena, environment);
                None
            },
            Stmt::Print { ref expression } => {
                let value = expression.evaluate(env_arena, environment);
                println!("{}", value);
                None
            },
            // Stmt::Let { ref name, ref initializer } => {
            //     let value = initializer.evaluate(env_arena, environment);
            //     env_arena.define(environment, &name.lexeme, value);
            //     None
            // },
            Stmt::If { ref condition, ref then, ref els } => {
                if condition.is_true(env_arena, environment) {
                    return then.execute(env_arena, environment);
                } else if let Some(stmt) = els {
                    return stmt.execute(env_arena, environment);
                }
                None
            },
            Stmt::While { ref condition, ref body } => {
                while condition.is_true(env_arena, environment) {
                    return body.execute(env_arena, environment);
                }
                None
            },
            // Stmt::Fun { ref name, ref params, ref body } => {
            //     let fun = Fun::Code { name: String::from_str(&name.lexeme).expect("str expected"), params: params.clone(), body: Rc::clone(body), closure: environment };
            //     env_arena.define(environment, &name.lexeme, Value::Fun(fun));
            //     None
            // },
            // Stmt::Return { keyword: _, ref value } => {
            //     let v = value.evaluate(env_arena, environment);
            //     Some(v)
            // },
            Stmt::Faran => None,
            Stmt::Ke => None
        }
    }

    pub fn compile(&self) -> String {
        match self {
            Stmt::Expression { expression } => {
                expression.compile()
            },
            Stmt::Print { expression } => {
                let mut res = String::new();

                writeln!(&mut res, "   ; print {}", expression.fmt_output()).unwrap();
                write!(&mut res, "{}", expression.compile()).unwrap();

                writeln!(&mut res, "   lea rcx, [msg]").unwrap();
                writeln!(&mut res, "   pop rdx").unwrap();
                writeln!(&mut res, "   call printf").unwrap();
                res
            },
            Stmt::Block { statements } => {
                let mut res = String::new();

                for stmt in statements {
                    write!(&mut res, "{}", stmt.compile()).unwrap();
                }

                res
            },
            Stmt::If { condition, then, els } => {
                let mut res = String::new();

                let mut rng = rand::thread_rng();
                let label = rng.gen_range(100..1000);

                writeln!(&mut res, "   ; if {}", condition.fmt_output()).unwrap();
                write!(&mut res, "{}", condition.compile()).unwrap();
                writeln!(&mut res, "   pop rax").unwrap();
                writeln!(&mut res, "   cmp rax, 1").unwrap();
                match els {
                    Some(e) => {
                        writeln!(&mut res, "   jne .ne_{}", label).unwrap();
                        write!(&mut res, "{}", then.compile()).unwrap();
                        writeln!(&mut res, "   jmp .end_{}", label).unwrap();
                        writeln!(&mut res, ".ne_{}:", label).unwrap();
                        write!(&mut res, "{}", e.compile()).unwrap();
                    },
                    _ => {
                        writeln!(&mut res, "   jne .end_{}", label).unwrap();
                        write!(&mut res, "{}", then.compile()).unwrap();
                    }
                }
                
                writeln!(&mut res, ".end_{}:", label).unwrap();

                res
            },
            Stmt::While { condition, body } => {
                let mut res = String::new();

                let mut rng = rand::thread_rng();
                let label = rng.gen_range(100..1000);

                writeln!(&mut res, "   ; while {}", condition.fmt_output()).unwrap();
                writeln!(&mut res, "   jmp .cond_{}", label).unwrap();
                writeln!(&mut res, ".body_{}:", label).unwrap();
                write!(&mut res, "{}", body.compile()).unwrap();
                writeln!(&mut res, ".cond_{}:", label).unwrap();
                write!(&mut res, "{}", condition.compile()).unwrap();
                writeln!(&mut res, "   pop rax").unwrap();
                writeln!(&mut res, "   cmp rax, 1").unwrap();
                writeln!(&mut res, "   je .body_{}", label).unwrap();

                res
            },
            Stmt::Faran => {
                let mut res = String::new();
                writeln!(&mut res, "   ; faran").unwrap();

                writeln!(&mut res, "   pop rax").unwrap();
                res
            },
            Stmt::Ke => {
                let mut res = String::new();
                writeln!(&mut res, "   ; ke").unwrap();

                writeln!(&mut res, "   pop rax").unwrap();
                writeln!(&mut res, "   push rax").unwrap();
                writeln!(&mut res, "   push rax").unwrap();
                res
            }
        }
    }
 }