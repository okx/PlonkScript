use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Layouter, Value},
    pasta::group::ff::PrimeField,
    plonk::{ConstraintSystem, Error, TableColumn},
};

/// RANGE = 256, values = [0..255]
///

#[derive(Debug, Clone)]
pub(super) struct RangeCheckTable<F: PrimeField, const RANGE: usize> {
    pub(super) value: TableColumn,
    _marker: PhantomData<F>,
}

impl<F: PrimeField, const RANGE: usize> RangeCheckTable<F, RANGE> {
    pub(super) fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let value = meta.lookup_table_column();

        Self {
            value,
            _marker: PhantomData,
        }
    }

    pub(super) fn load(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_table(
            || "load range-check table",
            |mut table| {
                let mut offset = 0;
                for i in 0..RANGE {
                    table.assign_cell(
                        || "assign cell",
                        self.value,
                        offset,
                        || Value::known(F::from(i as u64)),
                    )?;
                    offset += 1;
                }

                Ok(())
            },
        )
    }
}
