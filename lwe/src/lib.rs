use lin_alg::matrix::Matrix;
use lin_alg::vector::Vector;

use rand::Rng;
use rand_distr::{Distribution, Normal};

//== Random M x N Matrix ==//
pub fn rand_mod_matrix<const M: usize, const L: usize>(modulus: usize) -> Matrix<usize, M, L> {
    let mut rand = rand::thread_rng();
    let mut ret: Matrix<usize, M, L> = Matrix(
        [[const { std::mem::MaybeUninit::uninit() }; L]
            .map(|x| unsafe { x.assume_init() }); M],
    );

    for row in 0..M {
        for col in 0..L {
            ret[row][col] = rand.gen_range(0..modulus);
        }
    }

    ret
}

//== Error Matrix ==//
pub fn generate_error_matrix<const N: usize, const L: usize>(q: usize, alpha: f64) -> Matrix<usize, N, L> {
    let sigma = alpha * q as f64 / f64::sqrt(2.0 * std::f64::consts::PI);

    let discrete_normal = || Normal::new(0.0, sigma).unwrap().sample(&mut rand::thread_rng()).round() as usize % q;

    Matrix([[discrete_normal(); L]; N])
}

//== UnRounding ==//
pub fn f_inv<const L: usize>(vec: Vector<usize, L>, t: usize, q: usize) -> Vector<f64, L> {
    Vector(vec.map(|val| ((t as f64 / q as f64) * val as f64).round() % t as f64))
}

//== Rounding ==//
pub fn f<const L: usize>(vec: Vector<f64, L>) -> Vector<usize, L> {
    Vector(vec.map(|val| val.round() as usize))
}

//== Decrypt Function ==//
pub fn decrypt<const M: usize, const N: usize, const L: usize>(message: Vector<usize, L>, secret_key: Matrix<usize, M, L>, u: Vector<usize, M>) -> Vector<usize, L> {
    let temp = message - (secret_key.transpose() * u);
    return f(f_inv(temp, 20, 20));
}

//== Encrypt Function ==//
pub fn encrypt<const M: usize, const N: usize, const L: usize>(message: String) -> Vector<usize, L> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use lin_alg::matrix::Matrix;

    #[test]
    fn it_works() {

    }

    #[test]
    fn rand() {
        let rand: Matrix<usize, 6, 6> = rand_mod_matrix(18);
        println!("{rand:?}");
    }
}
