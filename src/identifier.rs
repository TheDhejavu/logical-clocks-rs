use base64::{engine::general_purpose, Engine};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Represents identifier
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(pub Vec<u8>);

impl Identifier {
    /// Creates a new Identifier with a random UUID
    pub fn new() -> Self {
        Identifier(Uuid::new_v4().as_bytes().to_vec())
    }

    /// Creates an Identifier from a byte vector
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Identifier(bytes)
    }

    /// Converts the Identifier to a string
    pub fn to_string(&self) -> String {
        if let Ok(uuid) = Uuid::from_slice(&self.0) {
            uuid.to_string()
        } else {
            let b64 = general_purpose::STANDARD.encode(&self.0);
            println!("{}", b64);
            b64 
        }
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Identifier::new()
    }
}
