use lin_alg::matrix::Matrix;
use lin_alg::vector::Vector;

use rand::Rng;
use rand_distr::{Distribution, Normal};

//== Random M x N Matrix ==//
pub fn rand_mod_matrix<const M: usize, const L: usize>(modulus: usize) -> Matrix<isize, M, L> {
    let mut rand = rand::thread_rng();
    let mut ret: Matrix<isize, M, L> = Matrix(
        [[const { std::mem::MaybeUninit::uninit() }; L].map(|x| unsafe { x.assume_init() }); M],
    );

    for row in 0..M {
        for col in 0..L {
            ret[row][col] = rand.gen_range(0..modulus) as isize;
        }
    }

    ret
}

//== Error Matrix ==//
pub fn generate_error_matrix<const N: usize, const L: usize>(
    q: usize,
    alpha: f64,
) -> Matrix<usize, N, L> {
    let sigma = alpha * q as f64 / f64::sqrt(2.0 * std::f64::consts::PI);
    println!("{sigma}");

    let ret = [[0.0; L]; N];

    let discrete_normal = || {
        let mut rand = rand::thread_rng();
        Normal::new(0.0, sigma)
            .unwrap()
            .sample(&mut rand)
            .round()
            % q as f64
    };

    Matrix(ret.map(|row| row.map(|_| discrete_normal() as usize)))
}

//== Perturbation Vector ==//
pub fn random_perturbation_vector<const N: usize>(r: isize) -> Vector<isize, N> {
    let mut rand = rand::thread_rng();
    Vector([rand.gen_range(-r..=r); N])
}

//== UnRounding ==//
pub fn f_inv<const L: usize>(vec: Vector<isize, L>, t: usize, q: usize) -> Vector<f64, L> {
    Vector(vec.map(|val| ((t as f64 / q as f64) * val as f64).round() % t as f64))
}

//== Rounding ==//
pub fn f<const L: usize>(vec: Vector<f64, L>) -> Vector<isize, L> {
    Vector(vec.map(|val| val.round() as isize))
}

//== Decrypt Function ==//
#[allow(unused)]
pub fn decrypt<const M: usize, const N: usize, const L: usize>(
    message: Vector<usize, L>,
    secret_key: Matrix<usize, M, L>,
    u: Vector<usize, M>,
) -> Vector<usize, L> {
    todo!()
}

//== Encrypt Function ==//
#[allow(unused)]
pub fn encrypt<const M: usize, const N: usize, const L: usize>(
    message: String,
) -> Vector<usize, L> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use lin_alg::matrix::Matrix;

    #[test]
    fn it_works() {
        const M: usize = 20;
        const N: usize = const { M + 1 };
        const L: usize = const { N + 1 };

        let to_arr = |message: String| -> [usize; L] {
            let mut ret = [0; L];
            for (i, char) in message.chars().take(L).enumerate() {
                ret[i] = char as usize;
            }
            ret
        };

        let message: Vector<usize, L> = Vector(to_arr("this is a message".to_string()));

        const Q: usize = 200;
        const T: usize = 200;

        let secret_key: Matrix<isize, M, L> = rand_mod_matrix(Q); // Q
        let public_a: Matrix<isize, N, M> = rand_mod_matrix(Q); // T
        let error_mat: Matrix<isize, N, L> = generate_error_matrix(Q, 0.001).to_isize();
        println!("{error_mat:?}");
        let public_p = (public_a * secret_key) + error_mat;

        let perturb: Vector<isize, N> = random_perturbation_vector(900);
        let u = public_a.transpose() * perturb;

        let c = (public_p.transpose() * perturb) + message.to_isize();

        let temp = c - (secret_key.transpose() * u);
        let temp = f_inv(temp, T, Q);
        let temp = f(temp);
        let decrypted = temp;

        println!("{decrypted:?}");
        println!("{message:?}");

        assert_eq!(decrypted, message.to_isize())
    }

    #[test]
    fn rand() {
        let rand: Matrix<usize, 6, 6> = rand_mod_matrix(18).to_usize();
        println!("{rand:?}");
    }
}
