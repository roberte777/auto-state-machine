use std::{collections::HashMap, time::Duration};

use crate::mymacro::IntoCallback;
use crate::AutoClient;

use crate::context::{Callback, StoredCallback};
pub struct AutoClientBuilder<S = ()> {
    handlers: Option<HashMap<String, StoredCallback<S>>>,
    tick_rate: Duration,
    initial_state: Option<String>,
    user_context: S,
}

impl AutoClientBuilder<()> {
    pub fn new() -> Self {
        Self {
            handlers: None,
            tick_rate: Duration::from_millis(50),
            initial_state: None,
            user_context: (),
        }
    }
}

impl<S> AutoClientBuilder<S>
where
    S: Clone,
{
    pub fn add_state<I, C: Callback<S> + 'static>(
        self,
        name: String,
        f: impl IntoCallback<I, S, Callback = C>,
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
    pub fn build(self) -> AutoClient<S> {
        let handlers = self.handlers.unwrap();
        let initial_state = self.initial_state.unwrap();
        AutoClient::new(handlers, self.tick_rate, initial_state, self.user_context)
    }
}
#[cfg(test)]
mod tests {
    fn test1(_: AutoClientContext, _: ()) -> String {
        println!("test1");
        "test2".to_string()
    }
    fn test2(_: AutoClientContext, TickRate(r): TickRate, _: ()) -> String {
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
