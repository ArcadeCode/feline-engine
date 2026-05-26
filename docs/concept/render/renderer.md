# What's is the ``Renderer``?

The `Renderer` is the core component responsible for managing the rendering process in Feline Engine. It handles the drawing of all graphical elements, manages the rendering pipeline, and ensures that visual assets are displayed correctly on screen. The `Renderer` provides a high-level API for developers to interact with the rendering system, allowing them to create scenes, add drawable objects, and control rendering settings. It abstracts away the complexities of the underlying graphics API, making it easier for developers to focus on creating their games without worrying about low-level rendering details.

## Lifecycle
1. **Creation**: The `Renderer` is instantiated, initializing the rendering context and backend.
2. **Configuration**: The `Renderer` is configured with settings using `loadConfiguration()` such as resolution, V-Sync, and other rendering options.
3. **Initialization**: The `Renderer` sets up a `Scene` and necessary resources, shaders, and pipelines required for rendering using `initialize()`.
4. **Rendering Loop**: The `Renderer` enters the main rendering loop, where it processes and renders the scene continuously using `renderScene()`.
5. **Cleanup**: Upon application termination or when the `Renderer` is no longer needed, it cleans up resources and shuts down the rendering backend.

> [!NOTE] The Renderer configuration must be set before initialization. Changing configuration after initialization requieres using `reloadConfig()` which may lead to performance hits due to graphics pipeline recreation. See more information on [Renderer Configuration](./renderer_config.md).
>
> [!NOTE] The `initialize()` method can be called without calling `setConfiguration()`, in this case the renderer will use default configuration settings.
>
> [!NOTE] Initializing multiple Renderer instances is supported but only one Renderer can be active at a time. See more information on [What happen when there is multiple renderers called?](#what-happen-when-there-is-multiple-renderers-called)

## Responsibilities
The Renderer is responsible for:
- Executing the rendering pipeline
- Submitting draw calls to the graphics backend
- Managing render passes and frame lifecycle

The Renderer is NOT responsible for:
- Game logic
- Scene ownership
- Asset loading

## Basic Usage
1. First establish a `Renderer` instance.
2. Create a `Scene` and populate it with drawable objects.
> [!NOTE] If none `Scene` is created, the renderer will call a custom Scene internally to avoid crashing. This special Scene will be empty and will not render anything, her signature is `NullScene`. NullScene is used when the developer forgets to create a Scene or wants to clear the current Scene. and always return a critical in the log to inform the developer that no Scene was provided.
3. Call the `renderScene()` method on the `Renderer` instance, passing the `Scene` to be rendered.

```cpp
#include "feline/graphics/renderer.h"

// Create a renderer instance
feline::Renderer renderer;
// Initialize the renderer with default configuration
// Here a Scene is created internally.
renderer.initialize();

// Populate the scene with drawable objects
// ... (add objects to the scene)
feline::DrawableObject obj;
renderer.scene.addDrawable(obj);

// The scene will be rendered to the screen at anytime the render loop is called.
// To stop rendering, simply clear the scene or delete call an objects.
renderer.clearScene();
// or
obj.destroy();
```

## Custom Scenes
Developers can create custom `Scene` instances to manage their drawable objects. The `Renderer` can render any `Scene` provided to it. Here's how to create and use a custom `Scene`:
1. First, create a `Scene` instance.
2. Populate the `Scene` with drawable objects.
3. Pass the custom `Scene` to the `loadScene` method of the `Renderer`.

```cpp
#include "feline/graphics/renderer.h"
#include "feline/graphics/scene.h"

// Create a custom scene
feline::Scene customScene;
// You can populate the scene with drawable objects here or later
// ... (add objects to the custom scene)

// Create a renderer instance
feline::Renderer renderer;
// Set the custom scene to the renderer
renderer.loadScene(customScene);
// Initialize the renderer with default configuration
renderer.initialize();

// You also can add drawable objects after loading the scene
feline::DrawableObject obj;
customScene.scene.addDrawable(obj);

// Finally render the custom scene
renderer.renderScene();
```
### Associated Methods
- `initialize()`: Initializes the renderer and prepares it for rendering.
- `loadScene(&feline::Scene)`: Loads custom scene into the renderer.
- `reloadScene(&feline::Scene, bool forceImmediateRecompilation)`: Reloads a new scene into the renderer at runtime.
- `renderScene(&feline::Scene)`: Renders the provided scene.
- `clearScene()`: Clears all drawable objects from the current scene.

### Associated errors
- `[CRITICAL] No Scene provided to Renderer. Using NullScene which is empty. Please provide a valid Scene to render.`: This error occurs when the `Renderer` is initialized without a valid `Scene`. The renderer will use an empty `NullScene` to avoid crashing, but the developer should provide a proper `Scene` for rendering.
- `[ERROR] Failed to load Scene into Renderer. Please ensure the Scene is valid and properly initialized.`: This error occurs if there is an issue loading the provided `Scene` into the `Renderer`. The developer should check the `Scene` for validity and initialization before loading it.
- `[ERROR] Already initialized Renderer cannot load a new Scene. Please reload the Renderer or use reloadScene() method.`: This error occurs when attempting to load a new `Scene` into an already initialized `Renderer`. The developer should either reload the entire `Renderer` or use the `reloadScene()` method to change the scene at runtime.

--- 

## What happen when there is multiple renderers called?
When multiple `Renderer` instances are created and used within the same application, each instance operates independently, managing its own rendering context and resources. However, only one `Renderer` should be actively rendering to the screen at any given time to avoid conflicts and ensure proper display of graphics. If multiple `Renderer` instances are used, developers should ensure that they coordinate their rendering calls appropriately, typically by designating one `Renderer` as the primary renderer for the main game loop while others may be used for off-screen rendering or specific tasks. To do this, the developer can use the `setActive()` method on the desired `Renderer` instance to make it the current active renderer before calling the `renderScene()` method.
```cpp
#include "feline/graphics/renderer.h"

feline::Renderer renderer1; // This renderer is active by default because it's the first created
renderer1.setActive(); // Set renderer1 as the active renderer
// Because renderer1 is already active, this call is optional here and will lead to an log :
// [INFO] Renderer 'renderer1' is already active. But setActive() was called again.

feline::Renderer renderer2;
renderer2.setActive(); // Now renderer2 is the active renderer
// By calling setActive() on renderer2, it becomes the current active renderer. Any other renderer instances will not render to the screen until they are set as active again.

// If we send a event reader on the same tick :
feline::eda::OnTickEvent::registerListener([&]() {
    // WARNING : NOT THE FINISH EDA SYSTEM IMPLEMENTATION
    renderer1.setActive(); // renderer1 is now active
    renderer2.setActive(); // renderer2 is now active again
});
// In this case technically, only renderer2 will be active at the end of the tick, as it was the last one to call setActive(). But to ensure proper rendering behavior feline engine will send an critical warning in the log to inform the developer that multiple renderers were set active in the same tick, which may lead to unexpected rendering results. And rederer2 will be the only one rendering to the screen.
// The critical warning will be like this :
// [CRITICAL] Multiple Renderer instances were set active in the same tick. Only the last active renderer will render to the screen. Please ensure that only one renderer is set active per tick to avoid rendering conflicts.
```

## Renderer Configuration
The `Renderer` can be configured with various settings to customize its behavior and performance. Some common configuration options include:
- **VSync**: Enable or disable vertical synchronization to prevent screen tearing.
- **Resolution**: Set the rendering resolution for the output display.
- **Fullscreen Mode**: Toggle between windowed and fullscreen rendering modes.
- **Anti-Aliasing**: Configure anti-aliasing settings to improve visual quality.
- **Frame Rate Limit**: Set a maximum frame rate for rendering to control performance.
- **Clear Color**: Define the color used to clear the screen before rendering each frame.
- **Debugging Options**: Enable debugging features such as wireframe mode or performance overlays.
- **Shader Quality**: Adjust the quality settings for shaders to balance performance and visual fidelity.

> [!NOTE] See the [Renderer Configuration API](./renderer_config.md) for more details.

