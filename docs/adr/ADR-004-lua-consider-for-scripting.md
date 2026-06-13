# ADR-004 | Lua for scripting

**Statut** : Accepted  
**Date** : 2026-06-13

## Context
The Engine need an High level scripting language for allowing developers to create game logic and behavior without having to write low-level code.

## Decision
The engine will use Lua as the scripting language for simple high level scripting. Lua is a lightweight, fast, and easy-to-learn scripting language that is widely used in game development. It has a small footprint and can be easily embedded into the engine, making it a good choice for scripting game logic and behavior.

But the main language to use when developing a game is C++ for performance reasons. Lua is only used for scripting game logic and behavior, not for performance-critical code. For example to add mod support or simple visual novel scripting, Lua is a good choice.

## Results
- Choosing Lua for scripting language in the engine for game logic and behavior.

## Considered alternatives:
- Python: Python is a popular scripting language that is widely used in game development, but it may be too heavy for the engine's needs and may not be as performant as Lua. Lua also use a loop and a stack for executing code, which can be more efficient than Python's interpreter-based execution model.
- JavaScript using Rhino: JavaScript is a widely used scripting language, but it may not be as performant as Lua and may not be as easy to integrate into the engine. Rhino is a JavaScript engine written in Java, which may add complexity to the engine's architecture.