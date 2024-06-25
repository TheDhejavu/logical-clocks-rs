pub use crate::lamport_clock::{LamportClock, Identifier, LamportTime};
pub use crate::vclock::VClock;

mod lamport_clock;
mod vclock;