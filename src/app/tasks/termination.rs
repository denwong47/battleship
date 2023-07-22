use std::{
    mem,
    ops::DerefMut,
    sync::{Arc, RwLock},
};
use tokio::sync::Notify;

use crate::error::AppError;

#[derive(Debug, Default)]
pub struct TerminationToken {
    notify: Arc<Notify>,
    result: RwLock<Option<Result<(), AppError>>>,
}

impl TerminationToken {
    /// Create a new [`TerminationToken`] for termination signaling among threads.
    pub fn new() -> Self {
        Self::default()
    }

    /// Notify this [`TerminationToken`] to end.
    pub fn notify(&self, result: Result<(), AppError>) {
        if let Ok(mut write_ref) = self
            .result
            .write()
            .map_err(|_| AppError::LockPoisoned("Termination Token"))
        {
            *write_ref = Some(result);
        };

        self.notify.notify_waiters()
    }

    /// The coroutine to listen for any termination notifications.
    ///
    /// Typically used as one of the [`tokio::select`] members.
    pub async fn task(self: Arc<Self>) -> Result<(), AppError> {
        match async_ctrlc::CtrlC::new() {
            Ok(ctrlc) => {
                tokio::select! {
                    _ = ctrlc => {
                        Ok(())
                    },
                    _ = self.notify.notified() => {
                        self.submit_result(Ok(()))
                            .await
                            .unwrap_or(Err(AppError::CtrlCError {
                                message: "Cannot submit result after notification.".to_owned()
                            }))
                    },
                }
            }
            Err(err) => Err(AppError::CtrlCError {
                message: err.to_string(),
            }),
        }
    }

    /// Replace the current result embedded in this [`TerminationToken`] with another
    /// instance. The existing one is returned.
    async fn submit_result(&self, result: Result<(), AppError>) -> Option<Result<(), AppError>> {
        let mut some_result = Some(result);

        match self.result.write() {
            Ok(mut result_ref) => {
                mem::swap(&mut some_result, result_ref.deref_mut());

                some_result
            }
            Err(_) => {
                panic!(
                    "`TerminationToken` has a poisoned `RwLock` for its `result`. Intended result: {:?}",
                    some_result.unwrap()
                )
            }
        }
    }
}
