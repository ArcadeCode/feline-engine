# TODO
This Todo file represent all phase of developpement of Feline Engine and the Feline Editor.

## Phase 1 : Design (current)
> This phase is only planning and designing comportements of each components.

- [X] [Answer design questions](./questions.md)
- [X] Choosing the bridge to comunicate states and data between components.
- [ ] Rendering design (see [render section](./render/main.md))
    - [ ] Define `Renderer` architecture and its interface with OpenGL backend
    - [ ] Design `Scene` structure and lifetime (creation, update, destruction)
    - [ ] Design `DrawableObject` base type and its subtypes (sprites, shapes, text)
    - [ ] Define culling strategy (viewport-based, frustum culling)
    - [ ] Define batching strategy (group by texture/shader)
    - [ ] Define post-processing pipeline (color grading, bloom, screen-space effects)
    - [ ] Decide on shader management (precompiled ? runtime compilation ?)
    - [ ] Decide on color space (JzAzBz vs classic RGB)
    - [ ] Define UI renderer integration (overlay on top of SDL3 window, imGui binding)
    - [ ] Design abstraction layer for potential future backends (OpenGL primary, extensible)
- [ ] Audio design (see [audio section](./audio/main.md))
    - [X] Define mixing responsibilities: miniaudio (playback & effects) + SDL3 (mixing)
    - [X] Design `IAudioSource` interface (position, direction, volume, repeat, resource handle)
    - [X] Design `IAudioEffect` interface (process, setParameter, getParameter)
    - [X] Design `AudioBuffer` (in-memory loading for short sounds)
    - [X] Design `AudioStream` (chunked streaming for long tracks, configurable chunk size)
    - [X] Design `AudioManager` singleton (play, stop, stop_all, master volume)
    - [ ] Replace generic `IAudioEffect::process` with miniaudio-native effect chain
    - [ ] Design static effect compilation workflow (pre-bake effects on asset files)
    - [ ] Define audio/gameplay synchronisation strategy (latency, clock alignment with GameLoop)
    - [ ] Define spatial audio model (2D positional audio, cone angle, omni vs directional)
- [ ] ECS system design (see [ECS section](./core/eda/main.md))
    - [X] Design EDA bus (`Service`, `Event`, `Subscriber` model)
    - [X] Define how to declare a custom `Event` (inheritance from `Event`)
    - [ ] Document how to register as a `Service` (event publisher)
    - [ ] Document how to register as a `Subscriber` (event listener)
    - [ ] Define component storage layout (SoA vs AoS vs archetypes)
    - [ ] Define entity identification strategy (64-bit ID with generation counter)
    - [ ] Define ECS/thread synchronization model (read/write locks, double buffering ?)
    - [ ] Design Job Scheduler DAG (dependency graph between ECS systems)
    - [ ] Define `Resource<T>` type contract (handle, resource ID, lifetime)
    - [ ] Define allocator strategy (arena, pool, per-subsystem allocators)
- [ ] Pipeline & build design (see [compilation pipeline](./compilation.md))
    - [X] Define multi-language compilation pipeline (C++ → LLVM IR via Clang, Rust → LLVM IR via rustc)
    - [X] Define LLVM IR linking strategy (llvm-link + opt global optimisations)
    - [X] Define cross-platform binary targets (Linux x86_64, Windows x86_64, macOS x86_64)
    - [ ] Define Makefile/CMake/Cargo automation structure
    - [ ] Define debug vs release vs shipping build configurations
    - [ ] Design CLI packaging tool (asset bundling, DLL management, `--dlls` flag)
- [ ] Open architecture questions to answer before Phase 2
    - [ ] What is a `Player` from a code perspective ?
    - [ ] What is a `Scene` from a memory perspective ?
    - [ ] How is the main loop controlled ? (deltaTime owner, synchronisation)
    - [ ] How is the engine lifecycle managed ? (init, update, shutdown hooks)
    - [ ] How are assets loaded ? (runtime streaming vs precompiled packages)
    - [ ] Does the engine need a unified log system ?
    - [ ] Does the engine need an integrated profiling system ?

## Phase 2 : Core writing
> During the second phase, only the code features will be written. A core feature is defined by whether the engine can or cannot run without this component.

- [ ] EDA system development (`feline_core`)
    - [ ] `Event` base class
    - [ ] `EDA` singleton bus (publish / subscribe)
    - [ ] Service registration API
    - [ ] Subscriber registration & callback dispatch
