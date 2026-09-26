# ZK Cryptography Library

A low-level cryptography library written in Rust, implementing the mathematical and cryptographic building blocks behind zero-knowledge proof systems from scratch.

The project starts with finite-field arithmetic and will progressively build toward a complete zkSNARK proving and verification system.

## Roadmap

```text
Finite Field Arithmetic
        ↓
Polynomial Arithmetic
        ↓
Elliptic Curve Operations
        ↓
Commitment Schemes
        ↓
Constraint System / R1CS
        ↓
Witness Generation
        ↓
Prover
        ↓
Verifier
        ↓
zkSNARK
```

## Current Focus

### Finite Field Arithmetic

The first component is a finite-field implementation.

It will provide:

* Addition
* Subtraction
* Multiplication
* Division
* Modular inverse
* Exponentiation
* Equality
* Serialization

Example:

```rust
let a = FieldElement::new(5);
let b = FieldElement::new(4);

let result = a * b;
```

All operations are performed within the selected finite field.

## Planned Components

### Polynomial Arithmetic

Operations over polynomials represented using field elements:

* Addition
* Subtraction
* Multiplication
* Evaluation
* Division
* Interpolation

### Elliptic Curves

Implementation of:

* Curve points
* Point addition
* Point doubling
* Scalar multiplication

### Commitment Schemes

Cryptographic commitments that allow a prover to commit to a value while keeping the value hidden.

```text
value + randomness
        ↓
   commitment
```

### Constraint System

Represent computations as mathematical constraints.

For example:

```text
x² = 25
```

can be represented as:

```text
x * x = 25
```

The private value `x` forms part of the witness, while `25` can be a public input.

### Prover & Verifier

The eventual goal is:

```text
Private Input ─┐
               ├──→ Prover ──→ Proof
Public Input ──┘                  │
                                  ↓
                             Verifier
                                  │
                                  ↓
                             True / False
```

The verifier should be able to verify that the computation was performed correctly without receiving the private witness.

## Development

This project is being built incrementally, with each mathematical layer tested before moving to the next one.

```text
[ ] Finite Field
[ ] Polynomial
[ ] Elliptic Curve
[ ] Commitment Scheme
[ ] R1CS
[ ] Witness Generation
[ ] Prover
[ ] Verifier
[ ] zkSNARK
```

## Goals

* Implement the core mathematics from scratch
* Understand the internals of zero-knowledge proof systems
* Write the implementation in idiomatic Rust
* Keep cryptographic components modular
* Maintain extensive unit and property-based tests
* Eventually build a working zkSNARK system

## Warning

This is an educational/research implementation and is **not intended for production cryptographic use**.
