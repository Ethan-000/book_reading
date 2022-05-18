
use crate::sumcheck::protocol::IPForSumCheck;
use crate::sumcheck::protocol::verifier::VerifierMsg;
use crate::sumcheck::polynomial_rep::PolynomialRep;
use ark_ff::Field;
use ark_poly::{DenseMultilinearExtension, MultilinearExtension};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Read, SerializationError, Write};



#[derive(Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct ProverMsg<F: Field> {
    pub evaluations: Vec<F>,
}

pub struct ProverState<F: Field> {
    random_points: Vec<F>,
    round: usize,
    pub max_num_multiplicands: usize,
    pub num_of_variables: usize,
    // the first field store the coefficient of the product of
    // polynomials that is specifed by the second field
    pub coeff_and_index: Vec<(F, Vec<usize>)>,
    pub poly_extensions: Vec<DenseMultilinearExtension<F>>,
}



impl<F: Field> IPForSumCheck<F>{
    pub fn prover_init(
        polynomial_rep: &PolynomialRep<F>,
    ) -> ProverState<F> {

        

        let poly_extension = polynomial_rep
                        .poly_extensions
                        .iter()
                        .map(|extension| extension.clone())
                        .collect();

        ProverState {
            random_points: Vec::with_capacity(polynomial_rep.num_of_variables),
            round: 0,
            max_num_multiplicands: polynomial_rep.max_num_multiplicands,
            num_of_variables: polynomial_rep.num_of_variables,
            coeff_and_index: polynomial_rep.coeff_and_index.clone(),
            poly_extensions: poly_extension,
        }
    }

    pub fn prove_round(
        mut prover_state: ProverState<F>,
        v_msg: &Option<VerifierMsg<F>>,
    ) -> (ProverMsg<F>, ProverState<F>) {


        if let Some(msg) = v_msg {
            if prover_state.round == 0 {
                panic!("first round should be prover first.");
            }
            prover_state.random_points.push(msg.rand_point);

            // fix argument
            let i = prover_state.round;
            let r = prover_state.random_points[i - 1];
            for multiplicand in prover_state.poly_extensions.iter_mut() {
                *multiplicand = multiplicand.fix_variables(&[r]);
            }
        } else {
            if prover_state.round > 0 {
                panic!("verifier message is empty");
            }
        }

        prover_state.round += 1;

        let round = prover_state.round;
        let num_variables = prover_state.num_of_variables;
        let degree = prover_state.max_num_multiplicands; 

       

        let mut evaluation_sum = Vec::with_capacity(degree + 1);
        evaluation_sum.resize(degree + 1, F::zero());
        
        // from https://eprint.iacr.org/2019/317.pdf algorithm3
        for i in 0..1 << (num_variables - round) {
            let mut t_f = F::zero();
            for t in 0..degree + 1 {
                // evaluate P_round(t)
                for (coefficient, indexes) in &prover_state.coeff_and_index {
                    let num_multiplicands = indexes.len();
                    let mut product = *coefficient;
                    for j in 0..num_multiplicands {
                        let table = &prover_state.poly_extensions[indexes[j]]; // j's range is checked in init
                        product *= table[i << 1] * (F::one() - t_f)
                            + table[(i << 1) + 1] * t_f;
                    }
                    evaluation_sum[t] += product;
                }
                t_f += F::one();
            }
        }

        (
            ProverMsg {
                evaluations: evaluation_sum,
            },
            prover_state,
        )

    }
}