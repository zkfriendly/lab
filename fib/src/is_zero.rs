use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, ConstraintSystem, Expression, VirtualCells},
};

pub struct IsZeroConfig<F: FieldExt> {
    pub value_inv: Column<Advice>,
    pub is_zero_expr: Expression<F>, // it's zero if it's not zero :)
}

pub struct IsZeroChip<F: FieldExt> {
    config: IsZeroConfig<F>,
    _marker: std::marker::PhantomData<F>,
}

impl<F: FieldExt> IsZeroChip<F> {
    pub fn construct(config: IsZeroConfig<F>) -> Self {
        Self {
            config,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        zero_check: impl FnOnce(&mut VirtualCells<'_, F>) -> Expression<F>,
        value: impl FnOnce(&mut VirtualCells<'_, F>) -> Expression<F>,
        value_inv: Column<Advice>,
    ) -> IsZeroConfig<F> {
        let is_zero_expr = Expression::Constant(F::zero());

        meta.create_gate("is zero check", |meta| {
            /// This is the expression that we want to be zero
            ///    value   |  value_inv   |   1 - value * value_inv | value * (1 - value * value_inv)
            ///      x     |      1/x     |             0           |           0
            ///      x     |      0       |             1           |           x
            ///      0     |      1/x     |             1           |           0
            let zero_check = zero_check(meta);
            let value = value(meta);
        });

        config
    }
}
