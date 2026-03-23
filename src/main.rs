use std::env;

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

    // Debug output to verify the parsed PATH directories
    println!("Directories in PATH: {:?}", paths);

    // Print the parsed commands for verification (to be replaced in future steps)
    println!("Commands to search for: {:?}", args);
}
