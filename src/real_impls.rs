use crate::{FileExec, Repl, Usage};
use std::path::Path;

pub struct RealUsage;
impl Usage for RealUsage {
    fn show(&self) -> Result<(), String> {
        println!("Usage: interpreter [file]");
        Err("Usage shown".to_string())
    }
}

pub struct RealFileExec;
impl FileExec for RealFileExec {
    fn exec(&self, file: &Path) -> Result<(), String> {
        println!("Executing file: {}", file.display());
        // 実際のファイル実行処理をここに実装
        Ok(())
    }
}

pub struct RealRepl;
impl Repl for RealRepl {
    fn start(&self) -> Result<(), String> {
        println!("Starting REPL mode...");
        // 実際のREPL処理をここに実装
        Ok(())
    }
}
