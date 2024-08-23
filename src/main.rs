use abyss_lang::{
    env::Environment,
    eval::{display_error_with_source, evaluate, EvalError, EvalResult},
    format::format_ast,
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
    /// Format the input script file
    Align {
        /// The path to the script file
        script: String,
    },
}

/// Sets up the AbySS configuration directory in the user's home directory.
/// This directory is used to store configuration files such as history logs.
///
/// # Returns
/// A `PathBuf` representing the path to the AbySS directory.
fn setup_abyss_directory() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Unable to find home directory");
    let abyss_dir = home_dir.join(".abyss");

    if !abyss_dir.exists() {
        fs::create_dir_all(&abyss_dir).expect("Unable to create ~/.abyss directory");
    }

    abyss_dir
}

/// Returns the path to the AbySS history file stored in the AbySS directory.
///
/// # Returns
/// A `PathBuf` representing the path to the history file.
fn get_history_file_path() -> PathBuf {
    let abyss_dir = setup_abyss_directory();
    abyss_dir.join("abyss_history.log")
}

/// Executes a given AbySS script by parsing and evaluating it in a new environment.
///
/// # Arguments
/// * `script` - A string containing the AbySS script to be executed.
fn execute_script(script: &str) {
    let mut env = Environment::new();

    match parse(script) {
        Ok(pair) => {
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() != Rule::EOI {
                    match build_ast(inner_pair) {
                        Ok(ast) => match evaluate(&ast, &mut env) {
                            Ok(_) => {}
                            Err(e) => {
                                let error_message = e.to_string();
                                match e {
                                    EvalError::UndefinedVariable(_, line_info)
                                    | EvalError::InvalidOperation(_, line_info)
                                    | EvalError::NegativeExponent(line_info)
                                    | EvalError::TypeError(_, line_info) => {
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
        Err(e) => panic!("Error: {}", e),
    }
}

/// Formats the provided AbySS script by parsing and reconstructing it with proper indentation.
///
/// # Arguments
/// * `script` - A string containing the AbySS script to be formatted.
fn execute_format(script: &str) {
    match parse(script) {
        Ok(pair) => {
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() != Rule::EOI {
                    match build_ast(inner_pair) {
                        Ok(ast) => {
                            let formatted_code = format_ast(&ast, 0);
                            println!("{}", formatted_code);
                        }
                        Err(e) => panic!("Error: {}", e),
                    }
                }
            }
        }
        Err(e) => panic!("Error: {}", e),
    }
}

/// Starts the interactive AbySS interpreter, allowing the user to enter and execute AbySS code line by line.
///
/// # Arguments
/// * `debug` - A boolean flag to enable debug mode, which prints the AST of the parsed code.
fn start_interpreter(debug: bool) {
    println!("Starting AbySS interpreter...");
    println!("Type 'exit' or press Ctrl+D to exit the interpreter.\n");

    let mut current_session_code = String::new();
    let mut current_statement = String::new();
    let mut env = Environment::new();

    let history_path = get_history_file_path();
    let mut rl = Editor::<(), FileHistory>::new().expect("Error: Failed to create editor");
    let _ = rl.load_history(&history_path);
    let _ = rl.set_max_history_size(1000);

    loop {
        let prompt = format!(
            "AbySS > {}",
            "  ".repeat(
                current_statement.matches('{').count() - current_statement.matches('}').count()
            ),
        )
        .blue()
        .bold();
        let readline = rl.readline(&prompt.to_string());

        match readline {
            Ok(line) => {
                match line.trim() {
                    "exit" => {
                        println!("EXIT: Exiting interpreter...");
                        break;
                    }
                    "clear" => {
                        current_statement.clear();
                        continue;
                    }
                    "show" => {
                        println!("=== Current Session Code ===");
                        println!("{}", &current_session_code);
                        println!("============================");
                        current_statement.clear();
                        continue;
                    }
                    _ => {}
                }

                match rl.add_history_entry(line.as_str()) {
                    Ok(_) => {}
                    Err(err) => println!("Failed to add history: {:?}", err),
                }

                current_statement.push_str(&line);

                let open_braces = current_statement.matches('{').count();
                let close_braces = current_statement.matches('}').count();

                if open_braces == close_braces && current_statement.ends_with(';') {
                    match parse(&current_statement) {
                        Ok(pair) => {
                            for inner_pair in pair.into_inner() {
                                if inner_pair.as_rule() != Rule::EOI {
                                    match build_ast(inner_pair) {
                                        Ok(ast) => {
                                            if debug {
                                                println!("{}", format!("AST: {:?}", ast).yellow());
                                            }
                                            match evaluate(&ast, &mut env) {
                                                Ok(result) => {
                                                    current_session_code
                                                        .push_str(&format_ast(&ast, 0));
                                                    current_session_code.push('\n');
                                                    match result {
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
                                                        _ => {}
                                                    }
                                                }
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
                println!("CTRL-C: Restarting interpreter...");
                current_session_code.clear();
                current_statement.clear();
                env = Environment::new();
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D: Exiting interpreter...");
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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Invoke { script } => {
            if let Ok(contents) = fs::read_to_string(script) {
                execute_script(&contents);
            } else {
                eprintln!("Error: Could not read the script file.");
            }
        }
        Commands::Cast { debug } => {
            start_interpreter(*debug);
        }
        Commands::Align { script } => {
            if let Ok(contents) = fs::read_to_string(script) {
                execute_format(&contents);
            } else {
                eprintln!("Error: Could not read the script file.");
            }
        }
    }
}
