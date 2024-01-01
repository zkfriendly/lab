use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{layouter, Layouter},
    plonk::{ConstraintSystem, Error, TableColumn},
};

/// a lookup table of values of NUM_BITS length
/// num_bits = 3 => 8 values

#[derive(Clone)]
pub(super) struct RangeCheckTable<F: FieldExt, const NUM_BITS: usize> {
    pub(super) value: TableColumn,
    _marker: std::marker::PhantomData<F>,
}

impl<F: FieldExt, const NUM_BITS: usize> RangeCheckTable<F, NUM_BITS> {
    pub(super) fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let value = meta.lookup_table_column();

        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }

    pub(super) fn load(&self, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_table(
            || "range table",
            |mut table| {
                let mut offset = 0;
                for i in 0..(1 << NUM_BITS) {
                    table.assign_cell(|| "assign cell", self.value, offset, || Ok(F::from(i)))?;
                    offset = offset + 1;
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}
