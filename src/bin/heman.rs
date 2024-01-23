use heman::{cli::cli, translate_to_api_calls};

fn main() -> () {
    let command = cli();
    match translate_to_api_calls(command.get_matches()) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
