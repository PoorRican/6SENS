//! Implement observer design pattern to implement control system based off of polling of `Input` objects
//!
//! # Description
//! The goal of a dedicated `Publisher` implementation being stored as a field is to add a layer of indirection
//! between `Input` and `Output` to serve as a bridge. Both input and output should be unaware of the other.
//! However, events generated by Input::read() are routed to Publisher::notify() which is propagated to
//! Subscriber implementations and therefore events are passed to outputs.
//!
//! `Publisher` objects should be stored a struct which implements `Input`. When `Input::read()` is called,
//! `Input::publisher().notify()` should also be called as well. `notify()` should thereby call
//! `Subscriber::evaluate()` on any listeners.

use crate::action::{Subscriber, SchedRoutineHandler};
use crate::helpers::Def;
use crate::io::IOEvent;

/// Trait to implement on Input objects
pub trait Publisher {
    type Inner;

    fn subscribers(&self) -> &[Def<Self::Inner>];
    fn subscribe(&mut self, subscriber: Def<Self::Inner>);

    fn notify(&mut self, data: &IOEvent);
}

/// Concrete instance of publisher object
#[derive(Default)]
pub struct PublisherInstance {
    subscribers: Vec<Def<<PublisherInstance as Publisher>::Inner>>,
    scheduled: SchedRoutineHandler,
}

impl PublisherInstance {
    /// Attempt to run scheduled `Routine` structs
    pub fn attempt_routines(&mut self) {
        self.scheduled.attempt_routines()
    }
}

impl Publisher for PublisherInstance {
    type Inner = Box<dyn Subscriber>;

    fn subscribers(&self) -> &[Def<Self::Inner>] {
        &self.subscribers
    }

    fn subscribe(&mut self, subscriber: Def<Self::Inner>) {
        self.subscribers.push(subscriber)
    }

    /// Call [`Subscriber::evaluate()`] on all associated [`Subscriber`] implementations.
    fn notify(&mut self, data: &IOEvent) {
        for subscriber in self.subscribers.iter_mut() {
            // TODO: `IOEvent` shall be sent to `OutputDevice` and shall be logged
            // TODO: results should be aggregated
            subscriber.try_lock().unwrap().evaluate(data);
        }
    }
}
