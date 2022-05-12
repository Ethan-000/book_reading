
use rstest::rstest;
use freivalds_algorithm::Freivalds;
use ark_bls12_381::Fq as F;
use ark_std::UniformRand;
use ndarray::{arr2, Array2};


#[rstest]
fn test_freivalds() {
    let mut rng = ark_std::rand::thread_rng();
    let a: Array2<F> = arr2(&[[F::rand(&mut rng), F::rand(&mut rng)], [F::rand(&mut rng), F::rand(&mut rng)]]);
    let b: Array2<F> = arr2(&[[F::rand(&mut rng), F::rand(&mut rng)], [F::rand(&mut rng), F::rand(&mut rng)]]);
    let frv = Freivalds::set_up(a, b);
    let c = frv.prove();
    let verified = frv.verify(c);
    assert!(verified);
}