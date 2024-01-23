# heman: HTTP error manual

A library containing the HTTP error code registry and a CLI for querying it.

## Installation

```
cargo install heman
```

## Comman Line Interface (CLI)

Use `heman help` for instructions how to use the CLI.

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

Which one was the unofficial code with the teapot?:

```
$ heman --unofficial search pot
418 I'm a teapot, [RFC2324, Section 2.3.3]
```

## Environment Variable

The search in the unofficial HTTP error code registry can also be enabled by setting the `HEMAN_SEARCH_UNOFFICIAL_REGISTRY` environment variable. Witch that it is not necessary to pass the `--unofficial` in the CLI.

## Name

heman comes from **H**TTP **E**rror **Man**ual.

# Acknowledgements

* IANA's [Hypertext Transfer Protocol (HTTP) Status Code Registry][iana]
* Evert Pot's [Series of posts on HTTP status codes][evert]

[iana]: https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml
[evert]: https://evertpot.com/http/
