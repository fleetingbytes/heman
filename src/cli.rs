use clap::{arg, value_parser, Command};

pub fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Search HTTP errors by code or substring")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .arg(arg!(-u --unofficial "lookup also the unofficial HTTP error code registry"))
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
