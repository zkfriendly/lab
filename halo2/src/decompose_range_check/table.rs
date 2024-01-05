use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::Layouter,
    plonk::{ConstraintSystem, Error, TableColumn},
};

/// a lookup table of values of NUM_BITS length
/// num_bits = 3 => 8 values

#[derive(Clone, Debug)]
pub(super) struct RangeCheckTable<F: FieldExt, const NUM_BITS: usize> {
    pub(super) value: TableColumn,
    pub(super) num_bits: TableColumn,
    _marker: std::marker::PhantomData<F>,
}

impl<F: FieldExt, const NUM_BITS: usize> RangeCheckTable<F, NUM_BITS> {
    pub(super) fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let value = meta.lookup_table_column();
        let num_bits = meta.lookup_table_column();

        Self {
            value,
            num_bits,
            _marker: std::marker::PhantomData,
        }
    }

    fn log2(&self, x: u64) -> u64 {
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

    pub(super) fn load(&self, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_table(
            || "range table",
            |mut table| {
                let mut offset = 0;
                for i in 0..(1 << NUM_BITS) {
                    // 0,0 is a bug
                    table.assign_cell(|| "assign cell", self.value, offset, || Ok(F::from(i)))?;

                    let num_bits = self.log2(i);

                    table.assign_cell(
                        || "numbits table",
                        self.num_bits,
                        offset,
                        || Ok(F::from(num_bits)),
                    )?;
                    offset = offset + 1;
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}
