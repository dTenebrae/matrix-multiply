use std::time::Instant;

use rand::prelude::*;
use std::cmp::min;

const N: usize = 2000;
const D: usize = 50;

fn generate_matrix(is_random: bool) -> Vec<u32> {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut arr: Vec<u32> = vec![0u32; N * N];
    if is_random {
        for i in 0..N {
            for j in 0..N {
                arr[i * N + j] = rng.gen_range(0..10);
            }
        }
    }
    arr
}

fn naive_multiply(arr1: &[u32], arr2: &[u32], res_arr: &mut [u32]) {
    for i in 0..N {
        for j in 0..N {
            let mut sum = 0;
            for k in 0..N {
                unsafe {
                    sum += (*arr1.get_unchecked(i * N + k)) * (*arr2.get_unchecked(k * N + j));
                }
            }
            res_arr[i * N + j] = sum;
        }
    }
}

fn naive_iter_mult(arr1: &[u32], arr2: &[u32], res_arr: &mut [u32]) {
    for (ci, ai) in res_arr.chunks_exact_mut(N).zip(arr1.chunks_exact(N)) {
        for (aik, bk) in ai.iter().zip(arr2.chunks_exact(N)) {
            for (cij, bkj) in ci.iter_mut().zip(bk.iter()) {
                *cij += (*aik) * (*bkj);
            }
        }
    }
}

fn opt_multiply(arr1: &[u32], arr2: &[u32], res_arr: &mut [u32]) {
    for o in 0..=(N / D) {
        for i in 0..N {
            for j in 0..N {
                let mut sum: u32 = 0;
                for k in o * D..min(N, (o + 1) * D) {
                    unsafe {
                        sum += *arr1.get_unchecked(i * N + k) * *arr2.get_unchecked(k * N + j)
                    }
                }
                res_arr[i * N + j] = sum;
            }
        }
    }
}

fn test_func<F>(mut mul_func: F, arr1: &[u32], arr2: &[u32], res_arr: &mut [u32], f_name: &str)
where
    F: FnMut(&[u32], &[u32], &mut [u32]),
{
    let start = Instant::now();
    mul_func(arr1, arr2, res_arr);
    let duration = start.elapsed();
    println!("Time elapsed in {}() is: {:?}", f_name, duration);
}

fn main() {
    let left_mtx = generate_matrix(true);
    let right_mtx = generate_matrix(true);

    let mut output_mtx1 = generate_matrix(false);
    let mut output_mtx2 = output_mtx1.clone();
    let mut output_mtx3 = output_mtx1.clone();

    test_func(
        naive_multiply,
        &left_mtx,
        &right_mtx,
        &mut output_mtx1,
        "naive_multiply",
    );
    test_func(
        naive_iter_mult,
        &left_mtx,
        &right_mtx,
        &mut output_mtx2,
        "naive_iter_mult",
    );
    test_func(
        opt_multiply,
        &left_mtx,
        &right_mtx,
        &mut output_mtx3,
        "opt_multiply",
    );
}
