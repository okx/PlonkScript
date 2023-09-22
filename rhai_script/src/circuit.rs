use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{floor_planner::V1, *},
    dev::{FailureLocation, MockProver, VerifyFailure},
    pasta::{group::ff::PrimeField, Fp},
    plonk::*,
    poly::Rotation,
};

use crate::SimplifiedConstraitSystem;
use crate::CONTEXT;

#[derive(Default, Debug)]
pub struct MyCircuit<F: PrimeField> {
    pub scs: SimplifiedConstraitSystem,
    pub _marker: PhantomData<F>,
}

#[derive(Debug, Clone)]
pub struct CommonConfig<F: PrimeField> {
    advices: Vec<Column<Advice>>,
    fixeds: Vec<Column<Fixed>>,
    selectors: Vec<Selector>,
    instances: Vec<Column<Instance>>,
    _marker: PhantomData<F>,
}

impl<F: PrimeField> Circuit<F> for MyCircuit<F> {
    type Config = CommonConfig<F>;
    type FloorPlanner = V1;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // let value = meta.advice_column();
        // CommonConfig::configure(meta, value)

        let mut advices = Vec::new();
        let mut fixeds = Vec::new();
        let mut selectors = Vec::new();
        let mut instances = Vec::new();

        // build columns
        let scs = unsafe { CONTEXT.clone() };
        for col in scs.columns {
            match col.ctype {
                crate::system::ColumnType::Advice => advices.push(meta.advice_column()),
                crate::system::ColumnType::Selector => selectors.push(meta.selector()),
                crate::system::ColumnType::Fixed => fixeds.push(meta.fixed_column()),
                crate::system::ColumnType::Instance => instances.push(meta.instance_column()),
            }
        }

        // enable_equality
        for c in advices.clone() {
            meta.enable_equality(c);
        }
        for c in instances.clone() {
            meta.enable_equality(c);
        }

        // gates
        // for g in scs.ga
        // meta.create_gate("add", |meta| {
        //     //
        //     // col_a | col_b | col_c | selector
        //     //   a      b        c       s
        //     //
        //     let s = meta.query_selector(selector);
        //     let a = meta.query_advice(col_a, Rotation::cur());
        //     let b = meta.query_advice(col_b, Rotation::cur());
        //     let c = meta.query_advice(col_c, Rotation::cur());
        //     vec![s * (a + b.clone() * Expression::Constant(F::from(2 as u64)) - c)]
        // });

        CommonConfig {
            advices,
            fixeds,
            selectors,
            instances,
            _marker: PhantomData,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        // layouter.assign_region(
        //     || "first row",
        //     |mut region| {
        //         self.config.selector.enable(&mut region, 0)?;

        //         let a_cell = region.assign_advice_from_instance(
        //             || "f(0)",
        //             self.config.instance,
        //             0,
        //             self.config.cols[0],
        //             0,
        //         )?;

        //         let b_cell = region.assign_advice_from_instance(
        //             || "f(1)",
        //             self.config.instance,
        //             1,
        //             self.config.cols[1],
        //             0,
        //         )?;

        //         let c_cell = region.assign_advice(
        //             || "a + b",
        //             self.config.cols[2],
        //             0,
        //             || a_cell.value().copied() + b_cell.value().copied() * Value::known(F::from(2)),
        //         )?;

        //         // Copy the value from b & c in previous row to a & b in current row
        //         prev_b.copy_advice(|| "a", &mut region, self.config.cols[0], 0)?;
        //         prev_c.copy_advice(|| "b", &mut region, self.config.cols[1], 0)?;

        //         let c_cell = region.assign_advice(
        //             || "c",
        //             self.config.cols[2],
        //             0,
        //             || prev_b.value().copied() + prev_c.value().copied() + prev_c.value().copied(),
        //         )?;

        //         Ok((a_cell, b_cell, c_cell))
        //     },
        // );

        // config.table.load(&mut layouter)?;

        // config.assign(
        //     layouter.namespace(|| "Assign simple value"),
        //     self.value,
        //     RANGE,
        // )?;
        // config.assign(
        //     layouter.namespace(|| "Assign larger value"),
        //     self.large_value,
        //     LOOKUP_RANGE,
        // )?;

        Ok(())
    }
}
