use clap::Parser;

#[derive(Parser)]
#[command(name = "secho")]
#[command(version = "0.0.1")]
#[command(about = "Rust echo command", long_about = None)]
struct Cli {
    name: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli.name);
}
