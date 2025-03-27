use bde::analyzer::analyze_schema;
use bde::configuration::{Args, Commands, get_configuration};
use clap::Parser;
use colored::*;
use std::path::Path;
use std::process;

fn main() {
    let args = Args::parse();

    // Check if configuration file exists
    if !Path::new("bde.yaml").exists() {
        eprintln!(
            "{}",
            "Error: Configuration file 'bde.yaml' not found in the current directory."
                .red()
                .bold()
        );
        eprintln!(
            "{}",
            "Please create a bde.yaml file or run from a directory containing one."
                .red()
                .bold()
        );
        process::exit(1);
    }

    // Try to parse the configuration
    let configuration = match get_configuration() {
        Ok(config) => config,
        Err(e) => {
            eprintln!(
                "{}",
                "Error: Failed to parse configuration file:".red().bold()
            );
            eprintln!("{}", e.to_string().red().bold());
            eprintln!(
                "{}",
                "Please check your bde.yaml file for syntax errors."
                    .red()
                    .bold()
            );
            process::exit(1);
        }
    };

    match args.cmd {
        Commands::Generate => {
            println!("Generate");
            analyze_schema(configuration)
        }
    }
}
