# AGENTS.md — VoxTech Coding Rules

This file defines non-negotiable rules for Codex/agents working in this repository.
Follow these rules by default unless the user explicitly overrides them.

---

## 0. Project context

VoxTech (VT) is a performance-sensitive voxel/sandbox project.
Correctness and performance matter, but changes should be as small and local as possible.

---

## 1. Non-negotiables (must follow)

- Rust edition: **Rust 2024**.
- **`unsafe` is prohibited by default.**
  - If you believe `unsafe` is required, STOP and ask for confirmation with:
    1) why safe Rust is insufficient,
    2) exact unsafe scope,
    3) safety invariants,
    4) how it will be tested/validated.
- Do **not** introduce new crates unless the user explicitly approves.
- Prefer **small, local changes** over large refactors.
- Always run / keep code compatible with:
  - `cargo fmt`
  - `cargo clippy --all-targets --all-features` (no new warnings)

---

## 2. Dependency policy (freshness)

- When suggesting crates/APIs, use up-to-date information.
  - If uncertain, say so and propose a safe default with minimal assumptions.
- Avoid trendy/fragile dependencies; prefer standard library solutions when practical.

---

## 3. Performance & memory rules

- Minimize heap allocations (`Box`, `Vec`, `String`) and re-allocations.
  - Prefer stack storage, fixed-size arrays, smallvec-like patterns **without adding deps**.
- Reuse buffers and memory where possible.
- Avoid hidden allocations in hot paths (formatting, collecting, cloning).
- Prefer data-oriented layouts (SoA) and cache-friendly iteration.
- Prefer explicit lifetimes/borrows over cloning.
- If you change a hot path, include a quick note on expected perf impact.

---

## 4. Output format (how to respond)

When asked to implement or modify code:

1) Brief plan (3–7 bullets).
2) Provide changes as a **unified diff** when feasible.
3) Include a short **validation checklist**:
   - commands to run (fmt/clippy/test/bench),
   - edge cases considered.

If something is ambiguous, ask a single focused question OR propose reasonable defaults
and clearly label them as assumptions.

---

## 5. Documentation & comments

- Use doc comments actively:
  - `//!` for module-level docs, `///` for items.
- Comments should be primarily **English**.
  - Leave a blank line and add a short Japanese supplement when helpful.

Example:

```rust
/// Updates the region dirty flags based on chunk dirty events.
///
/// 日本語補足: ChunkのDirtyを受けてRegion側の集計フラグを更新する。
```

---

## 6. Testing & benchmarks (preferred)

- Add tests when behavior changes:
  - unit tests for pure logic,
- property-like/random tests if it helps.
  - For performance-sensitive changes, prefer adding or updating a criterion benchmark
  - only if it already exists in the repo; otherwise propose a minimal bench approach.