

use ark_bls12_381::Fq as F;
use ark_std::UniformRand;
use ndarray::{Array1, Array2};
pub struct Freivalds {
    pub a: Array2<F>,
    pub b: Array2<F>,
    pub x: Array1<F>,
}

impl Freivalds {

    pub fn set_up(a: Array2<F>, b: Array2<F>) -> Self {
        let mut rng = ark_std::rand::thread_rng();
        let r = F::rand(&mut rng);
        let n = a.shape()[0];
        let mut x = Array1::from(vec![r; n]);
        for i in 1..n {
            let mut tmp = x[i-1].clone();
            tmp = tmp * &r;
            x[i] = tmp;
        }
        Freivalds {
            a,
            b,
            x
        }
    }

    pub fn prove(&self) -> Array2<F> {
        let c = self.a.dot(&self.b);
        c
    }

    pub fn verify(&self, c: Array2<F>) -> bool {
        let x = self.x.clone();
        let y = c.dot(&x);
        let bx = self.b.dot(&x);
        let z = self.a.dot(&bx);
        y == z
    }
}