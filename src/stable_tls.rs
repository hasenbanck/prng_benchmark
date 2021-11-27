use crate::{
    Lehmer64, Mwc256XXA64, Pcg64, Pcg64Fast, Random, RomuJr, RomuTrio, Splitmix64, Tylo64, Wyrand,
    Xoshiro256plusplus,
};

thread_local! {
    static TYLO64: Tylo64 = Tylo64::default();
}

#[inline(always)]
pub fn tylo64_seed() {
    TYLO64.with(|rng| rng.seed())
}

#[inline(always)]
pub fn tylo64_u64() -> u64 {
    TYLO64.with(|rng| rng.u64())
}

thread_local! {
    static ROMUJR: RomuJr = RomuJr::default();
}

#[inline(always)]
pub fn romu_jr_seed() {
    ROMUJR.with(|rng| rng.seed())
}

#[inline(always)]
pub fn romu_jr_u64() -> u64 {
    ROMUJR.with(|rng| rng.u64())
}

thread_local! {
    static ROMUTRIO: RomuTrio = RomuTrio::default();
}

#[inline(always)]
pub fn romu_trio_seed() {
    ROMUTRIO.with(|rng| rng.seed())
}

#[inline(always)]
pub fn romu_trio_u64() -> u64 {
    ROMUTRIO.with(|rng| rng.u64())
}

thread_local! {
    static LEHMER64: Lehmer64 = Lehmer64::default();
}

#[inline(always)]
pub fn lehmer64_seed() {
    LEHMER64.with(|rng| rng.seed())
}

#[inline(always)]
pub fn lehmer64_u64() -> u64 {
    LEHMER64.with(|rng| rng.u64())
}

thread_local! {
    static MWC256XXA64: Mwc256XXA64 = Mwc256XXA64::default();
}

#[inline(always)]
pub fn mwc256xxa64_seed() {
    MWC256XXA64.with(|rng| rng.seed())
}

#[inline(always)]
pub fn mwc256xxa64_u64() -> u64 {
    MWC256XXA64.with(|rng| rng.u64())
}

thread_local! {
    static WYRAND: Wyrand = Wyrand::default();
}

#[inline(always)]
pub fn wyrand_seed() {
    WYRAND.with(|rng| rng.seed())
}

#[inline(always)]
pub fn wyrand_u64() -> u64 {
    WYRAND.with(|rng| rng.u64())
}

thread_local! {
    static SPLITMIX64: Splitmix64 = Splitmix64::default();
}

#[inline(always)]
pub fn splitmix64_seed() {
    SPLITMIX64.with(|rng| rng.seed())
}

#[inline(always)]
pub fn splitmix64_u64() -> u64 {
    SPLITMIX64.with(|rng| rng.u64())
}

thread_local! {
    static XOSHIRO256PLUSPLUS: Xoshiro256plusplus = Xoshiro256plusplus::default();
}

#[inline(always)]
pub fn xoshiro256plusplus_seed() {
    XOSHIRO256PLUSPLUS.with(|rng| rng.seed())
}

#[inline(always)]
pub fn xoshiro256plusplus_u64() -> u64 {
    XOSHIRO256PLUSPLUS.with(|rng| rng.u64())
}

thread_local! {
    static PCG64: Pcg64 = Pcg64::default();
}

#[inline(always)]
pub fn pcg64_seed() {
    PCG64.with(|rng| rng.seed())
}

#[inline(always)]
pub fn pcg64_u64() -> u64 {
    PCG64.with(|rng| rng.u64())
}

thread_local! {
    static PCG64FAST: Pcg64Fast = Pcg64Fast::default();
}

#[inline(always)]
pub fn pcg64fast_seed() {
    PCG64FAST.with(|rng| rng.seed())
}

#[inline(always)]
pub fn pcg64fast_u64() -> u64 {
    PCG64FAST.with(|rng| rng.u64())
}
