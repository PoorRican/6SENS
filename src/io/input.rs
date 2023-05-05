use crate::action::{Command, IOCommand, Publisher};
use crate::errors::ErrorType;
use crate::helpers::Def;
use crate::io::{
    no_internal_closure, Device, DeviceMetadata, DeviceType, IODirection, IOEvent, IOKind, IdType,
};
use crate::storage::{Chronicle, Log};

#[derive(Default)]
pub struct GenericInput {
    metadata: DeviceMetadata,
    log: Option<Def<Log>>,
    publisher: Option<Publisher>,
    command: Option<IOCommand>,
}

// Implement traits
impl Device for GenericInput {
    /// Creates a mock sensor which returns a value
    ///
    /// # Arguments
    /// * `name`: arbitrary name of sensor
    /// * `id`: arbitrary, numeric ID to differentiate from other sensors
    ///
    /// returns: MockPhSensor
    fn new(name: String, id: IdType, kind: Option<IOKind>) -> Self
    where
        Self: Sized,
    {
        let kind = kind.unwrap_or_default();

        let metadata: DeviceMetadata = DeviceMetadata::new(name, id, kind, IODirection::Input);

        let publisher = None;
        let command = None;
        let log = None;

        Self {
            metadata,
            log,
            publisher,
            command,
        }
    }

    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }

    fn add_command(mut self, command: IOCommand) -> Self
    where
        Self: Sized,
    {
        self.command = Some(command);
        self
    }

    fn set_log(&mut self, log: Def<Log>) {
        self.log = Some(log);
    }

    fn into_variant(self) -> DeviceType {
        DeviceType::Input(self)
    }
}

impl GenericInput {
    /// Execute low-level GPIO command
    fn rx(&self) -> Result<IOEvent, ErrorType> {
        let read_value = if let Some(command) = &self.command {
            let result = command.execute(None).unwrap();
            result.unwrap()
        } else {
            return Err(no_internal_closure());
        };

        Ok(self.generate_event(read_value))
    }

    /// Propagate `IOEvent` to all subscribers.
    ///
    /// No error is raised when there is no associated publisher.
    fn propagate(&mut self, event: &IOEvent) {
        if let Some(publisher) = &mut self.publisher {
            publisher.propagate(&event);
        };
    }

    /// Get IOEvent, add to log, and propagate to publisher/subscribers
    ///
    /// Primary interface method during polling.
    ///
    /// # Notes
    /// This method will fail if there is no associated log
    pub fn read(&mut self) -> Result<IOEvent, ErrorType> {
        let event = self.rx().expect("Error returned by `rx()`");

        self.propagate(&event);

        self.add_to_log(event);

        Ok(event)
    }

    /// Create and set publisher or silently fail
    pub fn init_publisher(&mut self) -> &mut Self {
        match self.publisher {
            None => {
                self.publisher = Some(Publisher::default());
            }
            _ => {
                eprintln!("Publisher already exists!");
            }
        }
        self
    }

    pub fn publisher_mut(&mut self) -> &mut Option<Publisher> {
        &mut self.publisher
    }

    pub fn publisher(&self) -> &Option<Publisher> {
        &self.publisher
    }

    pub fn has_publisher(&self) -> bool {
        match self.publisher {
            Some(_) => true,
            None => false,
        }
    }
}

impl Chronicle for GenericInput {
    fn log(&self) -> Option<Def<Log>> {
        self.log.clone()
    }
}

// Testing
#[cfg(test)]
mod tests {
    use crate::action::{IOCommand};
    use crate::io::{Device, GenericInput, RawValue};
    use crate::storage::Chronicle;

    const DUMMY_OUTPUT: RawValue = RawValue::Float(1.2);
    const COMMAND: IOCommand = IOCommand::Input(move || DUMMY_OUTPUT);

    #[test]
    fn test_rx() {
        let mut input = GenericInput::default();

        input.command = Some(COMMAND);

        let event = input.rx().unwrap();
        assert_eq!(event.data.value, DUMMY_OUTPUT);
    }

    #[test]
    fn test_read() {
        let mut input = GenericInput::default().init_log(None);
        let log = input.log();

        input.command = Some(COMMAND);

        assert_eq!(log.clone().unwrap().try_lock().unwrap().iter().count(), 0);

        let event = input.read().unwrap();
        assert_eq!(event.data.value, DUMMY_OUTPUT);
        assert_eq!(event.data.kind, input.kind());

        // assert that event was added to log
        assert_eq!(log.unwrap().try_lock().unwrap().iter().count(), 1);
    }

    /// Test `::add_publisher()` and `::has_publisher()`
    #[test]
    fn test_init_publisher() {
        let mut input = GenericInput::default();

        assert_eq!(false, input.has_publisher());

        input.init_publisher();

        assert_eq!(true, input.has_publisher());
    }

    #[test]
    fn test_init_log() {
        let mut input = GenericInput::default();

        assert_eq!(false, input.has_log());

        input = input.init_log(None);

        assert_eq!(true, input.has_log());
    }
}
