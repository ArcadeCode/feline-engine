#pragma once
#include <unordered_map>
#include <vector>
#include <iostream>

// Authorize a total of 4,294,967,296 different events
// It seem enough to any situation, maybe a bit too big.
using EventID = std::uint32_t;

class Event {
    private:
        const EventID m_id;
    public:
        explicit Event(unsigned int id) noexcept : m_id(id) {}
        virtual ~Event() = default;

        // this->id getter
        EventID id() const noexcept { return this->m_id; }
};

class Service {
    public:
        virtual ~Service() = default;
        virtual void on_event(const Event& e) = 0;
};

class EventBus {
    private:
        std::unordered_map<EventID, std::vector<Service*>> subscribers;

    public:
        int subscribe(const Event& e, Service* subscriber);

        int unsubscribe(const Event& e, Service* subscriber);
        
        int publish(const Event& e);
};