use wasm_bindgen::prelude::*;

// 64 = 4 × 16
// 4bit の数字を16個並べる配列
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Array64 {
    array: u64,
}
impl Array64 {
    pub fn new() -> Self {
        Array64 { array: 0 as u64 }
    }
    /// Set i-th element as v
    /// 
    /// # Arguments
    /// * `i` - index (i < 16)
    /// * `v` - value (v < 16)
    pub fn set(&mut self, i: usize, v: u8) {
        // assert(i < 16);
        // assert(v < 16);

        let mask = 0xfu64 << (i * 4);
        let mask = u64::MAX ^ mask;
        self.array &= mask;
        self.array |= (v as u64) << (i * 4);
    }
    /// Get i-th element
    /// 
    /// # Arguments
    /// * `i` - index (i < 16)
    pub fn get(&self, i: usize) -> u8 {
        let mask = 0xfu64;
        ((self.array >> (i * 4)) & mask) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_time_array() {
        use std::mem;
        assert_eq!(8, mem::size_of::<Array64>());
    }

    #[test]
    fn test_array() {
        let mut array = Array64::new();

        assert_eq!(0, array.array);
        for i in 0..16 {
            array.set(i, 15 - i as u8);
        }
        for i in 0..16 {
            assert_eq!(15 - i as u8, array.get(i));
        }
        assert_eq!(
            0b0000_0001_0010_0011_0100_0101_0110_0111_1000_1001_1010_1011_1100_1101_1110_1111
                as u64,
            array.array
        );
        for i in 0..16 {
            array.set(i, i as u8);
        }
        for i in 0..16 {
            assert_eq!(i as u8, array.get(i));
        }
        assert_eq!(
            0b1111_1110_1101_1100_1011_1010_1001_1000_0111_0110_0101_0100_0011_0010_0001_0000
                as u64,
            array.array
        );
    }
}
