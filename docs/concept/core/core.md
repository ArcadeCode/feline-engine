# Core design

The code under the `<feline_core>` library, without this library none project can run. The code contain all main computing tools and sensible functions of the engine. A bad implementation can affect the performance of every component of the engine.

## Languages
The core is mainly write in C++, see [ADR-006](../../adr/ADR-006-cpp-for-the-core.md) for more informations.

During compilation, the core is compile to LLVM IR following the [compilation.md](../compilation.md) and we expose an api bridge (flat C interface).

## Core implemented components
- **Gameloop** : The Gameloop is the center of everything, the loop call all logics, prepare interpolations of ticks. It also manage when the rendered is called.
- **Event bridge (EDA)** : Handle all asynchronous notifications and orchestration.
- **Job scheduler** : Manages the execution of jobs in parallel, with a focus on ECS updates. Currently implemented in C++ alongside the rest of the core. It is the primary candidate for a future partial migration to Rust (see [ADR-006](../../adr/ADR-006-cpp-for-the-core.md)), due to the memory-safety guarantees Rust's ownership model offers for concurrent workloads.
- **Entity System Component (ECS)** : This core component handle how game data are stored and use.