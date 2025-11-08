# Qu'est-ce qu'un 

## Schéma
```
                  ┌────────────────────────────┐
                  │         Game.exe           │
                  │    (uses Feline Engine)    │
                  └────────────┬───────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────┐
│                     FELINE ENGINE (Core .lib)         │
│                                                       │
│   ┌────────────────────────────┐                      │
│   │     Core Systems Layer     │                      │
│   │  (statiquement lié)        │                      │
│   │                            │                      │
│   │  • MemoryManager           │                      │
│   │  • Logger / Profiler       │                      │
│   │  • TimeManager             │                      │
│   │  • JobSystem / Threads     │                      │
│   │  • ResourceManager         │                      │
│   │  • ECS / EntityRegistry    │                      │
│   │  • EventBus  ←─────────────┼────┐                 │
│   └────────────────────────────┘    │                 │
│                                     │                 |
│   ┌─────────────────────────────────┘                 |
│   │                                                   |
│   ▼                                                   ▼
│   ┌────────────────────────────┐       ┌────────────────────────────┐
│   │ Dynamic Module: Renderer   │       │ Dynamic Module: Audio      │
│   │ (Renderer.dll/.so)         │       │ (Audio.dll/.so)            │
│   │                            │       │                            │
│   │ • Init(RenderContext)      │       │ • Init(AudioDevice)        │
│   │ • Subscribe(WindowEvents)  │       │ • Subscribe(GameEvents)    │
│   │ • Emit(RenderStatsEvent)   │       │ • Emit(SoundFinishedEvent) │
│   └──────────────┬─────────────┘       └──────────────┬─────────────┘
│                  │                                    │
│   ┌──────────────┴─────────────┐       ┌──────────────┴─────────────┐
│   │ Dynamic Module: Input      │       │ Dynamic Module: Physics    │
│   │ (Input.dll/.so)            │       │ (Physics.dll/.so)          │
│   │ • Emit(KeyPressedEvent)    │       │ • Subscribe(CollisionEvt)  │
│   │ • Subscribe(WindowFocusEvt)│       │ • Emit(ForceAppliedEvent)  │
│   └────────────────────────────┘       └────────────────────────────┘
│                                                               │
│     [Communication exclusive via EventBus — jamais directe]   │
└───────────────────────────────────────────────────────────────┘
```