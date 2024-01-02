// this is sopposed to check if a given value is less than a given range

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Layouter,
    plonk::{Advice, Column, ConstraintSystem, Constraints, Error, Expression, Selector},
    poly::Rotation,
};

#[derive(Clone)]
struct RangeCheckConfig<F: FieldExt, const RANGE: usize> {
    value: Column<Advice>,
    selector: Selector,
    _marker: std::marker::PhantomData<F>,
}

impl<F: FieldExt, const RANGE: usize> RangeCheckConfig<F, RANGE> {
    fn configure(meta: &mut ConstraintSystem<F>, value: Column<Advice>) -> Self {
        let selector = meta.selector();

        meta.create_gate("Range Check", |meta| {
            let value = meta.query_advice(value, Rotation::cur());
            let q_range_check = meta.query_selector(selector);
            let range_check = |value: Expression<F>| {
                (0..RANGE).into_iter().fold(value.clone(), |acc, el| {
                    acc * (value.clone() - Expression::Constant(F::from(el as u64)))
                })
            };
            Constraints::with_selector(q_range_check, [("range check", range_check(value))])
        });

        Self {
            value,
            selector,
            _marker: std::marker::PhantomData,
        }
    }

    fn assign(&self, mut layouter: impl Layouter<F>, value: F) -> Result<(), Error> {
        layouter.assign_region(
            || "assign value",
            |mut r| {
                self.selector.enable(&mut r, 0)?;
                r.assign_advice(|| "assign value", self.value, 0, || Ok(value))?;
                Ok(())
            },
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use halo2_proofs::{circuit::SimpleFloorPlanner, dev::MockProver, pasta::Fp, plonk::Circuit};

    use super::*;

    #[derive(Default)]
    struct RangeCheckCircuit<F: FieldExt, const RANGE: usize> {
        value: Option<F>,
    }

    impl<F: FieldExt, const RANGE: usize> Circuit<F> for RangeCheckCircuit<F, RANGE> {
        type Config = RangeCheckConfig<F, RANGE>;
        type FloorPlanner = SimpleFloorPlanner;

        fn without_witnesses(&self) -> Self {
            Self::default()
        }

        fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
            let range_check_advice = meta.advice_column();
            RangeCheckConfig::configure(meta, range_check_advice)
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<F>,
        ) -> Result<(), Error> {
            config.assign(
                layouter.namespace(|| "range check syn"),
                self.value.unwrap(),
            )?;
            Ok(())
        }
    }

    #[test]
    fn range_check_test() {
        const RANGE: usize = 8;

        for i in 0..RANGE {
            let circuit = RangeCheckCircuit::<Fp, RANGE> {
                value: Some(Fp::from(i as u64)),
            };
            let prover = MockProver::run(4, &circuit, vec![]).unwrap();
            prover.assert_satisfied()
        }
    }
}
