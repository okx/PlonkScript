#![allow(dead_code)]

use halo2_proofs::pasta::{group::ff::PrimeField, Fp};

pub fn get_known_value<F>(s: String) -> Option<F>
where
    F: PrimeField,
{
    match s {
        s if s.starts_with("0x") => {
            let mut bytes = F::Repr::default();
            let mut view = bytes.as_mut();
            hex::decode_to_slice(&s[2..], &mut view).ok()?; //.expect("Decoding failed");
            view.reverse();
            F::from_repr_vartime(bytes)
            // match F::from_repr_vartime(bytes) {
            //     None => panic!("Decoding failed: {}", s),
            //     Some(x) => x,
            // }
        }
        _ => Some(F::from(s.parse::<u64>().ok()?)),
    }
}

pub fn fp_to_string(f: &Fp) -> String {
    "0x".to_string()
        + &*f
            .to_repr()
            .as_ref()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<Vec<String>>()
            .join("")
}
