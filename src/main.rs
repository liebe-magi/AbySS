use abyss::{
    eval::{evaluate_statements, Environment},
    parser::parse_statements,
};
use clap::{Parser, Subcommand};
use std::fs;

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
    // スクリプトをパースし、実行
    println!("Executing script: \n{}", script);
    // ここにAbyssスクリプトのパースと実行のロジックを追加
}

fn start_interpreter() {
    // インタープリタモードの実装
    println!("Starting Abyss interpreter...");
    // ここにインタープリタのロジックを追加
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
