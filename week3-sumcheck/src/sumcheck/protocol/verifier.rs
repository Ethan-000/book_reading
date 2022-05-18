
use ark_ff::Field;
use crate::sumcheck::protocol::prover::ProverMsg;
use crate::sumcheck::protocol::IPForSumCheck;
use crate::sumcheck::polynomial_rep::PolyInfoForVerifier;
use ark_std::rand::RngCore;
use ark_std::vec::Vec;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Read, SerializationError, Write};

#[derive(Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct VerifierMsg<F: Field> {
    pub rand_point: F,
}

pub struct VerifierState<F: Field> {
    random_points: Vec<F>,
    round: usize,
    max_multiplicands: usize,
    finished: bool,
    pub num_of_variables: usize,
    polynomials_received: Vec<Vec<F>>,
}

impl<F: Field> IPForSumCheck<F>{
    pub fn verifier_init(info: &PolyInfoForVerifier) -> VerifierState<F> {
        VerifierState {
            round: 1,
            random_points: Vec::with_capacity(info.num_of_variables),
            num_of_variables: info.num_of_variables,
            max_multiplicands: info.max_num_multiplicands,
            finished: false,
            polynomials_received: Vec::with_capacity(info.num_of_variables),
        }
    }

    pub fn generate_message<R: RngCore>(
        rng: &mut R,
    ) -> VerifierMsg<F> {
        let rand_point = F::rand(rng);
        VerifierMsg {
            rand_point,
        }
    }

    pub fn verify_round<R: RngCore>(
        mut verifier_state: VerifierState<F>,
        p_msg: ProverMsg<F>,
        rng: &mut R,
    ) -> (Option<VerifierMsg<F>>, VerifierState<F>) {
        

        let verifier_msg = Self::generate_message(rng);
        verifier_state.random_points.push(verifier_msg.rand_point);
        verifier_state.polynomials_received.push(p_msg.evaluations);

        if verifier_state.round == verifier_state.num_of_variables {
            verifier_state.finished = true;
        } else {
            verifier_state.round += 1;
        }

       

        (Some(verifier_msg), verifier_state)
    }

    pub fn acceptance(
        verifier_state: VerifierState<F>,
        claimed_f: F,
    ) -> Result<bool, crate::Error> {
        let mut expected = claimed_f;


        for i in 0..verifier_state.num_of_variables {
            let evaluations = &verifier_state.polynomials_received[i];
            if evaluations.len() != verifier_state.max_multiplicands + 1 {
                panic!("incorrect number of evaluations");
            }
            let p0 = evaluations[0];
            let p1 = evaluations[1];
            if p0 + p1 != expected {
                return Err(crate::Error::Reject(Some(
                    "Prover message is not consistent with the claim.".into(),
                )));
            }
            expected = interpolate_uni_poly(evaluations, verifier_state.random_points[i]);
        }

        Ok(true)
    }
}

pub(crate) fn interpolate_uni_poly<F: Field>(p_i: &[F], eval_at: F) -> F {
    let mut result = F::zero();
    let mut i = F::zero();
    for term in p_i.iter() {
        let mut term = *term;
        let mut j = F::zero();
        for _ in 0..p_i.len() {
            if j != i {
                term = term * (eval_at - j) / (i - j)
            }
            j += F::one();
        }
        i += F::one();
        result += term;
    }

    result
}