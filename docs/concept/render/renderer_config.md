# Renderer configuration

The renderer configuration in Feline Engine allows developers to customize how graphics are rendered in their applications. This configuration includes settings for resolution, anti-aliasing, texture quality, and more.

Some of the configurable options can only be set at compile-time, while others can be adjusted at runtime through configuration files or in-game settings menus. The renderer always can recompile at runtime but will need to recreate the graphics pipeline which can be an expensive operation depending on the complexity of the scene and can lead to ticks loss during the recompilation process.

By using `reloadConfig()` method, the developer send a new configuration to the renderer at runtime, with an option to force immediate recompilation of the graphics pipeline. This allows the GameLoop to await for the renderer to finish the recompilation before continuing, ensuring that the new settings are applied without any rendering artifacts or inconsistencies.

```cpp
renderer.reloadConfig(NULL); // Reloads the configuration and recompiles the graphics pipeline when the engine is idle.
renderer.reloadConfig(NULL, forceImmediateRecompilation=true); // Forces immediate recompilation of the graphics pipeline. In this case the GameLoop interpolate the ticks during the recompilation process to avoid stuttering.
```

> [!NOTE] By using `NULL` instead of giving a configuration file or object, the renderer will reload the last used configuration. This can be used as a debug feature to reset the renderer settings to the last known good configuration.

All configurations in a renderer object are managed through the `RendererConfiguration` class, which provides an interface for setting and retrieving configuration options. Any `Renderer` instance contain a `RendererConfiguration` object that can be accessed and modified as needed. To get the current configuration of a renderer, you can use the `getConfig()` method. To set or update the configuration, you can use the `setConfig()` method, passing in a configuration object or file.

```cpp
// Example usage of RendererConfiguration
RendererConfiguration config = renderer.getConfig();
config.setResolution(1920, 1080);
renderer.setConfig(config);
```

## `RendererConfiguration` Class
The `RendererConfiguration` class encapsulates all the settings related to the renderer's behavior and performance. It provides methods to get and set various configuration options, allowing developers to fine-tune the rendering process according to their needs. It's basically a json-like class that can be serialized/deserialized to/from configuration files.

```cpp
enum class AntiAliasing {
    NONE,
    MSAA_2X,
    MSAA_4X,
    MSAA_8X,
    FXAA,
    TAA
};

class RendererConfiguration {
public:
    // Getters
    bool autoSaveConfigurationEnabled() const;
    Resolution getResolution() const;
    bool isVSyncEnabled() const;
    AntiAliasing getAntiAliasing() const;
    unsigned int getFrameRateLimit() const;
    Vector4 getClearColor() const;
    bool isWireframeModeEnabled() const;
    bool isPerformanceOverlayEnabled() const;
    bool isValidationLayersEnabled() const;
    bool isDebugOutputEnabled() const;
    bool isShaderDebuggingEnabled() const;
    bool isFrameCaptureEnabled() const;
    bool isGPUProfilingEnabled() const;
    bool isCPUProfilingEnabled() const;

    // Setters
    void enableAutoSaveConfiguration(bool enable);
    void setResolution(uint32_t width, uint32_t height);
    void enableVSync(bool enable);
    void setAntiAliasing(AntiAliasing aa);
    unsigned int setFrameRateLimit(unsigned int limit);
    void setClearColor(float r, float g, float b, float a);
    void enableWireframeMode(bool enable);
    void enablePerformanceOverlay(bool enable);
    void enableValidationLayers(bool enable);
    void enableDebugOutput(bool enable);
    void enableShaderDebugging(bool enable);
    void enableFrameCapture(bool enable);
    void enableGPUProfiling(bool enable);
    void enableCPUProfiling(bool enable);

    // Serialization/Deserialization
    // We use JSON format for configuration files
    void loadFromFile(const std::string& filename);
    void saveToFile(const std::string& filename) const;
};
```

## Example `RendererConfiguration` Usage
```cpp
#include "feline/graphics/renderer_configuration.h"
#include "feline/graphics/renderer.h"
// Create a renderer configuration object
feline::RendererConfiguration config;
config.setResolution(2560, 1440);
config.enableVSync(true);
config.setAntiAliasing(feline::AntiAliasing::TAA);
config.setFrameRateLimit(60);
config.setClearColor(0.1f, 0.1f, 0.1f, 1.0f);
// NOTE: SetClearColor need to accept float Vectror4 as well
config.enablePerformanceOverlay(true);
config.enableValidationLayers(true);

// Create a renderer instance and apply the configuration
feline::Renderer renderer;
renderer.setConfig(&config);
renderer.initialize();
// Now the renderer is configured and ready to use
rederer.renderScene();
// And at the end of the application, you can save the current configuration to a file
renderer.getConfig().saveToFile("renderer_config.json");
// But in case enableAutoSaveConfiguration was set to true, the configuration will be saved automatically on each end of the renderer loop :² configuration will be saved into the `gameFolder/config/renderer/{id}.json`
// The ID of the rendere is gave by feline Engine and will be restored automatically at the next engine initialization. We remind you than Feline Engine handle Ressource management for the developer.
```


