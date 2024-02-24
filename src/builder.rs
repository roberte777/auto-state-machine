use std::{collections::HashMap, time::Duration};

use crate::mymacro::IntoCallback;
use crate::AutoClient;

use crate::context::{Callback, StoredCallback};
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
    use crate::{context::AutoClientContext, extractor::TickRate};

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
