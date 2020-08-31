//! Service module

use async_trait::async_trait;
use std::time::SystemTime;

pub mod controller;
pub use controller::*;
pub mod manager;
pub use manager::*;
pub mod runner;

#[async_trait]
pub trait ServiceTask {
    async fn run_service_check(&mut self) -> Result<(), String>;

    /// ServiceTasks are triggered often ( 1s interval )
    /// This method should tell whether run also this ServiceTask in current run.
    fn should_run(&self) -> bool;

    /// Should run when elapsed time from last run is higher then interval
    fn should_run_last_interval(
        &self,
        last_run: Option<SystemTime>,
        interval: std::time::Duration,
    ) -> bool {
        match last_run {
            Some(last_run) => {
                let elapsed = last_run.elapsed();
                if let Ok(elapsed) = elapsed {
                    elapsed > interval // true else false
                } else {
                    false
                }
            }
            None => true,
        }
    }
}

#[derive(Debug)]
pub enum Message {
    RunCheck,
    Terminate,
    StartTimer,
    StopTimer,
}
