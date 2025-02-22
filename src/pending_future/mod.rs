use std::{future::Future, sync::Arc};

use crate::priority::Priority;

mod count_mode;
mod time_mode;

// pub fn new_pending_future<F>(mode: Mode, priority: Priority, f: F) -> PendingFuture<F>
// where
//     F: Future + Send + 'static,
//     F::Output: Send + 'static,
// {
//     match mode {
//         Mode::CountMode => PendingFuture::CountMode(count_mode::PendingFuture::new(priority, f)),
//         Mode::TimeMode => {
//             PendingFuture::TimeMode(time_mode::TimeModePendingFuture::new(priority, f))
//         }
//     }
// }

#[cfg(test)]
mod tests;
