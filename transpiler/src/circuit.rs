use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{floor_planner::V1, *},
    pasta::group::ff::PrimeField,
    plonk::*,
    poly::Rotation,
};

use crate::system::{cell_expression::ToField, CellExpression};
use crate::{engine::DEFAULT_INSTANCE_COLUMN_NAME, CONTEXT};

#[derive(Default, Debug)]
pub struct MyCircuit<F: PrimeField> {
    // pub scs: SimplifiedConstraitSystem,
    pub _marker: PhantomData<F>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CommonConfig<F: PrimeField> {
    advices: Vec<(String, Column<Advice>)>,
    fixeds: Vec<(String, Column<Fixed>)>,
    selectors: Vec<(String, Selector)>,
    instances: Vec<(String, Column<Instance>)>,
    acells: Vec<(String, AssignedCell<F, F>)>,
    _marker: PhantomData<F>,
}

impl<F: PrimeField> Circuit<F> for MyCircuit<F> {
    type Config = CommonConfig<F>;
    type FloorPlanner = V1;
    // type FloorPlanner = SimpleFloorPlanner;

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
        let acells = Vec::new();

        if unsafe { CONTEXT.signals.len() > 0 } {
            instances.push((
                DEFAULT_INSTANCE_COLUMN_NAME.to_string(),
                meta.instance_column(),
            ));
        }

        // build columns
        let scols = unsafe { CONTEXT.columns.clone() };
        for col in scols {
            match col.ctype {
                crate::system::ColumnType::Advice => advices.push((col.name, meta.advice_column())),
                crate::system::ColumnType::Selector => selectors.push((col.name, meta.selector())),
                crate::system::ColumnType::Fixed => fixeds.push((col.name, meta.fixed_column())),
                crate::system::ColumnType::Instance => {
                    instances.push((col.name, meta.instance_column()))
                }
            }
        }

        // enable_equality
        for c in advices.clone() {
            meta.enable_equality(c.1);
        }
        for c in instances.clone() {
            meta.enable_equality(c.1);
        }

        let config = CommonConfig {
            advices,
            fixeds,
            selectors,
            instances,
            acells,
            _marker: PhantomData,
        };

        // build gates
        let sgates = unsafe { CONTEXT.gates.clone() };
        for (gname, gate) in sgates {
            meta.create_gate(Box::leak(gname.into_boxed_str()), |meta| {
                vec![convert_to_gate_expression(meta, config.clone(), gate)]
            });
        }

        config
    }

    fn synthesize(
        &self,
        mut config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let sregions = unsafe { CONTEXT.regions.clone() };

        for r in sregions {
            layouter.assign_region(
                || r.name.clone(),
                |mut region| {
                    for ins in r.instructions.clone() {
                        match ins {
                            crate::system::Instruction::EnableSelector(c) => config
                                .get_selector(c.column.name)
                                .enable(&mut region, c.index as usize)?,
                            crate::system::Instruction::AssignFixed(f, exp) => {
                                let acell = region.assign_fixed(
                                    || "fixed",
                                    config.get_fixed(f.column.name),
                                    f.index as usize,
                                    || config.convert_to_value(exp.clone()),
                                )?;
                                config.acells.push((f.name, acell));
                            }
                            crate::system::Instruction::AssignAdvice(a, exp) => {
                                let acell = region.assign_advice(
                                    || "advice",
                                    config.get_advice(a.column.name),
                                    a.index as usize,
                                    || config.convert_to_value(exp.clone()),
                                )?;
                                config.acells.push((a.name, acell));
                            }
                            crate::system::Instruction::AssignAdviceFromConstant(_, _) => todo!(),
                            crate::system::Instruction::AssignAdviceFromInstance(a, b) => {
                                let acell = region.assign_advice_from_instance(
                                    || "instance",
                                    config.get_instance(b.column.name),
                                    b.index as usize,
                                    config.get_advice(a.column.name),
                                    a.index as usize,
                                )?;
                                config.acells.push((a.name, acell));
                            }
                            crate::system::Instruction::ConstrainEqual(a, b) => {
                                let acell = config.get_assigned_cell(a.name);
                                let bcell = config.get_assigned_cell(b.name);
                                region.constrain_equal(acell.cell(), bcell.cell())?;
                            }
                            crate::system::Instruction::ConstrainConstant() => todo!(),
                        };
                    }

                    Ok(())
                },
            )?;
        }

        Ok(())
    }
}

