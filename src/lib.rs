//! An implementation of the [Jump Consistent Hash Algorithm](https://arxiv.org/pdf/1406.2294.pdf).
//!
//! # Example
//!
//! ```
//! extern crate jumphash;
//!
//! let jh = jumphash::JumpHasher::new();
//! let slot_count = 100;
//! let slot_for_key = jh.slot(&"key", slot_count);
//! ```

use rand::RngCore;
use siphasher::sip::SipHasher13;
use std::hash::{Hash, Hasher};

/// A default jump hash instance with the default, recommended hash function.
#[derive(Clone, Copy, Debug)]
pub struct JumpHasher {
    hs: SipHasher13,
}

impl Default for JumpHasher {
    /// Returns a non-deterministic `JumpHasher` structure.
    fn default() -> JumpHasher {
        let mut rng = rand::thread_rng();
        Self::new_with_keys(rng.next_u64(), rng.next_u64())
    }
}

impl JumpHasher {
    /// Returns a non-deterministic `JumpHasher` structure.
    pub fn new() -> JumpHasher {
        JumpHasher::default()
    }

    /// Returns a deterministic `JumpHasher` structure, seeded with two 64-bit keys.
    #[inline]
    pub fn new_with_keys(k1: u64, k2: u64) -> JumpHasher {
        JumpHasher {
            hs: SipHasher13::new_with_keys(k1, k2),
        }
    }

    /// Returns a slot for the key `key`, out of `slot_count` available slots.
    pub fn slot<T: Hash + ?Sized>(&self, key: &T, slot_count: u32) -> u32 {
        debug_assert!(slot_count > 0);
        let mut hs = self.hs;
        key.hash(&mut hs);
        let mut h = hs.finish();
        let (mut b, mut j) = (-1i64, 0i64);
        while j < slot_count as i64 {
            b = j;
            h = h.wrapping_mul(2862933555777941757).wrapping_add(1);
            j = ((b.wrapping_add(1) as f64) * (((1u64 << 31) as f64) / (((h >> 33) + 1) as f64)))
                as i64;
        }
        b as u32
    }
}

/// A jump hash instance with a custom hash function.
#[derive(Clone, Copy, Debug)]
pub struct CustomJumpHasher<H: Hasher + Clone> {
    hs: H,
}

impl<H: Hasher + Clone> CustomJumpHasher<H> {
    /// Initializes jump hash instance with a custom hash function.
    /// If the function is going to be used with untrusted inputs, it is recommended to use
    /// a non-deterministic, cryptographic hash function.
    pub fn new(hasher: H) -> CustomJumpHasher<H> {
        CustomJumpHasher { hs: hasher }
    }

    /// Returns a slot for the key `key`, out of `slot_count` available slots.
    pub fn slot<T: Hash + ?Sized>(&self, key: &T, slot_count: u32) -> u32 {
        debug_assert!(slot_count > 0);
        let mut hs = self.hs.clone();
        key.hash(&mut hs);
        let mut h = hs.finish();
        let (mut b, mut j) = (-1i64, 0i64);
        while j < slot_count as i64 {
            b = j;
            h = h.wrapping_mul(2862933555777941757).wrapping_add(1);
            j = ((b.wrapping_add(1) as f64) * (((1u64 << 31) as f64) / (((h >> 33) + 1) as f64)))
                as i64;
        }
        b as u32
    }
}

#[test]
fn test_basic() {
    let j = JumpHasher::new_with_keys(0, 0);
    assert_eq!(j.slot(&"test1", 10000000), 8970050);
    assert_eq!(j.slot(&"test2", 1000), 10);
    assert_eq!(j.slot(&"test3", 1000), 76);
    assert_eq!(j.slot(&"test4", 1000), 161);
    assert_eq!(j.slot(&"test5", 50), 33);
    assert_eq!(j.slot(&"", 1000), 392);
    assert_eq!(j.slot(&"testz", 1), 0);

    let key = vec![1u8, 2u8];
    assert_eq!(j.slot(&[1u8, 2u8], 1000), 785);
    assert_eq!(j.slot(&key, 1000), 785);
    assert_eq!(j.slot(key.as_slice(), 1000), 785);

    let j = JumpHasher::new();
    assert_ne!(j.slot(&"test1", 1000), 8970050);
    let h0 = j.slot(&"test2", 1000);
    assert_ne!(JumpHasher::new().slot(&"test2", 1000), h0);
}

#[test]
fn test_custom_hash() {
    let j = CustomJumpHasher::new(SipHasher13::new_with_keys(0, 0));
    assert_eq!(j.slot(&"test1", 10000000), 8970050);
    assert_eq!(j.slot(&"test2", 1000), 10);
    assert_eq!(j.slot(&"test3", 1000), 76);
    assert_eq!(j.slot(&"test4", 1000), 161);
    assert_eq!(j.slot(&"test5", 50), 33);
    assert_eq!(j.slot(&"", 1000), 392);
    assert_eq!(j.slot(&"testz", 1), 0);
    let mut rng = rand::thread_rng();
    let j = CustomJumpHasher::new(SipHasher13::new_with_keys(rng.next_u64(), rng.next_u64()));
    assert_ne!(j.slot(&"test1", 1000), 8970050);
    let h0 = j.slot(&"test2", 1000);
    assert_ne!(
        CustomJumpHasher::new(SipHasher13::new()).slot(&"test2", 1000),
        h0
    );
}
