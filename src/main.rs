use clap::{command, Arg, ArgAction};
// use std::env;

fn main() {
    let matches = command!()
        .arg(Arg::new("no_newline").help("Do not print newline").short('n').long("no_newline").action(ArgAction::SetTrue))
        .arg(Arg::new("input").help("Inputs to print").default_value("").action(ArgAction::Append))// .arg(arg!(--brand <VALUE>))
        .get_matches();

    let no_newline = *matches.get_one::<bool>("no_newline").unwrap();
    let text = &matches
        .get_many::<String>("input")
        .unwrap()
        .map(|v| v.as_str())
        .collect::<Vec<_>>().join(" ");

    if no_newline { print!("{text}") } else { println!("{text}") };
}
