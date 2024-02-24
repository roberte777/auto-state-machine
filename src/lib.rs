use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub trait Callback<T, E>
where
    T: Send,
    E: Clone + Send,
{
    fn call(&self, t: &T) -> E;
}
impl<T, E, F> Callback<T, E> for F
where
    F: Fn(&T) -> E + 'static,
    T: Send,
    E: Clone + Send,
{
    fn call(&self, t: &T) -> E {
        self(t)
    }
}
pub enum Lifecycle {
    Stopped,
    Paused,
    Running,
}

#[derive(Clone)]
pub struct AutoClient<T, E> {
    context: T,
    states: Arc<HashMap<E, Box<dyn Callback<T, E> + Send + Sync>>>,
    initial_state: E,
    tick_rate: Duration,
    current_state: Arc<Mutex<E>>,
    lifecycle: Arc<Mutex<Lifecycle>>,
}
impl<T, E> AutoClient<T, E>
where
    T: Send + Clone + 'static,
    E: std::hash::Hash + Eq + Clone + Send + std::marker::Sync + 'static,
{
    pub fn new(
        context: T,
        states: HashMap<E, Box<dyn Callback<T, E> + Send + Sync>>,
        initial_state: E,
        tick_rate: Duration,
    ) -> Self {
        Self {
            context,
            states: Arc::new(states),
            initial_state: initial_state.clone(),
            tick_rate,
            current_state: Arc::new(Mutex::new(initial_state)),
            lifecycle: Arc::new(Mutex::new(Lifecycle::Stopped)),
        }
    }

    pub fn run(&mut self) {
        let states = self.states.clone();
        let current_state = self.current_state.clone();
        let context = self.context.clone();
        let tick_rate = self.tick_rate;
        {
            let mut lifecycle = self.lifecycle.lock().unwrap();
            *lifecycle = Lifecycle::Running;
        }
        let lifecycle = self.lifecycle.clone();
        thread::spawn(move || {
            // if lifecycle is paused, sleep, if lifecycle is running, run
            loop {
                let lifecycle_guard = lifecycle.lock().unwrap();
                match *lifecycle_guard {
                    Lifecycle::Paused => {
                        drop(lifecycle_guard);
                        std::thread::sleep(tick_rate);
                    }
                    Lifecycle::Running => {
                        drop(lifecycle_guard);
                        let mut curr = current_state.lock().unwrap();
                        let callback = states.get(&curr).unwrap();
                        let transition = callback.call(&context);
                        *curr = transition;
                        drop(curr);
                        std::thread::sleep(tick_rate);
                    }
                    Lifecycle::Stopped => {
                        drop(lifecycle_guard);
                        return;
                    }
                }
            }
        });
    }
    pub fn run_blocking(&mut self) {
        loop {
            match *self.lifecycle.lock().unwrap() {
                Lifecycle::Paused => {
                    std::thread::sleep(self.tick_rate);
                }
                Lifecycle::Running => {
                    let mut curr = self.current_state.lock().unwrap();
                    let callback = self.states.get(&curr).unwrap();
                    let transition = callback.call(&self.context);
                    *curr = transition;
                    std::thread::sleep(self.tick_rate);
                }
                Lifecycle::Stopped => {
                    return;
                }
            }
        }
    }
    pub fn pause(&mut self) {
        *self.lifecycle.lock().unwrap() = Lifecycle::Paused;
    }
    pub fn resume(&mut self) {
        *self.lifecycle.lock().unwrap() = Lifecycle::Running;
    }
    pub fn stop(&mut self) {
        *self.lifecycle.lock().unwrap() = Lifecycle::Stopped;
    }
    pub fn get_current_state(&self) -> E {
        self.current_state.lock().unwrap().clone()
    }
}

#[derive(Default)]
pub struct AutoClientBuilder<T, E> {
    context: Option<T>,
    states: Option<HashMap<E, Box<dyn Callback<T, E> + Send + Sync>>>,
    initial_state: Option<E>,
    tick_rate: Option<Duration>,
}

