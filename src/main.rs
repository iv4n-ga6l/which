use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Collect command-line arguments, skipping the first one (program name)
    let args: Vec<String> = env::args().skip(1).collect();

    // If no arguments are provided, print usage and exit
    if args.is_empty() {
        eprintln!("Usage: which <command> [command ...]");
        std::process::exit(1);
    }

    // Read the PATH environment variable
    let path_var = env::var("PATH").unwrap_or_else(|_| {
        eprintln!("Error: PATH environment variable is not set.");
        std::process::exit(1);
    });

    // Split PATH into directories
    let paths: Vec<&str> = path_var.split(':').collect();

    // Iterate over each command provided as an argument
    for command in args {
        match find_command_in_path(&command, &paths) {
            Some(found_path) => println!("{}", found_path), // Output only the path
            None => eprintln!("{}: command not found", command),
        }
    }
}

/// Searches for a command in the given list of directories.
/// Returns the full path to the command if found and executable, otherwise None.
fn find_command_in_path(command: &str, paths: &[&str]) -> Option<String> {
    for dir in paths {
        let full_path = Path::new(dir).join(command);

        // Check if the path exists, is a file, and is executable
        if full_path.is_file() {
            if let Ok(metadata) = fs::metadata(&full_path) {
                if metadata.permissions().mode() & 0o111 != 0 {
                    return Some(full_path.to_string_lossy().to_string());
                }
            }
        }
    }
    None
}
