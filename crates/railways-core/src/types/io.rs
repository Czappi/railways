use tokio::sync::broadcast::{
    error::{RecvError, SendError},
    Receiver, Sender,
};

pub trait NodeIO {
    type Value: Clone;

    fn is_unused(&self) -> bool;

    fn name(&self) -> &str;
}

pub struct Source<T: Clone> {
    name: String,
    sender: Sender<T>,
}

impl<T: Clone> NodeIO for Source<T> {
    type Value = T;

    fn is_unused(&self) -> bool {
        self.sender.receiver_count() == 0
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl<T: Clone> Source<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sender: Sender::new(16),
        }
    }

    pub fn send(&self, value: T) -> Result<usize, SendError<T>> {
        self.sender.send(value)
    }

    pub fn subscribe(&self) -> Receiver<T> {
        self.sender.subscribe()
    }
}

pub struct RequiredTarget<T: Clone> {
    name: String,
    receiver: Receiver<T>,
}

impl<T: Clone> NodeIO for RequiredTarget<T> {
    type Value = T;

    fn is_unused(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl<T: Clone> RequiredTarget<T> {
    pub fn new(name: impl Into<String>, receiver: Receiver<T>) -> Self {
        Self {
            name: name.into(),
            receiver,
        }
    }

    pub async fn receive(&mut self) -> Result<T, RecvError> {
        self.receiver.recv().await
    }
}

pub struct OptionalTarget<T: Clone> {
    name: String,
    receiver: Option<Receiver<T>>,
}

impl<T: Clone> NodeIO for OptionalTarget<T> {
    type Value = T;

    fn is_unused(&self) -> bool {
        self.receiver.is_none()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl<T: Clone> OptionalTarget<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            receiver: None,
        }
    }

    pub fn set_receiver(&mut self, receiver: Receiver<T>) {
        self.receiver = Some(receiver);
    }

    pub async fn receive(&mut self) -> Option<Result<T, RecvError>> {
        match self.receiver {
            Some(ref mut receiver) => match receiver.recv().await {
                Ok(value) => Some(Ok(value)),
                Err(e) => Some(Err(e)),
            },
            None => None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
