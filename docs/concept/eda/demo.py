from typing import Callable, Dict, List, Type, Optional
import logging

# lightweight, reusable logger for this module
_logger = logging.getLogger("feline_engine.demo")
if not _logger.hasHandlers():
    handler = logging.StreamHandler()
    handler.setFormatter(
        logging.Formatter("%(asctime)s %(levelname)-8s %(message)s", "%Y-%m-%d %H:%M:%S")
    )
    # Allow the handler to emit DEBUG records
    handler.setLevel(logging.DEBUG)
    _logger.addHandler(handler)
    # Make sure the logger itself is set to DEBUG so debug() calls are not filtered out
    _logger.setLevel(logging.DEBUG)


def logger(message: str, level: Optional[str] = "info") -> None:
    """
    Simple module logger wrapper.
    Usage: logger("something happened"), logger("oops", "error")
    """
    level = (level or "info").lower()
    if level == "debug":
        _logger.debug(message)
    elif level in ("warn", "warning"):
        _logger.warning(message)
    elif level == "error":
        _logger.error(message)
    elif level == "critical":
        _logger.critical(message)
    else:
        _logger.info(message)

# --- Event Definition ---
class Event:
    """Base class for all events."""
    pass
# --- Event Bus ---
class EventBus:
    def __init__(self):
        # Mapping event type -> list of listeners
        self.subscribers: Dict[Type[Event], List[Callable[[Event], None]]] = {}
        logger("[EventBus] EventBus initialized", "debug")

    def subscribe(self, event_type: Type[Event], handler: Callable[[Event], None]):
        """Register a handler for a specific event type."""
        logger("[EventBus] Subscribing handler to event type: {}".format(event_type.__name__), "debug")
        if event_type not in self.subscribers:
            self.subscribers[event_type] = []
        self.subscribers[event_type].append(handler)

    def publish(self, event: Event):
        """Notify all handlers subscribed to this event type."""
        logger("[EventBus] Publishing event: {}".format(type(event).__name__), "debug")
        for handler in self.subscribers.get(type(event), []):
            logger("[EventBus] Notifying handler: {}".format(handler.__name__), "debug")
            handler(event)


# --- Service using EventBus ---
class Service:
    def __init__(self, event_bus: EventBus):
        self.event_bus = event_bus
        self.subscribe()  # auto-subscribe on init

    def subscribe(self):
        """Override in subclasses to declare which events to listen for."""
        pass

    def publish(self, event: Event):
        """Helper to publish events."""
        self.event_bus.publish(event)

############################
# --- Example services --- #
############################
class UserRegisteredEvent(Event):
    '''Event triggered when a user registers.'''
    def __init__(self, user_id: int, username: str):
        self.user_id = user_id
        self.username = username

class EmailService(Service):
    '''Sends welcome emails when users register.'''
    def subscribe(self):
        self.event_bus.subscribe(UserRegisteredEvent, self.on_user_registered)

    def on_user_registered(self, event: UserRegisteredEvent):
        logger(f"[EmailService] Sending welcome email to {event.username}")


class UserService(Service):
    '''Handles user registrations.'''
    def register_user(self, user_id: int, username: str):
        logger(f"[UserService] Registered user {username}")
        self.publish(UserRegisteredEvent(user_id, username))


# --- Example usage ---
if __name__ == "__main__":
    bus = EventBus()
    email_service = EmailService(bus)
    user_service = UserService(bus)

    user_service.register_user(1, "Alice")
