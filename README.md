# PRNG Benchmark

A simple benchmark in Rust to benchmark some PRNGs under extremely simple conditions.

Please refer the source code / documentation of the PRN about how to use them.
Some PRNG need their state especially prepared to work correctly.

The PRNG for this benchmark should be as portable and simple as possible (no custom CPU extensions / SIMD).

# Candidates

* [Lehmer64](https://lemire.me/blog/2019/03/19/the-fastest-conventional-random-number-generator-that-can-pass-big-crush/)
* [Mwc256XXA64](https://github.com/tkaitchuck/Mwc256XXA64)
* [PCG64](https://www.pcg-random.org/)
* [Romu](https://www.romu-random.org/)
* [Splitmix64](https://prng.di.unimi.it/splitmix64.c)
* [Tylo64](https://github.com/numpy/numpy/issues/16313#issuecomment-641897028)
* [Wyrand](https://github.com/wangyi-fudan/wyhash)
* [Xoshiro256++](https://prng.di.unimi.it/)

# License

The license of the benchmark code itself is public domain.

The license of the respective PRNG is documented inside the source code.
