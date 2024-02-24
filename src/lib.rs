pub mod context;
use std::{collections::HashMap, time::Duration};

use crate::context::{AutoClientContext, Callbacks, FromContext};

pub struct AutoClientBuilder<T> {
    handlers: Option<HashMap<String, Box<dyn Callbacks<T>>>>,
    tick_rate: Duration,
    initial_state: Option<String>,
}

impl<T> Default for AutoClientBuilder<T>
where
    T: FromContext,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AutoClientBuilder<T>
where
    T: FromContext,
{
    pub fn new() -> Self {
        Self {
            initial_state: None,
            handlers: None,
            tick_rate: Duration::from_millis(50),
        }
    }

    pub fn add_state<F>(self, name: String, f: F) -> Self
    where
        F: 'static + Callbacks<T> + Sized,
    {
        let handler: Box<dyn Callbacks<T>> = Box::new(f);
        let mut handlers = self.handlers.unwrap_or_default();
        handlers.insert(name, handler);
        Self {
            handlers: Some(handlers),
            ..self
        }
    }
    pub fn tick_rate(mut self, tick_rate: Duration) -> Self {
        self.tick_rate = tick_rate;
        self
    }
    pub fn initial_state(mut self, initial_state: String) -> Self {
        self.initial_state = Some(initial_state);
        self
    }
    pub fn build(self) -> AutoClient<T> {
        let handlers = self.handlers.unwrap();
        let initial_state = self.initial_state.unwrap();
        AutoClient::new(handlers, self.tick_rate, initial_state)
    }
}
pub struct AutoClient<T> {
    handlers: HashMap<String, Box<dyn Callbacks<T>>>,
    tick_rate: Duration,
    context: AutoClientContext,
}
impl<T> AutoClient<T>
where
    T: FromContext,
{
    pub fn new(
        handlers: HashMap<String, Box<dyn Callbacks<T>>>,
        tick_rate: Duration,
        initial_state: String,
    ) -> Self {
        Self {
            handlers,
            tick_rate,
            context: AutoClientContext {
                tick_rate: tick_rate.as_millis() as u32,
                current_state: initial_state.clone(),
                initial_state,
            },
        }
    }
    pub fn run(&mut self) {
        loop {
            let handler = self.handlers.get(&self.context.current_state).unwrap();
            let output = handler.call(&self.context);
            println!("{}", output);
            self.context.current_state = output;
            std::thread::sleep(self.tick_rate);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut client = AutoClientBuilder::new()
            .add_state(
                "test".to_string(),
                Box::new(|context: AutoClientContext| context.current_state.clone()),
            )
            .initial_state("test".to_string())
            .build();
        client.run();
    }
}
