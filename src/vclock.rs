use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use bincode::{self, Error as BincodeError};
use std::cmp::Ordering;

use crate::Identifier;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Vector {
    data: HashMap<Identifier, u64>,
}

impl Vector {
    /// Creates a new empty vector
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::Vector;
    /// 
    /// let vector = Vector::new();
    /// ```
    pub fn new() -> Self {
        Vector {
            data: HashMap::new(),
        }
    }

    /// Adds an identifier to the vector and initializes it to zero
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::{Identifier, Vector};
    /// 
    /// let mut vector = Vector::new();
    /// let id = Identifier::new();
    /// vector.add(id);
    /// ```
    pub fn add(&mut self, id: Identifier) -> &mut Self {
        self.data.insert(id, 0);
        self
    }

    /// Converts the vector to a `HashMap`
    fn to_hashmap(self) -> HashMap<Identifier, u64> {
        self.data
    }
}

/// Represents a vector clock
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct VClock {
    vector: HashMap<Identifier, u64>,
}

impl VClock {
    /// Creates a new empty vector clock
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::VClock;
    /// 
    /// let vclock = VClock::new();
    /// ```
    pub fn new() -> Self {
        VClock {
            vector: HashMap::new(),
        }
    }

    /// Creates a new vector clock with the given vector
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::{VClock, Vector};
    /// 
    /// let vector = Vector::new();
    /// let vclock = VClock::with_vector(vector);
    /// ```
    pub fn with_vector(vector: Vector) -> Self {
        VClock { vector: vector.to_hashmap() }
    }

    /// Increments the logical clock for the current node
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::{VClock, Identifier};
    /// 
    /// let mut vclock = VClock::new();
    /// let id = Identifier::new();
    /// vclock.increment(&id);
    /// ```
    pub fn increment(&mut self, node_id: &Identifier) {
        let entry = self.vector.entry(node_id.clone()).or_insert(0);
        *entry += 1;
    }

    /// Merges another vector clock into this one
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::VClock;
    /// 
    /// let mut vclock1 = VClock::new();
    /// let mut vclock2 = VClock::new();
    /// vclock1.merge(&vclock2);
    /// ```
    pub fn merge(&mut self, other: &VClock) {
        for (node, &counter) in &other.vector {
            let entry = self.vector.entry(node.clone()).or_insert(0);
            *entry = (*entry).max(counter);
        }
    }

    /// Checks if this vector clock happened before another vector clock
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::VClock;
    /// 
    /// let vclock1 = VClock::new();
    /// let vclock2 = VClock::new();
    /// let result = vclock1.happened_before(&vclock2);
    /// ```
    pub fn happened_before(&self, other: &VClock) -> bool {
        let mut happened_before = false;

        for (node, &self_counter) in &self.vector {
            let other_counter = other.vector.get(node).unwrap_or(&0);
            if self_counter > *other_counter {
                return false;
            }
            if self_counter < *other_counter {
                happened_before = true;
            }
        }

        for (node, &other_counter) in &other.vector {
            let self_counter = self.vector.get(node).unwrap_or(&0);
            if other_counter > *self_counter {
                happened_before = true;
            }
        }

        happened_before
    }

    /// Returns the current vector clock time
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::VClock;
    /// 
    /// let vclock = VClock::new();
    /// let time = vclock.time();
    /// ```
    pub fn time(&self) -> VClockTime {
        VClockTime(self.vector.clone())
    }
}

impl PartialOrd for VClockTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut is_less = false;
        let mut is_greater = false;

        let keys: HashSet<_> = self.0.keys().chain(other.0.keys()).collect();

        for key in keys {
            let self_counter = self.0.get(key).unwrap_or(&0);
            let other_counter = other.0.get(key).unwrap_or(&0);

            if self_counter < other_counter {
                is_less = true;
            }
            if self_counter > other_counter {
                is_greater = true;
            }

            if is_less && is_greater {
                return None;
            }
        }

        match (is_less, is_greater) {
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct VClockTime(pub HashMap<Identifier, u64>);

impl VClockTime {
    /// Serializes the vector clock time to bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::VClock;
    /// 
    /// let mut vclock = VClock::new();
    /// let bytes = vclock.time().to_bytes().unwrap();
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>, BincodeError> {
        bincode::serialize(&self.0)
    }

    /// Deserializes the vector clock time from bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use logical_clocks_rs::{VClock, VClockTime};
    /// 
    /// let mut vclock = VClock::new();
    /// let bytes = vclock.time().to_bytes().unwrap();
    /// let time = VClockTime::from_bytes(&bytes);
    /// ```
    pub fn from_bytes(data: &[u8]) -> Result<Self, BincodeError> {
        let clock: HashMap<Identifier, u64> = bincode::deserialize(data)?;
        Ok(VClockTime(clock))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vclock_increment() {
        let id = Identifier::new();
        let mut vclock = VClock::new();

        vclock.increment(&id);
        let current_time = vclock.time();
        assert_eq!(*current_time.0.get(&id).unwrap(), 1);
    }

    #[test]
    fn test_vclock_merge() {
        let mut vclock1 = VClock::new();
        let mut vclock2 = VClock::new();

        let id1 = Identifier::new();
        let id2 = Identifier::new();
       
        vclock1.increment(&id1);
        vclock2.increment(&id2);
        vclock2.increment(&id2);

        vclock1.merge(&vclock2);
        let current_time = vclock1.time();

        assert_eq!(*current_time.0.get(&id1).unwrap(), 1);
        assert_eq!(*current_time.0.get(&id2).unwrap(), 2);
    }

    #[test]
    fn test_vclock_serialization() {
        let id = Identifier::new();
        let mut vclock = VClock::new();

        vclock.increment(&id);
        let time = vclock.time();

        let serialized = time.to_bytes();
        assert!(serialized.is_ok());

        let deserialized = VClockTime::from_bytes(&serialized.unwrap());
        
        assert!(deserialized.is_ok());
        assert_eq!(time, deserialized.unwrap());
    }

    #[test]
    fn test_vclock_equality() {
        let mut vclock1 = VClock::new();
        let mut vclock2 = VClock::new();

        let id1 = Identifier::new();
        let id2 = Identifier::new();

        vclock1.increment(&id1);
        vclock1.increment(&id1);
        vclock1.increment(&id1);
        vclock2.increment(&id2);

        vclock1.merge(&vclock2);
        vclock2.merge(&vclock1);

        assert_eq!(vclock1.time(), vclock2.time());
    }

    #[test]
    fn test_vclock_happened_before() {
        let mut vclock1 = VClock::new();
        let mut vclock2 = VClock::new();

        let id1 = Identifier::new();
        let id2 = Identifier::new();

        vclock1.increment(&id1);
        vclock1.increment(&id2);

        vclock2.increment(&id1);
        vclock2.increment(&id2);
        vclock2.increment(&id2);

        assert!(vclock1.happened_before(&vclock2));
        assert!(!vclock2.happened_before(&vclock1));
    }

    #[test]
    fn test_vclock_with_vector() {
        let mut vector = Vector::new();
        let id1 = Identifier::new();
        let id2 = Identifier::new();

        vector.add(id1.clone()).add(id2.clone());

        let vclock = VClock::with_vector(vector);
        let current_time = vclock.time();

        assert_eq!(*current_time.0.get(&id1).unwrap(), 0);
        assert_eq!(*current_time.0.get(&id2).unwrap(), 0);
    }
}
