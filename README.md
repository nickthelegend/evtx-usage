# evtx-usage

> A small Rust lab for reading Windows Event Log (`.evtx`) files and filtering security sign-in events.

## Overview

`evtx-usage` is a compact learning project that shows how to parse Windows Event Log
(`.evtx`) files in Rust. It opens a sample `Security` log, walks every record, and
deserializes each event's XML into typed structs so specific fields can be inspected.
The included example filters for logon events (Event ID `4624`) matching a chosen
`LogonType`, printing the record ID and timestamp of each match. It is intended as a
reference/experiment rather than a production tool.

## Features

- Parses `.evtx` files record-by-record with the [`evtx`](https://crates.io/crates/evtx) crate.
- Deserializes each event's XML payload into typed Rust structs via `quick-xml` + `serde`.
- Filters `Security` log entries for Event ID `4624` (an account was successfully logged on) by `LogonType`.
- Prints the matching event's record ID and creation timestamp.
- Ships a sample `security.evtx` file so the example runs out of the box.
- Includes a standalone helper snippet (`src/sad.rs`) that shells out to `wevtutil epl` to export the live Application, Security, and System logs on a Windows host.

## Tech Stack

- **Language:** Rust (edition 2021)
- **Event log parsing:** `evtx` 0.8
- **XML deserialization:** `quick-xml` 0.27 (with the `serialize` feature) + `serde` 1.0

## Getting Started

Requires a [Rust toolchain](https://rustup.rs/) (Cargo).

```bash
# Clone
git clone https://github.com/nickthelegend/evtx-usage.git
cd evtx-usage

# Build and run against the bundled sample log
cargo run

# Release build
cargo build --release
```

The example reads `samples/security.evtx` (resolved relative to `CARGO_MANIFEST_DIR`),
so it works from any directory as long as it's run through Cargo.

The log-export helper in `src/sad.rs` is a separate snippet that depends on the Windows
`wevtutil` utility; it is not wired into the default binary target.

## Project Structure

```
evtx-usage/
├── Cargo.toml              # Package manifest and dependencies
├── Cargo.lock
├── src/
│   ├── main.rs             # Parses samples/security.evtx and filters logon events
│   └── sad.rs              # Helper snippet: export live Windows logs via wevtutil
└── samples/
    ├── security.evtx       # Sample Windows Security event log
    └── LocaleMetaData/     # Locale metadata for the sample log
```

---

Built by [nickthelegend](https://github.com/nickthelegend) · [nickthelegend.tech](https://nickthelegend.tech)
