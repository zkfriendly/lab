use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Layouter},
    plonk::*,
    poly::Rotation,
};

mod table;
use table::*;

/// This gadget range-constrains an element witnessed in the circuit to be N bits.
///
/// Internally, this gadget uses the `range_check` helper, which provides a K-bit
/// lookup table.
///
/// Given an element `value`, we use a running sum to break it into K-bit chunks.
/// Assume for now that N | K, and define C = N / K.
///
///     value = [b_0, b_1, ..., b_{N-1}]   (little-endian)
///           = c_0 + 2^K * c_1  + 2^{2K} * c_2 + ... + 2^{(C-1)K} * c_{C-1}
///
/// Initialise the running sum at
///                                 value = z_0.
///
/// Consequent terms of the running sum are z_{i+1} = (z_i - c_i) * 2^{-K}:
///
///                           z_1 = (z_0 - c_0) * 2^{-K}
///                           z_2 = (z_1 - c_1) * 2^{-K}
///                              ...
///                       z_{C-1} = c_{C-1}
///                           z_C = (z_{C-1} - c_{C-1}) * 2^{-K}
///                               = 0
///
/// One configuration for this gadget could look like:
///
///     | running_sum |  q_decompose  |  table_value  |
///     -----------------------------------------------
///     |     z_0     |       1       |       0       |
///     |     z_1     |       1       |       1       |
///     |     ...     |      ...      |      ...      |
///     |   z_{C-1}   |       1       |      ...      |
///     |     z_C     |       0       |      ...      |
///
/// Stretch task: use the tagged lookup table to constrain arbitrary bitlengths
/// (even non-multiples of K)

#[derive(Debug, Clone)]
struct DecomposeConfig<F: FieldExt, const LOOKUP_NUM_BITS: usize> {
    running_sum: Column<Advice>,
    q_decompose: Selector,
    table: RangeCheckTable<F, LOOKUP_NUM_BITS>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt, const LOOKUP_NUM_BITS: usize> DecomposeConfig<F, LOOKUP_NUM_BITS> {
    fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let table = RangeCheckTable::configure(meta);
        let q_decompose = meta.complex_selector();
        let running_sum = meta.advice_column();

        // Range-constrain each K-bit chunk `c_i = z_i - z_{i+1} * 2^K` derived from the running sum.
        meta.lookup(|meta| {
            let q_decompose = meta.query_selector(q_decompose);
            let z = meta.query_advice(running_sum, Rotation::cur());
            let next_z = meta.query_advice(running_sum, Rotation::next());

            let mut c = next_z * F::from(1 << LOOKUP_NUM_BITS);
            c = z - c;

            vec![(q_decompose.clone() * c, table.value)]
        });
        Self {
            running_sum,
            q_decompose,
            table,
            _marker: PhantomData,
        }
    }

    fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        value: AssignedCell<Assigned<F>, F>,
        num_bits: usize,
    ) -> Result<(), Error> {
        let mut z = value.clone();

        layouter.assign_region(
            || "assign z",
            |mut r| {
                let mut offset = 0;
                let value = value.copy_advice(|| "z0", &mut r, self.running_sum, offset);

                offset = offset + 1;

                let value = z.value().unwrap().evaluate().to_le_bytes();

                // let z = (z.value().unwrap() - )

                value
            },
        )?;
        // 0. Copy in the witnessed `value`
        // 1. Compute the interstitial running sum values {z_0, ..., z_C}}
        // 2. Assign the running sum values
        // 3. Make sure to enable the relevant selector on each row of the running sum
        // 4. Constrain the final running sum `z_C` to be 0.
        todo!()
    }
}
