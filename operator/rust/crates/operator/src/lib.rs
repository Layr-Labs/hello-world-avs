// Start creating tasks and respond appropriately

/// Create createNewTask at regular intervals with random task names
mod spam_tasks;
pub use spam_tasks::*;
/// Register Operator and monitor for NewTaskCreated event
pub mod start_operator;

pub mod test_utils;