- [ ] GameLoop & TPS fixer (`feline_core`)
    - [ ] Fixed-timestep tick loop
    - [ ] deltaTime computation and exposure
    - [ ] Engine lifecycle hooks (init, update, shutdown)
    - [ ] Input handling step (step 1 of game loop)
- [ ] Type `Resource<T>` & Allocators (`feline_core`)
    - [ ] `Resource<T>` handle with resource ID
    - [ ] Arena allocator
    - [ ] Pool allocator
    - [ ] Per-subsystem allocator manager
    - [ ] `ResourceManager` (load, cache, unload assets)
- [ ] Render engine (`feline_graphic`)
    - [ ] Vulkan context initialisation & SDL3 window binding
    - [ ] `Renderer` class
    - [ ] `Scene` container
    - [ ] `DrawableObject` base type
    - [ ] Culling pass
    - [ ] Batching pass
    - [ ] Shader loading (SPIR-V precompiled)
    - [ ] Post-processing pass
    - [ ] imGui overlay binding (debug HUD: FPS, memory, tick rate)
- [ ] ECS system development (Rust, `feline_core`)
    - [ ] Component storage (chosen layout: SoA / archetypes)
    - [ ] Entity ID generation (64-bit + generation counter)
    - [ ] ECS array alignment
    - [ ] ECS system registration and execution order
    - [ ] Thread-safe read/write access to components
    - [ ] Job Scheduler
        - [ ] Job definition and submission API
        - [ ] DAG dependency resolution
        - [ ] Thread pool management
        - [ ] ECS system parallel dispatch
- [ ] Audio engine (`feline_audio`)
    - [ ] miniaudio initialisation bridged with SDL3 mixer
    - [ ] `AudioBuffer` implementation
    - [ ] `AudioStream` implementation (chunked loading)
    - [ ] `IAudioSource` implementation
    - [ ] `IAudioEffect` chain (miniaudio-native)
    - [ ] `AudioManager` singleton
    - [ ] 2D spatial audio (position, cone angle)

## Phase 3 : Core testing & optimisation
> This third phase is made to check if Phase 1 was successful. If some tests reveal undiscovered problems, we will fall back to Phase 1.

- [ ] EDA stress testing
    - [ ] High-frequency publish/subscribe throughput
    - [ ] Concurrent publishers and subscribers
    - [ ] Event ordering guarantees
- [ ] GameLoop stress testing
    - [ ] Fixed-TPS accuracy under load
    - [ ] deltaTime stability
    - [ ] Lifecycle hooks correctness (init → update → shutdown)
- [ ] `Resource<T>` & allocator testing
    - [ ] Memory leak checker
    - [ ] Stress allocations (arena, pool)
    - [ ] Error handling (out-of-memory, invalid handles)
    - [ ] ResourceManager cache hit/miss correctness
- [ ] ECS stress testing & error handling
    - [ ] Large entity count performance benchmark
    - [ ] Component add/remove under load
    - [ ] Thread-safety validation (race condition detection)
    - [ ] Entity ID recycling correctness
- [ ] Job scheduler stress testing & error handling
    - [ ] DAG resolution correctness (dependency ordering)
    - [ ] Deadlock detection
    - [ ] Thread pool saturation behaviour
    - [ ] Error propagation from failed jobs
- [ ] Render engine optimisation
    - [ ] Batching efficiency benchmarks (draw call count)
    - [ ] Culling correctness and perf under large scenes
    - [ ] Vulkan validation layer clean run (no errors, no warnings)
- [ ] Audio engine testing
    - [ ] Buffer vs Stream playback correctness
    - [ ] Effect chain processing correctness
    - [ ] Spatial audio accuracy
    - [ ] AudioManager thread-safety

## Phase 4a : Specific components development
> This phase consists of developing features not necessary for the engine to run.

- [ ] Audio engine extras (`feline_audio`)
    - [ ] Static Effect Compiler tool (pre-bake `IAudioEffect` chains onto asset files)
    - [ ] Audio bus system (BGM, SFX, UI channels with independent volumes)
- [ ] Resource-specific types (`feline_core`)
    - [ ] i18n resource type (localisation strings)
    - [ ] Tilemap resource type
    - [ ] Shader resource type
