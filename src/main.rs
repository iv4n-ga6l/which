use std::env;

fn main() {
    // Collect command-line arguments, skipping the first one (program name)
    let args: Vec<String> = env::args().skip(1).collect();

    // If no arguments are provided, print usage and exit
    if args.is_empty() {
        eprintln!("Usage: which <command> [command ...]");
        std::process::exit(1);
    }

    // Print the parsed commands for verification (to be replaced in future steps)
    println!("Commands to search for: {:?}", args);
}
