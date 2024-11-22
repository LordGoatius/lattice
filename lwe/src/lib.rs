use lin_alg::matrix::Matrix;
use lin_alg::vector::Vector;

use rayon::iter::ParallelIterator;

use rand::Rng;
use rand_distr::{Distribution, Normal};

//== Random M x N Matrix ==//
pub fn rand_mod_matrix<const M: usize, const L: usize>(modulus: usize) -> Matrix<f64, M, L> {
    let mut rand = rand::thread_rng();
    let mut ret: Matrix<f64, M, L> = Matrix(
        [[const { std::mem::MaybeUninit::uninit() }; L].map(|x| unsafe { x.assume_init() }); M],
    );

    for row in 0..M {
        for col in 0..L {
            ret[row][col] = rand.gen_range(0..modulus) as f64;
        }
    }

    ret
}

//== Error Matrix ==//
pub fn generate_error_matrix<const N: usize, const L: usize>(
    q: usize,
    alpha: f64,
) -> Matrix<f64, N, L> {
    let sigma = alpha * q as f64 / f64::sqrt(2.0 * std::f64::consts::PI);

    let ret = [[0.0; L]; N];

    let discrete_normal = || {
        let mut rand = rand::thread_rng();
        Normal::new(0.0, sigma).unwrap().sample(&mut rand)
    };

    Matrix(ret.map(|row| row.map(|_| discrete_normal())))
}

//== Perturbation Vector ==//
pub fn random_perturbation_vector<const N: usize>(r: f64) -> Vector<f64, N> {
    let mut rand = rand::thread_rng();
    let vec = [0.0; N];
    Vector(vec.map(|_| rand.gen_range(-r..=r)))
}

//== UnRounding ==//
pub fn f_inv<const L: usize>(vec: Vector<f64, L>, t: usize, q: usize) -> Vector<f64, L> {
    Vector(vec.map(|val| ((t as f64 / q as f64) * val as f64).round() % t as f64))
}

//== Rounding ==//
pub fn f<const L: usize>(vec: Vector<f64, L>) -> Vector<f64, L> {
    Vector(vec.map(|val| val.round()))
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
    use std::sync::{Arc, Mutex};

    use super::*;
    use lin_alg::matrix::Matrix;
    use rayon::iter::{self, IntoParallelIterator};

    #[test]
    fn it_works() {
        const M: usize = 20;
        const N: usize = const { M + 1 };
        const L: usize = const { N + 1 };

        let to_arr = |message: String| -> [f64; L] {
            let mut ret = [0.0; L];
            for (i, char) in message.chars().take(L).enumerate() {
                ret[i] = char as usize as f64;
            }
            ret
        };

        let message: Vector<f64, L> = Vector(to_arr("this is a message".to_string()));

        const Q: usize = 200;
        const T: usize = 200;

        let secret_key: Matrix<f64, M, L> = rand_mod_matrix(Q); // Q
        let public_a: Matrix<f64, N, M> = rand_mod_matrix(Q); // T
        let error_mat: Matrix<f64, N, L> = generate_error_matrix(Q, 0.00088);
        let public_p: Matrix<f64, N, L> = (public_a * secret_key) + error_mat;

        let perturb: Vector<f64, N> = random_perturbation_vector(0.5);
        println!("Pert: {perturb:?}");
        let u = public_a.transpose() * perturb;

        let c = (public_p.transpose() * perturb) + message;
        //let c2 = c + Vector([1.0; L]);
        println!("Enc: {c:?}");

        let temp = c - (secret_key.transpose() * u);
        let decrypted = f(f_inv(temp, T, Q));
        //let decrypted2 = f(f_inv(c2 - (secret_key.transpose() * u), T, Q));

        println!("{decrypted:?}");
        println!("{message:?}");

        let to_message = |message: Vector<f64, L>| -> String {
            message
                .map(|x| char::from_u32(x.abs().trunc() as u32).unwrap())
                .iter()
                .filter(|x| !x.is_control())
                .collect()
        };

        println!("{:?}", to_message(decrypted));
        //println!("{:?}", to_message(decrypted2));
        assert_eq!(message, Vector(decrypted.map(|x| x.abs())));
    }

    #[test]
    fn test_panic() {
        let total = Arc::new(Mutex::new(Vec::new()));

        (0..usize::pow(2, 24)).into_par_iter().for_each(|_| {
            let mut total = total.lock().unwrap();
            total.push(std::panic::catch_unwind(|| it_works()));
        });

        let (worked, errors): (Vec<_>, Vec<_>) = Arc::try_unwrap(total)
            .unwrap()
            .into_inner()
            .unwrap()
            .into_iter()
            .partition(Result::is_ok);
        println!("Worked: {}", worked.len());
        println!("Errors: {}", errors.len());
    }

    #[test]
    fn rand() {
        let rand: Matrix<f64, 6, 6> = rand_mod_matrix(18);
        println!("{rand:?}");
    }
}
