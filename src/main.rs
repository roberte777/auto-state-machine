use std::time::Duration;

use autoclienttools::AutoClientBuilder;

fn move_to_2(context: &i32) -> i32 {
    println!("move_to_2");
    2
}
fn move_to_1(context: &i32) -> i32 {
    println!("move_to_1");
    1
}
fn main() {
    let mut client = AutoClientBuilder::new()
        .with_context(0)
        .register_state(1, move_to_2)
        .register_state(2, move_to_1)
        .with_initial_state(1)
        .with_tick_rate(Duration::from_millis(50))
        .build();
    client.run();
    std::thread::sleep(Duration::from_millis(55));
    assert_eq!(client.get_current_state(), 1);
    println!("passed 1");
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(client.get_current_state(), 2);
    println!("passed 2");
    client.stop();
}
