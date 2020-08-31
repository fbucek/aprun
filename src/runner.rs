use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use super::ServiceManager;

pub enum RunnerEvent {
    Run(String),
    Message(String),
    Stop,
}

/// Control used to control ServiceRunnerDirect -> running in separate thread
pub struct RunnerController {
    pub runner: Arc<ServiceRunnerDirect>,
}

/// ServiceRunnerDirect is running in thread and is accessible from Actix server using Control
pub struct ServiceRunnerDirect {
    /// wait time between checks ( usually one minute )
    pub wait_time: AtomicU64,
    pub stop: AtomicBool,
    pub running: AtomicBool,
    pub service_manager: Arc<Mutex<ServiceManager>>,
}

impl ServiceRunnerDirect {
    pub fn new(service_manager: Arc<Mutex<ServiceManager>>) -> Arc<Self> {
        Arc::new(ServiceRunnerDirect {
            wait_time: AtomicU64::new(1000),
            stop: AtomicBool::new(false),
            running: AtomicBool::new(false),
            service_manager,
        })
    }

    /// It will start loop running awaiting 3 sec ( not blocking current thread )
    ///
    /// ## Note
    ///
    /// self.stop.load(Ordering::Relaxed) this does not make sense -> it will be checked once per interval ( it wont stop already running check )
    /// So this is not needed -> messages could be send throught std::sync::mpsc channel
    /// This should be redesigned
    async fn start(&self) {
        let wait = self.wait_time.load(Ordering::Relaxed);
        let mut interval = tokio::time::interval(Duration::from_millis(wait));

        loop {
            if self.stop.load(Ordering::Relaxed) {
                info!("Control: ServiceRunnerDirect stop");
                self.running.swap(false, Ordering::Relaxed);
                break;
            }

            // Create interval -> first call imediatelly -> next will wait
            interval.tick().await;

            {
                // // Write lock in separate scope -> will be destroyed when it goes out of scope
                // let mut service_manager = self.service_manager.lock().await; // Checker write lock
                // match service_manager.load() {
                //     Ok(_) => info!("Service manager reloaded"),
                //     Err(err) => error!("Checker reload error: {:?}", err),
                // }
            }

            let mut service_manager = self.service_manager.lock().await;
            service_manager
                .async_parallel_check()
                .await
                .unwrap_or_else(|err| error!("Not possible to finish checks {:?}", err));
        }
    }
}

/// Control is controlling ServiceRunnerDirect
impl RunnerController {
    pub fn new(runner: &Arc<ServiceRunnerDirect>) -> Arc<Mutex<Self>> {
        // must be in Arc because it is shared amog http actix threads
        Arc::new(Mutex::new(RunnerController {
            runner: runner.clone(),
        }))
    }

    pub async fn run(&self) {
        if self.runner.running.load(Ordering::Relaxed) {
            warn!("ServiceRunnerDirect alread running");
        } else {
            self.runner.stop.swap(false, Ordering::Relaxed);
            self.runner.running.swap(true, Ordering::Relaxed);
            let runner = self.runner.clone();
            // Process each socket concurrently.
            tokio::spawn(async move { runner.start().await });

            info!("ServiceRunnerDirect started");
        }
    }

    pub fn stop(&self) {
        self.runner.stop.swap(true, Ordering::Relaxed);
        info!("Stopping runner");
    }
}
