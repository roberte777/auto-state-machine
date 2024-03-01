//! # MyCrate (Crate Name Placeholder)
//!
//! This crate provides a flexible, state-driven automation client designed to manage and execute
//! state transitions based on user-defined logic and conditions. It's built to facilitate complex
//! workflows where operations need to occur in a specific order, with pauses, resumes, and stops,
//! making it ideal for simulations, automated processes, or game logic.
//!
//! ## Features
//!
//! - **State Management**: Define states and their associated callbacks to manage the flow of your application.
//! - **Control Flow**: Dynamically control the execution flow with pause, resume, and stop functionalities.
//! - **Tick Rate Control**: Specify the rate at which the client checks and updates states, allowing for
//!   fine-tuned control over execution speed.
//! - **User Context**: Pass a user-defined context through states, enabling stateful operations and data
//!   persistence across state transitions.
//!
//! ## Quick Start
//!
//! Add `mycrate` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mycrate = "0.1.0"
//! ```
//!
//! ### Example
//!
//! Here's a basic example to get you started:
//!
//! ```rust
//! use autostatemachine::{StateMachineBuilder, StateMachineContext, extractor::State};
//! use std::time::Duration;
//!
//! fn sample_callback(context: StateMachineContext, State(user_context): State<()>) -> String {
//!     println!("Current state: {}", context.current_state);
//!     "init".to_string()
//! }
//!
//! let mut client = StateMachineBuilder::new(())
//!     .add_state("init".to_string(), sample_callback)
//!     .initial_state("init".to_string())
//!     .tick_rate(Duration::from_secs(1))
//!     .build();
//!
//! client.run();
//! // The client is now running, transitioning from "init" to "next_state"
//! // according to the logic you've defined.
//! std::thread::sleep(Duration::from_millis(50));
//! client.stop();
//! ```
//!
//! ## Control Flow Methods
//!
//! - `run()`: Start the client's execution, allowing state transitions to occur.
//! - `pause()`: Pause the execution, freezing the current state.
//! - `resume()`: Resume execution from the current state.
//! - `stop()`: Stop execution, resetting to the initial state.
//!
//! This crate aims to simplify the creation of automated, state-driven systems with minimal boilerplate
//! and high flexibility. For more detailed documentation and advanced usage, please refer to the specific
//! module and method documentation within the crate.
pub mod builder;
mod callback;
pub mod context;
pub mod extractor;
pub use builder::StateMachineBuilder;
pub use context::StateMachineContext;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use callback::StoredCallback;

pub struct StateMachine<S>
where
    S: Clone + Send + Sync + 'static,
{
    handlers: Arc<HashMap<String, StoredCallback<S>>>,
    tick_rate: Duration,
    context: Arc<Mutex<StateMachineContext>>,
    user_context: S,
}
impl<S> StateMachine<S>
where
    S: Clone + Send + Sync,
{
    pub fn new(
        handlers: HashMap<String, StoredCallback<S>>,
        tick_rate: Duration,
        initial_state: String,
        user_context: S,
    ) -> Self {
        Self {
            handlers: Arc::new(handlers),
            tick_rate,
            context: Arc::new(Mutex::new(StateMachineContext {
                tick_rate,
                current_state: initial_state.clone(),
                initial_state,
                life_cycle: context::LifeCycle::Stopped,
            })),
            user_context,
        }
    }
    pub fn get_context(&self) -> StateMachineContext {
        self.context.lock().unwrap().clone()
    }
    pub fn get_user_context(&self) -> &S {
        &self.user_context
    }
    pub fn get_tick_rate(&self) -> &Duration {
        &self.tick_rate
    }
    pub fn pause(&mut self) {
        self.context.lock().unwrap().life_cycle = context::LifeCycle::Paused;
    }
    pub fn resume(&mut self) {
        self.context.lock().unwrap().life_cycle = context::LifeCycle::Running;
    }
    pub fn stop(&mut self) {
        self.context.lock().unwrap().life_cycle = context::LifeCycle::Stopped;
    }
    pub fn run(&mut self) {
        self.context.lock().unwrap().life_cycle = context::LifeCycle::Running;
        let context = self.context.clone();
        let user_context = self.user_context.clone();
        let handlers = self.handlers.clone();
        std::thread::spawn(move || loop {
            let mut context_guard = context.lock().unwrap();
            let tick_rate = context_guard.tick_rate;
            match context_guard.life_cycle {
                context::LifeCycle::Paused => {
                    drop(context_guard);
                    std::thread::sleep(tick_rate);
                }
                context::LifeCycle::Stopped => {
                    context_guard.life_cycle = context::LifeCycle::Stopped;
                    context_guard.current_state = context_guard.initial_state.clone();
                    break;
                }
                context::LifeCycle::Running => {
                    let handler = handlers.get(&context_guard.current_state).unwrap();
                    let output = handler.call(&context_guard, &mut user_context.clone());
                    context_guard.current_state = output;
                    drop(context_guard);
                    std::thread::sleep(tick_rate);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::StateMachineBuilder;

    fn test1(_: StateMachineContext) -> String {
        println!("test1");
        "test2".to_string()
    }
    fn test2(_: StateMachineContext) -> String {
        println!("test2");
        "test1".to_string()
    }
    #[test]
    fn test_basic_run() {
        let mut client = StateMachineBuilder::new("".to_string())
            .add_state("test1".to_string(), test1)
            .add_state("test2".to_string(), test2)
            .initial_state("test1".to_string())
            .build();
        client.run();
        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(client.get_context().current_state, "test2");
        std::thread::sleep(Duration::from_millis(51));
        assert_eq!(client.get_context().current_state, "test1");
        client.stop();
    }
    #[test]
    fn test_pause() {
        let mut client = StateMachineBuilder::new("".to_string())
            .add_state("test1".to_string(), test1)
            .add_state("test2".to_string(), test2)
            .initial_state("test1".to_string())
            .build();
        client.run();
        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(client.get_context().current_state, "test2");
        client.pause();
        std::thread::sleep(Duration::from_millis(51));
        assert_eq!(client.get_context().current_state, "test2");
        client.resume();
        std::thread::sleep(Duration::from_millis(51));
        assert_eq!(client.get_context().current_state, "test1");
        client.stop();
    }
}
