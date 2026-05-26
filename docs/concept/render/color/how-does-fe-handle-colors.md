# How does Feline Engine handle colors?

> [!NOTE]
> This document **need a rework**, JzAzBz is maybe to complex for an 2D engine this simple, we should maybe use a more simple color space like HSL, HSV or juste sRGB.

---

> [!WARNING]
> This section is old and need a rework, the way we handle colors in the engine is not final and may change in the future.

Feline Engine use JzAzBz color space to represent colors. JzAzBz is a perceptually uniform color space, which means that the distance between two colors in this space corresponds to the perceived difference between those colors. This allows for more accurate color representation and manipulation, especially when it comes to blending and interpolation.

However Feline Engine also provides support for RGB color space, which is more commonly used in graphics applications. When a color is specified in RGB, Feline Engine converts it to JzAzBz for internal processing. This allows developers to work with colors in the familiar RGB format while still benefiting from the advantages of the JzAzBz color space for rendering and color manipulation.

In addition to RGB, Feline Engine also supports other color formats such as hexadecimal and HSL. These formats are also converted to JzAzBz for internal processing. This flexibility allows developers to choose the color format that best suits their needs while still ensuring accurate color representation and manipulation within the engine.

## How does JzAzBz is handled by the engine ?
When a color is specified in JzAzBz, Feline Engine uses it directly for rendering and color manipulation. The engine takes advantage of the perceptual uniformity of the JzAzBz color space to perform operations such as blending, interpolation, and color adjustments in a way that maintains the perceived differences between colors.

The engine handles JzAzBz colors with shaders that are optimized for this color space, allowing for efficient rendering and accurate color representation. This means that when you specify a color in JzAzBz, you can expect it to be rendered accurately and consistently across different devices and display settings.

## Frontend Interfaces
When the developper want to specify a color in Feline Engine, they can use the RGB space color or the JzAzBz color space. The engine provides interfaces for both color spaces, allowing developers to choose the one that best suits their needs.

```cpp
struct ColorSpace {
    SRGB,
    HSL,
    JZAZBZ
}

class Color {
public:
    static Color FromRGB(float r, float g, float b, float alpha);
    static Color FromHex(uint32_t hex, float alpha);
    static Color FromHSL(float h, float s, float l, float alpha);
    static Color FromJzAzBz(float jz, float az, float bz, float alpha);

    RGB ToRGB() const;
    // We don't export to Hex cause we don't want using Hex format in
    // the engine.
    HSL ToHSL() const;
    JzAzBz ToJzAzBz() const;

    // Todo adding +, - operations between Color objects

private:
    LinearColor internal_color; // canonical internal representation using JzAzBz
};
```

> [!NOTE]
> When developers use Color and do operations on, this operations are send to the GPU Job Manager.

## Backend Interfaces
Under the hood, any color is stored as a floating-point `Vector4` store into a `LinearColor` :

```cpp
class LinearColor
{
public:
    float jz;
    float az;
    float bz;
    float alpha;

    // Pure math operations only
};

enum class ColorEncoding
{
    UNORM,
    FLOAT16,
    FLOAT32
};

enum class DynamicRange
{
    SDR, 
    HDR10, // HDR10 is a high-dynamic-range (HDR) format that allows for a wider range of brightness and color than standard dynamic range (SDR) content.
    scRGB // scRGB is a wide-gamut color space that can represent a wider range of colors than sRGB, making it suitable for high-dynamic-range (HDR) content.
};

struct RenderTargetFormat
{
    ColorEncoding encoding;  // UNORM, FLOAT16, FLOAT32
    DynamicRange range;      // SDR, HDR10, scRGB
};
```

When sending a color to the GPU, the engine will convert it to a `Vector4` of 4 floats and send it to the GPU. The GPU will then use this color for rendering and color manipulation in the shaders.

JzAzBz colors are handled in the shaders using optimized algorithms that take advantage of the perceptual uniformity of the color space. This allows for accurate color representation and manipulation while maintaining performance.

Howerver JzAzBz colors are not directly supported by display devices, so when a color is sent to the GPU, it is converted back to best color space compatible with the display device, which is usually sRGB but can be :
- 

