# AGENTS.md

## Crate layout

- `lenex` (repo root, `src/`): core model + XML ser/de. Zero FFI deps.
- `lenex-py`: PyO3 bindings, separate crate.
- `lenex-js`: napi-rs bindings, separate crate.
- Never merge these into one bindings crate behind feature flags — tried, reverted.

## Model spec: `doc/` is the only source of truth

`doc/lenex-3.0-schema.md` is mandatory for every struct under `src/model/`: field names, XML element/attribute names, and required-vs-optional all come from it — never guessed, never copied from another Lenex implementation. Check it first. If it's ambiguous or silent on something, fall back to `doc/lenex-3.0-spec.pdf` — never the reverse. Code vs. spec disagree → spec wins, fix the code. Fields the spec has but the code doesn't yet are expected (incremental build-out); fields the code has but the spec doesn't are a bug.

## Model struct conventions

- One struct per file. A type only used inside a parent lives in that parent's submodule, e.g. `Contact` → `src/model/constructor/contact.rs`. No `mod.rs`.
- All fields `pub`.
- Derive: `Serialize, Deserialize, Debug, Default, PartialEq, Builder, Clone`.
- `#[serde(rename = "UPPERCASE")]` at struct level. Attribute fields: `#[serde(rename = "@x")]`. Child-element fields: `#[serde(rename = "CHILD")]` (no `@`).
- Optional field: `Option<T>` + `#[serde(default, skip_serializing_if = "Option::is_none")]`.
- `quick-xml` is the only XML dep (picked for streaming support) — don't add a second one without re-justifying.

## Builder (`bon`)

- Required fields: `#[builder(start_fn, into)]`, declared contiguously first.
- Optional fields: `#[builder(into)]` only.
- `#[builder(...)]` goes above `#[serde(...)]` on a field.
- `into` on every field so `&str` works where the field is `String`.
- Single `.build()` even for nested structs: a parent field takes a built value or an open child builder via `#[builder(into)]`, enabled by pairing every buildable struct with:
    ```rust
    impl<S: xxx_builder::IsComplete> From<XxxBuilder<S>> for Xxx {
        fn from(b: XxxBuilder<S>) -> Xxx { b.build() }
    }
    ```
    Copy this for every new struct; don't reinvent it.

## Tests — identical shape in every model module

```rust
#[cfg(test)]
mod tests {
    use quick_xml::se::to_string;
    use super::Xxx;

    const MINIMAL: &str = r#"<XXX .../>"#;

    fn from_str(str: &str) -> Result<Xxx, quick_xml::de::DeError> {
        quick_xml::de::from_str::<Xxx>(str)
    }

    #[test]
    fn builder() { /* one call through Xxx::builder(...).build() */ }

    #[test]
    fn required_attributes() {
        // one expect_err per required field, fixture omits exactly that field
    }

    #[test]
    fn optional_attributes() {
        // parse MINIMAL, assert_eq! full struct, assert_eq!(MINIMAL, to_string(&parsed)...)
        // >1 optional: add a WITH_OPTIONALS const covering all, field-by-field + round-trip
    }
}
```

No `tag_names_are_locked` test; no one-test-per-required-field split (both tried, reverted).

## Python bindings (`lenex-py/src/**`)

- Module tree mirrors `lenex/src/model/**` 1:1, one `Py*`-prefixed struct per file. Never one big `bindings.rs`.
- `#[pyclass(name = "X", get_all, set_all)]` + `#[new]` (+ `#[pyo3(signature = ...)]` for keyword optionals) + `From<&PyX> for ::lenex::X`
    - `From<::lenex::X> for PyX` via `Python::attach(|py| ...)`.
- Nested struct fields **must** be `Py<PyChild>`, never the bare child struct: a bare struct makes `get_all` return a fresh copy each access, so Python-side mutation of a nested object silently no-ops.
- Open item: `Py<Child>`/`From` boilerplate duplicates core fields, can drift. Macro requested, not built — check before hand-writing a new binding struct.

## JS bindings (`lenex-js/src/**`)

napi-rs, currently just `pub use ::lenex::Lenex;` — far behind `lenex-py`. Error conversion is per-call-site (orphan rule blocks a blanket `From<lenex::Error> for napi::Error`). If expanded, mirror `lenex-py`'s one-module-per-struct layout, not a single file.

## Definition of done

Before calling any Rust change finished, all three must pass (scope with `-p <crate>` for a single-crate change):

```
cargo test
cargo clippy --all-targets -- -Dwarnings
cargo fmt --check
```

## Hard rules

- Never install or modify system-level tooling (language runtimes, version managers, global packages) without asking first.
- Flag any `Cargo.toml` or workspace-member edit not explicitly requested, even a one-liner — don't fold it in silently.
- "Show me" = preview only, don't write files. Wait for "apply" / "go ahead".
- "current code" means the live files, not git history or an earlier plan.
- Fence subagent diffs to the requested scope; report anything touched outside it instead of folding it in.
