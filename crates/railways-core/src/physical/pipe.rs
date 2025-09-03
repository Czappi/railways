use crossbeam_channel::{Receiver, Sender};
use thiserror::Error;

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
    pub fn send(&self, value: T) -> Result<(), PipeError<T>> {
        self.inner
            .send(value)
            .map_err(|err| PipeError::Send(err.into_inner()))
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
    pub fn receive(&self) -> Result<T, PipeError<T>> {
        self.inner.recv().map_err(|_| PipeError::Receive)
    }

    pub fn receive_n<const N: usize>(&self) -> Result<[T; N], PipeError<T>> {
        self.inner
            .iter()
            .take(N)
            .collect::<Vec<T>>()
            .try_into()
            .map_err(|_| PipeError::Receive)
    }
}
