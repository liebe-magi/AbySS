use abyss_lang::{
    env::{Environment, SymbolTable},
    eval::{evaluate, EvalResult},
    parser::{build_ast, parse, Rule},
};
use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "abyss")]
#[command(about = "AbySS: Advanced-scripting by Symbolic Syntax", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a .aby script file
    Invoke {
        /// The path to the script file
        script: String,
    },
    /// Start the interactive interpreter
    Cast {
        /// Enable debug mode
        #[arg(long)]
        debug: bool,
    },
}

fn execute_script(script: &str) {
    // 環境を初期化
    let mut st = SymbolTable::new();
    let mut env = Environment::new();

    // スクリプトをパースして評価
    match parse(script) {
        Ok(pair) => {
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() != Rule::EOI {
                    let ast = build_ast(inner_pair, &mut st);
                    match evaluate(&ast, &mut env) {
                        Ok(_) => {}
                        Err(e) => panic!("Error: {}", e),
                    }
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn start_interpreter(debug: bool) {
    println!("Starting AbySS interpreter...");
    println!("Type 'exit' or press Ctrl+D to exit the interpreter.");

    let mut current_statement = String::new();
    let mut st = SymbolTable::new();
    let mut env = Environment::new();

    loop {
        print!("{}", "\nAbySS> ".blue().bold());
        io::stdout().flush().unwrap(); // プロンプトを表示

        let mut input = String::new();
        input.clear();
        // EOF (Ctrl+D) を検知するために read_line の戻り値を確認
        let read_result = io::stdin().read_line(&mut input);

        // EOF を検知してインタープリタを終了
        if let Ok(0) = read_result {
            println!("");
            break;
        }

        let trimmed_input = input.trim();

        if trimmed_input == "exit" {
            break; // "exit" が入力されたらインタープリタ終了
        }

        // 文を結合
        current_statement.push_str(trimmed_input);

        // 文の末尾がセミコロンで終わっているかチェック
        if current_statement.ends_with(';') {
            // セミコロンで終わっていれば、文をパースして評価
            match parse(&current_statement) {
                Ok(pair) => {
                    for inner_pair in pair.into_inner() {
                        if inner_pair.as_rule() != Rule::EOI {
                            let ast = build_ast(inner_pair, &mut st);

                            // --debug フラグが有効な場合、ASTを表示
                            if debug {
                                println!("{}", format!("AST: {:?}", ast).yellow());
                            }

                            match evaluate(&ast, &mut env) {
                                Ok(result) => match result {
                                    EvalResult::Omen(b) => match b {
                                        true => println!("{}", "boon".green()),
                                        false => println!("{}", "hex".green()),
                                    },
                                    EvalResult::Arcana(n) => {
                                        println!("{}", format!("{}", n).green())
                                    }
                                    EvalResult::Aether(n) => {
                                        println!("{}", format!("{}", n).green())
                                    }
                                    EvalResult::Rune(s) => println!("{}", s.green()),
                                    EvalResult::Abyss => {}
                                },
                                Err(e) => println!("{}", format!("Error: {}", e).red()),
                            }
                        }
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
            current_statement.clear();
        }
    }

    println!("\nExiting AbySS interpreter...");
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Invoke { script } => {
            // .abyファイルを読み込んで実行
            if let Ok(contents) = fs::read_to_string(script) {
                execute_script(&contents);
            } else {
                eprintln!("Error: Could not read the script file.");
            }
        }
        Commands::Cast { debug } => {
            // インタープリタモードの開始
            start_interpreter(*debug);
        }
    }
}
