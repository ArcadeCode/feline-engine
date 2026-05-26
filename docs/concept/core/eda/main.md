# EDA system
Feline Engine use an EDA system for all engine system. The EDA system can be called by using :
```cpp
#include <FelineEngine/EDA/EDA.h>
EDA::getInstance();
```

When we talk about EDA system in Feline Engine, we will referer to this definitions :
- `Service` : The author of an event publish.
- `Event` : A defined type of event that can be publish on the bus.
- `Subscriber` : An subscriber to an event who listen for an event to be publish.

## How to create an event
In some case, you will need to make your own `Event` class, to do this you only need to create a class inheritance :
```cpp
#include <FelineEngine/EDA/Event.h>
class SendMyCustomEvent : Event {
    //
}
```

## How to become an Service
## How to become an Subscriber