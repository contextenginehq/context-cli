# context-cli — Implementation Progress

## Status: v0 complete — compiles and passes all tests (12/12)

The CLI binary `context` implements three commands: `build`, `resolve`, `inspect`. All frozen exit codes (0-7) are implemented. Determinism is verified through integration tests that exercise the full build→resolve pipeline.

---

## Decisions

| Decision | Resolution |
|----------|------------|
| Binary name | `context` (matches spec command syntax: `context build`, `context resolve`) |
| `ingest` command | Removed. v0 uses a single `build` command that includes ingestion. |
| `serve` command | Deferred to post-v0. MCP server is a standalone binary (`mcp-context-server`). |
| `--format` default | `json` (compact). `pretty` available via `--format pretty`. Matches CLI spec "emit JSON by default". |
| Exit codes | Frozen 0-7 per spec. Clap usage errors map to exit 1. Domain errors map to 2-7 via context-core error types. |

---

## Completed

- [x] `Cargo.toml` — `[[bin]] name = "context"`, context-core path dep, walkdir, tempfile dev-dep
- [x] `src/main.rs` — Three subcommands (Build, Resolve, Inspect), error→exit code dispatch
- [x] `src/exit_codes.rs` — Frozen constants 0-7, `CliError` type, `From<CacheBuildError>`, `From<SelectionError>`
- [x] `src/commands/mod.rs` — Declares `build`, `inspect`, `resolve`
- [x] `src/commands/build.rs` — Walk sources, ingest .md files, build cache via context-core
- [x] `src/commands/resolve.rs` — Load cache, run selection, serialize JSON/pretty to stdout
- [x] `src/commands/inspect.rs` — Load manifest, compute stats, output JSON
- [x] `tests/determinism.rs` — 5 tests: build+resolve determinism, rebuild determinism, inspect determinism, zero budget, empty query
- [x] `tests/exit_codes.rs` — 7 tests: missing cache (×2), invalid manifest (×2), nonexistent sources, --force rebuild, stderr prefix
- [x] Deleted `src/commands/ingest.rs` (not in v0 spec)
- [x] `spec_refs.md` — Updated with all governing specs

### Error Mapping (implemented)

| Error | Exit Code |
|-------|-----------|
| `CacheBuildError::Io(_)` | 6 (IO_ERROR) |
| `CacheBuildError::OutputExists(_)` | 6 (IO_ERROR) |
| `CacheBuildError::Serialization(_)` | 7 (INTERNAL_ERROR) |
| `CacheBuildError::FilenameCollision(_)` | 7 (INTERNAL_ERROR) |
| `CacheBuildError::DuplicateDocumentId(_)` | 5 (CACHE_INVALID) |
| `CacheBuildError::InvalidVersionFormat(_)` | 7 (INTERNAL_ERROR) |
| `SelectionError::InvalidBudget(_)` | 3 (INVALID_BUDGET) |
| `SelectionError::CacheError` | 5 (CACHE_INVALID) |
| `std::io::Error` (NotFound) | 4 (CACHE_MISSING) |
| `std::io::Error` (other) | 6 (IO_ERROR) |
| `serde_json::Error` (manifest) | 5 (CACHE_INVALID) |

---

## Test Results

```
tests/determinism.rs     5 passed
tests/exit_codes.rs      7 passed
────────────────────────────────
Total                   12 passed
```

---

## Remaining Work

### P1 — Functional gaps

- [ ] **Full cache verification for `inspect`** — Depends on context-core `verify_cache()` function (context-core P1). Until that exists, `valid` field in inspect output is approximate (checks file existence only, not hash verification or orphan detection).

### P1 — Enterprise Ingestion CLI (see `context-specs/plans/enterprise_ingest_plan.md` Phase 1)

- [ ] **Refactor `build` to use connector pipeline** — Replace direct walkdir logic with `FilesystemSource` + `ingest_from_source()`. All 12 existing tests must pass unchanged. Determinism: old path vs new path must produce identical caches.
- [ ] **`--source-type` flag** — Add to `build` command (default: `filesystem`). Dispatch to appropriate `DocumentSource` implementation.
- [ ] **`--connector-config <path>` flag** — JSON config file for connector-specific settings (auth tokens, endpoints, pagination). Validate at startup before ingestion begins.
- [ ] **Exit code mapping for `ConnectorError`** — Map connector errors to appropriate frozen exit codes (6 for IO/auth, 7 for internal).

### P2 — Test gaps

- [ ] **Build edge cases** — empty source dir, source dir with no `.md` files, source with non-UTF-8 files
- [ ] **Pretty vs JSON format test** — verify both produce parseable JSON, differ only in whitespace
- [ ] **Connector pipeline regression tests** — Verify `FilesystemSource` path produces byte-identical caches to pre-refactor `build`

### P3 — Post-v0

- [ ] **`context serve`** — Thin wrapper over `mcp-context-server` logic. Requires extracting server into a library crate first.
- [ ] **`context ingest`** — Separate ingestion step if intermediate format is needed.
- [ ] **Progress reporting** — For large builds, report progress to stderr.
- [ ] **`--verbose` flag** — Diagnostic output for debugging.

---

## File Inventory

```
context-cli/
├── Cargo.toml
├── progress.md
├── spec_refs.md
├── README.md
├── src/
│   ├── main.rs
│   ├── exit_codes.rs
│   └── commands/
│       ├── mod.rs
│       ├── build.rs
│       ├── resolve.rs
│       └── inspect.rs
└── tests/
    ├── determinism.rs
    └── exit_codes.rs
```

---

## Spec References

| Spec | Governs |
|------|---------|
| `interfaces/cli_spec.md` | Command list, arguments, exit codes, output format |
| `core/context_cache.md` | Cache structure, build process, verification checks |
| `core/context_selection.md` | Selection algorithm, scoring, ordering, budgeting |
| `core/document_model.md` | Document identity, versioning, metadata rules |
| `core/mcp/context.resolve.md` | Normative output schema (CLI and MCP must match) |
| `core/mcp/error_schema.md` | Error codes (mapped to exit codes) |
| `core/milestone_zero.md` | v0 scope and success criteria |
