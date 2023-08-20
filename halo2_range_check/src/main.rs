mod table;
use std::{fs::File, io::Write};

use table::RangeCheckTable;

// use std::{clone, marker::PhantomData};

/// <table>
/// <head>
///
/// | value | q_range_check | q_lookup | table_value |
/// | ----- | ------------- | -------- | ----------- |
/// |   v   |       1       |     0    |      0      |
/// |   v'  |       0       |     1    |      1      | // for larger using lookup table
///
use halo2_proofs::{
    circuit::{floor_planner::V1, *},
    dev::{FailureLocation, MockProver, VerifyFailure},
    pasta::{group::ff::PrimeField, Fp},
    plonk::*,
    poly::Rotation,
};

#[derive(Debug, Clone)]
struct RangeCheckConfig<F: PrimeField, const RANGE: usize, const LOOKUP_RANGE: usize> {
    value: Column<Advice>,
    q_range_check: Selector,
    q_lookup: Selector,
    table: RangeCheckTable<F, RANGE>,
    // _marker: PhantomData<F>,
}

impl<F: PrimeField, const RANGE: usize, const LOOKUP_RANGE: usize>
    RangeCheckConfig<F, RANGE, LOOKUP_RANGE>
{
    fn configure(meta: &mut ConstraintSystem<F>, value: Column<Advice>) -> Self {
        let q_range_check = meta.selector();

        let q_lookup = meta.complex_selector();

        // Configure a lookuptable
        let table = RangeCheckTable::configure(meta);

        let config = Self {
            q_range_check,
            q_lookup,
            value,
            table: table.clone(),
            // _marker: PhantomData,
        };

        // range check gate
        meta.create_gate("range check", |meta| {
            let q_range_check = meta.query_selector(q_range_check);
            let value = meta.query_advice(value, Rotation::cur());

            let range_check = |range, value: Expression<F>| {
                (0..range).fold(value.clone(), |acc, cv| {
                    acc * (Expression::Constant(F::from_u128(cv as u128)) - value.clone())
                })
            };

            Constraints::with_selector(q_range_check, [("range check", range_check(RANGE, value))])
        });

        // range-check lookup
        meta.lookup(|meta| {
            let q_lookup = meta.query_selector(q_lookup);
            let value = meta.query_advice(value, Rotation::cur());

            vec![(q_lookup * value, table.value)]
        });

        config
    }

    fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        value: Value<Assigned<F>>,
        range: usize,
    ) -> Result<(), Error> {
        assert!(range <= LOOKUP_RANGE);

        if range <= RANGE {
            layouter.assign_region(
                || "assign value for simple range check",
                |mut region| {
                    let offset = 0;

                    self.q_range_check.enable(&mut region, offset)?;

                    region.assign_advice(|| "assign value", self.value, offset, || value)?;

                    Ok(())
                },
            )
        } else {
            layouter.assign_region(
                || "assign value for lookup range check",
                |mut region| {
                    let offset = 0;

                    self.q_lookup.enable(&mut region, offset)?;

                    region.assign_advice(|| "assign value", self.value, offset, || value)?;

                    Ok(())
                },
            )
        }
    }
}

#[derive(Default, Debug)]
struct MyCircuit<F: PrimeField, const RANGE: usize, const LOOKUP_RANGE: usize> {
    value: Value<Assigned<F>>,
    large_value: Value<Assigned<F>>,
}

impl<F: PrimeField, const RANGE: usize, const LOOKUP_RANGE: usize> Circuit<F>
    for MyCircuit<F, RANGE, LOOKUP_RANGE>
{
    type Config = RangeCheckConfig<F, RANGE, LOOKUP_RANGE>;
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
        config.table.load(&mut layouter)?;

        config.assign(
            layouter.namespace(|| "Assign simple value"),
            self.value,
            RANGE,
        )?;
        config.assign(
            layouter.namespace(|| "Assign larger value"),
            self.large_value,
            LOOKUP_RANGE,
        )?;

        Ok(())
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct MockProverJsonRoot {
//     k: usize,
//     n: usize,
//     cs: MockProverJsonConstraintSystem,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct MockProverJsonConstraintSystem {
//     num_fixed_columns: usize,
//     num_advice_columns: usize,
//     num_instance_columns: usize,
//     num_selectors: usize,
// }

fn main() {
    let k = 4;
    const RANGE: usize = 8; // 3-bit value
    const LOOKUP_RANGE: usize = 256; // 8-bit value

    // Successful cases
    for i in 0..RANGE {
        let circuit = MyCircuit::<Fp, RANGE, LOOKUP_RANGE> {
            value: Value::known(Fp::from(i as u64).into()),
            large_value: Value::known(Fp::from(i as u64).into()),
        };

        let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        prover.assert_satisfied();
        if i == 6 {
            // println!("{:#?}", prover);
            // println!("{}", serde_json::to_string_pretty(&prover).unwrap());
            // println!("{:#?}", &circuit);
            let d = format!("{:#?}", prover);
            // let dd = parse(&d);
            // let ddd = format!("{}", serde_json::to_string_pretty(&prover).unwrap());
            // println!("{}", d);
            // let mut file = File::create("layout.rust").unwrap();
            let mut file = File::create(
                "/Users/oker/2-Project/02-zkkyc/halo2visualizer/packages/cli/src/input.rust",
            )
            .unwrap();
            file.write_all(d.as_bytes()).unwrap();
        }
    }

    // Out-of-range `value = 8`
    {
        let circuit = MyCircuit::<Fp, RANGE, LOOKUP_RANGE> {
            value: Value::known(Fp::from(RANGE as u64).into()),
            large_value: Value::known(Fp::from(LOOKUP_RANGE as u64).into()),
        };
        let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        assert_eq!(
            prover.verify(),
            Err(vec![
                VerifyFailure::ConstraintNotSatisfied {
                    constraint: ((0, "range check").into(), 0, "range check").into(),
                    location: FailureLocation::InRegion {
                        region: (1, "assign value for simple range check").into(),
                        offset: 0
                    },
                    cell_values: vec![(((Any::Advice, 0).into(), 0).into(), "0x8".to_string())]
                },
                VerifyFailure::Lookup {
                    lookup_index: 0,
                    location: FailureLocation::InRegion {
                        region: (2, "assign value for lookup range check").into(),
                        offset: 0
                    }
                }
            ])
        );
    }
}
