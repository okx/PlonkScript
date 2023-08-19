use std::{clone, marker::PhantomData};

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{floor_planner::V1, *},
    dev::{FailureLocation, MockProver, VerifyFailure},
    pasta::Fp,
    plonk::*,
    poly::Rotation,
};

#[derive(Debug, Clone)]
struct RangeCheckConfig<F: FieldExt, const RANGE: usize> {
    value: Column<Advice>,
    q_range_check: Selector,
    _marker: PhantomData<F>,
}

impl<F: FieldExt, const RANGE: usize> RangeCheckConfig<F, RANGE> {
    fn configure(meta: &mut ConstraintSystem<F>, value: Column<Advice>) -> Self {
        let q_range_check = meta.selector();

        let config = Self {
            q_range_check,
            value,
            _marker: PhantomData,
        };

        // range check gate
        meta.create_gate("range check", |meta| {
            let q_range_check = meta.query_selector(q_range_check);
            let value = meta.query_advice(value, Rotation::cur());

            let range_check = |range, value: Expression<F>| {
                (0..range).fold(value.clone(), |acc, cv| {
                    acc * (Expression::Constant(F::from(cv as u64)) - value.clone())
                })
            };

            Constraints::with_selector(q_range_check, [("range check", range_check(RANGE, value))])
        });

        config
    }

    fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        value: Value<Assigned<F>>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "assign value",
            |mut region| {
                let offset = 0;

                self.q_range_check.enable(&mut region, offset);

                region.assign_advice(|| "assign value", self.value, offset, || value)?;

                Ok(())
            },
        )
    }
}

#[derive(Default)]
struct MyCircuit<F: FieldExt, const RANGE: usize> {
    value: Value<Assigned<F>>,
}

impl<F: FieldExt, const RANGE: usize> Circuit<F> for MyCircuit<F, RANGE> {
    type Config = RangeCheckConfig<F, RANGE>;
    type FloorPlanner = V1;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let value = meta.advice_column();
        RangeCheckConfig::configure(meta, value)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        config.assign(layouter.namespace(|| "Assign value"), self.value)?;

        Ok(())
    }
}

fn main() {
    let k = 4;
    const RANGE: usize = 8; // 3-bit value

    // Successful cases
    for i in 0..RANGE {
        let circuit = MyCircuit::<Fp, RANGE> {
            value: Value::known(Fp::from(i as u64).into()),
        };

        let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        prover.assert_satisfied();
        if i == 6 {
            println!("{:#?}", prover);
        }
    }

    // Out-of-range `value = 8`
    {
        let circuit = MyCircuit::<Fp, RANGE> {
            value: Value::known(Fp::from(RANGE as u64).into()),
        };
        let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        assert_eq!(
            prover.verify(),
            Err(vec![VerifyFailure::ConstraintNotSatisfied {
                constraint: ((0, "range check").into(), 0, "range check").into(),
                location: FailureLocation::InRegion {
                    region: (0, "assign value").into(),
                    offset: 0
                },
                cell_values: vec![(((Any::Advice, 0).into(), 0).into(), "0x8".to_string())]
            }])
        );
    }
}
