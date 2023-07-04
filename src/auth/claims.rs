use std::collections::HashMap;

/// A collection of claims about a User
#[derive(Debug, Clone, Default)]
pub struct Claims {
    /// A map containing the claims
    claims: HashMap<String, ClaimValue>,
}

impl Claims {
    /// Create a new Claims object
    pub fn new() -> Self {
        Self {
            claims: HashMap::new(),
        }
    }

    pub(crate) fn from_inner(claims: HashMap<String, ClaimValue>) -> Self {
        Self { claims }
    }

    pub(crate) fn into_inner(self) -> HashMap<String, ClaimValue> {
        self.claims
    }

    /// Add a claim to the Claims object
    pub fn add_claim(&mut self, name: &str, value: ClaimValue) {
        self.claims.insert(name.to_string(), value);
    }

    /// Remove a claim from the Claims object
    pub fn remove_claim(&mut self, name: &str) {
        self.claims.remove(name);
    }

    /// Get a claim from the Claims object
    pub fn get_claim(&self, name: &str) -> Option<&ClaimValue> {
        self.claims.get(name)
    }

    /// Checks if the Claims object contains a claim
    pub fn has_claim(&self, name: &str) -> bool {
        self.claims.contains_key(name)
    }
}

/// A representation of valid claim values.
#[derive(Debug, Clone)]
pub enum ClaimValue {
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Array(Vec<ClaimValue>),
    Object(HashMap<String, ClaimValue>),
}

impl ClaimValue {
    /// Return the value as a str if it is a String, None otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Return the value as a String if it is a String, None otherwise.
    pub fn as_string(self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

impl TryFrom<ClaimValue> for String {
    type Error = ClaimValueError;

    fn try_from(value: ClaimValue) -> Result<Self, Self::Error> {
        match value {
            ClaimValue::String(s) => Ok(s),
            ClaimValue::Bool(b) => Ok(b.to_string()),
            ClaimValue::Int(i) => Ok(i.to_string()),
            ClaimValue::Float(f) => Ok(f.to_string()),

            _ => Err(ClaimValueError::NotStringConvertible),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClaimValueError {
    #[error("ClaimValue cannot be converted to String")]
    NotStringConvertible,
}
