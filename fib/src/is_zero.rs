use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Layouter, Region, SimpleFloorPlanner},
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, Error, Expression, Instance, Selector,
        VirtualCells,
    },
    poly::Rotation,
};

#[derive(Clone, Debug)]
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

        meta.create_gate("is zero check gate", |meta| {
            // This is the expression that we want to be zero
            //    value   |  value_inv   |   1 - value * value_inv | value * (1 - value * value_inv)
            //      x     |      1/x     |             0           |           0
            //      x     |      0       |             1           |           x
            //      0     |      0       |             1           |           0
            //      0     |      y       |             1           |           0
            let zero_check = zero_check(meta);
            let value = value(meta);
            let value_inv = meta.query_advice(value_inv, Rotation::cur());

            let is_zero_expr = Expression::Constant(F::one()) - (value.clone() * value_inv);
            vec![(zero_check * value * is_zero_expr)]
        });

        IsZeroConfig {
            value_inv,
            is_zero_expr,
        }
    }

    pub fn assign(&self, r: &mut Region<'_, F>, value: F) -> Result<(), Error> {
        let value_inv = value.invert().unwrap_or(F::zero());
        r.assign_advice(
            || "value inverse",
            self.config.value_inv,
            0,
            || Ok(value_inv),
        )?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct FunctionConfig<F: FieldExt> {
    pub a: Column<Advice>,
    pub b: Column<Advice>,
    pub c: Column<Advice>,
    pub out: Column<Advice>,
    pub selector: Selector,
    pub a_is_zero_config: IsZeroConfig<F>,
}

pub struct FunctionChip<F: FieldExt> {
    pub config: FunctionConfig<F>,
}

impl<F: FieldExt> FunctionChip<F> {
    fn construct(config: FunctionConfig<F>) -> Self {
        Self { config }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> FunctionConfig<F> {
        let a = meta.advice_column();
        let b = meta.advice_column();
        let c = meta.advice_column();
        let out = meta.advice_column();
        let selector = meta.selector();

        meta.enable_equality(a);
        meta.enable_equality(b);
        meta.enable_equality(c);
        meta.enable_equality(out);

        let zero_check_advice = meta.advice_column();

        let a_is_zero_config = IsZeroChip::configure(
            meta,
            |meta| meta.query_selector(selector),
            |meta| meta.query_advice(a, Rotation::cur()),
            zero_check_advice,
        );

        meta.create_gate("if a == 0 { b } else { c }", |r| {
            let selector = r.query_selector(selector);

            let b = r.query_advice(b, Rotation::cur());
            let c = r.query_advice(c, Rotation::cur());
            let output = r.query_advice(out, Rotation::cur());

            let a_is_zero = a_is_zero_config.is_zero_expr.clone();

            vec![
                selector.clone() * a_is_zero.clone() * (c - output.clone()),
                selector * (Expression::Constant(F::one()) - a_is_zero) * (b - output),
            ]
        });

        FunctionConfig {
            a,
            b,
            c,
            out,
            selector,
            a_is_zero_config,
        }
    }

    pub fn assign(&self, mut layouter: impl Layouter<F>, a: F, b: F, c: F) -> Result<(), Error> {
        let a_is_zero_chip = IsZeroChip::construct(self.config.a_is_zero_config.clone());
        layouter.assign_region(
            || "function regions",
            |mut r| {
                self.config.selector.enable(&mut r, 0)?;
                a_is_zero_chip.assign(&mut r, a)?;
                r.assign_advice(|| "a", self.config.a, 0, || Ok(a))?;
                r.assign_advice(|| "b", self.config.b, 0, || Ok(b))?;
                r.assign_advice(|| "c", self.config.c, 0, || Ok(c))?;

                // calculate out here:
                let out = match a == F::zero() {
                    true => b,
                    false => c,
                };

                r.assign_advice(|| "out", self.config.out, 0, || Ok(out))?;

                Ok(())
            },
        )?;

        Ok(())
    }
}

#[derive(Default)]
pub struct FunctionCircuit<F: FieldExt> {
    pub a: F,
    pub b: F,
    pub c: F,
}

impl<F: FieldExt> Circuit<F> for FunctionCircuit<F> {
    type Config = FunctionConfig<F>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> FunctionConfig<F> {
        FunctionChip::configure(meta)
    }

    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<F>) -> Result<(), Error> {
        let chip = FunctionChip::construct(config);
        chip.assign(layouter, self.a, self.b, self.c)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use halo2_proofs::{dev::MockProver, pasta::Fp};

    use super::*;

    #[test]
    fn test_is_zero() {
        let c = FunctionCircuit {
            a: Fp::from(0),
            b: Fp::from(1),
            c: Fp::from(2),
        };

        let prover = MockProver::run(5, &c, vec![]).unwrap();

        prover.assert_satisfied();
    }
}