impl<T, E> AutoClientBuilder<T, E>
where
    T: Send + Clone + 'static,
    E: std::hash::Hash + Eq + Clone + Send + std::marker::Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            context: None,
            states: None,
            initial_state: None,
            tick_rate: None,
        }
    }

    pub fn with_context(mut self, state: T) -> Self {
        self.context = Some(state);
        self
    }

    pub fn register_state(
        mut self,
        transition: E,
        callback: impl Callback<T, E> + Send + Sync + 'static,
    ) -> Self {
        let mut states = self.states.take().unwrap_or_default();
        states.insert(transition, Box::new(callback));
        self.states = Some(states);
        self
    }

    pub fn with_initial_state(mut self, initial_state: E) -> Self {
        self.initial_state = Some(initial_state);
        self
    }

    pub fn with_tick_rate(mut self, tick_rate: Duration) -> Self {
        self.tick_rate = Some(tick_rate);
        self
    }

    pub fn build(self) -> AutoClient<T, E> {
        let context = self.context.expect("Context not set");
        let states = self.states.expect("States not set");
        let initial_state = self.initial_state.expect("Initial state not set");
        let tick_rate = self.tick_rate.unwrap_or(Duration::from_millis(50));
        AutoClient::new(context, states, initial_state, tick_rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Duration;

    struct TestCallback;

    impl Callback<i32, i32> for TestCallback {
        fn call(&self, _context: &i32) -> i32 {
            1
        }
    }

    #[test]
    fn test_with_initial_state() {
        let builder = AutoClientBuilder::<i32, i32>::new().with_initial_state(1);
        assert_eq!(builder.initial_state, Some(1));
    }

    #[test]
    fn test_with_tick_rate() {
        let builder =
            AutoClientBuilder::<i32, i32>::new().with_tick_rate(Duration::from_millis(50));
        assert_eq!(builder.tick_rate, Some(Duration::from_millis(50)));
    }

    #[test]
    fn test_build() {
        let mut states = HashMap::new();
        states.insert(
            1,
            Box::new(TestCallback) as Box<dyn Callback<i32, i32> + Send + Sync>,
        );
        let builder = AutoClientBuilder::<i32, i32>::new()
            .with_context(0)
            .register_state(1, TestCallback)
            .with_initial_state(1)
            .with_tick_rate(Duration::from_millis(50));
        let client = builder.build();
        assert_eq!(client.get_current_state(), 1);
        assert_eq!(client.tick_rate, Duration::from_millis(50));
    }
    #[test]
    fn test_register_state() {
        let builder = AutoClientBuilder::<i32, i32>::new().register_state(1, TestCallback);
        assert!(builder.states.unwrap().contains_key(&1));
    }

    #[test]
    fn test_new() {
        let builder = AutoClientBuilder::<i32, i32>::new();
        assert!(builder.states.is_none());
        assert!(builder.initial_state.is_none());
        assert!(builder.tick_rate.is_none());
    }

    #[test]
    #[should_panic(expected = "Initial state not set")]
    fn test_build_without_initial_state() {
        let builder = AutoClientBuilder::<i32, i32>::new()
            .with_context(0)
            .register_state(1, TestCallback)
            .build(); // Should panic because initial state is not set
    }

    #[test]
    #[should_panic(expected = "States not set")]
    fn test_build_without_states() {
        let builder = AutoClientBuilder::<i32, i32>::new()
            .with_context(0)
            .with_initial_state(1);
        builder.build(); // Should panic because states are not set
    }

    #[test]
    fn test_run_blocking() {
        let mut client = AutoClientBuilder::new()
            .with_context(0)
            .register_state(1, TestCallback)
            .with_initial_state(1)
            .with_tick_rate(Duration::from_millis(50))
            .build();
        let mut thread_client = client.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(100));
            thread_client.stop();
        });
        client.run_blocking();
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(client.get_current_state(), 1);
    }
    #[test]
    fn test_pause_resume() {
        let mut client = AutoClientBuilder::new()
            .with_context(0)
            .register_state(1, TestCallback)
            .with_initial_state(1)
            .with_tick_rate(Duration::from_millis(50))
            .build();
        let mut thread_client = client.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(100));
            thread_client.pause();
            std::thread::sleep(Duration::from_millis(100));
            thread_client.resume();
            std::thread::sleep(Duration::from_millis(100));
            thread_client.stop();
        });
        client.run_blocking();
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(client.get_current_state(), 1);
    }
    #[test]
    fn test_run() {
        fn move_to_2(context: &i32) -> i32 {
            2
        }
        fn move_to_1(context: &i32) -> i32 {
            1
        }
        let mut client = AutoClientBuilder::new()
            .with_context(0)
            .register_state(1, move_to_2)
            .register_state(2, move_to_1)
            .with_initial_state(1)
            .with_tick_rate(Duration::from_millis(50))
            .build();
        client.run();
        std::thread::sleep(Duration::from_millis(60));
        assert_eq!(client.get_current_state(), 1);
        std::thread::sleep(Duration::from_millis(55));
        assert_eq!(client.get_current_state(), 2);
        client.stop();
    }
}
