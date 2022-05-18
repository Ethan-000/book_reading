


use ark_ff::Field;
use ark_poly::{DenseMultilinearExtension, MultilinearExtension};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Read, SerializationError, Write};

#[derive(Clone)]
pub struct PolynomialRep<F: Field> {
    pub max_num_multiplicands: usize,
    pub num_of_variables: usize,
    // the first field store the coefficient of the product of
    // polynomials that is specifed by the second field
    pub coeff_and_index: Vec<(F, Vec<usize>)>,
    pub poly_extensions: Vec<DenseMultilinearExtension<F>>,
}

#[derive(Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct PolyInfoForVerifier {
    pub num_of_variables: usize,
    pub max_num_multiplicands: usize,
}

impl<F: Field> PolynomialRep<F> {
    /// Extract the max number of multiplicands and number of variables of the list of products.
    pub fn info_for_verifier(&self) -> PolyInfoForVerifier {
        PolyInfoForVerifier {
            num_of_variables: self.num_of_variables,
            max_num_multiplicands: self.max_num_multiplicands,
        }
    }
}

impl<F: Field> PolynomialRep<F> {
    pub fn new(num_of_variables: usize) -> Self {
        PolynomialRep {
            num_of_variables: num_of_variables,
            coeff_and_index: Vec::new(),
            poly_extensions: Vec::new(),
            max_num_multiplicands: 0,
        }
    }

    pub fn evaluate(&self, point: &[F]) -> F {
        self.coeff_and_index
            .iter()
            .map(|(c, p)| {
                *c * p
                    .iter()
                    .map(|&i| self.poly_extensions[i].evaluate(point).unwrap())
                    .product::<F>()
            })
            .sum()
    }
}