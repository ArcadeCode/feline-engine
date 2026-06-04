# Feline Engine Allocator (Kitty)
Kitty is the main allocator of Feline Engine, it is a custom memory allocator designed to provide efficient memory management for game development. It is built on top of the api allocator and provides additional features such as memory pooling, fragmentation reduction, and debugging tools.

The main goal of kitty is a mutliple allocation strategy for different use cases, such as:
- **General-purpose allocator**: A general-purpose allocator for small to medium-sized allocations, optimized for speed and low fragmentation.
- **Large block allocator**: A specialized allocator for large blocks of memory, optimized for minimizing fragmentation and maximizing performance.
- **Pool allocator**: A pool allocator for frequently allocated and deallocated objects of the same size, such as game entities or components, optimized for speed and memory reuse.
- **Stack allocator**: A stack allocator for temporary allocations that follow a last-in-first-out (LIFO) pattern, optimized for speed and minimal overhead.
- **Custom allocators**: The ability for developers to create and integrate their own custom allocators for specific use cases or performance requirements.
  
All these allocators can be used directly or developer can just use `Kitty::new()` and the engine will select the best allocator for the use case.

## Communication with the allocator
Kitty isn't independant of the engine, Kitty allocate work with the engine and communicate by [EDA system](../eda/main.md).

Exemples of communication with the allocator:
- When an entity is created, the engine will send a message to the allocator to allocate memory for the entity and its components.
- When a texture is loaded, the engine will send a message to the allocator to allocate memory for the texture data.
- When a ressource is no longer needed, the engine will send a message to the allocator to deallocate the memory used by the ressource.

## Allocator API

> [!INFO]
> `Kitty` namespace directly expose a EDA pipeline to the main bus, calling `Kitty::new()` simplify this code :
> ```cpp
> // Without Kitty::new()
> #include <FelineEngine/EDA/EDA.h>
> EDA::getInstance().publish(AllocateAutoFormatCustomSizeEvent, size);
> ```
> This can be painful to write and read.

The allocator API provides a set of functions for allocating and deallocating memory, as well as for managing memory pools and debugging. Some of the key functions include:
- `Kitty::new(size_t size)`: Allocates a block of memory for a given size, automatically selecting the best allocator for the use case.
- `Kitty::delete(void* ptr)`: Deallocates a block of memory previously allocated.
- `Kitty::createPool(size_t objectSize, size_t poolSize)`: Creates a memory pool for objects of a specific size.
- `Kitty::destroyPool(size_t objectSize)`: Destroys a previously created memory pool.
- `Kitty::debug()`: Provides debugging information about memory usage, fragmentation, and allocation patterns.

> [!WARNING] Do not allocate ressource from kitty
> Kitty is a backend allocator, when you need to allocate a ressource, you should use the [Ressource Manager](../ressources/main.md) instead, the ressource manager will handle the communication with the allocator and provide additional features such as caching and reference counting. Do the same for entity, component, vector, etc... use the dedicated manager for this type of ressource.

## Why this name
"Kitty" is in one of the main rules of the Feline Engine philophy : "In our team, cats aren't just pets — they're a statement, a lifestyle. 🐾" To asserve cats superiority the Allocator (one of the most complex part of the Engine) was named after most powerfull creatures of the species : Kitten.