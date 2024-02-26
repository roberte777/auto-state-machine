# AutoClient Crate Documentation

## Overview

The AutoClient crate provides a flexible and intuitive way to create an 
automated actor with customizable behavior. It is built around the concept of 
states and callbacks, allowing users to define specific actions and transitions 
depending on the client's current state. This approach makes it ideal for 
creating complex workflows, automations, or state-driven applications.

## Getting Started

To use AutoClient, include it in your `Cargo.toml` file:

tomlCopy code

`[dependencies] auto_client = "0.1.0"`

Make sure to replace `"0.1.0"` with the latest version of the crate.

## AutoClientBuilder

The `AutoClientBuilder` struct is the entry point for creating an `AutoClient`.
It allows you to configure the client by adding states, setting a tick rate,
specifying an initial state, and providing a user context.

### Creating a New Builder

To start building your `AutoClient`, create a new instance of 
`AutoClientBuilder`:

```rust
use auto_client::AutoClientBuilder;
let builder = AutoClientBuilder::new(user_context);
```

The `user_context` parameter is a user-defined data structure that will be 
passed to each callback. It can be used to maintain state or share information 
between callbacks.

### Adding States

You can add states to your client using the `add_state` method. Each state 
requires a name and a callback function that defines the state's behavior:


```rust
builder.add_state("state_name".to_string(), callback_function);
```

The callback function can be any function that implements the `Callback`
trait, allowing for flexible state behavior definition.

### Setting the Tick Rate

The tick rate determines how often the client's state is updated. You can set 
the tick rate using the `tick_rate` method:

```rust
builder.tick_rate(Duration::from_millis(100));
```

### Specifying the Initial State

Before building your client, you must specify the initial state using the 
`initial_state` method:

```rust
builder.initial_state("initial_state_name".to_string());
```

### Building the AutoClient

Once all configurations are set, you can build your `AutoClient`:

```rust
let client = builder.build();
```

This method finalizes the builder and returns an instance of `AutoClient`.

### Example

Here's a complete example that demonstrates how to create an `AutoClient`
with two states:

```rust
use auto_client::{AutoClientBuilder, AutoClientContext, TickRate}; 
use std::time::Duration;  
fn main() {     
    fn test1(_: AutoClientContext) -> String {         
        println!("State: test1");         
        "test2".to_string()     
    }      
    fn test2(_: AutoClientContext, TickRate(r): TickRate) -> String {         
        println!("State: test2, TickRate: {:?}", r);
        "test1".to_string()     
    }      
    let client = AutoClientBuilder::new(())         
        .add_state("test1".to_string(), test1)         
        .add_state("test2".to_string(), test2)         
        .initial_state("test1".to_string())         
        .tick_rate(Duration::from_millis(100))         
        .build();      
    // Use the client... 
}
```

This example creates an `AutoClient` with two states (`test1` and `test2`) 
and a tick rate of 100 milliseconds. The client starts in the `test1` state 
and prints a message before transitioning to the `test2` state, which also 
prints a message and transitions back to `test1`.

### Advanced Usage

For advanced scenarios, such as modifying the user context within your handlers,
consider using thread-safe wrappers like `Arc<Mutex<T>>` or `Arc<RwLock<T>>`. 
This approach allows for safe concurrent access and modification of the shared 
context from multiple callbacks.

## AutoClient
The AutoClient struct is the core of your automated client, managing states, 
transitions, and the execution cycle based on predefined states and associated 
callbacks.

### Example Usage

To use `AutoClient`, you typically start by defining state callbacks, 
configuring an `AutoClientBuilder` instance, and then building your 
`AutoClient`. Once built, you can control the client's execution flow with 
its methods.

#### Creating and Running an AutoClient

```rust
use auto_client::{AutoClientBuilder, AutoClientContext};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut client = AutoClientBuilder::new(Arc::new(Mutex::new(MyContext::new())))
        .add_state("state1".to_string(), state1_handler)
        .add_state("state2".to_string(), state2_handler)
        .initial_state("state1".to_string())
        .tick_rate(Duration::from_millis(100))
        .build();

    client.run();

    // The client will now automatically transition between states based on the logic
    // defined in `state1_handler` and `state2_handler`.
}
```

#### Controlling Execution

```rust
// Assuming `client` has been initialized and started as above

// Pause execution
client.pause();

// Resume execution
client.resume();

// Stop execution
client.stop();
```

### Implementation Notes

*   The `run` method spawns a new thread where the client's execution cycle is 
managed. This allows the main program to continue running independently of the 
client's state transitions.

*   Pausing or stopping the client affects its internal lifecycle status, 
which is checked at each tick interval to determine whether to proceed with 
state transitions or to perform lifecycle management actions like halting 
execution.

