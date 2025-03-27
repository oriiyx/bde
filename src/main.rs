use bde::configuration::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
