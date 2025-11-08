# `Ressource<T>`

When we need to load any ressource (video, audio or text file) into the engine, the most convenient way will be to use a custom ressource wrapper who will manage `new` and `free` for the developer to help him. This wrapper will be called `Ressource<T>`, his location his :
```cpp
#include <feline>
const fe::utils::Ressource<> ressource;
```

## Reasons :
- Buffer or stream : One main reason to create a custom allocation system with ressource management is to handle streaming
- ECS : Another reason is to sync this allocations with the Entity Component System with a ressource Id.

## Signature :
```cpp
template <typename T>
class Ressource {
    public:
        Ressource(std::filesystem::path path)
        ~Ressource();

        constexpr ressource_id;


        force_buffer_loading();
        force_stream_loading(unsigned int chunkNb = 3, unsigned int chunkSize = 16_000_000);
    //
    private:
        //
}
```

## Error/Warning 
- **[ERROR] Unavailable space for buffer allocation** : When the allocator cannot allocate for the buffer.
- **[ERROR] Unavailable space for chunk allocation** : When the allocator cannot allocate a chunk of a stream.
- **[WARN] Buffer loading forced** : When a ressource is forced into the RAM as a unique buffer.
- **[WARN] Chunk size too large** : When a ressource is stream with chunks of size >=300Mo.
- **[WARN] Chunk size too shrink** : When a ressource is stream with chunks of size <64Ko.