use heman::{cli::cli, execute};

fn main() -> () {
    let command = cli();
    match execute(command.get_matches()) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
