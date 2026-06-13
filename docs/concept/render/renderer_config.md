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