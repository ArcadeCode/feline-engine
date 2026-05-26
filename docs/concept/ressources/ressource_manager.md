# `RessourceManager` class
The `RessourceManager` class in Feline Engine is responsible for managing the lifecycle of any `Ressource<RessourceType>`.

## Overview
The `RessourceManager` class provides methods to load, unload, and retrieve resources of various types (textures, fonts, shaders, audio) used in the engine. It ensures that resources are efficiently managed, preventing duplicate loading and optimizing memory usage.

When a resource is requested, the `RessourceManager` checks if it is already loaded. If it is, it returns a reference to the existing resource. If not, it loads the resource from disk, stores it in its internal cache, and then returns a reference to the newly loaded resource.

## Comunication with `RessourceManager`
`RessourceManager` is a standalone class that can be accessed globally within the Feline Engine by using the [EDA system](../eda/what-is-eda.md). To interact with the `RessourceManager`, you typically send requests through the EDA system by sending events that the `RessourceManager` listens to.

## Sending a Resource Load Request
To load a resource, you can send a `LoadRessourceEvent` to the EDA system. Here is an example of how to send a load request for a texture resource:

```cpp
#include <FelineEngine/EDA/EDA.h>
#include <FelineEngine/Ressources/RessourceManager.h>
#include <FelineEngine/Ressources/Ressource.h>

// Create a load request event for a texture resource
Ressource<Texture> myTexture("path/to/texture.png");
LoadRessourceEvent<Ressource<Texture>> loadTextureEvent(myTexture);
// Send the event to the EDA system
EDA::getInstance().sendEvent(loadTextureEvent);
// Retrieve the loaded resource from the event response
Ressource<Texture>* retrieveTexture = loadTextureEvent.getRessourceById(myTexture.getId());

if (retrieveTexture.content == myTexture.content) {
    // We retrieved the loaded texture successfully
} else {
    // Handle the error: resource not loaded
}
```

> [!NOTE] : You doesn't need to be concerned about the actual loading process. The `RessourceManager` will handle it internally when a `Ressource<RessourceType>` is used.

## Other Resource Management Operations
Here is all events that `RessourceManager` can handle:
- `POST LoadRessourceEvent<Ressource>`: Load a resource of type `Ressource`.
- `POST UnloadRessourceEvent<Ressource>`: Unload a resource of type `Ressource`.
- `POST LoadMultipleRessourcesEvent<Ressource[]>`: Load multiple resources of type `Ressource`.
- `POST UnloadMultipleRessourcesEvent<Ressource[]>`: Unload multiple resources of type `Ressource`.
- `GET IsRessourceLoadedEvent<Ressource>`: Check if a specific resource is currently loaded in the manager.
Some other operations can be done as "debug" operations:
- `GET GetLoadedRessourcesEvent`: Retrieve a list of all loaded resources of type `Ressource`.
- `POST ClearRessourcesEvent`: Clear all loaded resources from the manager.
- `GET RessourceUsageEvent<Ressource[]|Ressource>`: Get usage statistics for ressources such as memory usage and reference counts.

## How is `RessourceManager` called internally?
The `RessourceManager` is designed to be used internally by other systems and components of the Feline Engine. For example, when a game object requires a texture, it will send a load request to the `RessourceManager` through the EDA system. The `RessourceManager` will then handle the loading process and provide the requested resource to the game object.