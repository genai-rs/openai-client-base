//! What the mock holds: the scripts, and the requests it received.

use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::reply::Reply;

#[derive(Default)]
struct Script {
    queue: VecDeque<Reply>,
    always: Option<Reply>,
}

pub(crate) struct State {
    pub(crate) api_key: Option<String>,
    scripts: Mutex<HashMap<&'static str, Script>>,
    received: Mutex<Vec<(&'static str, Arc<dyn Any + Send + Sync>)>>,
    rejected: Mutex<Vec<String>>,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").finish_non_exhaustive()
    }
}

/// A poisoned lock only means a test panicked while holding it: the data is still what it was.
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl State {
    pub(crate) fn new(api_key: Option<String>) -> Self {
        State {
            api_key,
            scripts: Mutex::default(),
            received: Mutex::default(),
            rejected: Mutex::default(),
        }
    }

    pub(crate) fn push(&self, operation: &'static str, reply: Reply) {
        lock(&self.scripts)
            .entry(operation)
            .or_default()
            .queue
            .push_back(reply);
    }

    pub(crate) fn always(&self, operation: &'static str, reply: Reply) {
        lock(&self.scripts).entry(operation).or_default().always = Some(reply);
    }

    /// Records the request and takes the operation's next reply, if one is scripted.
    pub(crate) fn take<T: Any + Send + Sync>(
        &self,
        operation: &'static str,
        request: T,
    ) -> Option<Reply> {
        lock(&self.received).push((operation, Arc::new(request)));
        let mut scripts = lock(&self.scripts);
        let script = scripts.get_mut(operation)?;
        script.queue.pop_front().or_else(|| script.always.clone())
    }

    pub(crate) fn requests<T: Any + Clone>(&self) -> Vec<T> {
        lock(&self.received)
            .iter()
            .filter_map(|(_, request)| request.downcast_ref::<T>().cloned())
            .collect()
    }

    pub(crate) fn calls(&self, operation: &str) -> usize {
        lock(&self.received)
            .iter()
            .filter(|(op, _)| *op == operation)
            .count()
    }

    pub(crate) fn reject(&self, message: String) {
        lock(&self.rejected).push(message);
    }

    pub(crate) fn rejected(&self) -> Vec<String> {
        lock(&self.rejected).clone()
    }

    pub(crate) fn reset(&self) {
        lock(&self.scripts).clear();
        lock(&self.received).clear();
        lock(&self.rejected).clear();
    }
}
