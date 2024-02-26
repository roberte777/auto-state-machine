# AutoClient Crate Documentation
==============================

## Overview
--------

The AutoClient crate provides a flexible and intuitive way to create an automated client with customizable behavior. It is built around the concept of states and callbacks, allowing users to define specific actions and transitions depending on the client's current state. This approach makes it ideal for creating complex workflows, automations, or state-driven applications.

## Getting Started
---------------

To use AutoClient, include it in your `Cargo.toml` file:

tomlCopy code

`[dependencies] auto_client = "0.1.0"`

Make sure to replace `"0.1.0"` with the latest version of the crate.

## AutoClientBuilder
-----------------

The `AutoClientBuilder` struct is the entry point for creating an `AutoClient`. It allows you to configure the client by adding states, setting a tick rate, specifying an initial state, and providing a user context.

### Creating a New Builder

To start building your `AutoClient`, create a new instance of `AutoClientBuilder`:

rustCopy code

`use auto_client::AutoClientBuilder;  let builder = AutoClientBuilder::new(user_context);`

The `user_context` parameter is a user-defined data structure that will be passed to each callback. It can be used to maintain state or share information between callbacks.

### Adding States

You can add states to your client using the `add_state` method. Each state requires a name and a callback function that defines the state's behavior:

rustCopy code

`builder.add_state("state_name".to_string(), callback_function);`

The callback function can be any function that implements the `Callback` trait, allowing for flexible state behavior definition.

### Setting the Tick Rate

The tick rate determines how often the client's state is updated. You can set the tick rate using the `tick_rate` method:

rustCopy code

`builder.tick_rate(Duration::from_millis(100));`

### Specifying the Initial State

Before building your client, you must specify the initial state using the `initial_state` method:

rustCopy code

`builder.initial_state("initial_state_name".to_string());`

### Building the AutoClient

Once all configurations are set, you can build your `AutoClient`:

rustCopy code

`let client = builder.build();`

This method finalizes the builder and returns an instance of `AutoClient`.

### Example
-------

Here's a complete example that demonstrates how to create an `AutoClient` with two states:

rustCopy code

`use auto_client::{AutoClientBuilder, AutoClientContext, TickRate}; use std::time::Duration;  fn main() {     fn test1(_: AutoClientContext) -> String {         println!("State: test1");         "test2".to_string()     }      fn test2(_: AutoClientContext, TickRate(r): TickRate) -> String {         println!("State: test2, TickRate: {:?}", r);         "test1".to_string()     }      let client = AutoClientBuilder::new(())         .add_state("test1".to_string(), test1)         .add_state("test2".to_string(), test2)         .initial_state("test1".to_string())         .tick_rate(Duration::from_millis(100))         .build();      // Use the client... }`

This example creates an `AutoClient` with two states (`test1` and `test2`) and a tick rate of 100 milliseconds. The client starts in the `test1` state and prints a message before transitioning to the `test2` state, which also prints a message and transitions back to `test1`.

### Advanced Usage
--------------

For advanced scenarios, such as modifying the user context within your handlers, consider using thread-safe wrappers like `Arc<Mutex<T>>` or `Arc<RwLock<T>>`. This approach allows for safe concurrent access and modification of the shared context from multiple callbacks.

