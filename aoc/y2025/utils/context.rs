use std::sync::OnceLock;
use std::{env, fs, process};

static GLOBAL_CONTEXT: OnceLock<Context> = OnceLock::new();

pub struct Context {
    filepath: String,
    verbose: bool,
    number: Option<i32>,
}

impl Context {
    /// Reads the content of a file specified by the first command-line argument.
    ///
    /// # Exits
    ///
    /// This constructor will print an error string and terminate the program
    /// if it receives invalid command line parameters.
    fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut filepath = String::new();
        let mut verbose = false;
        let mut number = None;
        let mut iter = args.iter().skip(1);

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-v" => verbose = true,
                "-n" => {
                    if let Some(next_arg) = iter.next() {
                        if let Ok(num) = next_arg.parse::<i32>() {
                            number = Some(num);
                            continue;
                        } else {
                            eprintln!("Invalid number provided after -n");
                            process::exit(-1);
                        }
                    } else {
                        eprintln!("Expected a number after -n");
                        process::exit(-1);
                    }
                }
                _ => {
                    if filepath.is_empty() {
                        filepath = arg.clone();
                    } else {
                        eprintln!("Unexpected argument: {}", arg);
                        process::exit(-1);
                    }
                }
            }
        }
        Self {
            filepath,
            verbose,
            number,
        }
    }
    /// Reads and returns all lines of the file specified in the context struct.
    ///
    /// # Exits
    ///
    /// This function will print an error string and terminate the program
    /// if it cannot read the file.
    pub fn read_lines_or_exit(&self) -> String {
        match fs::read_to_string(&self.filepath) {
            Err(error) => {
                eprintln!("Error: could not open {}: {}", self.filepath, error);
                process::exit(2);
            }
            Ok(str) => str,
        }
    }

    pub fn get_number() -> i32 {
        let num = Self::get().number;
        if num.is_none() {
            eprintln!("Error: Missing number parameter '-n <number>'");
            process::exit(-1);
        }
        num.unwrap()
    }

    pub fn is_verbose(&self) -> bool {
        return self.verbose;
    }

    pub fn get() -> &'static Context {
        GLOBAL_CONTEXT.get_or_init(|| Context::new())
    }

    pub fn verbose(&self, args: std::fmt::Arguments) {
        if self.verbose {
            println!("(V) {}", args);
        }
    }

    pub fn verbose_inline(&self, args: std::fmt::Arguments) {
        if self.verbose {
            print!("(V) {}", args);
        }
    }
}

#[macro_export]
macro_rules! verboseln {
    ($($arg:tt)*) => {
        Context::get().verbose(format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => {
        Context::get().verbose_inline(format_args!($($arg)*));
    }
}
