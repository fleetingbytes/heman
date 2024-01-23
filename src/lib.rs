pub mod cli;
pub mod error;

use clap::ArgMatches;
use error::Error;
use libheman::{
    find_by_code, find_by_substring,
    status_code_registry::{CODE_REGISTRY, UNOFFICIAL_CODE_REGISTRY},
};
use std::env;

const KEY: &str = "HEMAN_SEARCH_UNOFFICIAL_REGISTRY";

pub fn execute(matches: ArgMatches) -> Result<(), Error> {
    let want_unofficial = env::var(KEY).is_ok() || matches.get_flag("unofficial");
    match matches.subcommand() {
        Some(("code", sub_matches)) => {
            let code: usize = *sub_matches.get_one("CODE").expect("required");
            if want_unofficial {
                let unofficial_result = find_by_code(code, &UNOFFICIAL_CODE_REGISTRY);
                if unofficial_result.is_some() {
                    return output(unofficial_result);
                }
            }
            output(find_by_code(code, &CODE_REGISTRY))
        }
        Some(("search", sub_matches)) => {
            let substring = sub_matches.get_one::<String>("STRING").expect("required");
            if want_unofficial {
                let unofficial_result = find_by_substring(substring, &UNOFFICIAL_CODE_REGISTRY);
                let official_result = find_by_substring(substring, &CODE_REGISTRY);
                output_iter(unofficial_result.chain(official_result))
            } else {
                output_iter(find_by_substring(substring, &CODE_REGISTRY))
            }
        }
        _ => unreachable!(),
    }
}

fn output(maybe_item: Option<(usize, &str, &str)>) -> Result<(), Error> {
    let item = maybe_item.ok_or(Error::Unassigned)?;
    println!("{} {}, {}", item.0, item.1, item.2);
    Ok(())
}

fn output_iter(
    it: impl Iterator<Item = &'static (usize, &'static str, &'static str)>,
) -> Result<(), Error> {
    let mut found_some = false;
    for item in it {
        if !found_some {
            found_some = true;
        }
        output(Some(*item))?;
    }
    if !found_some {
        return Err(Error::NotFound);
    }
    Ok(())
}
