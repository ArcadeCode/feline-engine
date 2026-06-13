# ADR-003 — Kitty: custom multi-method allocator

**Status**: Experimental  
**Date**: 2026-06-13

## Context
The engine needs a memory allocation strategy for ECS components, Chunks,
and graphic resources. A custom allocator would allow optimizing each use
case (pool for Tile/Chunk, stack for frame allocations, etc.) but represents
a significant technical investment for a solo project.

## Decision
We attempt the implementation of Kitty as a custom multi-method allocator,
starting with a simple pool allocator for ECS components.

This decision also sustain the educational goal of the engine: learning how to implement a custom allocator.

The ResourceManager abstracts the allocator as a **compile-time template
parameter** — zero runtime overhead, no vtable, no indirect call:

```cpp
template<typename Allocator = KittyPoolAllocator>
class ResourceManager { ... };
```

This abstraction exists for two reasons:
- **Testability**: inject a tracing allocator in tests to detect leaks
  and measure fragmentation without touching production code.
- **Fallback**: if Kitty proves too costly to implement in Phase 2/3,
  switching to mimalloc requires changing a single type alias.

## Consequences
- The ResourceManager interface must be designed around this template
  pattern from the start — not retrofitted later.
- If Kitty implementation fails, **mimalloc** (MIT license) is the
  identified fallback — drop-in replacement, no refactor needed.
- Kitty starts small: one allocator type (pool), one use case (ECS
  components). Complexity is added only when a concrete need arises.

## Alternatives considered
- **mimalloc from the start**: safer short-term, less educational.
- **jemalloc**: relevant but harder to integrate on Windows.
- **Runtime polymorphism (virtual)**: considered and rejected — vtable
  overhead on a hot path is unacceptable.