pub mod builder;
pub mod context;
pub mod extractor;
pub mod mymacro;
use std::{collections::HashMap, time::Duration};

use context::StoredCallback;

use crate::context::AutoClientContext;

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
