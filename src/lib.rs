#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

//! An implementation of the [Jump Consistent Hash Algorithm](https://arxiv.org/pdf/1406.2294.pdf).
//!
//! # Example
//!
//! ```
//! extern crate jumphash;
//!
//! let jh = jumphash::JumpHasher::new();
//! let slot_count = 100;
//! let slot_for_key = jh.slot("key", slot_count);
//! ```

extern crate rand;
extern crate siphasher;

use rand::Rng;
use siphasher::sip::SipHasher13;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
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
    pub fn new_with_keys(k1: u64, k2: u64) -> JumpHasher {
        JumpHasher { hs: SipHasher13::new_with_keys(k1, k2) }
    }

    /// Returns a slot for the key `key`, out of `slot_count` available slots.
    pub fn slot<T: Hash>(&self, key: &T, slot_count: u32) -> u32 {
        assert!(slot_count > 0);
        let mut hs = self.hs.clone();
        key.hash(&mut hs);
        let mut h = hs.finish();
        let (mut b, mut j) = (-1i64, 0i64);
        while j < slot_count as i64 {
            b = j;
            h = h.wrapping_mul(2862933555777941757).wrapping_add(1);
            j = ((b.wrapping_add(1) as f64) *
                 (((1u64 << 31) as f64) / (((h >> 33) + 1) as f64))) as i64;
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
    let j = JumpHasher::new();
    let mut f = false;
    for _ in 0..100 {
        if j.slot(&"test1", 1000) != 8970050 {
            f = true;
            break;
        }
    }
    assert_eq!(f, true);
    let h0 = j.slot(&"test2", 1000);
    let mut f = false;
    for _ in 0..100 {
        if JumpHasher::new().slot(&"test2", 1000) != h0 {
            f = true;
            break;
        }
    }
    assert_eq!(f, true);
}
