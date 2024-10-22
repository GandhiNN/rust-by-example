use clap::{Arg, Command};

pub fn cmd() -> clap::Command {
    Command::new("csvhandler")
        .version("1.0")
        .about("Toy project for CSV error handler")
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .value_parser(clap::value_parser!(i32))
                .required(true),
        )
}
