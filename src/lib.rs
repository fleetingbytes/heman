//! **H**ttp **E**rror **Man**ual
//!
//! `heman` is a [CLI](https://en.wikipedia.org/wiki/Command-line_interface)
//! for the [http-status-codes2](https://crates.io/crates/http-status-codes2) library which models
//! [IANA's](https://www.iana.org/)
//! [HTTP Status Code Registry](https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml).
//!
//! It allows you to quickly search for a HTTP status code by its code or description,
//! optionally providing the references and links to the
//! [RFC](https://en.wikipedia.org/wiki/Request_for_Comments)s where the status code was specified.
//!
//! # Usage
//! Use `heman help` for instructions how to use the CLI.
//! ## Examples
//!
//! Search by status code to find what that code means:
//!  ```bash
//!  $ heman code 401
//!  401 Unauthorized
//!  ```
//!
//! Search by a substring of the status code description:
//!  ```bash
//!  $ heman search redirect
//!  307 Temporary Redirect
//!  308 Permanent Redirect
//!  ```
//! ### Unofficial Code Registry
//! The [http-status-codes2](https://crates.io/crates/http-status-codes2) library also contains an unofficial code registry
//! of a handful of arbitrarily selected status codes which have been proposed
//! or used with some degree of popularity.
//! This unofficial registry can be optionally included in the query using the `--unofficial` flag:
//!  ```bash
//!  $ heman --unofficial search pot
//!  418 I'm a teapot
//!  ```
//!
//! ### Reference, Link
//! To see the reference to the HTTP status code, use the `--reference` flag. For a link to that reference, use `--link`:
//!  ```bash
//!  $ heman --reference code 403
//!  403 Forbidden, [RFC9110, Section 15.5.4]
//!  ```
//!  ```bash
//!  $ heman --link search timeout
//!  408 Request Timeout, https://www.rfc-editor.org/rfc/rfc9110.html#section-15.5.9
//!  504 Gateway Timeout, https://www.rfc-editor.org/rfc/rfc9110.html#section-15.6.5
//!  ```
//!  ```bash
//!  $ heman -rl code 300
//!  300 Multiple Choices, [RFC9110, Section 15.4.1], https://www.rfc-editor.org/rfc/rfc9110.html#section-15.4.1
//!  ```
//!
//! ### Environment Variables
//!
//! Instead of the CLI flags you can set these environment variables:
//!
//! | flag       | variable                            |
//! | ---------- | ----------------------------------- |
//! | reference  | `HEMAN_OUTPUT_REFERENCE`            |
//! | link       | `HEMAN_OUTPUT_LINK`                 |
//! | unofficial | `HEMAN_INCLUDE_UNOFFICIAL_REGISTRY` |
//!
//! **NOTE:** The heman CLI only checks if one of these environment variables is set. The value does not matter.
//!
//! ```bash
//! $ HEMAN_INCLUDE_UNOFFICIAL_REGISTRY=1
//! $ HEMAN_OUTPUT_REFERENCE=1
//! $ heman search pot
//! 418 I'm a teapot, [RFC2324, Section 2.3.2]
//! ```

#![deny(missing_docs)]
pub mod cli;
mod error;

use clap::ArgMatches;
use error::Error;
use http_status_codes2::{
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

/// CLI logic. Translates a command from the CLI to the corresponding API calls.
pub fn translate_to_api_calls(matches: ArgMatches) -> Result<(), Error> {
    let want_reference = matches.get_flag("reference") || env::var(REFERENCE).is_ok();
    let want_link = matches.get_flag("link") || env::var(LINK).is_ok();
    let want_unofficial = matches.get_flag("unofficial") || env::var(UNOFFICIAL).is_ok();
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
