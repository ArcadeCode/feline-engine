# Feline Engine, 2D/2.5D specialized game engine

Feline Engine is a modern game engine specialized for games with the following camera types:
- Top-down 2D (full view)
- 2D top-down with rotation (n° view)
- Isometric

Feline Engine is **intentionally focused on 2D/2.5D** to avoid GPU overdeveloped calcul for simple 2D spaces like in other engines where the 2D is considered as 3D space with 2D projection.

> [!WARNING] 
> A 3D version may exist in the future, but it will have a separate core to avoid using `Vector3` for 2D operations.

## Projets goals
### Main goals
- [ ] Developing a game engine in C++ and Rust which include :
  - A clock scheduler for main thread
  - A job scheduler for ECS parallelism calculation
  - Interfaces for communication between the engine and the user’s code such as `IAudioSource`, `IDrawable`, ...
  - An ECS system
  - A interoperability interface for coding ECS with Rust and C++.
- [ ] Developing a fully functional CI/CD pipeline for multi-languages testings, compilation, e2e, deployments.
- [ ] Developing a GUI tool for game developers with the following minimal tools :
  - Real-time rendering windows
  - Debugger with benchmarking
  - Ressources explorer
  - An ECS editor
> [!NOTE]
> This third goal isn't necessary. In fact this goal will be delayed after the first deploy-ready release. **GUI mustn't be obligatory for Feline to work !**

### Optional but recommended goals
- Provide a solid set of built-in ECS components to speed up the development of common gameplay features, such as:
  - Basic **physics-related components** (colliders, triggers, etc.)
  - **Tile-logic** — ECS components and systems designed for grid-based games (e.g., Factorio). This includes tile placement rules, adjacency logic, pathfinding helpers, and efficient map updates.
- Ensure the engine is **modular** enough to allow future-proof extensions, such as a dedicated `Physics` module for advanced operations or a `Visual Novel` module for narrative-driven games.
- Provide a simple **scripting language** or second-level interface to develop ECS logic faster, without requiring full C++/Rust expertise.

## Modules
These section explains all majors modules of the engine, by "major" we imply that this modules can be individually tested by unit-tests.

- `Render` — graphic rendering system based on [SDL3](https://github.com/libsdl-org/SDL) mounted on C++ wrapper for better clarity.
- `ECS Manager` — manages entities and components self-made in Rust for concurency security.
- `Game components` — ready-to-use ECS components for any game packed in the engine in Rust.
- [`Audio`](./audio/main.md) — audio playback and management based on [miniaudio](https://miniaud.io/) bridged with [SDL3](https://github.com/libsdl-org/SDL) mounted on C++ wrapper for better clarity.
- `Ressource manager` — Handler for importing and loading in RAM files who will be used by the engine.

## Tools and techs
### Used tool
Several tools will be used for this project:
- **Makefile**: To automate the entire pipeline.
- **CMake**: For C++ automation and C libary linking.
- **Cargo**: For Rust automation and testing.
- **LLVM**: For language linking.
- **Clang**: As C++, C and LLVM IR compilator.
- **Rustc**: As Rust compilator.
> [!TIP]
> The full pipeline [is documented here](./compilation.md)

### Librairies to use
- **SDL3** (C, dynamic): For window management and calling OpenGL/Vulkan to do more advances things, it will also manager the audio mixer.
- **miniaudio** (C, static): Manager audio playing, and send info to the mixer.
- **imGui** (C, static): Mainly for debugging purprose, will display on top of the SDL window informations such as framerate, tickrate, memory load, ... It will be directly linked to SDL3.

### Librairies to dev
- **feline_core.dll** (C, dynamic, necessary): Main librairy who contain tools to communicate between feline librairies and standalone tools such as the main allocator, tick manager, ECS manager or job manager. Without this librairie, Feline Engine cannot work.
- **feline_graphic.dll** (C, dynamic, unnecessary): Graphic interface and abstractions with SDL and graphic frameworks.
- **feline_audio.dll** (C, dynamic, unnecessary): Manage interface and abstactions with SDL mixer and miniaudio.

> [!NOTE]
> For optimization purprose, a game need to be compilable without unncessary librairies.

### Used tech
Several technologies will be used for this project :
- **C++17**: Mainly for the `filesystem` update.
- **C99**: Only because we use the same year version of C++.
- **Rust 2025**: For ECS and job sheduler modules.
- **SDL 3.2**: We use the 3.2.x because it's the latest at date of 2025/06/09.
- **Miniaudio v0.11.22**: We use the 0.11.22 because it's the latest at date of 2025/06/09.

## Game loops
Feline Engine follow this game loop :
1. 🕹️ Input handling
2. ⬛ Update entities
3. 📖 Update systems
4. 🌄 Render

> [!NOTE]
> In the future, this section will be more documented in a specific part of the official documentation with complete pipeline with functions names.