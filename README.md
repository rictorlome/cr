# cr

`cr` is a tiny tool I made to replace the names of characters in [`fountain`](https://www.fountain.io/) files.
Find and replace can get cumbersome with a screenplay, because character names appear in multiple cases.

## Usage

```bash
$ cr --help
cr 0.1.0
A small tool to replace names in files, respecting the case.

USAGE:
    cr [OPTIONS] --from <FROM> --to <TO>

OPTIONS:
    -f, --from <FROM>    The old name
    -h, --help           Print help information
    -p, --path <PATH>    Optional path of file to read in. If none supplied, will read from stdin
    -t, --to <TO>        The new name
    -V, --version        Print version information
```

`cr` will replace all instances of `from` with `to` in `files`.

## Rules

`cr` currently supports the following transformations:

- `ralph` -> `tony`
- `Ralph` -> `Tony`
- `RALPH` -> `TONY`
