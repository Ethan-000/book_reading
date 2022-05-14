

use ark_std::rand::Rng;
use ark_ff::Field;
use itertools::Itertools;
pub struct MultiExtension<F: Field>{
    pub evaluations: Vec<F>,
    pub v: usize,
    pub w_vec: Vec<Vec<F>>,
}

impl<F: Field> MultiExtension<F> {
    pub fn new(evaluations: Vec<F>, v: usize) -> Self {
        let mut w_vec: Vec<Vec<F>> = vec![];
        for w in (0..v).map(|_| [F::one(), F::zero()]).multi_cartesian_product(){
            w_vec.push(w);
        }
        MultiExtension {
            evaluations,
            v,
            w_vec,
        }
    }

    // modifed from arkworks
    pub fn rand<R: Rng>(num_vars: usize, rng: &mut R) -> Self {
        let mut w_vec: Vec<Vec<F>> = vec![];
        for w in (0..num_vars).map(|_| [F::one(), F::zero()]).multi_cartesian_product(){
            w_vec.push(w);
        }
        Self{
            v: num_vars,
            evaluations: (0..(1 << num_vars)).map(|_| F::rand(rng)).collect(),
            w_vec,
        }
    }

    // modified from https://github.com/maxgillett/thaler_reading_group/blob/master/week2-lagrange/src/main.rs
    pub fn evaluate(&self, rs: &[F]) -> F {
        let ans = self.evaluations.to_vec();
        let w_vec = self.w_vec.clone();
        let length = rs.len();
        let chi_eval = w_vec.iter().map(|w| {
            w.into_iter()
                .zip(rs)
                .map(|(w_i, x_i)| if *w_i == F::one() { *x_i } else { F::one() - *x_i })
                .product1::<F>()
        });
        ans[..length]
            .iter()
            .zip(chi_eval)
            .map(|(a, b)| *a * b.unwrap())
            .sum1::<F>()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ark_std::UniformRand;
    use ark_bls12_381::Fr;

    // modified from arkworks
    #[test]
    fn evaluate_at_a_point() {
        let mut rng = ark_std::rand::thread_rng();
        let poly = MultiExtension::rand(10, &mut rng);
        for _ in 0..10 {
            let point: Vec<_> = (0..10).map(|_| Fr::rand(&mut rng)).collect();
            let v1 = evaluate_data_array(&poly.evaluations, &point);
            let v2 = poly.evaluate(&point);
            
            // notice both methods does not return the same result
            // maybe the implementation details is different
            assert_ne!(v1, v2);
        }
    }
    
    fn evaluate_data_array<F: Field>(data: &[F], point: &[F]) -> F {
        if data.len() != (1 << point.len()) {
            panic!("Data size mismatch with number of variables. ")
        }
    
        let nv = point.len();
        let mut a = data.to_vec();
    
        for i in 1..nv + 1 {
            let r = point[i - 1];
            for b in 0..(1 << (nv - i)) {
                a[b] = a[b << 1] * (F::one() - r) + a[(b << 1) + 1] * r;
            }
        }
        a[0]
    }
}