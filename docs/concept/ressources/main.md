# `Ressource<RessourceType>` concept

The `Ressource<RessourceType>` concept in Feline Engine is designed to manage various types of resources used in 2D rendering, such as textures, fonts, and shaders. This concept provides a unified way to handle different resource types while ensuring type safety and efficient resource management.

## Overview
A `Ressource` is a template class that takes a `RessourceType` as a template parameter. Every 'Ressource' are manager by the `RessourceManager` class, which is responsible for loading, unloading, and caching resources to optimize performance and memory usage. This manager ensures that resources are only loaded once and shared across different parts of the engine as needed.

### `RessourceType` enum class
The `RessourceType` enum class defines the different types of resources that can be managed by the `Ressource` concept. It includes the following types:
- `Texture`: Represents image resources used for rendering sprites and backgrounds. It can be in various formats : PNG, JPEG, BPM, SVG.
- `Atlas`: Represents texture atlas resources used for optimizing multiple textures into a single image. It use a custom format `.fe.atlas`.
- `SpriteSheet`: Represents sprite sheet resources used for animations. It use a custom format `.fe.spritesheet`.
- `Animation`: Represents animation resources used for defining sequences of frames for animated sprites. It use a custom format `.fe.animation`.
- `Font`: Represents font resources used for rendering text. It can be in various formats : TTF, OTF, WOFF.
- `Material`: Represents material resources used for defining surface properties of 3D models. It use a custom format `.fe.material`.
- `Shader`: Represents shader programs used for rendering effects. It can be in various formats : GLSL, HLSL.
- `Audio`: Represents audio resources used for sound effects and music. It can be in various formats : WAV, MP3, OGG.
- `Config`: Represents configuration files used for storing settings and preferences. It can be in various formats : JSON, XML, YAML, CSV. See more for specific configuration files :
    - `RendererConfig`: [Renderer configuration](../render/renderer_config.md)
- `Localization`: Represents localization files used for supporting multiple languages. It can be in various formats : JSON, XML, YAML, CSV.
- `Binary`: A fallback type for unrecognized resource types. In this case, the `RessourceManager` will not attempt to compress or optimize the resource. And will load it as binary file.
- `Null` : Represents a null resource type, indicating the absence of a resource but contain headers of a ressource.
- `Mesh (future proof)`: Represents 3D mesh resources used for rendering 3D models. It can be in various formats : OBJ, FBX, GLTF.
    > [!WARNING] The `Mesh` resource type is planned for future implementation and is not currently supported.

The final signature :
```cpp
enum class RessourceType {
    TEXTURE,
    ATLAS,
    SPRITESHEET,
    ANIMATION,
    FONT,
    MATERIAL,
    SHADER,
    //MESH,
    AUDIO,
    CONFIG,
    LOCALIZATION,
    BINARY,
    NONE
}
```

> [!NOTE]
> All formats listed here is supported or will be supported in the future.

> [!NOTE]
> FE need a parser for all dictionary oritented formats (JSON, XML, YAML) to be able to use them as `Config` ressources.

> [!NOTE]
> It can be a good idea to use Unreal Engine or Unity format for `Atlas`, `SpriteSheet`, `Animation` and `Material` to simplify the integration of assets from these engines into Feline Engine, also this elaborate engines will do it more properly than feline.

> [!TIP]
> The `RessourceType` enum class can be extended by overloading the `Ressource` concept to include additional resource types as needed for specific use cases.