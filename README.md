# logical-clocks-rs

A logical clock is a mechanism for capturing chronological and causal relationships in a distributed system. This `crate` provides an implementation of `Lamport Clock`, `Vector Clock`, which are a type of logical clock used to order events in a distributed system.


## Usage

### Add to your `Cargo.toml`

Add the following to your `Cargo.toml` to include the `logical-clocks-rs` crate:

```toml
[dependencies]
logical-clocks-rs = { git =  "https://github.com/TheDhejavu/logical-clocks-rs.git" }
```

## Lamport Clock

### Overview

Lamport timestamps are used to capture the order of events in a distributed system. They follow these simple rules:

1. A process increments its counter before each event in that process.
2. When a process sends a message, it includes its counter value with the message.
3. On receiving a message, the receiver process sets its counter to be greater than the maximum of its own value and the received value before it considers the message received.

*Source: [Lamport & Vector Clocks](https://miafish.wordpress.com/2015/03/11/lamport-vector-clocks/)*


### Examples

#### Using Default Identifier

```rust
use logical_clocks_rs::{LamportClock, LamportTime, Identifier};

fn main() {
    // Create a new Lamport clock with a default identifier
    let clock = LamportClock::new();

    // Increment the clock and print the time
    let time1 = clock.increment();
    println!("Time after increment: {:?}", time1);

    // Get the current time and print it
    let time2 = clock.time();
    println!("Current time: {:?}", time2);

    // Compare a higher Lamport time from another process
    clock.compare(LamportTime(10, Identifier::default()));
    let time3 = clock.time();
    println!("Current time after witness: {:?}", time3);
}
```

#### Using Custom Identifier

```rust
use logical_clocks_rs::{LamportClock, Identifier};
use uuid::Uuid;

fn main() {
    // Create a new Lamport clock with a custom identifier
    let custom_identifier = Uuid::new_v4().as_bytes().to_vec();
    let custom_clock = LamportClock::with_custom_identifier(custom_identifier);

    // Increment the custom clock and print the time
    let custom_time1 = custom_clock.increment();
    println!("Custom clock time after increment: {:?}", custom_time1);

    // Get the current time of the custom clock and print it
    let custom_time2 = custom_clock.time();
    println!("Custom clock current time: {:?}", custom_time2);
}
```

## Vector Clock

### Overview

A vector clock is a data structure used for determining the partial ordering of events in a distributed system and detecting causality violations. Just as in Lamport timestamps, inter-process messages contain the state of the sending process's logical clock. A vector clock of a system of N processes is an array/vector of N logical clocks, one clock per process; a local "largest possible values" copy of the global clock-array is kept in each process.

*Source: [Wikipedia Vector Clocks](https://en.wikipedia.org/wiki/Vector_clock)*


### Examples

```rust
use logical_clocks_rs::{Identifier, VClock, Vector};

extern crate logical_clocks_rs;

fn main() {
    // === 
    // Create identifiers for nodes/processes
    let id1 = Identifier::new();
    let id2 = Identifier::new();

    // Create vector clocks for two nodes
    let mut vclock1 = VClock::new();
    let mut vclock2 = VClock::new();

    // Increment the clocks
    vclock1.increment(&id1);
    vclock1.increment(&id1);
    vclock2.increment(&id2);

    println!("VClock 1: {:?}", vclock1.time());
    println!("VClock 2: {:?}", vclock2.time());

    // Merge the second clock into the first
    vclock1.merge(&vclock2);
    println!("After merging VClock 2 into VClock 1: {:?}", vclock1.time());

    // ====  WITH VECTOR === 
    // Create a Vector and initialize it
    let mut vector = Vector::new();
    vector.add(id1.clone()).add(id2.clone());

    //  Initialize VClock with the vector
    let mut vclock4 = VClock::with_vector(vector);
    vclock4.increment(&id1);
    vclock4.increment(&id2);
}

```

## API Documentation

### LamportClock

#### Methods

- `new() -> Self`: Creates a new Lamport clock with a default identifier.
- `with_new_identifier(id: Identifier) -> Self`: Creates a new Lamport clock with a specified identifier.
- `with_custom_identifier(bytes: Vec<u8>) -> Self`: Creates a new Lamport clock with a custom identifier.
- `time(&self) -> LamportTime`: Returns the current value of the Lamport clock.
- `increment(&self) -> LamportTime`: Increments the Lamport clock and returns the new value.
- `compare(&self, v: LamportTime)`: Updates the local clock if necessary after witnessing a clock value from another process.
- `to_bytes(&self) -> Vec<u8>`: Serializes the Lamport clock to bytes.
- `from_bytes(data: &[u8]) -> Option<Self>`: Deserializes the Lamport clock from bytes.

### VectorClock

#### Methods

- `new() -> Self`: Creates a new empty vector clock.
- `with_vector(vector: Vector) -> Self`: Creates a new vector clock with the given vector.
- `increment(&mut self, node_id: &Identifier)`: Increments the logical clock for the current node.
- `merge(&mut self, other: &VClock)`: Merges another vector clock into this one.
- `happened_before(&self, other: &VClock) -> bool`: Checks if this vector clock happened before another vector clock.
- `time(&self) -> VClockTime`: Returns the current vector clock time.

### Vector

#### Methods

- `new() -> Self`: Creates a new empty vector.
- `add(&mut self, id: Identifier) -> &mut Self`: Adds an identifier to the vector and initializes it to zero.