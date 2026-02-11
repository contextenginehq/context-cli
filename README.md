# context-cli

[![Crates.io](https://img.shields.io/crates/v/context-cli.svg)](https://crates.io/crates/context-cli)
[![Docs.rs](https://docs.rs/context-cli/badge.svg)](https://docs.rs/context-cli)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

> `context-cli` is the command-line interface for the Context platform. It provides a deterministic, scriptable tool for building, inspecting, and auditing context caches across local development and CI/CD pipelines.

## Purpose

The CLI is the build-time control plane for deterministic context infrastructure. Determinism guarantees that identical inputs produce byte-identical outputs across machines, operating systems, and supported Rust versions.

- **CI/CD Native**: Seamlessly build context caches as build artifacts in your deployment pipeline.
- **Auditable**: Inspect cache manifests and content hashes to ensure context integrity.
- **Reproducible**: Every `resolve` operation is byte-identical to the results seen by your deployed AI agents.
- **On-Prem Ready**: Works entirely offline with zero network dependencies.

## Intended users

- **Platform Engineers** managing AI infrastructure
- **CI/CD Pipelines** producing deterministic context artifacts
- **Security & Compliance Teams** auditing agent inputs

## Commands

| Command | Responsibility |
|---------|----------------|
| `build` | Compile `.md` source documents into a deterministic, content-addressed cache. |
| `resolve` | Execute the selection engine locally to verify agent retrieval behavior. |
| `inspect` | Validate cache integrity and view metadata snapshots. |

## Usage

### Build a cache

```bash
context build --sources ./docs --cache ./my-cache
```

Recursively ingests markdown documents and produces an immutable cache directory. The cache contains all data required for deterministic selection, eliminating runtime indexing or external dependencies.

### Resolve context (Local Audit)

```bash
context resolve --cache ./my-cache --query "deployment architecture" --budget 4000
```

Verify exactly what context an agent will receive for a specific query and token budget. Results are output as JSON.

### Inspect metadata

```bash
context inspect --cache ./my-cache
```

Example JSON output:

```json
{
  "cache_version": "v0",
  "document_count": 42,
  "total_bytes": 102400,
  "valid": true
}
```

### CI/CD Integration

Use the CLI to build context caches as part of your deployment artifacts:

```bash
# Example CI build step
context build --sources ./docs --cache ./dist/context-cache
context inspect --cache ./dist/context-cache
```

## Platform Role

`context-cli` manages the lifecycle of deterministic context artifacts.

- Build and verify caches during development and CI/CD
- Audit context behavior locally before deployment
- Produce the immutable artifacts consumed by `mcp-context-server`

Agents never build context at runtime — they consume caches produced by this CLI.

## Exit codes

| Code | Meaning | MCP Error Equivalent |
|------|---------|----------------------|
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
