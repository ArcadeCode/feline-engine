# Particle system

Particles is separated in two group : Physical ones and View only one. The physicals are computed by the CPU under the `PhysicalParticle`, the CPU cannot process a lot and when the particles don't need complex physic, the view only `Particle` can be used, they're computed by the GPU and he can process a lot more of particle at the time.

## Define a particle `Particle`
```cpp

struct Particle {
    // Vector position
    x,
    y,
    z,
    // Vector orientation
    dx,
    dy,
    dz,
    texture     // Texture of the particle
    speed,      // Speed of the particle
    randomness, // By default 0 represent a random deviation of the particle from them axis.
}
```

## Define a particle `PhysicalParticle`
A `PhysicalParticle` is just a `Particle` with the `RigidBody` component [from the ECS system attach](../core/) it become an `Entity`. A physical particle can be block by other rigid body, having some special physics (such as bouncing effect, splashing like rain, ...). Here is his definition :

```cpp
class PhysicalParticle : Entity, Particle {
    Particle parameters
    mass,
}
```

## `Particle` batch computing and `ParticleGenerator`
Feline Engine have something called `ParticleBatch` which is a mouvement of multiple particle batch, it's operate by the `Entity` named `ParticleGenerator`, this is the main usage of `Particle` object, for example declaring smoke campsite in your game.

```cpp
class ParticleGenerator : Entity {
    Particle particle
    x,
    y,
    z,
    dx,
    dy,
    dz,
    speed,
    spread,
    force,
    randomness
}
```