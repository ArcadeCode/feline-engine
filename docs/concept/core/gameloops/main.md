# Game loops

> [!NOTE] Language to use
> For now, Rust seem the best language for the game loop, because the Job scheduler will wrote in Rust

There is two loops in a modern engine like FE :
- The Game/Logic loop
- The Render loop
Separating logic from rendering help to render at high frequency without speed up the game or slowing down depending of the computer.

## Logic loop
The logic loop using something called : **Tick**, each second 60 ticks are operate and each ticks the logic step from one, this imply :
- Calculate new ECS step.
- Retrieve user input.
Sometime the CPU will process less ticks by second due to some reasons such as to much ECS calculation, to prevent this, there is something called **TPS**, it's a mesure of how many Ticks by Seconds are process, if TPS < 60 the future second will try to process more ticks to balance the delay.

> [!NOTE] 60 TPS decision
> The fixed TPS desicion if by default 60 TPS due to the market doing the same, but this value have possible to be change.

## Render loop
The render loop using something called : **Frame**, each second many as possible frames are operate an each frames the full scene is rendered. The rendering is mainly GPU bound and can be fixed which is useful for things such as V-SYNC.

## Debuging, TPS & FPS
Each frames, TPS & FPS can be shown, this values represent how many Ticks & Frames are compute per seconds, the TPS need to be the more fix possible, it shown with the fixed tps : `59 / 60 TPS` and fps are shown dependly when the frames are fixed : `60 / 60 FPS` or unfixed : `130 FPS`. The TPS and FPS timer is called : `Timer`, the `FrameTimer` is a simple tool in read only, but `TickTimer` is a more elaborate component who store how much the GameLoop is late by the Fixed TPS. The timer store `retrieveTicks` who represent how many ticks to render the future second.

`TickTimer` can handle stress situations were to many ticks are lates, if there is more than 60 lates ticks (wich mean the engine is 1 second late) the component will ditribute late ticks on multiple seconds, basically. If the Timer is 60 ticks late, the next 10 seconds 6 ticks will be compute by seconds, and this distribution will being more and more distribute along the time by checking how much the late rate continue.

If the `TickTimer` cannot cath-up all the lates ticks, He have no more solution than skipping ticks, skipping ticks is a complex choice wich is difficult to design, in an idiomatic determistic deamon such as the FE engine when to skip some computation ?

TODO: Search on this

### Injecting lag
When testing the engine, we need to inject difficulty to the CPU compute rate, the most easy way to do it is to add delay when returning a Tick, this can be done by adding `DummyJob` locked on specific tick wich is essentially a single delay who block the process.

### Over catch-up
When testing the engine, we need to setup states where the `TickTimer` have to many ticks already computed for the future second, for this we can use `DummyJob` locked on specific tick to randomly give more simple task, for example, the main job is to do this compute task : `Pi * e^9` and some random timer, giving this compute task : `1+1`. This unpossible to determine simple task will speed up the tick rate and having a lot of this dummy on each ticks will kill tick frequence.