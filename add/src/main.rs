use clap::Parser;

/// A simple tool that prints back the argument it receives
#[derive(Parser)]
struct Args {
    /// The comment to print
    comment: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.comment);
}