

use ark_ff::Field;


use ark_std::marker::PhantomData;

pub mod prover;
pub mod verifier;


pub struct IPForSumCheck<F: Field> {
    #[doc(hidden)]
    _marker: PhantomData<F>,
}