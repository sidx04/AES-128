use std::ops::{BitXor, Mul};

#[derive(Debug, Clone, Copy)]
pub struct GF256(pub u8);

/// AES irreducible polynomial for GF(2^8): x^8 + x^4 + x^3 + x + 1
const IRREDUCIBLE_POLYNOMIAL: u16 = 0x11B;

impl GF256 {
    // Helper function to reduce modulo the irreducible polynomial
    fn reduce(value: u16) -> u8 {
        (value & 0xFF) as u8
    }
}

// Implement conversion from u8 to GF256
impl From<u8> for GF256 {
    fn from(value: u8) -> GF256 {
        GF256(value)
    }
}

// Implement Galois field multiplication using operator overloading
impl Mul for GF256 {
    type Output = u8;

    fn mul(self, rhs: GF256) -> u8 {
        let mut result: u16 = 0;
        let mut a = self.0;
        let mut b = rhs.0;

        for _ in 0..8 {
            if b & 1 != 0 {
                result ^= a as u16;
            }
            let high_bit_set = a & 0x80 != 0;
            a = (a << 1) & 0xFF;
            if high_bit_set {
                a ^= IRREDUCIBLE_POLYNOMIAL as u8; // Apply the irreducible polynomial
            }
            b >>= 1;
        }

        GF256::reduce(result)
    }
}

// Implement XOR (^) operation using operator overloading and returning u8
impl BitXor for GF256 {
    type Output = u8;

    fn bitxor(self, other: GF256) -> u8 {
        self.0 ^ other.0
    }
}
