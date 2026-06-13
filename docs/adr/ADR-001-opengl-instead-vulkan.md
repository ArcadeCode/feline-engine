# ADR-001 | OpenGL instead of Vulkan

**Statut** : Accepted  
**Date** : 2026-06-13

## Context
I want to use the Vulkan Graphic Api for his power and GPU bound but Vulkan add a lot of complexity and I  never experienced handling Graphic API.

## Decision
We start with OpenGL 4.6 core profile. The abstraction `IRenderer` in feline_graphic is made for allowing a transition to Vulkan in Phase 5+ without editing the engine public API.

## Results
- Limited MacOS support (OpenGL deprecated since Catalina) — acceptable in Phase 2.
- We learn the abstraction of graphic API in two steps: first the what (OpenGL), then the how to migrate (Vulkan). This is a pedagogical advantage.
- If we never migrate to Vulkan, it is not a failure — OpenGL is enough for all project goals.

## Considered alternatives:
- Vulkan from the start : too costly in learning time in Phase 2.
- SDL_GPU / WebGPU : interesting but external dependencies not wanted.