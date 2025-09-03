use std::sync::atomic::{AtomicU64, AtomicU8};

/// Node is in an idle state.
const EXEC_NOT_RUNNING: u8 = 0;

/// Node is waiting for values.
const EXEC_WAITING: u8 = 1;

/// Node got all values, and running the inner function.
const EXEC_RUNNING: u8 = 2;

pub struct ExecutionState {
    subtasks: AtomicU64,
    current_subtask: AtomicU64,
    state: AtomicU8,
}

impl ExecutionState {
    pub fn new() -> Self {
        Self {
            subtasks: AtomicU64::new(0),
            current_subtask: AtomicU64::new(0),
            state: AtomicU8::new(EXEC_NOT_RUNNING),
        }
    }

    pub fn is_waiting(&self) -> bool {
        self.state.load(std::sync::atomic::Ordering::Relaxed) == EXEC_WAITING
    }

    pub fn is_running(&self) -> bool {
        self.state.load(std::sync::atomic::Ordering::Relaxed) == EXEC_RUNNING
    }

    pub fn is_not_running(&self) -> bool {
        self.state.load(std::sync::atomic::Ordering::Relaxed) == EXEC_NOT_RUNNING
    }

    pub fn waiting(&self) {
        self.state
            .store(EXEC_WAITING, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn running(&self) {
        self.state
            .store(EXEC_RUNNING, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn not_running(&self) {
        self.state
            .store(EXEC_NOT_RUNNING, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_subtasks(&self, subtasks: u64) {
        self.subtasks
            .store(subtasks, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_current_subtask(&self, current: u64) {
        self.current_subtask
            .store(current, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn subtask_done(&self) {
        self.current_subtask
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn get_current_subtask(&self) -> u64 {
        self.current_subtask
            .load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn get_subtask_count(&self) -> u64 {
        self.subtasks.load(std::sync::atomic::Ordering::SeqCst)
    }
}
