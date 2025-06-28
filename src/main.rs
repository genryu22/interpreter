trait Usage {
    fn show(&self) -> i32;
}
trait FileExec {
    fn exec(&self, file: &str);
}
trait Repl {
    fn start(&self);
}

fn run(args: &[String], usage: &impl Usage, file_exec: &impl FileExec, repl: &impl Repl) -> i32 {
    match args.len() {
        n if n > 1 => usage.show(),
        1 => {
            file_exec.exec(&args[0]);
            0
        }
        _ => {
            repl.start();
            0
        }
    }
}

struct RealUsage;
impl Usage for RealUsage {
    fn show(&self) -> i32 {
        println!("Usage: interpreter [file]");
        64
    }
}

struct RealFileExec;
impl FileExec for RealFileExec {
    fn exec(&self, file: &str) {
        println!("Executing file: {}", file);
        // 実際のファイル実行処理をここに実装
    }
}

struct RealRepl;
impl Repl for RealRepl {
    fn start(&self) {
        println!("Starting REPL mode...");
        // 実際のREPL処理をここに実装
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = RealUsage;
    let file_exec = RealFileExec;
    let repl = RealRepl;
    let code = run(&args, &usage, &file_exec, &repl);
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockUsage {
        called: RefCell<bool>,
        code: i32,
    }
    impl Usage for MockUsage {
        fn show(&self) -> i32 {
            *self.called.borrow_mut() = true;
            self.code
        }
    }

    struct MockFileExec {
        called: RefCell<Option<String>>,
    }
    impl FileExec for MockFileExec {
        fn exec(&self, file: &str) {
            *self.called.borrow_mut() = Some(file.to_string());
        }
    }

    struct MockRepl {
        called: RefCell<bool>,
    }
    impl Repl for MockRepl {
        fn start(&self) {
            *self.called.borrow_mut() = true;
        }
    }

    #[test]
    fn test_usage_shown_when_args_gt_1() {
        let usage = MockUsage {
            called: RefCell::new(false),
            code: 42,
        };
        let file_exec = MockFileExec {
            called: RefCell::new(None),
        };
        let repl = MockRepl {
            called: RefCell::new(false),
        };
        let args = vec!["a".to_string(), "b".to_string()];
        let ret = run(&args, &usage, &file_exec, &repl);
        assert_eq!(ret, 42);
        assert!(*usage.called.borrow());
        assert!(file_exec.called.borrow().is_none());
        assert!(!*repl.called.borrow());
    }

    #[test]
    fn test_file_exec_when_args_eq_1() {
        let usage = MockUsage {
            called: RefCell::new(false),
            code: 99,
        };
        let file_exec = MockFileExec {
            called: RefCell::new(None),
        };
        let repl = MockRepl {
            called: RefCell::new(false),
        };
        let args = vec!["file.txt".to_string()];
        let ret = run(&args, &usage, &file_exec, &repl);
        assert_eq!(ret, 0);
        assert!(!*usage.called.borrow());
        assert_eq!(file_exec.called.borrow().as_deref(), Some("file.txt"));
        assert!(!*repl.called.borrow());
    }

    #[test]
    fn test_repl_when_args_is_empty() {
        let usage = MockUsage {
            called: RefCell::new(false),
            code: 77,
        };
        let file_exec = MockFileExec {
            called: RefCell::new(None),
        };
        let repl = MockRepl {
            called: RefCell::new(false),
        };
        let args: Vec<String> = vec![];
        let ret = run(&args, &usage, &file_exec, &repl);
        assert_eq!(ret, 0);
        assert!(!*usage.called.borrow());
        assert!(file_exec.called.borrow().is_none());
        assert!(*repl.called.borrow());
    }
}
