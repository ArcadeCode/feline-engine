# `Audio` (Rendu audio)
The module `audio/` is develop to give a high level abstraction for manage audio, he used [miniaudio](https://github.com/mackron/miniaudio) framework to work.
> miniaudio can be used with controlling a SDL project, [see documentation](https://miniaud.io/docs/examples/engine_sdl.html)

It is composed of this public interfaces :
- `IAudioSource` : An audio source interface who contain sound, positions and others elements.
- `IAudioEffect` : An abstraction of an effect to apply on an audio stream or buffer
- `AudioManager` : A singleton who is responsible for audio mixing, audio pools, channels mixing, and buffer or streaming decision.
And this privates class :
- `AudioBuffer` : An abstraction of a loaded in memory file at format : `.wav`, `.ogg`, `.mp3`, `.flac`
- `AudioStream` : An abstraction of a loaded in streaming file at format : `.wav`, `.ogg`, `.mp3`, `.flac`

## `IAudioSource`
This element is an interface of a concrete sound on the scene, what we call a "source".

### `IAudioSource` signature
```cpp
class IAudioSource {
    vec2 position;                 // Position in 2D space
    std::optional<vec2> direction; // nullopt = omni, value = directional
    float coneAngle = 2.0f * M_PI; // Default: full circle (omni)
    float volume = 1.0f;           // Linear [0.0, 1.0]
    unsigned int repeat = 1;       // 0 = infinite loop, >0 = number of plays
    Resource<AudioFile> path;      // Handle to audio resource

    apply(IAudioEffect effect);  // apply an effect on the audio source
};
```
> [!NOTE]
> If `IAudioSource.repeat` is `0`, we can callback it into the `IAudioManager` to stop it for playing.
> 
> `vec2` is a type alias of `fe::utils::Vector<T, N>` with parameters `<float, 2>`.

## `IAudioEffect`
Abstraction of an effect who can be applied on a `IAudioSource` or another `IAudioEffect`.

#### `IAudioEffect`
```cpp
class IAudioEffect {
public:
    virtual ~IAudioEffect() = default;

    // Process a chunk of audio samples
    // buffer: pointer to interleaved float samples (stereo = L,R,L,R...)
    // numSamples: total number of samples (not frames)
    // numChannels: number of channels (1 = mono, 2 = stereo, etc.)
    virtual void process(float* buffer, size_t numSamples, int numChannels) = 0;

    // Optional: update effect parameters dynamically
    virtual void setParameter(const std::string& name, float value) = 0;

    // Optional: retrieve parameter for GUI / debugging
    virtual float getParameter(const std::string& name) const = 0;
};
```

> [!WARNING]
> This is to change because now we use miniaudio, we will use the specific audio effect processing for.

> [!NOTE]
> `IAudioSource.apply` return a new Self instance of `IAudioSource` altered by the `IAudioEffect` apply. So we can chain effects like this :
> ```cpp
> source.apply(reverb).apply(distorsion).apply(lowFilter);
> ```

> [!WARNING]
> Developers need to understand that `IAudioEffect`s are dynamic and can be resource-intensive. It is recommended to apply static effects to files that will be reused. **In the future, Feline Engine will provide a standalone Static Effect Compiler tool.**

## `IAudioBuffer`
An private class used by the motor to expose an Audio Ressource as buffer who need to be push in RAM.

### `AudioBuffer` signature
```cpp
class AudioBuffer {
    private:
        ~AudioBuffer() = default;
    
    public:
        Resource<AudioFile> path; // Handle to audio resource
};
```
> [!NOTE]
> Ressource is a custom class who contain already an `ressourceId`.

## `AudioStream`
An private class used by the motor to expose an Audio Ressource as stream who is loader/unloaded chunk by chunk from the disk.

### `AudioStream` signature
```cpp
class AudioStream {
    private:
    ~AudioStream() = default;

    Resource<AudioFile>& path;           // Handle to audio resource
    unsigned int chunkSize = 20'000'000; // Size of a chunk, by default 20 Mb
    unsigned int numberChunkLoaded = 3;  // Number of chunks loaded in memory.
    // When the last chunk is done reading, he is unloaded.

    public:
        unsigned int get_chunk_size();
        set_chunk_size(unsigned int chunkSize);
        unsigned int get_max_chunks_in_memory();
        set_max_chunks_in_memory(unsigned int maxChunkLoaded);
        unsigned int get_max_loaded_chunks();

};
```
> [!NOTE]
> Ressource is a custom class who contain already an `ressourceId`.

## `AudioManager`
AudioManager is the main class of the Audio in the engine. This Manager is a singleton who is used to send `IAudioSource`'s.

### `AudioManager` signature
```cpp
class AudioStream {
    private:
        // It's depend of the library used for audio processing.

    public:
        play(IAudioSource& source); // To send an audio source to be played.
        stop(IAudioSource& source, bool awaitEnd); // Used to stop an IAudioSource who loop.
        stop_all(bool awaitEnd = false); // Stop every sources played.

        unsigned float get_master_volume(unsigned float volume); // Get master volume.
        set_master_volume(unsigned float volume); // Set a new master volume between 0.0 and 1.0 .
};
```