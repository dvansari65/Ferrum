// Hello-world halo2 circuit.
// Proves: "I know a private value x such that x + 5 = y", where y is public.
// No hashing, no tree — just enough to learn the halo2 API shape.

use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, SimpleFloorPlanner, Value},
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, Error, Fixed, Instance, Selector,
    },
    poly::Rotation,
};
use ff::PrimeField;

// ---------------------------------------------------------
// 1. CONFIG — describes the "shape" of the circuit: which
//    columns exist and what gate (constraint) ties them together.
// ---------------------------------------------------------
#[derive(Clone)]
struct AddConfig {
    x: Column<Advice>,      // private input column (holds x)
    y: Column<Advice>,      // private "working" column (holds x + 5)
    constant: Column<Fixed>, // the fixed constant 5, baked into the circuit
    instance: Column<Instance>, // public input column (holds the public y)
    selector: Selector,     // turns the add-gate on/off for a given row
}

// ---------------------------------------------------------
// 2. CHIP — the reusable piece of logic that knows how to
//    configure and use the AddConfig.
// ---------------------------------------------------------
struct AddChip<F: PrimeField> {
    config: AddConfig,
    _marker: std::marker::PhantomData<F>,
}

impl<F: PrimeField> Chip<F> for AddChip<F> {
    type Config = AddConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }
    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: PrimeField> AddChip<F> {
    fn construct(config: AddConfig) -> Self {
        Self { config, _marker: std::marker::PhantomData }
    }

    // This is where the actual RULE lives:
    // "x (times the selector) plus the fixed constant equals y"
    fn configure(meta: &mut ConstraintSystem<F>) -> AddConfig {
        let x = meta.advice_column();
        let y = meta.advice_column();
        let constant = meta.fixed_column();
        let instance = meta.instance_column();
        let selector = meta.selector();

        meta.enable_equality(y);
        meta.enable_equality(instance);
        meta.enable_constant(constant);

        // The custom gate: whenever `selector` is on for a row,
        // enforce  x + constant - y == 0   (i.e. x + constant = y)
        meta.create_gate("add", |meta| {
            let s = meta.query_selector(selector);
            let x = meta.query_advice(x, Rotation::cur());
            let y = meta.query_advice(y, Rotation::cur());
            let c = meta.query_fixed(constant, Rotation::cur());

            vec![s * (x + c - y)]
        });

        AddConfig { x, y, constant, instance, selector }
    }

    // This is where the actual VALUES get filled in for one proof.
    fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        x_val: Value<F>,
        const_val: F,
    ) -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(
            || "add row",
            |mut region| {
                self.config.selector.enable(&mut region, 0)?;

                region.assign_advice(|| "x", self.config.x, 0, || x_val)?;

                region.assign_fixed(
                    || "constant",
                    self.config.constant,
                    0,
                    || Value::known(const_val),
                )?;

                let y_val = x_val.map(|x| x + const_val);
                region.assign_advice(|| "y", self.config.y, 0, || y_val)
            },
        )
    }
}

// ---------------------------------------------------------
// 3. THE CIRCUIT ITSELF — plugs your specific private value
//    into the chip, and constrains the output to match the
//    public instance value.
// ---------------------------------------------------------
#[derive(Default)]
struct MyCircuit<F: PrimeField> {
    x: Value<F>, // the PRIVATE witness value
}

impl<F: PrimeField> Circuit<F> for MyCircuit<F> {
    type Config = AddConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        AddChip::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = AddChip::construct(config.clone());

        // constant = 5
        let five = F::from(5u64);

        let y_cell = chip.assign(layouter.namespace(|| "add"), self.x, five)?;

        // Tie the circuit's computed y to the PUBLIC instance column,
        // at row 0. This is the "public y" the verifier checks against.
        layouter.constrain_instance(y_cell.cell(), config.instance, 0)
    }
}

// ---------------------------------------------------------
// 4. USAGE SKETCH (not runnable as-is — depends on your
//    exact halo2_proofs version's prove/verify API):
// ---------------------------------------------------------
//
// let k = 4; // circuit size parameter, small for this toy circuit
// let x = Fp::from(7u64);       // private witness
// let y_public = Fp::from(12u64); // 7 + 5 = 12, the public claim
//
// let circuit = MyCircuit { x: Value::known(x) };
// let public_inputs = vec![y_public];
//
// // MockProver is the easiest way to test correctness first,
// // before dealing with real prove/verify + setup params:
// let prover = halo2_proofs::dev::MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
// assert_eq!(prover.verify(), Ok(()));