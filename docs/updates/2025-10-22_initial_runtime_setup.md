# Project Update: Javascript Runtime (Rust + deno_core)

## Goal

Build a lightweight Javascript runtime using Rust + deno_core, enabling:

- Users to write Javscript/Typescript that run on the backend.
- Integrate with legacy Java JAR logic (via a bridge).
- Future orchestration via a backend "control plane".

## Runtime domain Responsibility

- Manages: V8 isolates
- Handles: `db_query`, `http_fetch`, `legacy_jar_invoke`.
- Uses: **snapshots** for fast warm-start isolates (following cloudflare workers).

## Technical Highlights

### Why `deno_core`

- Provides a **low-level bridge to V8** without Deno's full runtime.
- Let me define **custom ops** and bootstrap logic freely.

### Found caveats with Rust and deno_core

- **Tooling load**
  - Running `rust-analyzer` can **block other cargo processes** (`build`, `check`) due to lock files or background indexing.
  - This is fine for DX, but on slower internet or older hardware it’s noticeable.
- **Tight crate coupling:**
  - `deno_core` and `deno_ast` (and related Deno ecosystem crates) are version-locked.
  - Updating one often means updating several and adjusting APIs (can become a larger refactor).

## Next Steps

1. Finalize **IsolatePool** implementation and resource limits.
2. Add first ops (`http.fetch`, `db.query`, `legacy.invoke`).
3. Integrate simple sidecar prototype (Java SPI `/invoke`).
4. Add basic metrics & logging for each isolate.
5. Prepare documentation diagram + module map for contributors.

*Authored by: Calvin Kamtoso
Date: 2025-10-22*
