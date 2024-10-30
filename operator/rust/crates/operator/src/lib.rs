//! Start creating tasks and respond appropriately
//! testing utils
/// Create createNewTask at regular intervals with random task names
pub mod spam_tasks;
/// Register Operator and monitor for NewTaskCreated event
pub mod start_operator;
/// test utils
#[cfg(test)]
pub mod test_utils;
