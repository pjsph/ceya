use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::rc::Rc;
use std::time::SystemTime;


use ast::{Value, Fun};
use clap::{Parser, Subcommand, command, Args};
use environment::EnvironmentArena;
use scanner::Scanner;

mod scanner;
mod ast;
mod parser;
mod environment;

#[derive(Parser)]
#[command(name = "ceya")]
#[command(author = "pjsph")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Sim(GeneralArgs),
    Com(GeneralArgs)
}

#[derive(Args)]
struct GeneralArgs {
    filepath: String
}

fn error(line: u32, message: &str) -> Error {
    Error::new(ErrorKind::Other, format!("[line {}] Error: {}", line, message))
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Sim(args) => {
            let mut source = String::new();
            File::open("./test.ceya").expect("file expected").read_to_string(&mut source).expect("string expected");
            let scanner = Scanner {
                source,
                tokens: vec![],
                start: 0usize,
                current: 0,
                line: 0            
            };
            let tokens = scanner.scan_tokens();
            //println!("{:?}", tokens);

            let mut env_arena = EnvironmentArena::new();
            let global_env = env_arena.add(None);

            env_arena.define(global_env, "clock", Value::Fun(Fun::Native { name: "clock".into(), callee: Rc::new(|_| Value::Number(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64)), params: vec![] }));

            let mut parser = parser::Parser {
                tokens,
                current: 0usize
            };
            let stmts = parser.parse();
            for stmt in stmts {
                stmt.execute(&mut env_arena, global_env);
            }
        },
        Commands::Com(args) => {
            
        }
    }
}
