use crossbeam_channel::{Receiver, Sender};

use thiserror::Error;

use crate::physical::{morsel::Morsel, pipe::information::PipeInformation, scope::TaskScope};

#[derive(Error, Debug)]
pub enum PipeError<T> {
    #[error("Couldn't receive from the pipe.")]
    Receive,
    #[error("Couldn't send this through the pipe.")]
    Send(T),
}

pub struct Pipe<T> {
    send: SendPipe<T>,
    receive: ReceivePipe<T>,
}

impl<T> Pipe<T> {
    pub fn new() -> Self {
        let (send, receive) = crossbeam_channel::unbounded();
        Self {
            send: SendPipe { inner: send },
            receive: ReceivePipe { inner: receive },
        }
    }

    pub fn sender(&self) -> SendPipe<T> {
        self.send.clone()
    }

    pub fn receiver(&self) -> ReceivePipe<T> {
        self.receive.clone()
    }
}

#[derive(Debug)]
pub struct SendPipe<T> {
    inner: Sender<T>,
}

impl<T> Clone for SendPipe<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> SendPipe<T> {
    pub fn send(&self, scope: &TaskScope, value: T) -> Result<(), PipeError<T>> {
        let scope = scope.child("SendPipe::send");
        scope.set_attributes(
            PipeInformation::builder()
                .operation(information::PipeOperation::Send)
                .build(),
        );

        self.inner
            .send(value)
            .map_err(|err| PipeError::Send(err.into_inner()))?;

        scope.add_event("value sent", Vec::new());

        drop(scope);

        Ok(())
    }
}

impl SendPipe<Morsel> {
    pub fn send_morsel(&self, scope: &TaskScope, value: Morsel) -> Result<(), PipeError<Morsel>> {
        self.send(scope, value.with_span_context(scope.span_context().clone()))
    }
}

#[derive(Debug)]
pub struct ReceivePipe<T> {
    inner: Receiver<T>,
}

impl<T> Clone for ReceivePipe<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> ReceivePipe<T> {
    pub fn receive(&self, scope: &TaskScope) -> Result<T, PipeError<T>> {
        let scope = scope.child("ReceivePipe::receive");
        scope.set_attributes(
            PipeInformation::builder()
                .operation(information::PipeOperation::Receive)
                .build(),
        );

        let value = self.inner.recv().map_err(|_| PipeError::Receive)?;

        Ok(value)
    }

    pub fn receive_n<const N: usize>(&self, scope: &TaskScope) -> Result<[T; N], PipeError<T>> {
        let scope = scope.child("ReceivePipe::receive");
        scope.set_attributes(
            PipeInformation::builder()
                .operation(information::PipeOperation::Receive)
                .message_count(N)
                .build(),
        );

        let values = self
            .inner
            .iter()
            .take(N)
            .collect::<Vec<T>>()
            .try_into()
            .map_err(|_| PipeError::Receive)?;

        Ok(values)
    }
}

impl ReceivePipe<Morsel> {
    pub fn receive_morsel(&self, scope: &TaskScope) -> Result<Morsel, PipeError<Morsel>> {
        let value = self.receive(scope)?;

        scope.add_link(value.span_context().clone(), Vec::new());

        Ok(value)
    }

    pub fn receive_n_morsel<const N: usize>(
        &self,
        scope: &TaskScope,
    ) -> Result<[Morsel; N], PipeError<Morsel>> {
        let values = self.receive_n(scope)?;

        for value in values.as_ref() {
            scope.add_link(value.span_context().clone(), Vec::new())
        }

        Ok(values)
    }
}

/// Complience with: https://opentelemetry.io/docs/specs/semconv/registry/attributes/messaging/
mod information {
    use opentelemetry::KeyValue;
    use typed_builder::TypedBuilder;

    #[derive(Debug)]
    pub enum PipeOperation {
        Send,
        Receive,
    }

    #[derive(Debug, TypedBuilder)]
    pub struct PipeInformation {
        /// - `messaging.operation.type`
        /// - `messaging.operation.name`
        operation: PipeOperation,

        /// - `messaging.destination.name`
        /// - `messaging.destination.anonymous`
        #[builder(default, setter(strip_option))]
        destination: Option<String>,

        /// For remote nodes
        /// - `server.address`
        #[builder(default, setter(strip_option))]
        server_address: Option<String>,

        /// For remote nodes
        /// - `messaging.client.id`
        #[builder(default, setter(strip_option))]
        client_id: Option<String>,

        /// For remote nodes
        /// - `messaging.message.id`
        #[builder(default, setter(strip_option))]
        message_id: Option<String>,

        /// - `messaging.batch.message_count`
        #[builder(default, setter(strip_option))]
        message_count: Option<usize>,
    }

    impl Into<Vec<KeyValue>> for PipeInformation {
        fn into(self) -> Vec<KeyValue> {
            let op = match self.operation {
                PipeOperation::Send => "send",
                PipeOperation::Receive => "receive",
            };

            let mut result = vec![
                KeyValue::new("messaging.operation.type", op),
                KeyValue::new("messaging.operation.name", op),
                KeyValue::new("messaging.system", "railways"),
            ];

            if let Some(destination) = self.destination {
                result.push(KeyValue::new("messaging.destination.name", destination));
                result.push(KeyValue::new("messaging.destination.anonymous", false));
            } else {
                result.push(KeyValue::new("messaging.destination.anonymous", true));
            }

            if let Some(server_address) = self.server_address {
                result.push(KeyValue::new("server.address", server_address));
            }

            if let Some(message_count) = self.message_count {
                result.push(KeyValue::new(
                    "messaging.batch.message_count",
                    message_count.to_string(),
                ));
            }

            result
        }
    }
}
