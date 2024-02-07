use std::marker::PhantomData;

use halo2_proofs::circuit::{Chip, Layouter, SimpleFloorPlanner};
use halo2_proofs::plonk::{Advice, Column, ConstraintSystem, Error, Fixed, Instance, Selector};
use halo2_proofs::{arithmetic::Field, circuit::Value, dev::MockProver, pasta::Fp, plonk::Circuit};

trait ChipInstructions<F: Field> {
    type Num;

    fn load_private(&self, layouter: impl Layouter<F>, value: Value<F>)
        -> Result<Self::Num, Error>;

    fn load_constant(
        &self,
        layouter: impl Layouter<F>,
        constant: F,
    ) -> Result<Self::Num, Error>;

    fn mul(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error>;

    fn expose_public(
        &self,
        layouter: impl Layouter<F>,
        value: Self::Num,
        row: usize, // what is this
    ) -> Result<(), Error>;
}

#[derive(Clone, Debug)]
struct FieldConfig {
    advice: [Column<Advice>; 2],
    instance: Column<Instance>,
    s_mul: Selector // I don't know what this is yet
}

struct FieldChip<F: Field> {
    config: FieldConfig,
    _marker: PhantomData<F>,
}
impl<F: Field> Chip<F> for FieldChip<F> {
    type Config = FieldConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: Field> FieldChip<F> {
    fn construct(config: <Self as Chip<F>>::Config) -> Self {
        Self {
            config,
            _marker: PhantomData
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>, advice: [Column<Advice>; 2], instance: Column<Instance>, constant: Column<Fixed>) -> <Self as Chip<F>>::Config {
        meta.enable_equality(instance);
        meta.enable_constant(constant);

        for col in &advice {
            meta.enable_equality(*col);
        }

        let s_mul = meta.selector();

        
    }
}

#[derive(Default, Debug)]
struct MyCircuit<F: Field> {
    constant: F,
    a: Value<F>,
    b: Value<F>,
}

impl<F: Field> Circuit<F> for MyCircuit<F> {
    type Config = FieldConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        todo!()
    }

    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<F>) -> Result<(), Error> {
        todo!()
    }
}

fn verify_correct_claim() {
    let k = 4; // 2^k total number of rows in the matrix

    // prepare private/public inputs
    let constant = Fp::from(7);
    let a = Fp::from(2);
    let b = Fp::from(3);
    let c: Fp = constant * a.square() * b.square(); // this is the formula we want to prove
    let mut public_inputs = vec![c];

    // create a circuit
    let circuit = MyCircuit {
        constant,
        a: Value::known(a),
        b: Value::known(b),
    };

    // create a proof
    let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).expect("PROVER_FAILED");

    // should pass
    assert_eq!(prover.verify(), Ok(()));
}

fn main() {
    println!("Running {:?}", <MyCircuit<Fp>>::default());
    verify_correct_claim();
}
