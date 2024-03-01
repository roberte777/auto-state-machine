//! Builder for StateMachine
//!
//! Provides a simple way to construct the functionality for your StateMachine
//! Tell it `what` to do by adding states and `how` to do it by providing a callback
//!
//! ```rust
//! use autostatemachine::blocking::{StateMachineBuilder, StateMachineContext, extractor::TickRate};
//! use std::time::Duration;
//! fn test1(_: StateMachineContext) -> String {
//!    println!("test1");
//!    "test2".to_string()
//! }
//! fn test2(_: StateMachineContext, TickRate(r): TickRate) -> String {
//!    println!("TickRate: {:?}", r);
//!    "test".to_string()
//! }
//! let client = StateMachineBuilder::new(())
//!     .add_state("test".to_string(), test1)
//!     .add_state("test2".to_string(), test2)
//!     .initial_state("test".to_string())
//!     .tick_rate(Duration::from_millis(100))
//!     .build();
//! ```
use std::{collections::HashMap, time::Duration};

use crate::blocking::callback::IntoCallback;
use crate::blocking::StateMachine;

use crate::blocking::callback::{Callback, StoredCallback};
/// Builder for StateMachine
pub struct StateMachineBuilder<S> {
    handlers: HashMap<String, StoredCallback<S>>,
    tick_rate: Duration,
    initial_state: Option<String>,
    user_context: S,
}

impl<S> StateMachineBuilder<S>
where
    S: Clone + Send + Sync,
{
    /// Create a new StateMachineBuilder
    ///
    /// # Arguments
    /// * `user_context` - The user context to be passed to the callbacks.
    /// Intended to act as some sort of state you can use in your callbacks.
    ///
    /// # Example
    /// ```rust no_run
    /// use autostatemachine::blocking::StateMachineBuilder;
    /// use std::sync::{Arc, Mutex};
    /// // Pass it in () if you don't care about user_context
    /// let client = StateMachineBuilder::new(()).build();
    /// // If you want to modify the user_context in your handlers,
    /// // use Arc<Mutex<T>> or Arc<RwLock<T>> to wrap your user_context
    /// // Then, the updates you make in your handlers will also be reflected
    /// // outside of the handlers
    /// let client = StateMachineBuilder::new(Arc::new(Mutex::new(0))).build();
    /// ```
    pub fn new(user_context: S) -> Self {
        Self {
            handlers: HashMap::new(),
            tick_rate: Duration::from_millis(50),
            initial_state: None,
            user_context,
        }
    }
    /// Add a state to the StateMachine
    /// # Arguments
    /// * `name` - The name of the state
    /// * `f` - The callback to be called when the state is active
    /// # Example
    /// ```rust
    /// use autostatemachine::blocking::{StateMachineBuilder, StateMachineContext};
    /// fn test1(_: StateMachineContext) -> String {
    ///   println!("test1");
    ///   "test1".to_string()
    /// }
    /// let client = StateMachineBuilder::new(())
    ///  .add_state("test".to_string(), test1)
    ///  .initial_state("test".to_string())
    ///  .build();
    ///  ```
    pub fn add_state<I, C: Callback<S> + 'static>(
        mut self,
        name: String,
        f: impl IntoCallback<I, S, Callback = C>,
    ) -> Self {
        self.handlers.insert(name, Box::new(f.into_callback()));
        self
    }
    pub fn tick_rate(mut self, tick_rate: Duration) -> Self {
        self.tick_rate = tick_rate;
        self
    }
    pub fn initial_state(mut self, initial_state: String) -> Self {
        self.initial_state = Some(initial_state);
        self
    }
    pub fn build(self) -> StateMachine<S> {
        if self.handlers.is_empty() {
            panic!("No states added");
        }
        let initial_state = self.initial_state.expect("Initial state not set");
        StateMachine::new(
            self.handlers,
            self.tick_rate,
            initial_state,
            self.user_context,
        )
    }
}
#[cfg(test)]
mod tests {
    fn test1(_: StateMachineContext) -> String {
        println!("test1");
        "test2".to_string()
    }
    fn test2(_: StateMachineContext, TickRate(r): TickRate) -> String {
        println!("TickRate: {:?}", r);
        "test".to_string()
    }
    use crate::{blocking::context::StateMachineContext, blocking::extractor::TickRate};

    use super::*;
    #[test]
    fn test_basic() {
        let client = StateMachineBuilder::new(())
            .add_state("test".to_string(), test1)
            .add_state("test2".to_string(), test2)
            .initial_state("test".to_string())
            .build();
        assert_eq!(client.get_context().current_state, "test");
        assert_eq!(client.get_tick_rate(), &Duration::from_millis(50));
        assert_eq!(client.get_user_context(), &());
        assert_eq!(client.handlers.len(), 2);
    }
    #[test]
    #[should_panic]
    fn test_no_states() {
        let _client = StateMachineBuilder::new(()).build();
    }
    #[test]
    #[should_panic]
    fn test_no_initial_state() {
        let _client = StateMachineBuilder::new(())
            .add_state("test".to_string(), test1)
            .add_state("test2".to_string(), test2)
            .build();
    }
}
