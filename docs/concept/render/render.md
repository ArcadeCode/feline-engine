# Rendering in Feline Engine
Feline Engine is a 2D engine, in the final transform, the GPU send an simple pixel matrix.

We remind that Feline Engine is designed to be a lightweight and efficient 2D game engine, who is based on OpenGL backend for rendering. The rendering system is responsible for drawing graphics on the screen, managing visual effects, and handling various graphical assets.

Also Feline engine will help the developer to manage the rendering pipeline, including tasks such as:
- Setting up the rendering context and window.
- Managing graphical resources (textures, sprites, shaders).
- Handling rendering loops and frame updates.
- Providing an abstraction layer for different graphics APIs (initially OpenGL, with potential support for Vulkan in the future).
- Optimizing rendering performance through techniques like batching and culling.
- Supporting 2D transformations (translation, rotation, scaling).
- Implementing basic lighting and shading effects suitable for 2D graphics.
- Facilitating the integration of UI elements and overlays.
- Providing debugging tools for visualizing rendering performance and issues.
- Supporting multiple platforms (Windows, Linux, macOS) with consistent rendering behavior.
- Offering extensibility for custom rendering effects and shaders.
- Facilitating the loading and management of graphical assets through an asset pipeline.
But the developer will have the freedom to implement more advanced rendering techniques if needed.

## What's is the `IGraphicBackend`?
The `IGraphicBacked` is an class who handle communication with the graphic backend, this component exist to give the opportunity to easily switch backend without recoding the complete `feline_graphic.dll` library.

## What's is the `IRenderer`?
The `IRenderer` is an essential component of the Feline Engine responsible for managing the rendering process. It acts as an interface between the engine and the underlying graphics API (OpenGL) to facilitate the drawing of 2D graphics on the screen.

See [Renderer](./renderer.md) for more details.

## What's is a `Scene` ?
The `Scene` in Feline Engine represents a collection of drawable objects, lights, and cameras that define what is to be rendered on the screen. It serves as a container for all visual elements in a particular level or environment of a game.

See [Scene](./scene.md) for more details.


### What's is a `DrawableObject` ?
The `DrawableObject` is a fundamental class which essentially include "Drawable trait" in Feline Engine that represents any object that can be rendered on the screen. This includes sprites, shapes, text, and other graphical elements. To be drawable is to be see in the rendering process.

#### Commons DrawableObject
Feline Engine offer some common `DrawableObject` such as :
- Plane, Cube, Polygon, Sphere.
- Mesh, Mesh2D, Mesh3D.
- TextMesh, TextShader

Plane, Cube, Polygon & Sphere are just group of polygon. Mesh, Mesh2D & Mesh3D represent loaded complex objects. Finally TextMesh represnt a text has a mesh  and TextShader represent a Text has a shader on a plane.


## Pipeline
The rendering pipeline in Feline Engine is designed to be efficient and flexible, allowing developers to create visually appealing 2D graphics with ease. The pipeline consists of several stages:
1. **Scene Setup**: Developers create a scene by adding drawable objects, such as sprites, shapes, and text. Each object has properties like position, size, rotation, and texture.
2. **Culling**: Before rendering, the engine performs culling to determine which objects are visible within the camera's viewport. Objects outside the viewport are skipped to optimize performance.
3. **Batching**: The engine groups drawable objects that share similar properties (like textures and shaders) into batches. This reduces the number of draw calls sent to the GPU, improving rendering efficiency.
4. **Rendering**: The engine processes each batch, applying the appropriate shaders and rendering the objects to the screen. The rendering process involves transforming object coordinates to screen space, applying textures, and executing shader programs.
5. **Post-Processing**: After rendering the scene, the engine can apply post-processing effects, such as color grading, bloom, and screen-space effects, to enhance the visual quality of the final output.
6. **Output**: The final rendered image is presented on the screen, completing the rendering cycle.