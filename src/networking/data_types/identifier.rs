use std::fmt;
use std::io::Read;
use std::str::FromStr;
use anyhow::anyhow;
use async_trait::async_trait;
use tokio::io::AsyncRead;
use crate::networking::data_types::{FieldRead, PacketRead, PacketWrite};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub namespace: String,
    pub value: String,
}

impl Identifier {
    pub fn new(namespace: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            value: value.into(),
        }
    }

    /// Creates a new identifier with the default "minecraft" namespace
    pub fn minecraft(value: impl Into<String>) -> Self {
        Self::new("minecraft", value)
    }

    /// Checks if a character is allowed in the namespace: [a-z0-9.-_]
    fn is_valid_namespace_char(c: char) -> bool {
        matches!(c, 'a'..='z' | '0'..='9' | '.' | '-' | '_')
    }

    /// Checks if a character is allowed in the value: [a-z0-9.-_/]
    fn is_valid_value_char(c: char) -> bool {
        matches!(c, 'a'..='z' | '0'..='9' | '.' | '-' | '_' | '/')
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.value)
    }
}

impl FromStr for Identifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (namespace, value) = match s.split_once(':') {
            Some((ns, val)) => (ns, val),
            None => ("minecraft", s),
        };

        if !namespace.chars().all(Self::is_valid_namespace_char) {
            return Err(anyhow!("Invalid character in identifier namespace: '{}'", namespace));
        }

        if !value.chars().all(Self::is_valid_value_char) {
            return Err(anyhow!("Invalid character in identifier value: '{}'", value));
        }

        Ok(Self {
            namespace: namespace.to_string(),
            value: value.to_string(),
        })
    }
}

// PACKET IMPLEMENTATION

// Writing: Convert to String ("ns:val"), then write that String
impl PacketWrite for Identifier {
    fn write_to(&self, buf: &mut Vec<u8>) {
        let s = self.to_string();
        s.write_to(buf);
    }
}

// Reading: Read as String, then Parse
#[async_trait]
impl PacketRead for Identifier {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        let s = <String as PacketRead>::read_from(stream).await?;
        Identifier::from_str(&s)
    }
}

impl FieldRead for Identifier {
    fn read_from<R: Read>(stream: &mut R) -> anyhow::Result<Self> {
        let s = <String as FieldRead>::read_from(stream)?;
        Identifier::from_str(&s)
    }
}

// HELPER TRAITS

// Allows: Identifier::from("stone") or Identifier::from("custom:item")
impl<T: AsRef<str>> From<T> for Identifier {
    fn from(s: T) -> Self {
        Identifier::from_str(s.as_ref()).unwrap_or_else(|_| {
            Identifier::minecraft("invalid_identifier")
        })
    }
}