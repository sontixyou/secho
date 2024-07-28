use clap::{Arg, ArgAction, ArgMatches, Command};

fn main() {
    let cli: ArgMatches = Command::new("secho")
        .author("sontixyou")
        .version("0.0.1")
        .about("print text")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text to print")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .long("no-newline")
                .help("Do not output the trailing newline")
                .action(ArgAction::SetFalse),
        )
        .get_matches();
    let text = cli.get_one::<String>("text").unwrap();
    println!("{:?}", text);
}
