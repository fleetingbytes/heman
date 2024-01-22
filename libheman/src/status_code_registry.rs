// Rust Representation of the Hypertext Transfer Protocol (HTTP) Status Code Registry
// source:
// https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml
// https://www.iana.org/assignments/http-status-codes/http-status-codes-1.csv
//
// source's last update: 2022-06-08

pub const CODE_REGISTRY: [(usize, &str, &str); 3] = [
    (100, "Continue", "[RFC9110, Section 15.2.1]"),
    (101, "Switching Protocols", "[RFC9110, Section 15.2.2]"),
    (102, "Processing", "[RFC2518]"),
];

// Great research on sources of unofficial error codes
// was done by Evert Pot, https://evertpot.com/http/
pub const UNOFFICIAL_CODE_REGISTRY: [(usize, &str, &str); 1] =
    [(418, "I'm a teapot", "[RFC2324, Section 2.3.3]")];
