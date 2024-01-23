pub mod cli;
mod error;

use clap::ArgMatches;
use error::Error;
use libheman::{
    find_by_code, find_by_substring,
    status_code_registry::{CODE_REGISTRY, UNOFFICIAL_CODE_REGISTRY},
};
use std::{
    env,
    io::{stdout, Write},
};

const REFERENCE: &str = "HEMAN_OUTPUT_REFERENCE";
const LINK: &str = "HEMAN_OUTPUT_LINK";
const UNOFFICIAL: &str = "HEMAN_INCLUDE_UNOFFICIAL_REGISTRY";

pub fn translate_to_api_calls(matches: ArgMatches) -> Result<(), Error> {
    let want_reference = env::var(REFERENCE).is_ok() || matches.get_flag("reference");
    let want_link = env::var(LINK).is_ok() || matches.get_flag("link");
    let want_unofficial = env::var(UNOFFICIAL).is_ok() || matches.get_flag("unofficial");
    match matches.subcommand() {
        Some(("code", sub_matches)) => {
            let code: usize = *sub_matches.get_one("CODE").expect("required");
            if want_unofficial {
                let unofficial_result = find_by_code(code, &UNOFFICIAL_CODE_REGISTRY);
                if unofficial_result.is_some() {
                    return output(unofficial_result, want_reference, want_link);
                }
            }
            output(
                find_by_code(code, &CODE_REGISTRY),
                want_reference,
                want_link,
            )
        }
        Some(("search", sub_matches)) => {
            let substring = sub_matches.get_one::<String>("STRING").expect("required");
            if want_unofficial {
                let unofficial_result = find_by_substring(substring, &UNOFFICIAL_CODE_REGISTRY);
                let official_result = find_by_substring(substring, &CODE_REGISTRY);
                output_iter(
                    unofficial_result.chain(official_result),
                    want_reference,
                    want_link,
                )
            } else {
                output_iter(
                    find_by_substring(substring, &CODE_REGISTRY),
                    want_reference,
                    want_link,
                )
            }
        }
        _ => unreachable!(),
    }
}

fn output(
    maybe_item: Option<(usize, &str, &str, &str)>,
    want_reference: bool,
    want_link: bool,
) -> Result<(), Error> {
    let item = maybe_item.ok_or(Error::Unassigned)?;
    let mut lock = stdout().lock();
    write!(lock, "{} {}", item.0, item.1).unwrap();
    if want_reference {
        write!(lock, ", {}", item.2).unwrap();
    }
    if want_link {
        write!(lock, ", {}", item.3).unwrap();
    }
    write!(lock, "\n").unwrap();
    Ok(())
}

fn output_iter(
    it: impl Iterator<Item = &'static (usize, &'static str, &'static str, &'static str)>,
    want_reference: bool,
    want_link: bool,
) -> Result<(), Error> {
    let mut found_some = false;
    for item in it {
        if !found_some {
            found_some = true;
        }
        output(Some(*item), want_reference, want_link)?;
    }
    if !found_some {
        return Err(Error::NotFound);
    }
    Ok(())
}
