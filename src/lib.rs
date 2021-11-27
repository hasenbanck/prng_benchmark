#![feature(thread_local)]

use std::cell::Cell;

#[cfg(feature = "nightly_thread_local")]
mod nightly_tls;
#[cfg(not(feature = "nightly_thread_local"))]
mod stable_tls;

#[cfg(feature = "nightly_thread_local")]
pub use nightly_tls::*;
#[cfg(not(feature = "nightly_thread_local"))]
pub use stable_tls::*;

pub trait Random {
    fn u64(&self) -> u64;
    fn seed(&self);

    fn fill_bytes(&self, slice: &mut [u8]) {
        let mut chunks = slice.chunks_exact_mut(8);
        for chunk in &mut chunks {
            chunk.copy_from_slice(&self.u64().to_ne_bytes())
        }
        chunks
            .into_remainder()
            .iter_mut()
            .for_each(|x| *x = self.u64() as u8);
    }
}

fn get_seed() -> u64 {
    let mut buffer = [0u8; 8];
    getrandom::getrandom(&mut buffer).expect("can't get seed");
    u64::from_ne_bytes(buffer)
}

/// Copyright 2020 Tyge Løvset.
pub struct Tylo64 {
    pub a: Cell<u64>,
    pub b: Cell<u64>,
    pub w: Cell<u64>,
    pub k: Cell<u64>, // Needs to be odd!
}

impl Default for Tylo64 {
    fn default() -> Self {
        let s = Self {
            a: Cell::new(0),
            b: Cell::new(0),
            w: Cell::new(0),
            k: Cell::new(0),
        };
        s.seed();
        s
    }
}

impl Random for Tylo64 {
    #[inline(always)]
    fn u64(&self) -> u64 {
        let b = self.b.get();

        self.w.set(self.w.get() + self.k.get());
        let out = self.a.get() ^ self.w.get();

        self.a.set((b + (b << 3)) ^ (b >> 11));
        self.b.set(((b << 24) | (b >> 40)) + out);

        out
    }

    fn seed(&self) {
        let mut k = get_seed();
        if k % 2 == 0 {
            k -= 1;
        }

        self.a.set(get_seed() | 1);
        self.b.set(get_seed() | 1);
        self.w.set(get_seed() | 1);
        self.k.set(k);
    }
}

/// Copyright 2020 Mark A. Overton
/// Licensed under Apache-2.0.
pub struct RomuJr {
    pub x: Cell<u64>,
    pub y: Cell<u64>,
}

impl Default for RomuJr {
    fn default() -> Self {
        let s = Self {
            x: Cell::new(0),
            y: Cell::new(0),
        };
        s.seed();
        s
    }
}

impl Random for RomuJr {
    #[inline(always)]
    fn u64(&self) -> u64 {
        let xp = self.x.get();

        self.x.set(15241094284759029579 * self.y.get());
        self.y.set(self.y.get() - xp);
        self.y.set((self.y.get() << 27) | (self.y.get() >> 37));

        xp
    }

    fn seed(&self) {
        self.x.set(get_seed() | 1);
        self.y.set(get_seed() | 1);
    }
}

/// Copyright 2020 Mark A. Overton
/// Licensed under Apache-2.0.
pub struct RomuTrio {
    pub x: Cell<u64>,
    pub y: Cell<u64>,
    pub z: Cell<u64>,
}

impl Default for RomuTrio {
    fn default() -> Self {
        let s = Self {
            x: Cell::new(0),
            y: Cell::new(0),
            z: Cell::new(0),
        };
        s.seed();
        s
    }
}

impl Random for RomuTrio {
    #[inline(always)]
    fn u64(&self) -> u64 {
        let xp = self.x.get();
        let yp = self.y.get();
        let zp = self.z.get();

        self.x.set(15241094284759029579 * zp);
        self.y.set(yp - xp);
        self.y.set((self.y.get() << 12) | (self.y.get() >> 52));
        self.z.set(zp - yp);
        self.z.set((self.z.get() << 44) | (self.z.get() >> 20));

        xp
    }

    fn seed(&self) {
        self.x.set(get_seed() | 1);
        self.y.set(get_seed() | 1);
        self.z.set(get_seed() | 1);
    }
}