impl<F: PrimeField> CommonConfig<F> {
    fn get_selector(&self, name: String) -> Selector {
        self.selectors
            .clone()
            .into_iter()
            .filter(|x| x.0 == name)
            .nth(0)
            .unwrap()
            .1
    }

    fn get_advice(&self, name: String) -> Column<Advice> {
        self.advices
            .clone()
            .into_iter()
            .filter(|x| x.0 == name)
            .nth(0)
            .unwrap()
            .1
    }

    fn get_fixed(&self, name: String) -> Column<Fixed> {
        self.fixeds
            .clone()
            .into_iter()
            .filter(|x| x.0 == name)
            .nth(0)
            .unwrap()
            .1
    }

    fn get_instance(&self, name: String) -> Column<Instance> {
        self.instances
            .clone()
            .into_iter()
            .filter(|x| x.0 == name)
            .nth(0)
            .unwrap()
            .1
    }

    fn get_assigned_cell(&self, name: String) -> AssignedCell<F, F> {
        if let Some(acell) = self.acells.iter().filter(|x| x.0 == name).nth(0) {
            return acell.1.clone();
        }
        panic!("cannot find assigned cell of {}", name)
    }

    fn query_column(&self, meta: &mut VirtualCells<F>, cell: crate::system::Cell) -> Expression<F> {
        let column = cell.column;
        match column.ctype {
            crate::system::ColumnType::Selector => meta.query_selector(
                self.selectors
                    .clone()
                    .into_iter()
                    .filter(|x| x.0 == column.name)
                    .nth(0)
                    .unwrap()
                    .1,
            ),
            crate::system::ColumnType::Advice => meta.query_advice(
                self.advices
                    .clone()
                    .into_iter()
                    .filter(|x| x.0 == column.name)
                    .nth(0)
                    .unwrap()
                    .1,
                match cell.index {
                    -1 => Rotation::prev(),
                    0 => Rotation::cur(),
                    1 => Rotation::next(),
                    _ => todo!(),
                },
            ),
            crate::system::ColumnType::Fixed => meta.query_fixed(
                self.fixeds
                    .clone()
                    .into_iter()
                    .filter(|x| x.0 == column.name)
                    .nth(0)
                    .unwrap()
                    .1,
            ),
            crate::system::ColumnType::Instance => todo!(),
        }
    }

    fn convert_to_value(&self, exp: CellExpression) -> Value<F> {
        match exp {
            CellExpression::Constant(c) => Value::known(c.to_field().unwrap()),
            CellExpression::CellValue(c) => match c.column.ctype {
                crate::system::ColumnType::Selector => {
                    self.get_assigned_cell(c.name).value().copied()
                }
                crate::system::ColumnType::Advice => {
                    self.get_assigned_cell(c.name).value().copied()
                }
                crate::system::ColumnType::Fixed => self.get_assigned_cell(c.name).value().copied(),
                crate::system::ColumnType::Instance => todo!(),
            },
            CellExpression::Negated(n) => -self.convert_to_value(*n),
            CellExpression::Product(a, b) => self.convert_to_value(*a) * self.convert_to_value(*b),
            CellExpression::Sum(a, b) => self.convert_to_value(*a) + self.convert_to_value(*b),
            CellExpression::Scaled(a, b) => {
                self.convert_to_value(*a) * self.convert_to_value(CellExpression::Constant(b))
            }
        }
    }
}

fn convert_to_gate_expression<F: PrimeField>(
    meta: &mut VirtualCells<F>,
    config: CommonConfig<F>,
    exp: CellExpression,
) -> Expression<F> {
    match exp {
        CellExpression::Constant(c) => Expression::Constant(c.to_field().unwrap()),
        CellExpression::CellValue(c) => match c.column.ctype {
            crate::system::ColumnType::Selector => config.query_column(meta, c),
            crate::system::ColumnType::Advice => config.query_column(meta, c),
            crate::system::ColumnType::Fixed => config.query_column(meta, c),
            crate::system::ColumnType::Instance => todo!(),
        },
        CellExpression::Negated(n) => -convert_to_gate_expression(meta, config.clone(), *n),
        CellExpression::Product(a, b) => {
            convert_to_gate_expression(meta, config.clone(), *a)
                * convert_to_gate_expression(meta, config.clone(), *b)
        }
        CellExpression::Sum(a, b) => {
            convert_to_gate_expression(meta, config.clone(), *a)
                + convert_to_gate_expression(meta, config.clone(), *b)
        }
        CellExpression::Scaled(a, b) => {
            convert_to_gate_expression(meta, config.clone(), *a) * b.to_field::<F>().unwrap()
        }
    }
}
