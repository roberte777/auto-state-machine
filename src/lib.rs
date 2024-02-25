pub mod builder;
pub mod callback;
pub mod context;
pub mod extractor;
use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use callback::StoredCallback;

use crate::context::AutoClientContext;

pub struct AutoClient<S>
where
    S: Clone + Send + Sync + 'static,
{
    handlers: Arc<HashMap<String, StoredCallback>>,
    tick_rate: Duration,
    context: Arc<Mutex<AutoClientContext>>,
    user_context: S,
}
impl<S> AutoClient<S>
where
    S: Clone + Send + Sync,
{
    pub fn new(
        handlers: HashMap<String, StoredCallback>,
        tick_rate: Duration,
        initial_state: String,
        user_context: S,
    ) -> Self {
        Self {
            handlers: Arc::new(handlers),
            tick_rate,
            context: Arc::new(Mutex::new(AutoClientContext {
                tick_rate,
                current_state: initial_state.clone(),
                initial_state,
                life_cycle: context::LifeCycle::Stopped,
            })),
            user_context,
        }
    }
    pub fn get_context(&self) -> AutoClientContext {
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
                    let mut boxed_state = Box::new(user_context.clone()) as Box<dyn Any>;
                    let output = handler.call(&context_guard, &mut boxed_state);
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
    use crate::builder::AutoClientBuilder;

    fn test1(_: AutoClientContext) -> String {
        println!("test1");
        "test2".to_string()
    }
    fn test2(_: AutoClientContext) -> String {
        println!("test2");
        "test1".to_string()
    }
    #[test]
    fn test_basic_run() {
        let mut client = AutoClientBuilder::new()
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
        let mut client = AutoClientBuilder::new()
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
