// // pub use ::gf256::*;
// use ::gf256::gf::gf;

// #[gf(polynomial = 101, generator = 1 )]
// type sgf;
// // # use ::gf256::*;
// // use ::gf256::gf::gf;

// #[gf(polynomial = 0b10011, generator = 0b0010)]
// type gf16;

// #[macro_use]
extern crate ff;

use ff::PrimeField;

#[derive(PrimeField)]
#[PrimeFieldModulus = "101"]
#[PrimeFieldGenerator = "1"]
#[PrimeFieldReprEndianness = "little"]
struct Fp([u64; 1]);

fn main() {
    // assert_eq!(gf16::new(0b1011) * gf16::new(0b1101), gf16::new(0b0110));
    // let a = sgf::new(102);
    // let b = sgf::new(101);
    // assert_eq!(a,b);
    // println!("{:?}", a.get());

    // let a = Fp([1, 0, 0, 0]);
    let a = Fp([1]);
    println!("{:?}", a);
}
