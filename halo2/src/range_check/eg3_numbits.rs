// this is sopposed to check if a given value is less than a given range
// uses lookup table for large ranges

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Layouter,
    plonk::{Advice, Column, ConstraintSystem, Error, Selector},
    poly::Rotation,
};

use self::table::RangeCheckTable;

#[derive(Clone)]
struct RangeCheckConfig<F: FieldExt, const RANGE: usize> {
    value: Column<Advice>,
    num_bits: Column<Advice>,
    selector: Selector,
    table: RangeCheckTable<F, 3>,
    _marker: std::marker::PhantomData<F>,
}

mod table;

impl<F: FieldExt, const RANGE: usize> RangeCheckConfig<F, RANGE> {
    fn configure(
        meta: &mut ConstraintSystem<F>,
        value: Column<Advice>,
        num_bits: Column<Advice>,
    ) -> Self {
        let selector = meta.complex_selector();
        let table: RangeCheckTable<F, 3> = RangeCheckTable::configure(meta);

        meta.lookup(|meta| {
            let q_lookup = meta.query_selector(selector);
            let value = meta.query_advice(value, Rotation::cur());
            let num_bits = meta.query_advice(num_bits, Rotation::cur());

            vec![
                (q_lookup.clone() * value, table.value),
                (q_lookup * num_bits, table.num_bits),
            ]
        });

        Self {
            value,
            num_bits,
            selector,
            table,
            _marker: std::marker::PhantomData,
        }
    }

    fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        value: F,
        num_bits: usize,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "assign value",
            |mut r| {
                self.selector.enable(&mut r, 0)?;
                r.assign_advice(|| "assign value", self.value, 0, || Ok(value))?;
                r.assign_advice(
                    || "assign num bits",
                    self.num_bits,
                    0,
                    || Ok(F::from(num_bits as u64)),
                )?;
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
        value: F,
        num_bits: usize,
    }

    impl<F: FieldExt, const RANGE: usize> Circuit<F> for RangeCheckCircuit<F, RANGE> {
        type Config = RangeCheckConfig<F, RANGE>;
        type FloorPlanner = SimpleFloorPlanner;

        fn without_witnesses(&self) -> Self {
            Self::default()
        }

        fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
            let range_check_advice = meta.advice_column();
            let num_bits = meta.advice_column();
            RangeCheckConfig::configure(meta, range_check_advice, num_bits)
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<F>,
        ) -> Result<(), Error> {
            config
                .table
                .load(layouter.namespace(|| "range check syn"))?;
            config.assign(
                layouter.namespace(|| "range check syn"),
                self.value,
                self.num_bits,
            )?;
            Ok(())
        }
    }

    fn log2(x: u64) -> u64 {
        // implement log 2 of x
        // https://stackoverflow.com/questions/11376288/fast-computing-of-log2-for-64-bit-integers
        let mut x = x;
        let mut result = 0;

        while x > 1 {
            x >>= 1;
            result += 1;
        }

        result
    }

    #[test]
    fn range_check_test() {
        const RANGE: usize = 8;

        for i in 0..RANGE {
            let circuit = RangeCheckCircuit::<Fp, RANGE> {
                value: Fp::from(i as u64),
                num_bits: log2(i as u64) as usize,
            };
            let prover = MockProver::run(10, &circuit, vec![]).unwrap();
            prover.assert_satisfied()
        }
    }
}
