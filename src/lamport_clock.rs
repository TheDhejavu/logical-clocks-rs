use std::sync::atomic::{AtomicU64, Ordering};
use std::cmp::Ordering as CmpOrdering;
use std::convert::TryInto;
use serde::{Serialize, Deserialize};
use crate::Identifier;

/// Represents a Lamport time value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LamportTime(pub u64, pub Identifier);

impl PartialOrd for LamportTime {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

impl Ord for LamportTime {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        self.0.cmp(&other.0).then_with(|| self.1.cmp(&other.1))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LamportClock {
    counter: AtomicU64,
    id: Identifier,
}

impl LamportClock {
    /// Creates a new Lamport clock with the counter initialized to 1.
    pub fn new() -> Self {
        LamportClock {
            counter: AtomicU64::new(1),
            id: Identifier::default(),
        }
    }

    /// Creates a new Lamport clock with a specified identifier.
    pub fn with_new_identifier(id: Identifier) -> Self {
        LamportClock {
            counter: AtomicU64::new(1),
            id,
        }
    }

    /// Creates a new Lamport clock with a custom identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::LamportClock;
    /// 
    /// let custom_id = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    /// let custom_clock = LamportClock::with_custom_identifier(custom_id);
    /// println!("Custom clock: {:?}", custom_clock);
    /// ```
    pub fn with_custom_identifier(bytes: Vec<u8>) -> Self {
        LamportClock {
            counter: AtomicU64::new(1),
            id: Identifier::from_bytes(bytes),
        }
    }

    /// Returns the current value of the Lamport clock.
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::LamportClock;
    /// 
    /// let clock = LamportClock::new();
    /// let current_time = clock.time();
    /// println!("Current Lamport time: {:?}", current_time);
    /// ```
    pub fn time(&self) -> LamportTime {
        LamportTime(self.counter.load(Ordering::SeqCst), self.id.clone())
    }

     /// The `increment` method increments the Lamport clock and returns the new value.
    /// This method is typically used to record an event in the process and update the clock value.
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::LamportClock;
    /// 
    /// let clock = LamportClock::new();
    /// let new_time = clock.increment();
    /// println!("New Lamport time: {:?}", new_time);
    /// ```
    ///
    pub fn increment(&self) -> LamportTime {
       // Atomically increment the counter by 1 and get the old value
       let old_value = self.counter.fetch_add(1, Ordering::SeqCst);
       LamportTime(old_value + 1, self.id.clone())
    }

    /// The `compare` method updates the local clock if necessary after witnessing a clock value
    /// from another process. This ensures the Lamport clock maintains a consistent logical order
    /// of events in a distributed system.
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::{LamportClock, Identifier, LamportTime};
    /// 
    /// let clock = LamportClock::new();
    /// let other_time = LamportTime(10, Identifier::default());
    /// clock.compare(other_time);
    /// println!("Updated Lamport time after witnessing: {:?}", clock.time());
    /// ```
    ///
    pub fn compare(&self, other_time: LamportTime) {
        loop {
            let current_time = LamportTime(self.counter.load(Ordering::SeqCst), self.id.clone());
            if other_time <= current_time {
                return;
            }

            match self.counter.compare_exchange(current_time.0, other_time.0 + 1, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
    }

    /// Serializes the Lamport clock to bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.counter.load(Ordering::SeqCst).to_be_bytes());
        bytes.extend_from_slice(&self.id.0);
        bytes
    }

    /// Deserializes the Lamport clock from bytes.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 8 {
            return None;
        }
        let count = u64::from_be_bytes(data[0..8].try_into().ok()?);
        let id = data[8..].to_vec();
        Some(LamportClock {
            counter: AtomicU64::new(count),
            id: Identifier(id),
        })
    }

}

impl Clone for LamportClock {
    fn clone(&self) -> Self {
        LamportClock {
            counter: AtomicU64::new(self.counter.load(Ordering::SeqCst)),
            id: self.id.clone(),
        }
    }
}


#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::Identifier;

    use super::*;

    #[test]
    fn test_increment() {
        let clock = LamportClock::new();

        // Increment the clock and check the time
        let time1 = clock.increment();
        assert!(time1 == LamportTime(2, clock.id.clone()));

        // Increment again and check the time
        let time2 = clock.increment();
        assert!(time2 == LamportTime(3, clock.id.clone()));
    }

    #[test]
    fn test_time() {
        let clock = LamportClock::new();

        // Increment the clock 3x
        clock.increment();
        clock.increment();
        clock.increment();

        // Check the current time
        let current_time = clock.time();
        // we also try to ensure that equality sign works because of PartialOrd trait
        assert!(current_time == LamportTime(4, clock.id.clone()));
    }

    #[test]
    fn test_compare() {
        let clock = LamportClock::new();

        // Initial time
        let initial_time = clock.time();
        assert_eq!(initial_time, LamportTime(1, clock.id.clone()));

        // Simulate receiving a message with a higher timestamp
        let received_time = LamportTime(10, Identifier::default());
        clock.compare(received_time);

        // The clock should now be incremented to 11
        let updated_time = clock.time();
        assert_eq!(updated_time, LamportTime(11, clock.id.clone()));
    }

    #[test]
    fn test_with_custom_identifier() {
        let custom_id =  Uuid::new_v4().as_bytes().to_vec();
        let custom_clock = LamportClock::with_custom_identifier(custom_id.clone());

        // Check the custom identifier
        assert_eq!(custom_clock.id, Identifier::from_bytes(custom_id.clone()));
    }
}