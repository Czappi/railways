use std::{any::Any, collections::HashMap, error::Error};

use thiserror::Error;

use crate::physical::{
    morsel::Morsel,
    pipe::{ReceivePipe, SendPipe},
    state::ExecutionState,
};

pub mod morsel;
pub mod pipe;
pub mod state;
pub mod task;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct ComputeError(#[from] pub Box<dyn Error>);

pub trait ComputeNode {
    fn spawn<'env, 'run>(
        &'env self,
        receivers: &'env [ReceivePipe<Morsel>],
        senders: &'env [SendPipe<Morsel>],
        state: &'run ExecutionState,
    ) -> Result<(), ComputeError>;

    fn name(&self) -> &str;

    fn state(&self) -> Option<HashMap<String, Box<dyn Any>>>;
}
