# Feline Engine, a game engine focused around 2D/2.5D views.

Feline Engine is a modern game engine specialized for games with the following camera types:
- Top-down 2D (full view)
- 2D top-down with rotation (n° view)
- Isometric

By specialized we imply delivering simple tools for this type of camera type. But Feline isn't just an simple 2D engine. He also handle 3D games.

> [!WARNING]
> Feline engine isn't an 2D game engine, it's an 2D/3D game engine who support modern rendering systems.

## Projets goals
### Main goals
- [ ] Developing a game engine which include :
    - A **clock scheduler** for main thread
    - A **job scheduler** for parallelizing ECS updates and other tasks.
    - Abstraction layer for helping developer's communicate with the engine such as `IAudioSource`, `IDrawable`, ...
    - An complete ECS system
    - A ressource manager for loading and caching assets.
    - A allocator manager for better memory management.
    - A rendering system based on SDL3 and OpenGL/Vulkan*.
    - An audio system based on miniaudio and SDL3 mixer.
- [ ] Developing a fully functional CI/CD pipeline for multi-languages testings, compilation, e2e, deployments.
- [ ] Developing a GUI tool (Feline editor) for game developers with the following minimal tools :
  - Real-time rendering windows
  - Debugger with benchmarking
  - Ressources explorer
  - An ECS editor
> [!NOTE]
> This third goal isn't necessary. In fact this goal will be delayed after the first deploy-ready release. **GUI mustn't be obligatory for Feline to work !**

> [!NOTE] Asterix
> For now, Feline Engine will use OpenGL but, we will used an abstraction layer and migrate to Vulkan later. This assure multiple good points :
> - Vulkan is hardly more complex than OpenGL, using Vulkan before the first release will make the release take a lot more time.
> - Using OpenGL & Vulkan will learn me everything I need on Graphic Backend and GPU development with a good difficulty progression.
> - This force me to use an abstraction layer between the backend and the engine which is an ideal idiomatic OOP schema.

### Optional but recommended goals
- Provide a solid set of built-in ECS components to speed up the development of common gameplay features, such as:
  - Basic **physics-related components** (colliders, triggers, etc.)
  - **Tile-logic** — ECS components and systems designed for grid-based games (e.g., Factorio). This includes tile placement rules, adjacency logic, pathfinding helpers, and efficient map updates.
- Ensure the engine is **modular** enough to allow future-proof extensions, such as a dedicated `Physics` module for advanced operations or a `Visual Novel` module for narrative-driven games.
- Provide a simple **scripting language** or second-level interface to develop ECS logic faster, without requiring full C++/Rust expertise.

## Modules
These section explains all majors modules of the engine, by "major" we imply that this modules can be individually tested by unit-tests.

- `Ressource Manager` — Handler for importing and loading in RAM files who will be used by the engine.
- `ECS Manager` — manages entities and components self-made in Rust for concurrency security.
    - `Game components` — ready-to-use ECS components for any game packed in the engine.
- `Job Scheduler (Manager)` — [manages the execution of jobs in parallel](./core/job/job.md), with a focus on ECS updates.

- `Render System` — graphic rendering system based on [SDL3](https://github.com/libsdl-org/SDL) mounted [on abstracted layer](./render/render.md).
- [`Audio System`](./audio/audio.md) — audio playback and management based on [miniaudio](https://miniaud.io/) bridged with [SDL3](https://github.com/libsdl-org/SDL) mounted on C++ wrapper for better clarity.

## Tools and techs
### Used tool
Several tools will be used for this project:
- **Makefile**: To automate the entire pipeline.
- **CMake**: For C++ automation and C library linking.
- **LLVM**: For project linking.
- **Clang**: As C++, C and LLVM IR compilator.
> [!TIP]
> The full pipeline [is documented here](./compilation.md)

### Librairies to use
- **OpenGL** (C, static): For the Graphic Backend
- **SDL3** (C, dynamic): For window management and calling OpenGL/Vulkan to do more advances things, it will also manager the audio mixer.
- **miniaudio** (C, static): Manager audio playing, and send info to the mixer.
- **imGui** (C, static): Mainly for debugging purpose, will display on top of the SDL window informations such as framerate, tickrate, memory load, ... It will be directly linked to SDL3.


### Librairies to dev
- **feline_core.dll** (Rust, dynamic, necessary): Main library who contain tools to communicate between feline librairies and standalone tools such as the main allocator, tick manager, ECS manager or job manager. Without this librairie, Feline Engine cannot work. Some part of this librairie will be made in Rust for better performance memory and allocation security, but the main interface will be in C for better integration with the rest of the engine.
- **feline_graphic.dll** (C++, dynamic, unnecessary): Graphic interface and abstractions with SDL and graphic frameworks.
- **feline_audio.dll** (C++, dynamic, unnecessary): Manage interface and abstractions with SDL mixer and miniaudio.

> [!NOTE]
> For optimization purpose, a game need to be compilable without unnecessary librairies. This mean than `<feline_audio.dll>` and `<feline_graphic.dll>` are optionals, but without you never will hear something or see anything.

### Used tech
Several technologies will be used for this project :
- **C++17**: Mainly for the `filesystem` update.
- **C99**: Only because we use the same year version of C++.
- **Rust 2025**: For ECS and job scheduler modules.
- **SDL 3.2**: We use the 3.2.x because it's the latest at date of 2025/06/09.
- **Miniaudio v0.11.22**: We use the 0.11.22 because it's the latest at date of 2025/06/09.

> [!WARNING]
> SDL we surely be removed, the current objective is an direct access to an graphic api.

## Game loops
Feline Engine follow this game loop :
1. 🕹️ Input handling
2. ⬛ Update entities
3. 📖 Update systems
4. 🌄 Render

> [!NOTE]
> In the future, this section will be more documented in a specific part of the official documentation with complete pipeline with functions names.