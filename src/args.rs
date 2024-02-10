use clap::{Arg, Command};

pub fn get_command() -> Command {
    let version = env!("CARGO_PKG_VERSION");
    Command::new("the_semicolonizer")
        .version(version)
        .about("CLI tool that adds semicolons to your input")
        .arg(
            Arg::new("input")
                .help("The input to add semicolons to")
                .allow_hyphen_values(true)
                .trailing_var_arg(true)
                .num_args(0..),
        )
}
