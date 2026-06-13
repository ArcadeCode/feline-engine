# `IGraphicBackend` abstract layer

Feline engine search **performances** and Vulkan is ideal for that, we use OpenGL for now because of the simplicity of the tool but if we cant to migrate later the engine need to adapt to the Graphic API used. `IGraphicBackend` has the role of maintaining a abstraction layer between the engine and the graphic API without worrying of changing the API later.

Each API connected is an `GraphicApiAdapter`, it process some fundamental translations between the `IGraphicBackend` and his Api. For now only `OpenGLGraphicApiAdapter` exist.

## Standalone
`IGraphicBackend` **is a standalone** only one is construct and handle any rendering service and by flow-on only one Adapter is called at the time. This is ensure by the `IGraphicBackendConstructEvent` event, is an `IGraphicBackend` publish this event and another `IGraphicBackend` subscribe to it a `MultipleGraphicBackendInterfaceWarn` warning, this result in the stopping of the last on allocated but it may in some case result in a critical error stopping the engine, to protect that, we need to force at compiling only one `IGraphicBackend`.

## Adding Adapter
All adapter constructing and listing is hardcoded into `./src/graphic/backend/adapters/`, adding one can be done by importing `IGraphicBackend` and overloading the class BUT it is not recommended to operate like this, instead you will prefer to create a pull request of the engine to add a new adapter. This project is open source and we want see it evolve.