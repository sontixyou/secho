use clap::Arg;
use clap::Command;
use clap::Parser;

#[derive(Parser)]
#[command(name = "secho")]
#[command(version = "0.0.1")]
#[command(about = "Rust echo command", long_about = None)]
struct Cli {
    name: Vec<String>,
}

fn main() {
    let cli = Command::new("secho")
        .author("sontixyou")
        .version("0.0.1")
        .about("print text")
        .arg(
            Arg::new("input text")
                .value_name("TEXT")
                .help("Input text to print")
                .required(true),
        )
        .get_matches();
}
