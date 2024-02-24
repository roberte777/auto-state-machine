pub mod context;
pub mod mymacro;
use std::{collections::HashMap, time::Duration};

use context::StoredCallback;
use mymacro::IntoCallback;

use crate::context::{AutoClientContext, Callback};

pub struct AutoClientBuilder {
    handlers: Option<HashMap<String, StoredCallback>>,
    tick_rate: Duration,
    initial_state: Option<String>,
}

impl Default for AutoClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoClientBuilder {
    pub fn new() -> Self {
        Self {
            initial_state: None,
            handlers: None,
            tick_rate: Duration::from_millis(50),
        }
    }

    pub fn add_state<I, C: Callback + 'static>(
        self,
        name: String,
        f: impl IntoCallback<I, Callback = C>,
    ) -> Self {
        let mut handlers = self.handlers.unwrap_or_default();
        handlers.insert(name, Box::new(f.into_callback()));
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
    pub fn build(self) -> AutoClient {
        let handlers = self.handlers.unwrap();
        let initial_state = self.initial_state.unwrap();
        AutoClient::new(handlers, self.tick_rate, initial_state)
    }
}
pub struct AutoClient {
    handlers: HashMap<String, StoredCallback>,
    tick_rate: Duration,
    context: AutoClientContext,
}
impl AutoClient {
    pub fn new(
        handlers: HashMap<String, StoredCallback>,
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
    fn test1(context: AutoClientContext) -> String {
        println!("test1");
        "test2".to_string()
    }
    fn test2(context: AutoClientContext, TickRate(r): TickRate) -> String {
        println!("TickRate: {:?}", r);
        "test".to_string()
    }
    use crate::context::TickRate;

    use super::*;
    #[test]
    fn test() {
        let mut client = AutoClientBuilder::new()
            .add_state("test".to_string(), test1)
            .add_state("test2".to_string(), test2)
            .initial_state("test".to_string())
            .build();
        client.run();
    }
}
