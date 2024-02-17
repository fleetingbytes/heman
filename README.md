# heman

Heman is the CLI for the [http-error-codes2][hec2], a library containing the HTTP error [code registry][iana].

The library also contains an unofficial code registry of a handful of arbitrarily selected error codes which have been proposed or used with some degree of popularity. This unofficial registry can be optionally included in the query.

## Name

The name *heman* comes from *H*TTP *E*rror *Man*ual.

## Installation

```
cargo install heman
```

## Command Line Interface (CLI)

Use `heman help` for instructions how to use the CLI.

### Examples

Find what code 401 means:

```
$ heman code 401
401 Unauthorized
```

Find the error codes for redirects:

```
$ heman search redirect
307 Temporary Redirect
308 Permanent Redirect
```

### Unofficial Code Registry

To include the unofficial error code registry in the search queries, pass the `--unofficial` flag:

```
$ heman --unofficial search pot
418 I'm a teapot
```

### Reference, Link

To see the reference to the HTTP error, use the `--reference` flag. For a link to that reference, use `--link`:

```
$ heman --reference code 403
403 Forbidden, [RFC9110, Section 15.5.4]

$ heman --link search timeout
408 Request Timeout, https://www.rfc-editor.org/rfc/rfc9110.html#section-15.5.9
504 Gateway Timeout, https://www.rfc-editor.org/rfc/rfc9110.html#section-15.6.5

$ heman -rl code 300
300 Multiple Choices, [RFC9110, Section 15.4.1], https://www.rfc-editor.org/rfc/rfc9110.html#section-15.4.1
```

### Environment Variables

Instead of the CLI flags you can set environment variables:

| flag       | variable                            |
| ---------- | ----------------------------------- |
| reference  | `HEMAN_OUTPUT_REFERENCE`            |
| link       | `HEMAN_OUTPUT_LINK`                 |
| unofficial | `HEMAN_INCLUDE_UNOFFICIAL_REGISTRY` |

```
$ HEMAN_INCLUDE_UNOFFICIAL_REGISTRY=1
$ HEMAN_OUTPUT_REFERENCE=1
$ heman search pot
418 I'm a teapot, [RFC2324, Section 2.3.2]
```

### Note

The heman CLI only checks if one of these environment variables is set. The value does not matter.


# Acknowledgements

* IANA's [Hypertext Transfer Protocol (HTTP) Status Code Registry][iana]
* Evert Pot's [Series of posts on HTTP status codes][evert]

[iana]: https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml
[evert]: https://evertpot.com/http/
[hec2]: https://crates.io/crate/http-error-codes2/
