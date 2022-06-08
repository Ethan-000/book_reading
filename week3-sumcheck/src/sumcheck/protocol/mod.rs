

use ark_ff::PrimeField;
use ark_std::marker::PhantomData;

pub mod prover;
pub mod verifier;


pub struct IPForSumCheck<F: PrimeField> {
    #[doc(hidden)]
    _marker: PhantomData<F>,
}