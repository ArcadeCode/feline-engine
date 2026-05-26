# What to test on the EDA system

## Event bus speed and spreading rates
The EDA sys need to be operational to high frequency event publishing, for exemple there is 60 ticks each second and each tick send :
- Tick started event
- Tick ended event
So already 120 events send by second. So the system need to accept at least 10,000 events second.

## Event overwriting
The EDA system need to handle if an system want to overlap the event of another one, the EDA **doesn't have the right to panic** never the system can fail because if the EDA sys fail, the Engine collapse.

## Event name checking
Every Event need to be write using PascalCase and finish the vent name by `Event` for example : `MyCustomEvent` but we prefer using a name who give what this event do and not is : `SendMyCustomEvent` will be prefer. Is an Event doesn't use the correct case, we need to send a warning : `Event : "xxx" don't use the correcte PascalCase + Event format`.