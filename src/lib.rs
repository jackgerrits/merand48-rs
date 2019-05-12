#[cfg(test)]
mod tests {
    use super::*;

    // Values generated using reference c++ code
    #[test]
    fn compare_against_reference() {
        assert_eq!(merand48(0), 7.51018524e-06);
        assert_eq!(merand48(1), 0.401708484);
        assert_eq!(merand48(100), 0.170098543);
        assert_eq!(merand48(100000), 0.0909534693);
    }

    #[test]
    fn check_bounds() {
        for i in 1..1000000
        {
            let rand = merand48(i);
            assert!(rand > 0.0);
            assert!(rand < 1.0);
        }
    }
}

const A: u64 = 0xeece66d5deece66d;
const C: u64 = 2147483647;
const BIAS: u64 = 127 << 23;

/// Quick random number generator used in VowpalWabbit. Values produced are between `0.0` and `1.0`.
///
///  [_See reference C++ implementation in VowpalWabbit_](https://github.com/VowpalWabbit/vowpal_wabbit/blob/45a8a7f5507015a17d585f7698eab20c963b592b/explore/explore_internal.h#L29)
///
/// ```
/// use merand48::*;
///
/// let mut seed = 12345;
/// let random_value = merand48(seed);
/// seed += 1;
/// ```
///
pub fn merand48(initial: u64) -> f32 {
    let value = A.wrapping_mul(initial).wrapping_add(C);
    let temp: i32 = (((value >> 25) & 0x7FFFFF) | BIAS) as i32;
    f32::from_bits(temp as u32) - 1.0
}
