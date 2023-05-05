//! Implements a control system based off of evaluating incoming data.

use crate::action::{Action, Comparison, SchedRoutineHandler, ThresholdAction};
use crate::io::{DeferredDevice, IOEvent, RawValue};

pub type BoxedAction = Box<dyn Action>;


/// Collection of actions for propagating single device input.
///
/// A [`Publisher`] has a 1-to-1 relationship with a input device and stores all [`Action`] instances
/// ("subscribers" as per observer design pattern) associated with that input device. When data is read from
/// input device ([`crate::io::GenericInput::rx()`], the generated [`IOEvent`] should be passed to to all
/// [`Action`] instances. This propagation of [`IOEvent`] is handled in [`crate::io::GenericInput::read()`],
/// which calls [`Publisher::propagate()`].
///
/// Additionally, [`Publisher`] maintains an internal collection of scheduled [`crate::action::Routine`]s
/// for output devices and maintains the function ([`Publisher::attempt_routines()`]), for executing those
/// scheduled commands at their scheduled time.
#[derive(Default)]
pub struct Publisher {
    actions: Vec<BoxedAction>,
    scheduled: SchedRoutineHandler,
}

impl Publisher {
    #[inline]
    /// Attempt to run scheduled [`Routine`]s.
    ///
    /// [`Routine`] instances are automatically added by internal [`Action`]s and are automatically cleared
    /// when executed.
    ///
    /// # See Also
    /// This is a facade for [`SchedRoutineHandler::attempt_routines()`], which contains more detailed notes.
    pub fn attempt_routines(&mut self) {
        self.scheduled.attempt_routines()
    }

    pub fn attach_threshold(&mut self,
                            name: &str,
                            threshold: RawValue,
                            trigger: Comparison,
                            output: Option<DeferredDevice>,
    ) -> &mut Self {
        let mut action = ThresholdAction::new(name.to_string(), threshold, trigger);

        if let Some(output) = output {
            action = action.set_output(output);
        }

        self.subscribe(action.into_boxed());
        self
    }

    /// Get collection of subscribed [`Actions`] (stored as [`BoxedAction`]).
    ///
    /// # Returns
    /// Slice of all [`BoxedAction`] associated with `self`
    pub fn subscribers(&self) -> &[BoxedAction] {
        &self.actions
    }

    /// Add passed [`Action`] to internal collection.
    ///
    /// # Parameters
    /// - `subscriber`: [`BoxedAction`] to add to internal store.
    pub fn subscribe(&mut self, subscriber: BoxedAction) {
        self.actions.push(subscriber)
    }

    /// Call [`Action::evaluate()`] on all associated [`Action`] implementations.
    /// # Parameters
    /// - `data`: Incoming [`IOEvent`] generated from [`crate::io::device::GenericInput::read()`]
    // TODO: scheduled routines should be returned, then added to `scheduled`
    pub fn propagate(&mut self, data: &IOEvent) {
        for subscriber in self.actions.iter_mut() {
            // TODO: `IOEvent` shall be sent to `OutputDevice` and shall be logged
            // TODO: results should be aggregated
            subscriber.evaluate(data);
        }
    }
}
