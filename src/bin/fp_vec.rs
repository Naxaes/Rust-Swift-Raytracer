#![allow(dead_code)]

use std::fmt;

// https://www.superkits.net/whitepapers/Fixed%20Point%20Representation%20&%20Fractional%20Math.pdf
// https://courses.cs.washington.edu/courses/cse467/08au/labs/l5/fp.pdf

/*
2^16 = 65536
1 / 2^16 = 0.0000152587890625

1     => 0.0000152587890625
65535 => 0.9999847412109375


*/

const PRECISION:   u64 = 152587890625;
const SHIFT_VALUE: i32 = 16;
const SHIFT_MASK:  i32 = (0b1 << SHIFT_VALUE) - 1;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Fraq(u16);
impl Fraq {
    pub fn half()    -> Self { Self(32768) }
    pub fn quarter() -> Self { Self(16384) }
}


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Fp32(i32);

impl Fp32 {
    // pub fn new(whole: i16, frac: Fraq) -> Self {
    //     if whole >= 1 {
    //         let a = (whole  as i32) << SHIFT_VALUE;
    //         let b = (frac.0 as i32) & SHIFT_MASK;
    //         Self(a | b)
    //     } else {
    //         let a = ((-1 * whole)  as i32) << SHIFT_VALUE;
    //         let b = (frac.0 as i32) & SHIFT_MASK;
    //         Self(!(a | b) + 1)
    //     }
    // }
    pub fn new(whole: i32) -> Self {
        let a = (whole  as i32) << SHIFT_VALUE;
        let b = (0      as i32) & SHIFT_MASK;
        Self(a | b)
    }

    pub fn whole(&self) -> i16 {
        (self.0 >> SHIFT_VALUE) as i16
    }
    pub fn fraq(&self)  -> u16 {
        (self.0 & SHIFT_MASK) as u16
    }

    pub fn add(&self, rhs: &Self) -> Self { Self(self.0 + rhs.0) }
    pub fn sub(&self, rhs: &Self) -> Self { Self(self.0 - rhs.0) }
    pub fn mul(&self, rhs: &Self) -> Self { Self(self.0 * rhs.0) }
}

