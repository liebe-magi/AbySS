use abyss_lang::{
    env::{Environment, SymbolTable},
    eval::{display_error_with_source, evaluate, EvalError, EvalResult},
    parser::{build_ast, parse, Rule},
};
use clap::{Parser, Subcommand};
use colored::*;
use dirs;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;
use std::fs;
use std::path::PathBuf;

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

fn setup_abyss_directory() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Unable to find home directory");
    let abyss_dir = home_dir.join(".abyss");

    if !abyss_dir.exists() {
        fs::create_dir_all(&abyss_dir).expect("Unable to create ~/.abyss directory");
    }

    abyss_dir
}

fn get_history_file_path() -> PathBuf {
    let abyss_dir = setup_abyss_directory();
    abyss_dir.join("abyss_history.log")
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
                    match build_ast(inner_pair, &mut st) {
                        Ok(ast) => match evaluate(&ast, &mut env) {
                            Ok(_) => {}
                            Err(e) => {
                                let error_message = e.to_string();
                                match e {
                                    EvalError::UndefinedVariable(_, line_info)
                                    | EvalError::InvalidOperation(_, line_info)
                                    | EvalError::NegativeExponent(line_info) => {
                                        display_error_with_source(
                                            script,
                                            line_info,
                                            &error_message,
                                        );
                                        return;
                                    }
                                }
                            }
                        },
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

    // rustylineのEditorを作成し、履歴を有効化
    let history_path = get_history_file_path();
    let mut rl = Editor::<(), FileHistory>::new().expect("Error: Failed to create editor");
    let _ = rl.load_history(&history_path);
    let _ = rl.set_max_history_size(1000);

    loop {
        let readline = rl.readline(&"\nAbySS> ".blue().bold().to_string());

        match readline {
            Ok(line) => {
                if line.trim() == "exit" {
                    break; // "exit" が入力されたらインタープリタ終了
                }

                match rl.add_history_entry(line.as_str()) {
                    Ok(_) => {} // 正常に履歴が追加された場合は何もしない
                    Err(err) => println!("Failed to add history: {:?}", err), // エラーが発生した場合にエラーメッセージを表示
                }

                // 文を結合
                current_statement.push_str(&line);

                // 文の末尾がセミコロンで終わっていれば、文をパースして評価
                if current_statement.ends_with(';') {
                    match parse(&current_statement) {
                        Ok(pair) => {
                            for inner_pair in pair.into_inner() {
                                if inner_pair.as_rule() != Rule::EOI {
                                    match build_ast(inner_pair, &mut st) {
                                        Ok(ast) => {
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
                                                    EvalResult::Rune(s) => {
                                                        println!("{}", s.green())
                                                    }
                                                    EvalResult::Abyss => {}
                                                },
                                                Err(e) => {
                                                    println!("{}", format!("Error: {}", e).red())
                                                }
                                            }
                                        }
                                        Err(e) => println!("{}", format!("Error: {}", e).red()),
                                    }
                                }
                            }
                        }
                        Err(e) => println!("{}", format!("Error: {}", e).red()),
                    }
                    current_statement.clear();
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(&history_path)
        .expect("Error: Failed to save history");
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
