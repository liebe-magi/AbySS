use abyss::{
    eval::{evaluate, Environment, EvalResult},
    parser::{build_ast, parse},
};
use clap::{Parser, Subcommand};
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
    Cast,
}

fn execute_script(script: &str) {
    // 環境を初期化
    let mut env = Environment::new();

    // スクリプトをパースして評価
    match parse(script) {
        Ok(pair) => {
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() != abyss::parser::Rule::EOI {
                    let ast = build_ast(inner_pair);
                    evaluate(&ast, &mut env);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn start_interpreter() {
    println!("Starting AbySS interpreter...");

    let mut current_statement = String::new();
    let mut env = Environment::new();
    loop {
        print!("AbySS> ");
        io::stdout().flush().unwrap(); // プロンプトを表示

        let mut input = String::new();
        input.clear();
        io::stdin().read_line(&mut input).unwrap(); // 標準入力を受け取る

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
                        if inner_pair.as_rule() != abyss::parser::Rule::EOI {
                            let ast = build_ast(inner_pair);
                            match evaluate(&ast, &mut env) {
                                EvalResult::Arcana(n) => println!("{}", n),
                                EvalResult::Rune(s) => println!("{}", s),
                                EvalResult::Abyss => {}
                            }
                        }
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
            current_statement.clear();
        }
    }
    println!("Exiting AbySS interpreter...");
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
        Commands::Cast => {
            // インタープリタモードの開始
            start_interpreter();
        }
    }
}
