use std::{env, fs, process};

pub struct Context {
    filepath: String,
}

impl Context {
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
}

/// Reads the content of a file specified by the first command-line argument.
///
/// # Exits
///
/// This function will print an error string and terminate the program
/// if it receives invalid command line parameters.
pub fn get_context() -> Context {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path-to-file>", &args[0]);
        process::exit(1)
    }
    Context {
        filepath: args[1].clone(),
    }
}
