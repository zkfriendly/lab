use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, ConstraintSystem, Expression},
};

pub struct IsZeroConfig<F: FieldExt> {
    pub value_inv: Column<Advice>,
    pub is_zero_expr: Expression<F>,
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
}