- [ ] Built-in ECS game components (`feline_core`)
    - [ ] Transform component (position, rotation, scale)
    - [ ] Collider / Trigger components (basic 2D physics)
    - [ ] Tile-logic components (grid placement, adjacency, pathfinding helpers)
    - [ ] Camera component (viewport, zoom)
- [ ] Lua scripting bridge
    - [ ] Lua → C++ transpiler (`.lua` → `.scrt.cpp`)
    - [ ] ECS manipulation API exposed to Lua (create/edit entities and components)
    - [ ] Static linking of compiled Lua scripts into game binary
- [ ] CLI packaging tool
    - [ ] Asset bundling (optimised packing)
    - [ ] Game compilation to executable + DLLs
    - [ ] `--dlls` flag for distributing shared libraries separately
    - [ ] Cross-platform packaging (Linux, Windows, macOS targets)

## Phase 4b : Feline Editor
> Feline Editor is a big toolbox made for developing games. This is a complete sub-project of Feline Engine. The editor must never be required for Feline Engine to function.

- [ ] Editor architecture design
    - [ ] Define editor / engine communication (EDA bus ? dedicated API ?)
    - [ ] Define editor build pipeline (standalone executable using `feline_graphic`)
- [ ] Real-time rendering window
    - [ ] Live scene preview using `Renderer`
    - [ ] Camera controls (pan, zoom)
- [ ] Debugger & benchmarking panel
    - [ ] FPS / tick rate monitor
    - [ ] Memory allocator usage display
    - [ ] Draw call counter
    - [ ] Job scheduler thread load view
- [ ] Resource explorer
    - [ ] Browse and import assets
    - [ ] Preview textures, audio files
    - [ ] Trigger static effect compilation from UI
- [ ] ECS editor
    - [ ] List entities and their components
    - [ ] Add / remove / edit components in real-time
    - [ ] System execution order visualiser
- [ ] Scene editor
    - [ ] Drag-and-drop `DrawableObject` placement
    - [ ] Entity hierarchy view
    - [ ] Scene save / load

## Phase 5 : Full stack testing
> Final phase — extensive testing before v1.0.0.

### Unit tests
> Full coverage of the engine. The engine must be fully typed and memory-safe.

- [ ] `feline_core` unit tests (EDA, GameLoop, Resource, ECS, Job Scheduler, Allocators)
- [ ] `feline_graphic` unit tests (Renderer, Scene, DrawableObject, culling, batching)
- [ ] `feline_audio` unit tests (AudioBuffer, AudioStream, IAudioSource, IAudioEffect, AudioManager)
- [ ] Rust modules unit tests via `cargo test` (ECS, Job Scheduler)
- [ ] C++ modules unit tests via GTest

### E2E Tests
> Testing human reactions such as plug-in and out multiple peripherals of the computer, and other human interactions.

- [ ] Peripheral plug/unplug (gamepad, keyboard, audio device) during runtime
- [ ] Window resize, minimise, alt-tab behaviour
- [ ] Multi-monitor handling
- [ ] Audio device switch during playback
- [ ] Cross-platform E2E (Linux, Windows, macOS binaries from LLVM pipeline)

### Full demo
> Build a small game using all features of the Engine to validate the full stack.

- [ ] Define demo game concept (isometric or top-down 2D)
- [ ] Use ECS + Job Scheduler for gameplay logic
- [ ] Use AudioManager with streaming BGM and buffered SFX
- [ ] Use Lua scripting for at least one system
- [ ] Package the demo with the CLI tool
- [ ] Run the demo on all 3 target platforms

## Phase 6 : Deployment & Future ideas
> End of the project. The engine runs autonomously at v1.0.0; all future features are additions.

- [ ] Publish `feline_core`, `feline_graphic`, `feline_audio` as versioned releases (`vX.Y.Za`)
- [ ] Write public-facing documentation (API reference, getting started guide)
- [ ] Set up CI/CD pipeline for automated multi-platform builds and E2E tests
    - [ ] C++ GTest pass → Clang LLVM IR
    - [ ] Rust `cargo test` pass → rustc LLVM IR
    - [ ] `llvm-link` + `opt` global optimisation
    - [ ] Platform binaries generation (Linux, Windows, macOS)
    - [ ] Automated E2E test suite execution
- [ ] Adding a Visual Novel template module.
- [ ] Adding a Polymorphic Stencil Compute Engine to compute Tilemap changes using a GPU.
- [ ] 3D extension exploration (not a short-term goal — see project.md warning).