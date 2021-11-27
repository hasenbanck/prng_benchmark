use std::cell::Cell;

use crate::{
    Lehmer64, Mwc256XXA64, Pcg64, Pcg64Fast, Random, RomuJr, RomuTrio, Splitmix64, Tylo64, Wyrand,
    Xoshiro256plusplus,
};

#[thread_local]
pub(crate) static TYLO64: Tylo64 = Tylo64 {
    a: Cell::new(3),
    b: Cell::new(3),
    w: Cell::new(3),
    k: Cell::new(3),
};

#[inline(always)]
pub fn tylo64_seed() {
    TYLO64.seed()
}

#[inline(always)]
pub fn tylo64_u64() -> u64 {
    TYLO64.u64()
}

#[thread_local]
pub(crate) static ROMUJR: RomuJr = RomuJr {
    x: Cell::new(3),
    y: Cell::new(3),
};

#[inline(always)]
pub fn romu_jr_seed() {
    ROMUJR.seed()
}

#[inline(always)]
pub fn romu_jr_u64() -> u64 {
    ROMUJR.u64()
}

#[thread_local]
pub(crate) static ROMUTRIO: RomuTrio = RomuTrio {
    x: Cell::new(3),
    y: Cell::new(3),
    z: Cell::new(3),
};

#[inline(always)]
pub fn romu_trio_seed() {
    ROMUTRIO.seed()
}

#[inline(always)]
pub fn romu_trio_u64() -> u64 {
    ROMUTRIO.u64()
}

#[thread_local]
pub(crate) static LEHMER64: Lehmer64 = Lehmer64 {
    state: Cell::new(3),
};

#[inline(always)]
pub fn lehmer64_seed() {
    LEHMER64.seed()
}

#[inline(always)]
pub fn lehmer64_u64() -> u64 {
    LEHMER64.u64()
}

#[thread_local]
pub(crate) static MWC256XXA64: Mwc256XXA64 = Mwc256XXA64 {
    x1: Cell::new(3),
    x2: Cell::new(3),
    x3: Cell::new(3),
    c: Cell::new(3),
};

#[inline(always)]
pub fn mwc256xxa64_seed() {
    MWC256XXA64.seed()
}

#[inline(always)]
pub fn mwc256xxa64_u64() -> u64 {
    MWC256XXA64.u64()
}

#[thread_local]
pub(crate) static WYRAND: Wyrand = Wyrand {
    state: Cell::new(3),
};

#[inline(always)]
pub fn wyrand_seed() {
    WYRAND.seed()
}

#[inline(always)]
pub fn wyrand_u64() -> u64 {
    WYRAND.u64()
}

#[thread_local]
pub(crate) static SPLITMIX64: Splitmix64 = Splitmix64 {
    state: Cell::new(3),
};

#[inline(always)]
pub fn splitmix64_seed() {
    SPLITMIX64.seed()
}

#[inline(always)]
pub fn splitmix64_u64() -> u64 {
    SPLITMIX64.u64()
}

#[thread_local]
pub(crate) static XOSHIRO256PLUSPLUS: Xoshiro256plusplus = Xoshiro256plusplus {
    s0: Cell::new(3),
    s1: Cell::new(3),
    s2: Cell::new(3),
    s3: Cell::new(3),
};

#[inline(always)]
pub fn xoshiro256plusplus_seed() {
    XOSHIRO256PLUSPLUS.seed()
}

#[inline(always)]
pub fn xoshiro256plusplus_u64() -> u64 {
    XOSHIRO256PLUSPLUS.u64()
}

#[thread_local]
pub(crate) static PCG64: Pcg64 = Pcg64 {
    state: Cell::new(3),
    inc: Cell::new(3),
};

#[inline(always)]
pub fn pcg64_seed() {
    PCG64.seed()
}

#[inline(always)]
pub fn pcg64_u64() -> u64 {
    PCG64.u64()
}

#[thread_local]
pub(crate) static PCG64FAST: Pcg64Fast = Pcg64Fast {
    state: Cell::new(3),
};

#[inline(always)]
pub fn pcg64fast_seed() {
    PCG64FAST.seed()
}

#[inline(always)]
pub fn pcg64fast_u64() -> u64 {
    PCG64FAST.u64()
}
