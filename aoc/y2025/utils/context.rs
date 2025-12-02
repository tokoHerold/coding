use std::{env, fs, process};

pub struct Context {
    filepath: String,
    verbose: bool,
}

impl Context {
    /// Reads the content of a file specified by the first command-line argument.
    ///
    /// # Exits
    ///
    /// This constructor will print an error string and terminate the program
    /// if it receives invalid command line parameters.
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 || args.len() > 3 {
            eprintln!("Usage: {} <path-to-file> [-v]", &args[0]);
            process::exit(1)
        }
        if args.len() == 3 && args[1] != "-v" && args[2] != "-v" {
            eprintln!("Usage: {} <path-to-file> [-v]", &args[0]);
        }
        if args[1] == "-v" {
            Context {
                filepath: args[2].clone(),
                verbose: true,
            }
        } else {
            Context {
                filepath: args[1].clone(),
                verbose: if args.len() == 2 { false }  else { true },
            }
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

    pub fn verbose(&self, args: std::fmt::Arguments) {
        if self.verbose {
            println!("(Verbose) {}", args);
        }
    }
}
