use std::path::Path;

mod core;
mod real_impls;

use core::{FileExec, Repl, Usage, run};
use real_impls::{RealFileExec, RealRepl, RealUsage};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = RealUsage;
    let file_exec = RealFileExec;
    let repl = RealRepl;
    let code = run(&args, &usage, &file_exec, &repl);
    std::process::exit(code);
}
