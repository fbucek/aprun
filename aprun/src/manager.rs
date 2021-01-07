use std::collections::HashMap;
use tracing::*;

use crate::ServiceTask;

/// Manager handle ServiceTasks
#[derive(Default)]
pub struct ServiceManager {
    pub service_tasks: HashMap<String, Box<dyn ServiceTask + Send>>,
}

impl ServiceManager {
    pub fn add_service_task<T: Into<String>>(
        &mut self,
        service_name: T,
        service_task: Box<dyn ServiceTask + Send>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.service_tasks.insert(service_name.into(), service_task);
        Ok(())
    }

    /// Will perform service_task in parallel
    ///
    /// 1. Creates `Vec::new with futures to run.
    /// 2. Executes futures in parallel ( finishes when the last future finishes )
    pub async fn async_parallel_check(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Add futures into vector
        let mut futures = Vec::new();
        for service_task in self.service_tasks.values_mut() {
            if service_task.should_run() {
                // TODO: Add timeout not to wait too longhttps://docs.rs/tokio/0.2.15/tokio/time/index.html
                futures.push(service_task.run_service_check());
            }
        }

        // Run tasks in parallel -> will finish when last task is finished
        let res = futures::future::join_all(futures).await;

        // Process all results
        for result in &res {
            if let Err(err) = result {
                error!("Service task finished with errror {:?}", err);
            }
        }
        Ok(()) // have to restun someting
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add unitests
    //use super::*;

    // #[test]
    // fn init_test() {
    //     let mut service_manager = ServiceManager::default();
    //     assert!(service_manager.load_from("../cfg/hosts.yml").is_ok());
    //     // assert_eq!(service_manager.hosts.len(), 1);
    //     // reload
    //     assert!(service_manager.load_from("../cfg/hosts.yml").is_ok());
    //     // assert_eq!(service_manager.hosts.len(), 1);
    // }
}
