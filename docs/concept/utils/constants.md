# Feline Engine global constant documentations

FE need to have a lot of already ready constants for many tasks but mainly for mathematical arithmetic.

- `constexpr double PI_D = 3.14159265358979323846;` : 64 bits Pi value
- `constexpr float  PI_F = static_cast<float>(PI_D);` : 32 bits Pi value
- ... Continue to add some values

## `fe::utils::Vector<T, N>` alias
- `Vec2 = Vector<float, 2>`  : Useful for 2D spatial.
- `Vec2i = Vector<int, 2>`  : Useful for fixed positioning.
- `Vec4 = Vector<float, 4>`  : Useful for color spaces and alpha store.