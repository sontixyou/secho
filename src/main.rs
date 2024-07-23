use clap::{Arg, ArgAction, Command, Parser};

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
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .long("no-newline")
                .value_name("Do not output the trailing newline")
                .action(ArgAction::SetFalse),
        )
        .get_matches();
}
