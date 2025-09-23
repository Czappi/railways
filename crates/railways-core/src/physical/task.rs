use std::{error::Error, sync::Arc};

use opentelemetry::{global::BoxedTracer, trace::Tracer};

use crate::physical::{
    morsel::Morsel,
    pipe::{ReceivePipe, SendPipe},
    scope::TaskScope,
    ComputeNode,
};

pub enum TaskPriority {
    /// Main tasks
    High,
    /// Auxiliary tasks which have dependents
    Normal,
    /// Background (DAG tail) tasks
    Low,
}

pub struct Task {
    receivers: Vec<ReceivePipe<Morsel>>,
    senders: Vec<SendPipe<Morsel>>,
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
            compute,
        }
    }

    pub async fn run(self, tracer: &BoxedTracer) -> Result<(), Box<dyn Error>> {
        let result = tracer.in_span(self.compute.name().to_owned(), |ctx| {
            let scope = TaskScope::new(ctx, tracer);

            let result =
                self.compute
                    .spawn(self.receivers.as_slice(), self.senders.as_slice(), &scope);

            if let Err(error) = &result {
                scope.record_error(error);
            }

            result
        });

        result?;
        Ok(())
    }
}
