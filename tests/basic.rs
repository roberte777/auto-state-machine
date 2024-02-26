use autoclienttools::AutoClientContext;
use autoclienttools::{extractor::TickRate, AutoClientBuilder};

#[test]
fn test_basic() {
    let client = AutoClientBuilder::new(())
        .add_state(
            "test".to_string(),
            |_: autoclienttools::AutoClientContext| {
                println!("test1");
                "test2".to_string()
            },
        )
        .add_state(
            "test2".to_string(),
            |_: AutoClientContext, TickRate(r): TickRate| {
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
