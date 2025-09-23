use std::{any::Any, collections::HashMap, error::Error};

use thiserror::Error;

use crate::physical::{
    morsel::Morsel,
    pipe::{ReceivePipe, SendPipe},
    scope::TaskScope,
};

pub mod morsel;
pub mod pipe;
pub mod scope;
pub mod task;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct ComputeError(#[from] pub Box<dyn Error>);

pub trait ComputeNode {
    fn spawn<'env, 'run>(
        &'env self,
        receivers: &'env [ReceivePipe<Morsel>],
        senders: &'env [SendPipe<Morsel>],
        scope: &'run TaskScope<'run>,
    ) -> Result<(), ComputeError>;

    fn name(&self) -> &str;
}
