use std::collections::HashMap;

pub trait Callback<T> {
    fn call(&self, t: &T);
}
impl<T, U: Fn(&T)> Callback<T> for U {
    fn call(&self, t: &T) {
        self(t);
    }
}

pub struct AutoClient<T> {
    state: T,
    states: HashMap<String, Box<dyn Callback<T> + 'static>>,
}

impl<T> AutoClient<T> {}

#[derive(Default)]
pub struct AutoClientBuilder<T> {
    state: Option<T>,
    states: Option<HashMap<String, Box<dyn Callback<T> + 'static>>>,
}

impl<T> AutoClientBuilder<T> {
    pub fn new() -> Self {
        Self {
            state: None,
            states: None,
        }
    }

    pub fn with_state(mut self, state: T) -> Self {
        self.state = Some(state);
        self
    }

    pub fn register_state(mut self, name: String, state: impl Callback<T> + 'static) -> Self {
        let mut states = self.states.unwrap_or_default();
        states.insert(name, Box::new(state));
        self.states = Some(states);
        self
    }

    pub fn build(self) -> AutoClient<T> {
        AutoClient {
            state: self.state.expect("State not set"),
            states: self.states.expect("States not set"),
        }
    }
}

#[test]
fn builder_test() {
    let state = |s: &i32| {
        println!("State: {}", s);
    };
    let autoclientbuilder = AutoClientBuilder::new()
        .with_state(1)
        .register_state("Default".to_string(), state)
        .build();
}
