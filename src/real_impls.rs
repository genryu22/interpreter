use crate::{FileExec, Repl, Usage};
use std::path::Path;

pub struct RealUsage;
impl Usage for RealUsage {
    fn show(&self) -> i32 {
        println!("Usage: interpreter [file]");
        64
    }
}

pub struct RealFileExec;
impl FileExec for RealFileExec {
    fn exec(&self, file: &Path) {
        println!("Executing file: {}", file.display());
        // 実際のファイル実行処理をここに実装
    }
}

pub struct RealRepl;
impl Repl for RealRepl {
    fn start(&self) {
        println!("Starting REPL mode...");
        // 実際のREPL処理をここに実装
    }
}
