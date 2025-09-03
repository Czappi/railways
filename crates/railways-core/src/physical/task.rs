use std::sync::Arc;

use crate::physical::{
    morsel::Morsel,
    pipe::{ReceivePipe, SendPipe},
    state::ExecutionState,
    ComputeNode,
};

pub enum TaskPriority {
    /// Main tasks
    High,
    /// Auxiliary tasks which have dependents
    Normal,
    /// Background tasks
    Low,
}

pub struct Task {
    receivers: Vec<ReceivePipe<Morsel>>,
    senders: Vec<SendPipe<Morsel>>,
    state: ExecutionState,
    compute: Arc<dyn ComputeNode>,
}

impl Task {
    pub fn new(
        receivers: Vec<ReceivePipe<Morsel>>,
        senders: Vec<SendPipe<Morsel>>,
        compute: Arc<dyn ComputeNode>,
    ) -> Self {
        Self {
            receivers,
            senders,
            state: ExecutionState::new(),
            compute,
        }
    }
}
