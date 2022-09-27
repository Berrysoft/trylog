# trylog
This crate is inspired by [`tracing-unwrap`](https://crates.io/crates/tracing-unwrap),
and provides `inspect` and `unwrap_or_default` series of methods.

It also supports all types implemented `Try`.

## Log level
| method                   | level |
| ------------------------ | ----- |
| `inspect_or_log*`        | info  |
| `unwrap_or_default_log*` | warn  |
| `unwrap_or_log*`         | error |