/// D. H. Lehmer, Mathematical methods in large-scale computing units.
/// Proceedings of a Second Symposium on Large Scale Digital Calculating
/// Machinery;
/// Annals of the Computation Laboratory, Harvard Univ. 26 (1951), pp. 141-146.
///
/// P L'Ecuyer,  Tables of linear congruential generators of different sizes and
/// good lattice structure. Mathematics of Computation of the American
/// Mathematical
/// Society 68.225 (1999): 249-260.
pub struct Lehmer64 {
    pub state: Cell<u128>,
}

impl Default for Lehmer64 {
    fn default() -> Self {
        let s = Self {
            state: Cell::new(0),
        };
        s.seed();
        s
    }
}

impl Random for Lehmer64 {
    #[inline(always)]
    fn u64(&self) -> u64 {
        let s = self.state.get();
        self.state
            .set(self.state.get().wrapping_mul(0xDA942042E4DD58B5));

        (s >> 64) as u64
    }

    fn seed(&self) {
        self.state
            .set((((get_seed() as u128) << 64) + get_seed() as u128) | 1);
    }
}

/// (c) 2021 Tom Kaitchuck
/// Licensed under MIT or Apache-2.0.
pub struct Mwc256XXA64 {
    pub x1: Cell<u64>,
    pub x2: Cell<u64>,
    pub x3: Cell<u64>,
    pub c: Cell<u64>,
}

impl Default for Mwc256XXA64 {
    fn default() -> Self {
        let s = Self {
            x1: Cell::new(0),
            x2: Cell::new(0),
            x3: Cell::new(0),
            c: Cell::new(0),
        };
        s.seed();
        s
    }
}

impl Random for Mwc256XXA64 {
    #[inline(always)]
    fn u64(&self) -> u64 {
        let t = (self.x3.get() as u128).wrapping_mul(0xDEB344657C0AF413);
        let low = t as u64;
        let high = (t >> 64) as u64;

        let result = (self.x3.get() ^ self.x2.get()).wrapping_add(self.x1.get() ^ high);
        let (x1, b) = low.overflowing_add(self.c.get());

        self.x3.set(self.x2.get());
        self.x2.set(self.x1.get());
        self.x1.set(x1);
        self.c.set(high.wrapping_add(b as u64));

        result
    }

    fn seed(&self) {
        self.x1.set(get_seed() | 1);
        self.x2.set(get_seed() | 1);
        self.x3.set(get_seed() | 1);
        self.c.set(get_seed() | 1);
    }
}

/// Licensed under "The Unlicense".
pub struct Wyrand {
    pub state: Cell<u64>,
}

impl Default for Wyrand {
    fn default() -> Self {
        let s = Self {
            state: Cell::new(0),
        };
        s.seed();
        s
    }
}

impl Random for Wyrand {
    #[inline(always)]
    fn u64(&self) -> u64 {
        let s = self.state.get();
        self.state
            .set(self.state.get().wrapping_add(0xA0761D6478BD642F));

        let c = (s ^ 0xE7037ED1A0B428DB) as u128 * s as u128;
        ((c >> 64) ^ c) as u64
    }

    fn seed(&self) {
        self.state.set(get_seed() | 1);
    }
}

/// Written in 2015 by Sebastiano Vigna.
///
/// To the extent possible under law, the author has dedicated all copyright
/// and related and neighboring rights to this software to the public domain
/// worldwide. This software is distributed without any warranty.
///
/// See <http://creativecommons.org/publicdomain/zero/1.0/>.
pub struct Splitmix64 {
    pub state: Cell<u64>,
}

impl Default for Splitmix64 {
    fn default() -> Self {
        let s = Self {
            state: Cell::new(0),
        };
        s.seed();
        s
    }
}

impl Random for Splitmix64 {
    #[inline(always)]
    fn u64(&self) -> u64 {
        let mut x = self.state.get();
        self.state.set(self.state.get() + 0x9E3779B97F4A7C15);
        x = (x ^ (x >> 30)) * 0xBF58476D1CE4E5B9;
        x = (x ^ (x >> 27)) * 0x94D049BB133111EB;
        x ^ (x >> 31)
    }

