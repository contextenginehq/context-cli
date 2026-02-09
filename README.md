# context-cli

[![Crates.io](https://img.shields.io/crates/v/context-cli.svg)](https://crates.io/crates/context-cli)
[![Docs.rs](https://docs.rs/context-cli/badge.svg)](https://docs.rs/context-cli)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

Command-line interface for the Context platform.

`context-cli` is the simplest way to build, inspect, and query Context caches locally or in CI. It is deterministic, scriptable, and designed for automation.

## Commands

| Command | Description |
|---------|-------------|
| `build` | Build a context cache from `.md` source documents |
| `resolve` | Resolve context for a query against a built cache |
| `inspect` | Inspect cache metadata and validity |

## Usage

### Build a cache

```bash
context build --sources ./docs --cache ./my-cache
```

Reads all `.md` files recursively from `--sources` and produces a deterministic cache directory at `--cache`. Use `--force` to overwrite an existing cache.

### Resolve context

```bash
context resolve --cache ./my-cache --query "deployment" --budget 4000
```

Outputs a JSON selection result to stdout. The output is byte-identical across runs for the same cache, query, and budget.

### Inspect a cache

```bash
context inspect --cache ./my-cache
```

Outputs cache metadata as JSON:

```json
{
  "cache_version": "sha256:...",
  "document_count": 3,
  "total_bytes": 955,
  "valid": true
}
```

## Output

- `resolve` and `inspect` write JSON to stdout
- Diagnostic messages go to stderr
- All output is deterministic

## Exit codes

| Code | Meaning | MCP Error Code |
|------|---------|----------------|
| 0 | Success | — |
| 1 | Usage error (bad arguments) | — |
| 2 | Invalid query | `invalid_query` |
| 3 | Invalid budget | `invalid_budget` |
| 4 | Cache missing | `cache_missing` |
| 5 | Cache invalid | `cache_invalid` |
| 6 | I/O error | `io_error` |
| 7 | Internal error | `internal_error` |

## Build

```bash
make build     # debug build
make test      # run all tests
make check     # cargo check + clippy
make release   # optimized build, binary copied to dist/
make clean     # remove artifacts
```

The release binary is named `context` and placed in `dist/`.

## Spec references

See `spec_refs.md` for links to the governing specifications.

---

"Context Engine" is a trademark of Context Engine Contributors. The software is open source under the [Apache License 2.0](LICENSE). The trademark is not licensed for use by third parties to market competing products or services without prior written permission.
