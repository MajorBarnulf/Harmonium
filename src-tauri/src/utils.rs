use std::{hash::Hash, marker::PhantomData};

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug)]
pub struct Id<T: 'static>(u64, PhantomData<&'static T>);

impl<T> Id<T> {
    pub fn new(inner: u64) -> Self {
        Self(inner, PhantomData)
    }

    pub fn into_u64(self) -> u64 {
        self.0
    }
}

impl<T: 'static> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T: 'static> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: 'static> Eq for Id<T> {}

impl<T: 'static> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: 'static> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

pub struct IdVisitor<T: 'static> {
    _phantom: PhantomData<fn() -> T>,
}

impl<T> IdVisitor<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for IdVisitor<T> {
    type Value = Id<T>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer in the u64 range")
    }
}

impl<'de, T: 'static> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u64(IdVisitor::new())
    }
}

impl<T> From<u64> for Id<T> {
    fn from(input: u64) -> Self {
        Self::new(input)
    }
}
impl<T> From<Id<T>> for u64 {
    fn from(input: Id<T>) -> Self {
        let Id(inner, _) = input;
        inner
    }
}
