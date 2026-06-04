# `Ressource<Texture>` ressource type
`Ressource<Texture>` is an important type in the engine, it give a representation of an 2D image for texturing assets.

## Allowed image formats
The engine accept multiple formats, here is the full list :
- **PNG** : PNG format is one of the most common format, it must be handle by the engine.
- **JPEG** : JPEG format is the common optimized format, it must be handle by the engine.
- **WEBP** : WebP allow a more precise optimization, it's a good alternative but for now not enough applications handle him, to carry this good format, the engine will accept him.
- **BPM** : BitMap format is usefull for some raw manipulations.
- **SVG** : SVG format allow none-pixel rule images, really usefull for non dithering effect.

Accepted format are store into the `enum TextureFormat`

## Engine format
This ressource is just a matrix, the `RessourceManager` handle 2D arrays in Buffer & Stream mode. The engine handle everything automatically for the developed but, he can specify more precicely the `RessourceType` using `Texture<FORMAT, MODE>`, example :
```cpp
#include <feline_core.dll>

Ressource<Texture<PNG, BUFFER>> img0;
Ressource<Texture<JPEG, BUFFER>> img1;
Ressource<Texture<SVG, STREAM>> img2;
Ressource<Texture<BPM, STREAM>> img3;
```

- The matrix can be directly read using `[][]` operator, and using `Ressource<Texture>.getSize()` you can run arround the texture safely, you also can use : `Ressource<Texture>.getPixel(x, y)`.

### Signature
```cpp
template <typename F>
template <typename M>
class Texture<F, M> {
public:
    new(Path path);

    Pixel getPixel(uint32_t x, uint32_t y);
    auto* getMatrixRef();

    void setPixel(uint32_t x, uint32_t y);

    Texture<F, M> crop(uint32_t ax, uint32_t ay, uint32_t bx, uint32_t by);
    void patch(Texture<F, M> patche, uint32_t x, uint32_t y) // Patch a texture on the current texture at x and y positions

    void export(TexturetType format); // Export to a specific format

    void applyEffect(ITextureEffect effect);
    void listEffect();
    void removeEffect(uint32 index);
};
```

## Effects

> [!NOTE]
> This section is look highly as the same as the [IAudioEffect](../../audio/main.md)

It's possible to add effects to a texture by using some integrate pixel manipulation tool, for that we use `TextureEffect` class, their contain operations to apply on texture, there is some common :
- `LighteningITextureEffect` : Edit the lightening of the texture.
- `BlurryITextureEffect` : Edit the blur of the texture.
- `SaturationITextureEffect` : Edit the saturation of an image.
- `ColorFilterITextureEffect` : Filter a texture color spectrum.

You can create your own by inherit the `ITextureEffect`.

> [!NOTE] 
> It's possible to apply a texture only on a specific part a texture but it's tricky, for this you need to `crop()` the Texture and apply the `ITextureEffect` on and finally, patch the Texture on the previous on, ideally you can create a new one and patch the two other on to get track of the effect pipeline.

### Preprocessing effects
> [!WARNING]
> Developers need to understand that `ITextureEffect`s are dynamic and can be resource-intensive. It is recommended to apply static effects to files that will be reused. **In the future, Feline Engine will provide a standalone Static Effect Compiler tool.**