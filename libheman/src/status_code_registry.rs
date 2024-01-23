// Rust Representation of the Hypertext Transfer Protocol (HTTP) Status Code Registry
// source:
// https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml
// https://www.iana.org/assignments/http-status-codes/http-status-codes-1.csv
//
// Source's last update: 2022-06-08
//
// Use python script `convert_source_data.py` to produce the
// Rust code tuples from the source csv file.

pub const CODE_REGISTRY: [(usize, &str, &str); 63] = [
    // (Value, "Description", "Reference")
    (100, "Continue", "[RFC9110, Section 15.2.1]"),
    (101, "Switching Protocols", "[RFC9110, Section 15.2.2]"),
    (102, "Processing", "[RFC2518]"),
    (103, "Early Hints", "[RFC8297]"),
    (200, "OK", "[RFC9110, Section 15.3.1]"),
    (201, "Created", "[RFC9110, Section 15.3.2]"),
    (202, "Accepted", "[RFC9110, Section 15.3.3]"),
    (
        203,
        "Non-Authoritative Information",
        "[RFC9110, Section 15.3.4]",
    ),
    (204, "No Content", "[RFC9110, Section 15.3.5]"),
    (205, "Reset Content", "[RFC9110, Section 15.3.6]"),
    (206, "Partial Content", "[RFC9110, Section 15.3.7]"),
    (207, "Multi-Status", "[RFC4918]"),
    (208, "Already Reported", "[RFC5842]"),
    (226, "IM Used", "[RFC3229]"),
    (300, "Multiple Choices", "[RFC9110, Section 15.4.1]"),
    (301, "Moved Permanently", "[RFC9110, Section 15.4.2]"),
    (302, "Found", "[RFC9110, Section 15.4.3]"),
    (303, "See Other", "[RFC9110, Section 15.4.4]"),
    (304, "Not Modified", "[RFC9110, Section 15.4.5]"),
    (305, "Use Proxy", "[RFC9110, Section 15.4.6]"),
    (306, "(Unused)", "[RFC9110, Section 15.4.7]"),
    (307, "Temporary Redirect", "[RFC9110, Section 15.4.8]"),
    (308, "Permanent Redirect", "[RFC9110, Section 15.4.9]"),
    (400, "Bad Request", "[RFC9110, Section 15.5.1]"),
    (401, "Unauthorized", "[RFC9110, Section 15.5.2]"),
    (402, "Payment Required", "[RFC9110, Section 15.5.3]"),
    (403, "Forbidden", "[RFC9110, Section 15.5.4]"),
    (404, "Not Found", "[RFC9110, Section 15.5.5]"),
    (405, "Method Not Allowed", "[RFC9110, Section 15.5.6]"),
    (406, "Not Acceptable", "[RFC9110, Section 15.5.7]"),
    (
        407,
        "Proxy Authentication Required",
        "[RFC9110, Section 15.5.8]",
    ),
    (408, "Request Timeout", "[RFC9110, Section 15.5.9]"),
    (409, "Conflict", "[RFC9110, Section 15.5.10]"),
    (410, "Gone", "[RFC9110, Section 15.5.11]"),
    (411, "Length Required", "[RFC9110, Section 15.5.12]"),
    (412, "Precondition Failed", "[RFC9110, Section 15.5.13]"),
    (413, "Content Too Large", "[RFC9110, Section 15.5.14]"),
    (414, "URI Too Long", "[RFC9110, Section 15.5.15]"),
    (415, "Unsupported Media Type", "[RFC9110, Section 15.5.16]"),
    (416, "Range Not Satisfiable", "[RFC9110, Section 15.5.17]"),
    (417, "Expectation Failed", "[RFC9110, Section 15.5.18]"),
    (418, "(Unused)", "[RFC9110, Section 15.5.19]"),
    (421, "Misdirected Request", "[RFC9110, Section 15.5.20]"),
    (422, "Unprocessable Content", "[RFC9110, Section 15.5.21]"),
    (423, "Locked", "[RFC4918]"),
    (424, "Failed Dependency", "[RFC4918]"),
    (425, "Too Early", "[RFC8470]"),
    (426, "Upgrade Required", "[RFC9110, Section 15.5.22]"),
    (428, "Precondition Required", "[RFC6585]"),
    (429, "Too Many Requests", "[RFC6585]"),
    (431, "Request Header Fields Too Large", "[RFC6585]"),
    (451, "Unavailable For Legal Reasons", "[RFC7725]"),
    (500, "Internal Server Error", "[RFC9110, Section 15.6.1]"),
    (501, "Not Implemented", "[RFC9110, Section 15.6.2]"),
    (502, "Bad Gateway", "[RFC9110, Section 15.6.3]"),
    (503, "Service Unavailable", "[RFC9110, Section 15.6.4]"),
    (504, "Gateway Timeout", "[RFC9110, Section 15.6.5]"),
    (
        505,
        "HTTP Version Not Supported",
        "[RFC9110, Section 15.6.6]",
    ),
    (506, "Variant Also Negotiates", "[RFC2295]"),
    (507, "Insufficient Storage", "[RFC4918]"),
    (508, "Loop Detected", "[RFC5842]"),
    (
        510,
        "Not Extended (OBSOLETED)",
        "[RFC2774][status-change-http-experiments-to-historic]",
    ),
    (511, "Network Authentication Required", "[RFC6585]"),
];

// Great research on sources of unofficial error codes
// was done by Evert Pot, https://evertpot.com/http/
pub const UNOFFICIAL_CODE_REGISTRY: [(usize, &str, &str); 4] = [
    (
        306,
        "Switch Proxy",
        "[https://datatracker.ietf.org/doc/html/draft-cohen-http-305-306-responses-00]",
    ),
    (418, "I'm a teapot", "[RFC2324, Section 2.3.3]"),
    (
        420,
        "Enhance your calm",
        "[https://evertpot.com/http/420-enhance-your-calm]",
    ),
    (
        430,
        "Would Block",
        "[https://datatracker.ietf.org/doc/html/draft-nottingham-http-pipeline-01]",
    ),
];
