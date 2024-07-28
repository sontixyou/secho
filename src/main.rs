use clap::{parser::ValuesRef, Arg, ArgAction, ArgMatches, Command};

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
    let text: ValuesRef<String> = cli.get_many::<String>("text").unwrap();
    let iterator_text = text.map(|s| s.to_string());
    let vec_text: Vec<String> = iterator_text.collect();
    println!("{}", vec_text.join(" "));
}
