use std::path::Path;

pub trait Usage {
    fn show(&self) -> Result<(), String>;
}
pub trait FileExec {
    fn exec(&self, file: &Path) -> Result<(), String>;
}
pub trait Repl {
    fn start(&self) -> Result<(), String>;
}

pub fn run(
    args: &[String],
    usage: &impl Usage,
    file_exec: &impl FileExec,
    repl: &impl Repl,
) -> Result<(), String> {
    match args.len() {
        n if n > 1 => usage.show(),
        1 => file_exec.exec(Path::new(&args[0])),
        _ => repl.start(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::path::Path;

    struct MockUsage {
        called: RefCell<bool>,
        result: Result<(), String>,
    }
    impl Usage for MockUsage {
        fn show(&self) -> Result<(), String> {
            *self.called.borrow_mut() = true;
            self.result.clone()
        }
    }

    struct MockFileExec {
        called: RefCell<Option<String>>,
        result: Result<(), String>,
    }
    impl FileExec for MockFileExec {
        fn exec(&self, file: &Path) -> Result<(), String> {
            *self.called.borrow_mut() = Some(file.display().to_string());
            self.result.clone()
        }
    }

    struct MockRepl {
        called: RefCell<bool>,
        result: Result<(), String>,
    }
    impl Repl for MockRepl {
        fn start(&self) -> Result<(), String> {
            *self.called.borrow_mut() = true;
            self.result.clone()
        }
    }

    #[test]
    fn test_usage_shown_when_args_gt_1() {
        let usage = MockUsage {
            called: RefCell::new(false),
            result: Err("usage error".to_string()),
        };
        let file_exec = MockFileExec {
            called: RefCell::new(None),
            result: Ok(()),
        };
        let repl = MockRepl {
            called: RefCell::new(false),
            result: Ok(()),
        };
        let args = vec!["a".to_string(), "b".to_string()];
        let ret = run(&args, &usage, &file_exec, &repl);
        assert_eq!(ret, Err("usage error".to_string()));
        assert!(*usage.called.borrow());
        assert!(file_exec.called.borrow().is_none());
        assert!(!*repl.called.borrow());
    }

    #[test]
    fn test_file_exec_when_args_eq_1() {
        let usage = MockUsage {
            called: RefCell::new(false),
            result: Ok(()),
        };
        let file_exec = MockFileExec {
            called: RefCell::new(None),
            result: Err("file error".to_string()),
        };
        let repl = MockRepl {
            called: RefCell::new(false),
            result: Ok(()),
        };
        let args = vec!["file.txt".to_string()];
        let ret = run(&args, &usage, &file_exec, &repl);
        assert_eq!(ret, Err("file error".to_string()));
        assert!(!*usage.called.borrow());
        assert_eq!(file_exec.called.borrow().as_deref(), Some("file.txt"));
        assert!(!*repl.called.borrow());
    }

    #[test]
    fn test_repl_when_args_is_empty() {
        let usage = MockUsage {
            called: RefCell::new(false),
            result: Ok(()),
        };
        let file_exec = MockFileExec {
            called: RefCell::new(None),
            result: Ok(()),
        };
        let repl = MockRepl {
            called: RefCell::new(false),
            result: Err("repl error".to_string()),
        };
        let args: Vec<String> = vec![];
        let ret = run(&args, &usage, &file_exec, &repl);
        assert_eq!(ret, Err("repl error".to_string()));
        assert!(!*usage.called.borrow());
        assert!(file_exec.called.borrow().is_none());
        assert!(*repl.called.borrow());
    }
}
