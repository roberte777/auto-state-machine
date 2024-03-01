use autostatemachine::StateMachineContext;
use autostatemachine::{extractor::TickRate, StateMachineBuilder};

#[test]
fn test_basic() {
    let client = StateMachineBuilder::new(())
        .add_state(
            "test".to_string(),
            |_: autostatemachine::StateMachineContext| {
                println!("test1");
                "test2".to_string()
            },
        )
        .add_state(
            "test2".to_string(),
            |_: StateMachineContext, TickRate(r): TickRate| {
                println!("TickRate: {:?}", r);
                "test".to_string()
            },
        )
        .initial_state("test".to_string())
        .build();
    assert_eq!(client.get_context().current_state, "test");
    assert_eq!(
        client.get_tick_rate(),
        &std::time::Duration::from_millis(50)
    );
    assert_eq!(client.get_user_context(), &());
}
