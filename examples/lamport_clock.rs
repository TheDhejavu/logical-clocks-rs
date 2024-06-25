extern crate logical_clocks_rs;

use logical_clocks_rs::{LamportClock, LamportTime, Identifier};
use uuid::Uuid;

fn main() {
    // === Using Default Identifier === 
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

    // === Using Custom Identifier === 
    let custom_identifier = Uuid::new_v4().as_bytes().to_vec();
    let custom_clock = LamportClock::with_custom_identifier(custom_identifier);

    // Increment the custom clock and print the time
    let custom_time1 = custom_clock.increment();
    println!("Custom clock time after increment: {:?}", custom_time1);

    // Get the current time of the custom clock and print it
    let custom_time2 = custom_clock.time();
    println!("Custom clock current time: {:?}", custom_time2);
}
