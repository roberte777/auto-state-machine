pub mod builder;
pub mod context;
pub mod extractor;
pub mod mymacro;
use std::{collections::HashMap, time::Duration};

use context::StoredCallback;

use crate::context::AutoClientContext;

pub struct AutoClient<S>
where
    S: Clone,
{
    handlers: HashMap<String, StoredCallback<S>>,
    tick_rate: Duration,
    context: AutoClientContext,
    user_context: S,
}
impl<S> AutoClient<S>
where
    S: Clone,
{
    pub fn new(
        handlers: HashMap<String, StoredCallback<S>>,
        tick_rate: Duration,
        initial_state: String,
        user_context: S,
    ) -> Self {
        Self {
            handlers,
            tick_rate,
            context: AutoClientContext {
                tick_rate,
                current_state: initial_state.clone(),
                initial_state,
            },
            user_context,
        }
    }
    pub fn run_blocking(&mut self) {
        loop {
            let handler = self.handlers.get(&self.context.current_state).unwrap();
            let output = handler.call(&self.context, &self.user_context.clone());
            println!("{}", output);
            self.context.current_state = output;
            std::thread::sleep(self.tick_rate);
        }
    }
    pub fn run(&mut self) {
        todo!()
    }
}
