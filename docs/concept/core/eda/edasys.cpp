#include "edasys.h"

using EventID = std::uint32_t; // Authorize a total of 4,294,967,296 different events

int EventBus::subscribe(const Event& e, Service* subscriber) {
    subscribers[e.id()].push_back(subscriber);
    return 0;
}

int EventBus::unsubscribe(const Event& e, Service* subscriber) {
    return 0;
}

int EventBus::publish(const Event& e) {
    // Usage of find to prevent panick when e.id doesn't exist,
    // when it == subsribers.end(), e.id doesn't have been found.
    // auto represent : std::unordered_map<EventID, std::vector<Service*>>::iterator it
    auto it = subscribers.find(e.id()); // it stand for iterator
    if (it != subscribers.end()) {
        for (Service* s : it->second) {
            s->on_event(e);
        }
    } else {
        // e.id doesn't exist
        return 1;
    }
    return 0;
}

class LoggerService : public Service {
public:
    void on_event(const Event& e) override {
        std::cout << "Event received: " << e.id() << std::endl;
    }
};

class LoggerEvent : public Event {
public:
    LoggerEvent(EventID id) : Event(id) {}
};

int main() {
    // 1. We initialize an EventBus and a Service
    EventBus bus;
    LoggerService logger;

    // 2. We initialize an event called e with id 0
    LoggerEvent e = LoggerEvent(0);
    // 3. We subscribe our Service to this event type
    bus.subscribe(e, &logger);
    
    // 4. We send an event of type e
    bus.publish(e);

    // 5. Logger receive the event, and print him.
}
