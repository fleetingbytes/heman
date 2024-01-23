# heman

A library (libheman) containing the HTTP error [code registry][iana] and a CLI for querying it. The library also contains an unofficial code registry with a handful of arbitrarily selected error codes which have been proposed or used with some degree of popularity. This unofficial registry can be optionally included in the query.

## Installation

```
cargo install heman
```

## Comman Line Interface (CLI)

Use `heman help` for instructions how to use the CLI. If a query results in one or more matches in the code registry, heman outputs the error code, error name, and the reference for each of the matches.

### Usage Examples

Find what code 401 means:

```
$ heman code 401
401 Unauthorized, [RFC9110, Section 15.5.2]
```

Find the error codes for redirects:

```
$ heman search redirect
307 Temporary Redirect, [RFC9110, Section 15.4.8]
308 Permanent Redirect, [RFC9110, Section 15.4.9]
```

## Unofficial Code Registry

To include the unofficial error code registry in the search queries, pass the `--unofficial` flag in the CLI. Alternatively, you can enable it by setting the  `HEMAN_INCLUDE_UNOFFICIAL_REGISTRY` environment variable.

### Usage Example

Which one was the unofficial code with the teapot?:

```
$ heman --unofficial search pot
418 I'm a teapot, [RFC2324, Section 2.3.3]

$ HEMAN_INCLUDE_UNOFFICIAL_REGISTRY=1
$ heman search pot
418 I'm a teapot, [RFC2324, Section 2.3.3]
```

### Note

The heman CLI only checks if the environment variable `HEMAN_INCLUDE_UNOFFICIAL_REGISTRY` exists. Its value does not matter.

## Name

heman comes from *H*TTP *E*rror *Man*ual.

# Acknowledgements

* IANA's [Hypertext Transfer Protocol (HTTP) Status Code Registry][iana]
* Evert Pot's [Series of posts on HTTP status codes][evert]

[iana]: https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml
[evert]: https://evertpot.com/http/
