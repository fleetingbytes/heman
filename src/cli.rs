//! Defines the CLI using [clap].

use clap::{arg, value_parser, Command};

/// Returns the model of the CLI as a [`Command`].
pub fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Search HTTP errors by code or substring")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .arg(arg!(-r --reference "output the references"))
        .arg(arg!(-l --link "output references as links"))
        .arg(arg!(-u --unofficial "include the unofficial HTTP error code registry in the query"))
        .subcommand(
            Command::new("code")
                .about("Search HTTP error by error code")
                .arg(arg!(<CODE> "HTTP error code").value_parser(value_parser!(usize)))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("search")
                .about("Search HTTP error by substring")
                .arg(arg!(<STRING> "Substring in error description"))
                .arg_required_else_help(true),
        )
}
