//! UUID version implementations for `TypeID`.
//!
//! This module provides structs representing different UUID versions
//! as specified in the UUID specification. Each version implements the
//! `UuidVersion` trait, allowing them to be used generically within the
//! `TypeID` system.

use std::ops::Deref;

use uuid::Uuid;

/// Trait for UUID versions used in `TypeID`.
///
/// This trait is implemented by all UUID version structs in this module,
/// allowing them to be used interchangeably where a UUID version is required.
pub trait UuidVersion: Deref<Target=Uuid> {}

/// Represents a Version 1 UUID (time-based).
///
/// Version 1 UUIDs are generated using a timestamp and node ID.
pub struct V1(Uuid);

impl UuidVersion for V1 {}

impl Default for V1 {
    /// Creates a new Version 1 UUID using the current timestamp.
    fn default() -> Self {
        Self(Uuid::now_v1(&Default::default()))
    }
}

impl Deref for V1 {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a Version 3 UUID (name-based, MD5 hash).
///
/// Version 3 UUIDs are generated by hashing a namespace and name using MD5.
pub struct V3(Uuid);

impl UuidVersion for V3 {}

impl Default for V3 {
    /// Creates a new Version 3 UUID using the DNS namespace and default name.
    fn default() -> Self {
        Self(Uuid::new_v3(&Uuid::NAMESPACE_DNS, Default::default()))
    }
}

impl Deref for V3 {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a Version 4 UUID (random).
///
/// Version 4 UUIDs are generated using random or pseudo-random numbers.
pub struct V4(Uuid);

impl UuidVersion for V4 {}

impl Default for V4 {
    /// Creates a new random Version 4 UUID.
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Deref for V4 {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a Version 5 UUID (name-based, SHA-1 hash).
///
/// Version 5 UUIDs are generated by hashing a namespace and name using SHA-1.
pub struct V5(Uuid);

impl UuidVersion for V5 {}

impl Deref for V5 {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for V5 {
    /// Creates a new Version 5 UUID using the DNS namespace and default name.
    fn default() -> Self {
        Self(Uuid::new_v5(&Uuid::NAMESPACE_DNS, Default::default()))
    }
}

/// Represents a Version 6 UUID (reordered time-based).
///
/// Version 6 UUIDs are similar to Version 1, but with improved privacy and monotonicity.
pub struct V6(Uuid);

impl UuidVersion for V6 {}

impl Deref for V6 {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for V6 {
    /// Creates a new Version 6 UUID using the current timestamp.
    fn default() -> Self {
        Self(Uuid::now_v6(&Default::default()))
    }
}

/// Represents a Version 7 UUID (time-ordered).
///
/// Version 7 UUIDs are time-ordered and use a Unix timestamp with millisecond precision.
pub struct V7(Uuid);

impl Deref for V7 {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UuidVersion for V7 {}

impl Default for V7 {
    /// Creates a new Version 7 UUID using the current timestamp.
    fn default() -> Self {
        Self(Uuid::now_v7())
    }
}

/// Represents a Nil UUID (all zeros).
///
/// A Nil UUID is a special case where all 128 bits are set to zero.
pub struct Nil(Uuid);

impl Deref for Nil {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UuidVersion for Nil {}

impl Default for Nil {
    /// Creates a new Nil UUID (all zeros).
    fn default() -> Self {
        Self(Uuid::nil())
    }
}