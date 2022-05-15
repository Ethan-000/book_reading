

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
        for w in (0..v).map(|_| [F::zero(), F::one()]).multi_cartesian_product(){
            w_vec.push(w);
        }
        MultiExtension {
            evaluations,
            v,
            w_vec,
        }
    }

    // modified from arkworks
    pub fn rand<R: Rng>(num_vars: usize, rng: &mut R) -> Self {
        let mut w_vec: Vec<Vec<F>> = vec![];
        for w in (0..num_vars).map(|_| [F::zero(), F::one()]).multi_cartesian_product(){
            w_vec.push(w);
        }
        Self{
            v: num_vars,
            evaluations: (0..(1 << num_vars)).map(|_| F::rand(rng)).collect(),
            w_vec,
        }
    }

    // based on Lemma 3.7
    // also inspired by https://github.com/maxgillett/thaler_reading_group/blob/master/week2-lagrange/src/main.rs
    pub fn evaluate(&self, rs: &[F]) -> F {
        let ans = self.evaluations.to_vec();
        let w_vec = self.w_vec.clone();
        let chi_eval: Vec<F> = w_vec.iter().map(|w| {
            w.into_iter()
                .zip(rs)
                .map(|(w_i, x_i)| if *w_i == F::one() { *x_i } else { F::one() - *x_i })
                .product::<F>()
        }).collect();
        ans
            .iter()
            .zip(chi_eval)
            .map(|(a, b)| *a * b)
            .sum::<F>()
    }

    // a recursive method modified from https://github.com/0xSage/thaler/blob/main/src/lagrange.rs
    pub fn evaluate_rec(&self, rs: &[F]) -> F {
        let ans = self.evaluations.to_vec();
        let length = ans.len();
        self.evaluate_rec_helper(&ans, &self.w_vec.clone(), rs, length)
    }

    fn evaluate_rec_helper(&self, ans: &Vec<F>, w_vec: &Vec<Vec<F>>, rs: &[F], length: usize) -> F {
        match length {
            0 => F::zero(),
            _ => self.evaluate_rec_helper(&ans, &w_vec, rs, length - 1) +
                ans[length - 1] * {
                    w_vec[length - 1]
                        .iter()
                        .zip(rs)
                        .map(|(&w, &r)| if w == F::one() { r } else { F::one() - r })
                        .product::<F>()
                },       
        }
    }

    // based on Lemma 3.8
    // also inspired by impl of arkworks
    pub fn evaluate_dp(&self, rs: &[F]) -> F {
        let mut ans = self.evaluations.to_vec();
        let length = rs.len();

        for i in 0..length {
            let r = rs[i];
            for j in 0..(1 << length - i - 1) {
                ans[j] = ans[j << 1] * (F::one() - r) + ans[(j << 1) + 1] * r
            }
        }

        ans[0]
    }

    // a dp method modified from https://github.com/0xSage/thaler/blob/main/src/lagrange.rs
    pub fn evaluate_dp_2(&self, rs: &[F]) -> F {
        let ans = self.evaluations.to_vec();

        let chi_eval = self.memoize(&rs.to_vec(), rs.len());
        ans
            .iter()
            .zip(chi_eval)
            .map(|(a, b)| *a * b)
            .sum::<F>()
    }

    fn memoize(&self, rs: &Vec<F>, v: usize) -> Vec<F> {
        match v {
            1 => {
                vec![ F::one() - rs[v - 1], rs[v - 1] ]
            }
            _ => self.memoize(rs, v - 1)
                .iter()
                .flat_map(|val| {
                    [
                        *val * (F::one() - rs[v - 1]),
                        *val * rs[v - 1],
                    ]
                })
                .collect(),
        }
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
        let poly = MultiExtension::rand(1, &mut rng);
        for _ in 0..1 {
            let point: Vec<_> = (0..1).map(|_| Fr::rand(&mut rng)).collect();
            let v1 = evaluate_data_array(&poly.evaluations, &point);
            let v2 = poly.evaluate(&point);
            let v3 = poly.evaluate_rec(&point);
            let v4 = poly.evaluate_dp(&point);
            let v5 = poly.evaluate_dp_2(&point);

            assert_eq!(v1, v2);
            assert_eq!(v1, v3);
            assert_eq!(v1, v4);
            assert_eq!(v1, v5);
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