    fn seed(&self) {
        self.state.set(get_seed() | 1);
    }
}

/// Written in 2019 by David Blackman and Sebastiano Vigna.
///
/// To the extent possible under law, the author has dedicated all copyright
/// and related and neighboring rights to this software to the public domain
/// worldwide. This software is distributed without any warranty.
///
/// See <http://creativecommons.org/publicdomain/zero/1.0/>.
pub struct Xoshiro256plusplus {
    pub s0: Cell<u64>,
    pub s1: Cell<u64>,
    pub s2: Cell<u64>,
    pub s3: Cell<u64>,
}

impl Default for Xoshiro256plusplus {
    fn default() -> Self {
        let s = Self {
            s0: Cell::new(0),
            s1: Cell::new(0),
            s2: Cell::new(0),
            s3: Cell::new(0),
        };
        s.seed();
        s
    }
}

impl Random for Xoshiro256plusplus {
    #[inline(always)]
    fn u64(&self) -> u64 {
        #[inline(always)]
        fn xoshiro_rotl(x: u64, k: u64) -> u64 {
            (x << k) | (x >> (64 - k))
        }

        let result = xoshiro_rotl(self.s0.get() + self.s3.get(), 23) + self.s0.get();

        let t = self.s0.get() << 17;

        self.s2.set(self.s2.get() ^ self.s0.get());
        self.s3.set(self.s3.get() ^ self.s1.get());
        self.s1.set(self.s1.get() ^ self.s2.get());
        self.s0.set(self.s0.get() ^ self.s3.get());

        self.s2.set(self.s2.get() ^ t);

        self.s3.set(xoshiro_rotl(self.s3.get(), 45));

        result
    }

    fn seed(&self) {
        self.s0.set(get_seed() | 1);
        self.s1.set(get_seed() | 1);
        self.s2.set(get_seed() | 1);
        self.s3.set(get_seed() | 1);
    }
}

/// Copyright 2014-2019 Melissa O'Neill and the PCG Project contributors.
/// Licensed under MIT or Apache-2.0.
pub struct Pcg64 {
    pub state: Cell<u128>,
    pub inc: Cell<u128>,
}

impl Default for Pcg64 {
    fn default() -> Self {
        Self {
            state: Cell::new((((get_seed() as u128) << 64) + get_seed() as u128) | 1),
            inc: Cell::new((((get_seed() as u128) << 64) + get_seed() as u128) | 1),
        }
    }
}

impl Random for Pcg64 {
    #[inline(always)]
    fn u64(&self) -> u64 {
        let s = self.state.get();
        self.state.set(
            self.state
                .get()
                .wrapping_mul(0x2360ED051FC65DA44385DF649FCCF645)
                .wrapping_add(self.inc.get()),
        );

        let rot = (s >> 122) as u32;
        let xsl = ((s >> 64) as u64) ^ (s as u64);
        xsl.rotate_right(rot)
    }

    fn seed(&self) {
        self.state
            .set((((get_seed() as u128) << 64) + get_seed() as u128) | 1);
        self.inc
            .set((((get_seed() as u128) << 64) + get_seed() as u128) | 1);
    }
}

/// Copyright 2014-2019 Melissa O'Neill and the PCG Project contributors.
/// Licensed under MIT or Apache-2.0.
pub struct Pcg64Fast {
    pub state: Cell<u128>,
}

impl Default for Pcg64Fast {
    fn default() -> Self {
        let s = Self {
            state: Cell::new(0),
        };
        s.seed();
        s
    }
}

impl Random for Pcg64Fast {
    #[inline(always)]
    fn u64(&self) -> u64 {
        let s = self.state.get();
        self.state.set(
            self.state
                .get()
                .wrapping_mul(0x2360ED051FC65DA44385DF649FCCF645),
        );

        let rot = (s >> 122) as u32;
        let xsl = ((s >> 64) as u64) ^ (s as u64);
        xsl.rotate_right(rot)
    }

    fn seed(&self) {
        self.state
            .set((((get_seed() as u128) << 64) + get_seed() as u128) | 1);
    }
}