impl fmt::Display for Fp32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = self.0 >> SHIFT_VALUE;
        let fraq  = self.0 & SHIFT_MASK;
        let repr  = (fraq as u64) * PRECISION;

        write!(f, "{}.{:0>16}", whole, repr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_whole() {
        let a = Fp32::new(32);
        let b = Fp32::new(40);
        let c = a.add(&b);

        assert_eq!(c.whole(), 72);
        assert_eq!(c.fraq(),  0);
    }

    // #[test]
    // fn add_mixed() {
    //     let a = Fp32::new(32, Fraq::quarter());
    //     let b = Fp32::new(32, Fraq::quarter());
    //     let c = a.add(&b);
    //
    //     assert_eq!(c.whole(), 64);
    //     assert_eq!(c.fraq(),  Fraq::half().0);
    // }
    //
    // #[test]
    // fn add_decimal_overflow() {
    //     let a = Fp32::new(32, Fraq::half());
    //     let b = Fp32::new(32, Fraq::half());
    //     let c = a.add(&b);
    //
    //     assert_eq!(c.whole(), 65);
    //     assert_eq!(c.fraq(),   0);
    // }

    #[test]
    fn add_negative() {
        let a = Fp32::new( 32);
        let b = Fp32::new(-32);
        let c = a.add(&b);

        assert_eq!(c.whole(), 0);
        assert_eq!(c.fraq(),  0);
    }

    // #[test]
    // fn add_negative_with_decimals() {
    //     println!(" 32 = {:<012}         = {:0>32b}", 32, 32);
    //     println!("-32 = {:<012}         = {:0>32b}", -32, -32);
    //     println!(" 32 = {:<012}         = {:0>32b}", 32 << SHIFT_VALUE , 32 << SHIFT_VALUE);
    //     println!("-32 = {:<012}         = {:0>32b}", -32 << SHIFT_VALUE, -32 << SHIFT_VALUE);
    //
    //     let a = (32 << SHIFT_VALUE) + Fraq::half().0 as i32;
    //     let b = -a;
    //     let c = a + b;
    //     println!("a   = {:<012}         = {:0>32b}", a, a);
    //     println!("b   = {:<012}         = {:0>32b}", b, b);
    //     println!("a+b =  {:<012}        = {:0>32b}",  c, c);
    //
    //
    //     let a = Fp32::new(32,  Fraq::half());
    //     let b = Fp32::new(-32, Fraq::half());
    //     let c = a.add(&b);
    //
    //     println!("a   =  {} = {:0>32b}", a, a.0);
    //     println!("b   = {} = {:0>32b}", b, b.0);
    //     println!("a+b =   {} = {:0>32b}",  c, c.0);
    //
    //     assert_eq!(c.whole(), 0);
    //     assert_eq!(c.fraq(),  0);
    // }


    #[test]
    fn sub_to_positive() {
        let a = Fp32::new(32);
        let b = Fp32::new(16);
        let c = a.sub(&b);

        assert_eq!(c.whole(), 16);
        assert_eq!(c.fraq(),  0);
    }
    #[test]
    fn sub_to_negative() {
        let a = Fp32::new(16);
        let b = Fp32::new(32);
        let c = a.sub(&b);

        assert_eq!(c.whole(), -16);
        assert_eq!(c.fraq(),  0);
    }

    // #[test]
    // fn sub_mixed() {
    //     let a = Fp32::new(32, Fraq::half());
    //     let b = Fp32::new(16, Fraq::quarter());
    //     let c = a.sub(&b);
    //
    //     assert_eq!(c.whole(), 16);
    //     assert_eq!(c.fraq(),  Fraq::quarter().0);
    // }
    //
    // #[test]
    // fn sub_decimal_underflow() {
    //     let a = Fp32::new(16, Fraq::quarter());
    //     let b = Fp32::new(32, Fraq::half());
    //     let c = a.sub(&b);
    //
    //     let x = Fp32::new(-16, Fraq::quarter());
    //     println!("a   =  {} = {:0>32b}", a, a.0);
    //     println!("b   =  {} = {:0>32b}", b, b.0);
    //     println!("a-b = {} = {:0>32b}",  c, c.0);
    //     println!("exp = {} = {:0>32b}",  x, x.0);
    //
    //     assert_eq!(c.whole(), -16);
    //     assert_eq!(c.fraq(),   Fraq::quarter().0);
    // }

    // #[test]
    // fn mul_whole() {
    //     let a = Fp32::new(32, Fraq::half());
    //     let b = Fp32::new(2, Fraq(0));
    //     let c = a.mul(&b);
    //
    //     assert_eq!(c.whole(), 64);
    //     assert_eq!(c.fraq(),  0);
    // }
    //
    // #[test]
    // fn mul_decimal() {
    //     let a = Fp32::new(1, Fraq::half());
    //     let b = Fp32::new(2, Fraq::half());
    //     let c = a.mul(&b);
    //
    //     assert_eq!(c.whole(), 2);
    //     assert_eq!(c.fraq(),  Fraq::quarter().0);
    // }
}


// const PRECISION:   u64 = 152587890625;
const FRACTION_BITS:    i32 = 16;
const FRACTION_MASK:    i32 = (0b1 << SHIFT_VALUE) - 1;
const FRACTION_DIVISOR: i32 = 0b1 << SHIFT_VALUE;

pub fn main() {

    let price: i32 = ((503 << FRACTION_BITS) + (10 << FRACTION_BITS)) * (3) / 7;

    // (503 + 10) * (-3) / 7 = -219.8571428571

    println!(" {:<8} = {:0>32b}  |  {:0>16b}.{:0>16b}   |   {}.{}", price, price, (price >> FRACTION_BITS) as i16, (price & FRACTION_MASK) as u16, (price >> FRACTION_BITS), (price & FRACTION_MASK) as u16);

    let price = price * -1;
    println!("{:<8} = {:0>32b}  |  {:0>16b}.{:0>16b}   |  {}.{}", price, price, ((price >> FRACTION_BITS)+1) as i16, !(((price & FRACTION_MASK) as u16) - 1), ((price >> FRACTION_BITS)+1), !(((price & FRACTION_MASK) as u16) - 1));


    let a = 0b0000000001000001000000000000000;
    let b = -a;
    let c = a + b;
    println!("a   =  {:<010} = {:0>32b}", a, a);
    println!("b    = {:<010} = {:0>32b}", b, b);
    println!("a+b =  {:<010} = {:0>32b}",  c, c);
}

// 32.25 - 32.75 =



