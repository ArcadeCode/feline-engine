# ADR-005 | GameLoop in Rust with C ABI

**Statut** : Rejected
**Date** : 2026-06-18

## Context
The GameLoop is the central orchestrator of Feline Engine — it drives the tick system, synchronizes the Logic Loop and Render Loop, manages the `TickTimer` catch-up logic, and dispatches work to the Job Scheduler. Every subsystem depends on it either directly or indirectly.

The initial candidate was C, motivated by interoperability across the multi-language pipeline described in `compilation.md`. However, C is not the only language capable of exposing a stable cross-language ABI — Rust can expose an `extern "C"` interface that is binary-compatible with any C or C++ consumer, while retaining its safety guarantees internally.

## Decision
The GameLoop will be implemented in Rust, inside `feline_core`, and will expose a stable C ABI via `#[no_mangle] extern "C"` functions. This allows any part of the engine — C++, C, or future bindings — to call the GameLoop without friction, while keeping the implementation memory-safe and concurrency-sound.

```rust
#[no_mangle]
pub extern "C" fn gameloop_tick(state: *mut GameLoopState) -> TickResult {
    // ...
}

#[no_mangle]
pub extern "C" fn gameloop_init(config: *const GameLoopConfig) -> *mut GameLoopState {
    // ...
}
```

This pattern is idiomatic for Rust system libraries that need to remain callable from any language.

## Results
- The `TickTimer` catch-up logic, tick skipping, and loop synchronization are implemented under Rust's ownership and borrow checker — the compiler enforces the invariants that would otherwise be manual and error-prone in C.
- Full architectural coherence with the Job Scheduler, which is already in Rust inside `feline_core`. The GameLoop orchestrates the Job Scheduler directly, without a language boundary between them.
- The C ABI surface provides the same interoperability that motivated the original C decision, without sacrificing safety.
- The LLVM IR pipeline (`compilation.md`) is unaffected — Rust emits `.bc` directly via `rustc --emit=llvm-bc`, exactly like the Job Scheduler.

## Considered alternatives

- **C**: Provided the interoperability goal but at the cost of manual memory and state management on the most complex stateful component of the engine. The real interoperability mechanism in this pipeline is the C ABI, not the C language itself — Rust exposes the same ABI.
- **C++**: Easy OOP, already used in `feline_graphic` and `feline_audio`, but adds a compilation layer and an ABI that is less stable and less portable than `extern "C"`. The GameLoop does not need object-oriented design to work correctly.
- **Rust without C ABI**: Considered but rejected — a Rust-only interface would create friction for `feline_graphic` and `feline_audio` which are written in C++ and need to call into the GameLoop.