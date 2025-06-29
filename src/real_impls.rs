use crate::{FileExec, Repl, Usage};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub struct RealUsage;
impl Usage for RealUsage {
    fn show(&self) -> Result<(), String> {
        println!("Usage: interpreter [file]");
        Err("Usage shown".to_string())
    }
}

pub fn run(source: &str) -> Result<(), String> {
    Ok(())
}

pub struct RealFileExec;
impl FileExec for RealFileExec {
    fn exec(&self, file: &Path) -> Result<(), String> {
        println!("Executing file: {}", file.display());
        let content =
            fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;
        run(&content)
    }
}

pub struct RealRepl;
impl Repl for RealRepl {
    fn start(&self) -> Result<(), String> {
        println!("Starting REPL mode...");
        let stdin = io::stdin();
        let mut line = String::new();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            line.clear();
            if stdin.read_line(&mut line).is_err() {
                println!("入力エラー");
                break;
            }
            let input = line.trim_end();
            if input == "exit" || input == "quit" {
                break;
            }
            if let Err(e) = run(input) {
                println!("Error: {}", e);
            }
        }
        Ok(())
    }
}
