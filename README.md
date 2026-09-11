# lenex-rs

Rust library for parsing and generating **LENEX** files, with native bindings
for Node.js and Python.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![status: early development](https://img.shields.io/badge/status-early%20development-orange.svg)](#status)

## What is LENEX?

[LENEX](https://wiki.swimrankings.net/index.php/swimrankings:Lenex) is the XML-based
data exchange format used by swimming meet management software to share meets,
entries, results, splits, and records. It ships as either a single `.lef` XML file
or a zipped `.lxf` container.

## Project structure

This is a Cargo workspace with one core crate and two language bindings, so a fix
to the parser benefits every language at once:

| Crate                    | Description                                                                 |
| ------------------------ | --------------------------------------------------------------------------- |
| [`lenex-rs`](./lenex-rs) | Core Rust library — parsing, model, I/O                                     |
| [`lenex-py`](./lenex-py) | Python bindings ([PyO3](https://pyo3.rs)/[maturin](https://www.maturin.rs)) |
| [`lenex-js`](./lenex-js) | Node.js bindings ([napi-rs](https://napi.rs))                               |

## Status

🚧 Early development. The public API shape is in place (`Lenex::open`, `save`,
`from_lef_reader`/`from_lxf_reader`, `to_lef_writer`/`to_lxf_writer`, and the
in-memory `_bytes`/`_string` convenience methods), but parsing/serialization and
the domain model are not implemented yet. Not ready for real workloads.

## Usage

### Rust

```toml
[dependencies]
lenex = "0.1"
```

```rust
use lenex::Lenex;

let meet = Lenex::open("meet.lxf")?;
meet.save("meet.lef")?;
```

### Python

```bash
pip install lenex
```

```python
from lenex import Lenex, LenexError

meet = Lenex.open("meet.lxf")
meet.save("meet.lef")

# in-memory, e.g. for web uploads
meet = Lenex.from_lxf_bytes(data)
lef_text = meet.to_lef_string()

try:
    Lenex.open("missing.lxf")
except LenexError as e:
    print(e)
```

### Node.js

```bash
npm install lenex
```

```js
const { Lenex } = require("lenex");

const meet = Lenex.open("meet.lxf");
meet.save("meet.lef");

// in-memory, e.g. for web uploads
const meet2 = Lenex.fromLxfBuffer(buffer);
const lefText = meet2.toLefString();
```

## Development

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## Contributing

Issues and PRs welcome — see the [spec](#what-is-lenex) above for format
details before implementing new element support.

## License

MIT
