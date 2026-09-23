use crate::error::{ControllerError, ControllerResult};
use tokio::sync::watch;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct SimulationLoopHandle {
    stop_tx: watch::Sender<bool>,
    join_handle: JoinHandle<ControllerResult<()>>,
}

impl SimulationLoopHandle {
    pub fn new(
        stop_tx: watch::Sender<bool>,
        join_handle: JoinHandle<ControllerResult<()>>,
    ) -> Self {
        Self {
            stop_tx,
            join_handle,
        }
    }

    pub fn stop(&self) {
        let _ = self.stop_tx.send(true);
    }

    pub fn is_finished(&self) -> bool {
        self.join_handle.is_finished()
    }

    pub fn abort(&self) {
        self.join_handle.abort();
    }

    pub async fn join(self) -> ControllerResult<()> {
        match self.join_handle.await {
            Ok(result) => result,
            Err(join_err) => Err(ControllerError::InvalidData(format!(
                "Simulation loop task join error: {}",
                join_err
            ))),
        }
    }

    pub async fn stop_and_join(self) -> ControllerResult<()> {
        self.stop();
        self.join().await
    }
}