--- OLD ---

## Compile-time settings
> These settings are defined before compiling the engine and cannot be changed at runtime without recompiling.
- **Graphics API**: Choose between different graphics APIs such as Vulkan, OpenGL, or DirectX. This choice is typically made at compile-time to ensure optimal performance and compatibility.
    - The engine will primarily support OpenGL as the main graphics API due to its portability and performance benefits. However, an abstraction layer will be provided to allow for easier adaptation to other graphics APIs if needed.
- **Shader Model**: Define the shader model version to be used, which affects the features available for rendering.
- **Optimization Levels**: Set optimization levels for the renderer to balance performance and quality.
    - The engine will provide several optimization levels (e.g., low, medium, high) to allow developers to choose the best balance between performance and visual quality.
    - These levels will adjust settings such as texture resolution, shadow quality, and post-processing effects.
- **Feature Flags**: Enable or disable specific rendering features (e.g., ray tracing, tessellation) based on the target platform capabilities.
- **Debugging Options**: Include options for debugging graphics issues, such as enabling validation layers or debug output.

## Runtime settings
> These settings can be adjusted while the application is running, allowing for dynamic changes to rendering quality and performance.
- **Resolution**: Change the screen resolution to fit different display sizes or user preferences.
- **Anti-Aliasing**: Adjust the level of anti-aliasing to improve visual quality or performance.
- **Texture Quality**: Modify texture quality settings to balance performance and visual fidelity.
- **Post-Processing Effects**: Enable or disable post-processing effects such as bloom, motion, blur, and depth of field.
- **V-Sync** (boolean): Toggle vertical synchronization to prevent screen tearing. By default, V-Sync is disabled to allow for maximum frame rates.
- **Frame Rate Limit** (uint16): Set a maximum frame rate to reduce power consumption or heat generation, by default it is set to 0 which means unlimited frame rate.

## Configuration settings
Here is the list of all settings available for the renderer configuration:
- **graphics_api** (string): The graphics API to use (e.g., "OpenGL", "Vulkan", "DirectX"). Default is "OpenGL".
- **shader_model** (string): The shader model version (e.g., "5.0", "6.0"). Default is "5.0".
- **resolution** (object): An object containing `width` (uint32) and `height` (uint32) properties to set the rendering resolution. Default is `{ "width": 1920, "height": 1080 }`.
- **anti_aliasing** (string): The anti-aliasing method to use (e.g., null, "MSAA_2X", "MSAA_4X", "MSAA_8X", "FXAA", "TAA"). Default is "null".
- **v_sync** (boolean): Enable or disable V-Sync. Default is `false`.
- **frame_rate_limit** (uint16): The maximum frame rate limit (0 for unlimited). Default is `0`.
- **clear_color** (object): An object containing `r`, `g`, `b`, and `a` properties to define the clear color used to clear the screen before rendering each frame. Default is `{ "r": 0, "g": 0, "b": 0, "a": 1 }`.
- **post_processing_effects** (list of object): An array of post-processing effects objects settings such as `bloom`, `motion_blur` and `depth_of_field`. All this effects are disabled by default. Each effect object can have its own properties to configure its behavior herited from `feline::graphic::VisualEffect`. See [Visual Effects](./visual_effects.md) for more details.
- **debugging_options** (object): An object containing debugging options such as `wireframe_mode` (boolean) and `performance_overlay` (boolean). Default is `{ "wireframe_mode": false, "performance_overlay": false }`.

### Settings/debugging_options
The `debugging_options` setting allows developers to enable or disable specific debugging features to help diagnose rendering issues. This setting need to be set at compile-time only.
- **wireframe_mode** (boolean): When enabled, the renderer will display all objects in wireframe mode, allowing developers to see the underlying geometry of the scene. Default is `false`.
- **performance_overlay** (boolean): When enabled, the renderer will display a performance overlay on the screen, showing real-time statistics such as frame rate, draw calls, and memory usage. Default is `false`.
- **validation_layers** (boolean): When enabled, the renderer will use graphics API validation layers to check for errors and provide detailed debugging information. Default is `false`.
- **debug_output** (boolean): When enabled, the renderer will output debug messages to the console or log file, providing insights into rendering operations and potential issues. Default is `false`.
- **shader_debugging** (boolean): When enabled, the renderer will provide additional debugging information for shaders, such as compilation errors and performance metrics. Default is `false`.
- **frame_capture** (boolean): When enabled, the renderer will allow capturing frames for analysis, useful for debugging rendering issues. Default is `false`.
- **gpu_profiling** (boolean): When enabled, the renderer will collect GPU profiling data to help identify performance bottlenecks. Default is `false`.
- **cpu_profiling** (boolean): When enabled, the renderer will collect CPU profiling data to help identify performance bottlenecks. Default is `false`.

## Compatibility notes
During the development of Feline Engine, certain features and settings may be limited based on the version of the engine. By that we mean that versioning of the engine need to specifically state which features are supported in which version. This is particularly important for features that may not be fully implemented or stable in earlier versions of the engine.