use logical_clocks_rs::{Identifier, VClock, Vector};

extern crate logical_clocks_rs;

fn main() {
    // === 
    // Create identifiers for nodes
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
