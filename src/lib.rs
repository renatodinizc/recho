use clap::{command, Arg, ArgAction};

pub struct Args {
    input: String,
    no_newline: bool,
}

pub fn get_args() -> Args {
    let matches = command!()
        .arg(
            Arg::new("no_newline")
                .help("Do not print newline")
                .short('n')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("input")
                .help("Inputs to print")
                .default_value("")
                .action(ArgAction::Append)
                .trailing_var_arg(true),
        )
        .get_matches();

    let input = matches
        .get_many::<String>("input")
        .unwrap()
        .map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    Args {
        input,
        no_newline: *matches.get_one::<bool>("no_newline").unwrap(),
    }
}

pub fn process(args: Args) {
    if args.no_newline {
        print!("{}", args.input)
    } else {
        println!("{}", args.input)
    };
}
