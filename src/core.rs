use std::path::Path;

pub trait Usage {
    fn show(&self) -> i32;
}
pub trait FileExec {
    fn exec(&self, file: &Path);
}
pub trait Repl {
    fn start(&self);
}

pub fn run(
    args: &[String],
    usage: &impl Usage,
    file_exec: &impl FileExec,
    repl: &impl Repl,
) -> i32 {
    match args.len() {
        n if n > 1 => usage.show(),
        1 => {
            file_exec.exec(Path::new(&args[0]));
            0
        }
        _ => {
            repl.start();
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::path::Path;

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
        fn exec(&self, file: &Path) {
            *self.called.borrow_mut() = Some(file.display().to_string());
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
