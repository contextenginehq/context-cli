# Specification References

This crate implements the following specs from the canonical `context-specs` repository:

## Primary Specs

- [interfaces/cli_spec.md](../context-specs/interfaces/cli_spec.md) — Command list, arguments, exit codes, output format
- [core/cli/context_resolve.md](../context-specs/core/cli/context_resolve.md) — `resolve` command behavior, determinism contract, error categories

## Core Specs (via context-core)

- [core/context_cache.md](../context-specs/core/context_cache.md) — Cache structure, build process, verification
- [core/context_selection.md](../context-specs/core/context_selection.md) — Selection algorithm, scoring, ordering, budgeting
- [core/document_model.md](../context-specs/core/document_model.md) — Document identity, versioning, metadata
- [core/mcp/context.resolve.md](../context-specs/core/mcp/context.resolve.md) — Normative output schema (CLI and MCP must match)
- [core/mcp/error_schema.md](../context-specs/core/mcp/error_schema.md) — Error codes (mapped to CLI exit codes)

## Compliance

The CLI is the **reference implementation** for context resolution. MCP integrations must produce byte-identical output.

| Requirement | Enforcement |
|-------------|-------------|
| Determinism | `tests/determinism.rs` — diff test from spec |
| Exit codes | `tests/exit_codes.rs` — frozen mapping |
| Output schema | Serializes `context-core::SelectionResult` directly |
| Error mapping | `exit_codes.rs` — domain errors → frozen exit codes 2-7 |

## v0 Scope

- `build`, `resolve`, `inspect` commands implemented
- `serve` deferred — MCP server is a standalone binary (`mcp-context-server`)
- `ingest` removed — ingestion is part of `build`
