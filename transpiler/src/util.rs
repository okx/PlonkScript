#![allow(dead_code)]

use crate::system::CellExpression;
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
            .rev()
            .map(|x| format!("{:02x}", x))
            .collect::<Vec<String>>()
            .join("")
}

#[test]
fn test_fp_to_string() {
    let f = Fp::from(1);
    assert_eq!(
        fp_to_string(&f),
        "0x0000000000000000000000000000000000000000000000000000000000000001".to_string()
    );
}

pub fn convert_to_value(exp: CellExpression) -> Option<String> {
    match exp {
        CellExpression::Constant(c) => Some(c),
        CellExpression::CellValue(c) => match c.column.ctype {
            crate::system::ColumnType::Selector => todo!(),
            crate::system::ColumnType::Advice => c.value,
            crate::system::ColumnType::Fixed => c.value,
            crate::system::ColumnType::Instance => todo!(),
        },
        CellExpression::Negated(n) => {
            get_known_value::<Fp>(convert_to_value(*n)?).map(|x| fp_to_string(&(-x)))
        }
        CellExpression::Product(a, b) => {
            match (
                get_known_value::<Fp>(convert_to_value(*a)?),
                get_known_value::<Fp>(convert_to_value(*b)?),
            ) {
                (None, _) | (_, None) => None,
                (Some(a), Some(b)) => Some(fp_to_string(&(a * b))),
            }
        }
        CellExpression::Sum(a, b) => {
            match (
                get_known_value::<Fp>(convert_to_value(*a)?),
                get_known_value::<Fp>(convert_to_value(*b)?),
            ) {
                (None, _) | (_, None) => None,
                (Some(a), Some(b)) => Some(fp_to_string(&(a + b))),
            }
        }
        CellExpression::Scaled(a, b) => {
            match (
                get_known_value::<Fp>(convert_to_value(*a)?),
                get_known_value::<Fp>(convert_to_value(CellExpression::Constant(b))?),
            ) {
                (None, _) | (_, None) => None,
                (Some(a), Some(b)) => Some(fp_to_string(&(a * b))),
            }
        }
    }
}

#[test]
fn test_convert_to_value() {
    let a = "0x0000000000000000000000000000000000000000000000000000000000000002".to_string();
    let b = "0x0000000000000000000000000000000000000000000000000000000000002002".to_string();
    assert_eq!(
        Some("0x0000000000000000000000000000000000000000000000000000000000004004".to_string()),
        convert_to_value(CellExpression::Product(
            Box::new(CellExpression::Constant(a.clone())),
            Box::new(CellExpression::Constant(b.clone())),
        ))
    );
    assert_eq!(
        Some("0x0000000000000000000000000000000000000000000000000000000000002004".to_string()),
        convert_to_value(CellExpression::Sum(
            Box::new(CellExpression::Constant(a.clone())),
            Box::new(CellExpression::Constant(b.clone())),
        ))
    );
    assert_eq!(
        Some("0x0000000000000000000000000000000000000000000000000000000000004004".to_string()),
        convert_to_value(CellExpression::Scaled(
            Box::new(CellExpression::Constant(a.clone())),
            b.clone(),
        ))
    );
    assert_eq!(
        Some("0x40000000000000000000000000000000224698fc094cf91b992d30ecffffffff".to_string()),
        convert_to_value(CellExpression::Negated(Box::new(CellExpression::Constant(
            a.clone()
        ))))
    );
}
