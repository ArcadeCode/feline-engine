# ADR-006 | C++ for `<feline_core>`

**Statut** : Accepted
**Date** : 2026-07-22

## Context
The "core" of the engine is the main part of it, a bad implementation can cause an unusable laggy program. The first choice was to use Rust for this sensible system but Rust was too difficult to handle and I was too new to Rust to be able to develop an library this big.

## Decision
It was a little tricky to decide due to my fear of memory bugs in C and C++ but it was decided to use C++ for the core of Feline engine instead of Rust.

## Consequences
- Rework required across `docs/concept/` to reflect C++ as the current core language.
- The Rust code already written (`src/gameloop/*.rs`, `src/main.rs`, `Cargo.toml`) is preserved in the `archive/rust-core` branch for reference during a future migration, and removed from `main`.
- The C++/Rust interoperability pipeline (LLVM IR linking, see [compilation.md](../concept/compilation.md)) is deferred, not abandoned. It remains a target for future partial migration of the core (Job Scheduler, ECS) once Rust proficiency allows it.

### Isolation strategy
Because `<feline_core>`, `<feline_graphic>` and `<feline_audio>` already exist as separate libraries, the core must be treated as an independently replaceable unit. Two stable bridges are established between the core and the rest of the engine, so that a future language migration (partial or full) does not require rewriting the calling code:

- **Event bridge (EDA)** — all asynchronous notifications and orchestration (`TickStartedEvent`, `JobRequestEvent`, `ResourceLoadEvent`, ...) go through the existing EDA bus. Since `Event` types rely only on a primitive `EventID`, this bridge is trivially portable across languages via FFI.
- **Direct API bridge (flat C interface)** — synchronous, high-frequency calls (allocator, math primitives, ECS component access) go through an `extern "C"` interface with no exposed C++ templates or classes. This keeps the ABI stable regardless of the implementation language behind it.

No module outside the core may access core-internal data structures directly, through either bridge. This constraint is what will make a future Rust migration of specific components (Job Scheduler, ECS) possible without breaking `feline_graphic`, `feline_audio`, or gameplay code.

Migration, if it happens, will proceed **component by component** — never as a full core rewrite — starting with the Job Scheduler and ECS, the two components where Rust's concurrency safety guarantees matter most.

## Considered alternatives
- Rust was the first one, but as mentioned above this language is too complicated for me to work with now.
- C, but the Object-Oriented Programming tools that C++ offers are far more interesting.
- Zig was the last option I see 'cause the other languages I've heard about seem less fast, but I know less in Zig than Rust and it seems more complicated, so also rejected.