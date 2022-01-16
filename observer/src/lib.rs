use std::rc::{Rc, Weak};
use std::borrow::Borrow;

struct EventProducer<T : ?Sized> {
    listeners : Vec<Weak<T>>
}

impl<T: ?Sized> EventProducer<T>
{
    pub fn add_listener(&mut self, listener : Weak<T>) {
        self.listeners.push(listener);
    }

    pub fn new() -> EventProducer<T> {
        EventProducer {
            listeners: Vec::new()
        }
    }

    pub fn update_listeners(&mut self, update_fn : fn (& T, i32) -> (), new_val : i32) {
        for l in &self.listeners {
            if let Some(strong) = l.upgrade() {
                update_fn(strong.borrow(), new_val);
            } else {
                //TODO drop listener
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // Interface for talking to and keeping the temperature listeners in one container
    trait TemperatureListener {
        fn temperature_changed(&self, new_val : i32);
    }

    struct Thermometer {
        event_producer : EventProducer<dyn TemperatureListener>
    }

    impl Thermometer {
        fn update(&mut self, new_val : i32) {
            self.event_producer.update_listeners( TemperatureListener::temperature_changed, new_val);
        }

        fn add_listener(&mut self, listener : Rc<dyn TemperatureListener>) {
            self.event_producer.add_listener(Rc::downgrade(&listener));
        }

        fn new() -> Thermometer {
            Thermometer {event_producer : EventProducer::<dyn TemperatureListener>::new()}
        }
    }

    struct TestTemperatureListener {
        last_val : RefCell<i32>
    }

    impl TestTemperatureListener {
        fn new() -> TestTemperatureListener {
            TestTemperatureListener{
                last_val : RefCell::new(0)
            }
        }
    }

    impl TemperatureListener for TestTemperatureListener {
        fn temperature_changed(&self, new_val: i32) {
            println!("TestTemperatureListener temp value changed to {}", new_val);
            *self.last_val.borrow_mut() = new_val;
        }
    }

    impl AnotherTestTemperatureListener {
        fn new() -> AnotherTestTemperatureListener {
            AnotherTestTemperatureListener{
                last_val : RefCell::new(0)
            }
        }
    }

    struct AnotherTestTemperatureListener {
        last_val : RefCell<i32>
    }

    impl TemperatureListener for AnotherTestTemperatureListener {
        fn temperature_changed(&self, new_val: i32) {
            println!("AnotherTestTemperatureListener temp value changed to {}", new_val);
            *self.last_val.borrow_mut() = new_val;
        }
    }

    #[test]
    fn test_listener_registration()
    {
        let mut thermometer = Thermometer::new();
        let ttl = Rc::new(TestTemperatureListener::new());
        let ttl2 = Rc::new(TestTemperatureListener::new());
        let attl = Rc::new(AnotherTestTemperatureListener::new());
        thermometer.add_listener(ttl.clone());
        thermometer.add_listener(ttl2.clone());
        thermometer.add_listener(attl.clone());

        // TODO: Find out why ttl.strong_count() doesn't work?
        assert_eq!(1, Rc::strong_count(&ttl));
    }

    #[test]
    fn test_value_change()
    {
        let mut thermometer = Thermometer::new();
        let ttl = Rc::new(TestTemperatureListener::new());
        thermometer.add_listener(ttl.clone());
        thermometer.update(5);
        assert_eq!(5, *ttl.last_val.borrow());
    }
}

