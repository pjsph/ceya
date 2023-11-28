use std::fs::File;
use std::io::{Read, Error, ErrorKind, Write, self, BufReader, BufRead};
use std::process::Stdio;


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
    Sim(SimArgs),
    Com(ComArgs)
}

#[derive(Args)]
struct SimArgs {
    filepath: String
}

#[derive(Args)]
struct ComArgs {
    #[arg(short, default_value_t = false, help = "Run the program after compilation")]
    run: bool,
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

            // env_arena.define(global_env, "clock", Value::Fun(Fun::Native { name: "clock".into(), callee: Rc::new(|_| Value::Number(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64)), params: vec![] }));

            let mut parser = parser::Parser {
                tokens,
                current: 0usize
            };
            // let stmts = parser.parse();
            // for stmt in stmts {
            //     stmt.execute(&mut env_arena, global_env);
            // }
        },
        Commands::Com(args) => {
            let mut source = String::new();
            File::open(&args.filepath).expect("File not found.").read_to_string(&mut source).expect("Cannot read file.");

            let scanner = Scanner {
                source,
                tokens: vec![],
                current: 0,
                line: 0,
                start: 0
            };

            print!("Scanning source code... ");
            let tokens = scanner.scan_tokens();
            println!("OK");

            let mut parser = parser::Parser {
                tokens,
                current: 0
            };

            print!("Parsing tokens... ");
            let stmts = parser.parse();
            println!("OK");

            print!("Generating assembly... ");
            let mut file = File::create("output.asm").expect("Cannot create file.");

            writeln!(&mut file, "bits 64").unwrap();
            writeln!(&mut file, "default rel\n").unwrap();
            writeln!(&mut file, "segment .data").unwrap();
            writeln!(&mut file, "   msg db \"%d\", 0xd, 0xa, 0\n").unwrap();
            writeln!(&mut file, "segment .text\n").unwrap();
            writeln!(&mut file, "global main\n").unwrap();
            writeln!(&mut file, "extern ExitProcess").unwrap();
            writeln!(&mut file, "extern printf\n").unwrap();
            writeln!(&mut file, "main:").unwrap();

            for stmt in stmts {
                write!(&mut file, "{}", stmt.compile()).unwrap();
            }

            writeln!(&mut file, "\n   lea rcx, [msg]").unwrap();
            writeln!(&mut file, "   pop rdx").unwrap();
            writeln!(&mut file, "   call printf\n").unwrap();
            writeln!(&mut file, "   xor rcx, rcx").unwrap();
            writeln!(&mut file, "   call ExitProcess").unwrap();

            println!("OK");

            print!("Assembling program... ");
            let output = std::process::Command::new(".\\build.bat")
                                  .arg("release")
                                  .arg("output")
                                  .output().unwrap();
            // io::stdout().write_all(&output.stdout).unwrap();
            if output.status.success() {
                println!("OK");
            } else {
                println!("ERROR!");
            }

            if args.run {
                println!("Running program");
                std::process::Command::new(".\\msbuild\\output.exe")
                                      .stdout(Stdio::inherit())
                                      .output()
                                      .unwrap();
            }
        }
    }
}
