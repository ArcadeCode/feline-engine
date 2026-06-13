# Shader

A shader is a small high parallelizable program who is handle by the `IGraphicBackend`. Shader can be assign to `DrawableObject` for different behavior. Shader use the GLSL format.

## Applying a Shader
First you need to create a `.glsl` shader. After this you need to compile it, their is two methods :

### Runtime

Runtime compiling using `DrawableObject.apply()` If you send a `Shader` object instead of an `CompiledShader` the engine will compile it on runtime, it's will take some time, recommended at the game starting or during a transition.

Note an important fact, you can cache the compiled shader, to do that simply edit the `render.conf` at the line `cacheCompiledShader` set true, after this by default each shader is cached but you can bypass this default behavior by using `Shader.storeCache = false`.

### Compile time

At program compiling you can compile the shader, this is the default method, when compiling your program using LLVM each shader will be compiled and cached. I highly recommend this solution unless you shader are dynamically created or edited during the compile time which is not a idiomatic GPU computing logic.

## `Particle` are `Shader`
[`Particle`](./particle.md) are shaders computed by the GPU, this is important to note.